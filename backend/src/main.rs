// ============================================================
// main.rs - チーム決め一発くん バックエンドサーバー（修正版 v2）
//
// 【このファイルの概要】
// Apex Legendsのプレイヤー管理とチーム均衡化ロジックを提供するAPIサーバー。
// Axumフレームワークを使い、以下のREST APIを公開する:
//
//   GET    /api/players             - 全プレイヤー取得
//   POST   /api/players             - プレイヤー追加
//   PUT    /api/players/:id         - プレイヤー更新
//   DELETE /api/players/:id         - プレイヤー削除
//   GET    /api/config              - 設定取得（チーム数など）
//   PUT    /api/config              - 設定更新
//   POST   /api/teams/balance       - チーム分け実行
//   GET    /api/teams               - チーム分け結果取得
//   GET    /health                  - ヘルスチェック（Renderの起動確認用）
//   GET    /**                      - Vueアプリの静的ファイル配信（SPA対応）
//
// ────────────────────────────────────────────────────────────
// 【Renderデプロイ時の重要ポイント】
//
//   Renderは起動時に PORT 環境変数を自動的にセットする。
//   このコードは std::env::var("PORT") でその値を読み取り、
//   0.0.0.0:PORT でリッスンする。
//
//   0.0.0.0 = すべてのネットワークインターフェースで受け付ける（必須）
//   127.0.0.1 にバインドすると外部から繋がらない！
//
// 状態管理は Arc<RwLock<AppState>> でスレッドセーフに行う。
// RwLock は Mutex と違い、読み取りを同時複数スレッドに許可するため
// 読み取り頻度が高いAPIに適している。
// ============================================================

use axum::{
    extract::{Path, State}, // URLパスパラメータとアプリケーション状態
    http::{Method, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock}, // Mutexより読み取り並行性が高いRwLockを使用
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile}, // ServeDir=ディレクトリ配信, ServeFile=単一ファイル配信
};
use uuid::Uuid;

// ============================================================
// データ構造の定義
// ============================================================

// ───────────────────────────────────────────────────────────
// Rank: プレイヤーのランク列挙型
//
// serde(rename = "...") で JSON との変換時に日本語文字列を使う。
// フロントエンドのドロップダウン値と完全に一致させること。
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rank {
    #[serde(rename = "ブロンズ")]
    Bronze,
    #[serde(rename = "シルバー")]
    Silver,
    #[serde(rename = "ゴールド")]
    Gold,
    #[serde(rename = "プラチナ")]
    Platinum,
    #[serde(rename = "ダイヤモンド")]
    Diamond,
    #[serde(rename = "マスター")]
    Master,
    #[serde(rename = "プレデター")]
    Predator,
}

impl Rank {
    // ─────────────────────────────────────────────────────
    // score() - ランクの「戦力スコア」を返す
    //
    // 【設計の意図】
    // 上位ランク（マスター・プレデター）のスコアを意図的に
    // 非線形に大きくすることで、貪欲法によるチーム分けが
    // これらの強いプレイヤーを自動的に分散させる効果がある。
    //
    //   ブロンズ(1) シルバー(2) ゴールド(3) プラチナ(5)
    //   ダイヤモンド(7) マスター(10) プレデター(15)
    // ─────────────────────────────────────────────────────
    pub fn score(&self) -> u32 {
        match self {
            Rank::Bronze   => 1,
            Rank::Silver   => 2,
            Rank::Gold     => 3,
            Rank::Platinum => 5,
            Rank::Diamond  => 7,
            Rank::Master   => 10,
            Rank::Predator => 15,
        }
    }
}

// ───────────────────────────────────────────────────────────
// Player: プレイヤー情報の構造体
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// バックエンドが生成したUUID（変更不可の一意識別子）
    pub id: String,
    /// プレイヤー名（入力値。重複可）
    pub name: String,
    /// ランク
    pub rank: Rank,
    /// チーム分け後に重複名があれば "名前(1)","名前(2)" 形式でセットされる
    /// 通常時は None → フロント側で name をそのまま表示する
    pub display_name: Option<String>,
}

// ───────────────────────────────────────────────────────────
// Team: チーム情報の構造体
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// チーム番号（1始まり）
    pub id: usize,
    /// このチームのプレイヤーリスト
    pub players: Vec<Player>,
    /// チームの合計スコア（均衡度の指標）
    pub total_score: u32,
}

// ───────────────────────────────────────────────────────────
// AppConfig: アプリ設定
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 作るチームの数（デフォルト: 2、管理者が変更可能）
    pub team_count: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { team_count: 2 }
    }
}

// ───────────────────────────────────────────────────────────
// AppState: サーバーが保持するすべての状態（インメモリDB）
//
// 【なぜ Arc<RwLock<AppState>> を使うのか】
//
//   Axumは複数スレッドでリクエストを並行処理する。
//   そのため状態を共有するにはスレッドセーフな仕組みが必要。
//
//   Arc   = Atomic Reference Counted ポインタ
//           複数スレッドから同じデータを「共同所有」できる
//
//   RwLock = Read-Write Lock（読み書きロック）
//           ・読み取り（read_lock）: 複数スレッドが同時にOK
//           ・書き込み（write_lock）: 1スレッドのみ、他は待つ
//
//   読み取りAPIが多く書き込みAPIが少ない本アプリに最適。
//   Mutexは読み取りも排他なのでRwLockの方が効率が良い。
// ───────────────────────────────────────────────────────────
#[derive(Debug, Default)]
pub struct AppState {
    /// 登録済みプレイヤーリスト
    pub players: Vec<Player>,
    /// 最後のチーム分け結果（未実行なら None）
    pub teams: Option<Vec<Team>>,
    /// アプリ設定
    pub config: AppConfig,
}

// ── 型エイリアス（毎回 Arc<RwLock<AppState>> と書くのが大変なので）──
type SharedState = Arc<RwLock<AppState>>;

// ============================================================
// APIリクエスト/レスポンス型定義
// ============================================================

#[derive(Debug, Deserialize)]
pub struct AddPlayerRequest {
    pub name: String,
    pub rank: Rank,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlayerRequest {
    pub name: String,
    pub rank: Rank,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub team_count: usize,
}

#[derive(Debug, Serialize)]
pub struct BalanceResult {
    pub teams: Vec<Team>,
    /// 重複名解決済みプレイヤーリスト（フロント側の名前表示更新用）
    pub players: Vec<Player>,
}

// ============================================================
// チーム均衡化ロジック
// ============================================================

// ───────────────────────────────────────────────────────────
// resolve_duplicate_names - 重複名に連番サフィックスを付ける
//
// 例: ["あかし", "たろう", "あかし"] → ["あかし(1)", "たろう", "あかし(2)"]
//
// チーム分け実行時のみ呼び出す。
// （プレイヤー登録時は元の name を保持する）
// ───────────────────────────────────────────────────────────
fn resolve_duplicate_names(players: &mut Vec<Player>) {
    // 各名前の出現回数をカウント
    let mut name_count: HashMap<String, usize> = HashMap::new();
    for p in players.iter() {
        *name_count.entry(p.name.clone()).or_insert(0) += 1;
    }

    // 重複する名前にインデックスを付けるカウンター
    let mut name_idx: HashMap<String, usize> = HashMap::new();

    for p in players.iter_mut() {
        if name_count[&p.name] > 1 {
            let idx = name_idx.entry(p.name.clone()).or_insert(0);
            *idx += 1;
            p.display_name = Some(format!("{}({})", p.name, idx));
        } else {
            p.display_name = None;
        }
    }
}

// ───────────────────────────────────────────────────────────
// balance_teams - 戦力均衡化チーム分けアルゴリズム
//
// 【アルゴリズム: Greedy Number Partitioning（貪欲法）】
//
// 「複数の袋に重さのある石を入れる」問題に相当する古典的アルゴリズム。
// 最適解ではないが、実用上十分に均衡したチームが作れる。
//
// 手順:
//   1. プレイヤーをスコア降順（強い順）でソート
//      → プレデター(15)が最初に処理されるため、チームへの影響が大きい
//        プレイヤーが先に各チームに分散される
//
//   2. 各プレイヤーを「現在合計スコアが最小のチーム」に追加
//      → チームのスコア差が縮まる方向に自然に振り分けられる
//
// 【例】プレデター+マスター+マスター+ゴールド を 2チームに分ける
//   Step1: プレデター(15) → [チームA: 15] [チームB:  0]
//   Step2: マスター(10)  → [チームA: 15] [チームB: 10]  ← Bが最小なので
//   Step3: マスター(10)  → [チームA: 25] [チームB: 10]  ← Aが最小なので? いや違う
//          実際は → [チームA: 15] [チームB: 20]  ← Aが最小なので
//   Step4: ゴールド(3)   → [チームA: 18] [チームB: 20]
//   結果: A=18, B=20、差=2（非常に均衡）
// ───────────────────────────────────────────────────────────
fn balance_teams(players: &[Player], team_count: usize) -> Vec<Team> {
    // 安全弁: チーム数は最低1、最大はプレイヤー数
    let team_count = team_count.max(1).min(players.len().max(1));

    if players.is_empty() {
        return (1..=team_count)
            .map(|i| Team { id: i, players: vec![], total_score: 0 })
            .collect();
    }

    // ── Step 1: スコア降順でソート ──
    let mut sorted: Vec<Player> = players.to_vec();
    sorted.sort_by(|a, b| b.rank.score().cmp(&a.rank.score()));

    // ── Step 2: チームの合計スコアと所属プレイヤーを管理 ──
    let mut team_scores: Vec<u32> = vec![0; team_count];
    let mut team_players: Vec<Vec<Player>> = vec![vec![]; team_count];

    // ── Step 3: 各プレイヤーを最小スコアのチームに割り当て ──
    for player in sorted {
        // 最も合計スコアが低いチームのインデックスを取得
        let min_idx = team_scores
            .iter()
            .enumerate()
            .min_by_key(|(_, &s)| s)
            .map(|(i, _)| i)
            .unwrap_or(0);

        team_scores[min_idx] += player.rank.score();
        team_players[min_idx].push(player);
    }

    // ── Step 4: Team 構造体に変換して返す ──
    team_players
        .into_iter()
        .enumerate()
        .map(|(i, players)| {
            let total_score = players.iter().map(|p| p.rank.score()).sum();
            Team { id: i + 1, players, total_score }
        })
        .collect()
}

// ============================================================
// APIハンドラー関数群
//
// 【RwLockの使い方】
//   読み取り: state.read().unwrap()  ← 複数スレッド同時OK
//   書き込み: state.write().unwrap() ← 排他ロック
// ============================================================

/// GET /health - ヘルスチェック（Renderが起動確認に使う）
///
/// このエンドポイントが 200 OK を返さないと Render はデプロイ失敗と判断する。
/// 処理は極めてシンプル（DBアクセスなし）で素早く応答できるようにしてある。
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

/// GET /api/players - 全プレイヤー取得
async fn get_players(State(state): State<SharedState>) -> impl IntoResponse {
    // 読み取りロック（複数スレッドが同時に取得可能）
    let state = state.read().unwrap();
    Json(serde_json::json!({ "players": state.players }))
}

/// POST /api/players - プレイヤーを新規追加
async fn add_player(
    State(state): State<SharedState>,
    Json(req): Json<AddPlayerRequest>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap(); // 書き込みロック

    let new_player = Player {
        id: Uuid::new_v4().to_string(),
        name: req.name.trim().to_string(),
        rank: req.rank,
        display_name: None,
    };

    let added = new_player.clone();
    state.players.push(new_player);
    state.teams = None; // プレイヤー変更で既存チーム結果は無効化

    (StatusCode::CREATED, Json(serde_json::json!({ "player": added })))
}

/// PUT /api/players/:id - プレイヤー情報を更新
async fn update_player(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePlayerRequest>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();

    if let Some(player) = state.players.iter_mut().find(|p| p.id == id) {
        player.name = req.name.trim().to_string();
        player.rank = req.rank;
        player.display_name = None;

        let updated = player.clone();
        state.teams = None;
        return (
            StatusCode::OK,
            Json(serde_json::json!({ "player": updated })),
        );
    }

    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": "プレイヤーが見つかりません" })),
    )
}

/// DELETE /api/players/:id - プレイヤーを削除
async fn delete_player(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();

    let before = state.players.len();
    state.players.retain(|p| p.id != id);

    if state.players.len() < before {
        state.teams = None;
        StatusCode::NO_CONTENT.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "プレイヤーが見つかりません" })),
        )
            .into_response()
    }
}

/// GET /api/config - 設定取得
async fn get_config(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    Json(serde_json::json!({ "config": state.config }))
}

/// PUT /api/config - 設定更新（チーム数変更）
async fn update_config(
    State(state): State<SharedState>,
    Json(req): Json<UpdateConfigRequest>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();

    // チーム数の最小値は 2
    state.config.team_count = req.team_count.max(2);
    state.teams = None;

    Json(serde_json::json!({ "config": state.config }))
}

/// POST /api/teams/balance - チーム分けを実行する（管理者専用）
///
/// このエンドポイントが本アプリの核心。
/// 1. 重複プレイヤー名を解決
/// 2. 均衡化アルゴリズムでチームに振り分け
/// 3. 結果を保存してレスポンスで返す
async fn balance_teams_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let mut state = state.write().unwrap();

    if state.players.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤーが登録されていません" })),
        )
            .into_response();
    }

    // チーム数はプレイヤー数を超えないように制限
    let team_count = state.config.team_count.min(state.players.len());

    // 重複名解決（元のプレイヤーリストも更新する）
    let mut resolved = state.players.clone();
    resolve_duplicate_names(&mut resolved);

    // 解決した display_name を元のリストに反映
    for r in &resolved {
        if let Some(orig) = state.players.iter_mut().find(|p| p.id == r.id) {
            orig.display_name = r.display_name.clone();
        }
    }

    // 均衡化アルゴリズム実行
    let teams = balance_teams(&resolved, team_count);
    state.teams = Some(teams.clone());

    Json(serde_json::json!(BalanceResult { teams, players: resolved })).into_response()
}

/// GET /api/teams - 最後のチーム分け結果を返す
async fn get_teams(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    match &state.teams {
        Some(teams) => Json(serde_json::json!({ "teams": teams })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "チーム分けがまだ実行されていません" })),
        )
            .into_response(),
    }
}

// ============================================================
// メイン関数 - サーバーの起動
// ============================================================
#[tokio::main]
async fn main() {
    // ── ログ初期化 ──
    // RUST_LOG=info（または debug）を環境変数でコントロールできる
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    // ────────────────────────────────────────────────────────
    // ポート番号の決定
    //
    // 【Renderでの動作】
    //   Renderは起動時に PORT 環境変数を自動的にセットする。
    //   本コードはその値を読み取る。PORT が未設定の場合（ローカル開発時）は
    //   デフォルト値 8080 を使用する。
    //
    // 【重要】
    //   Renderが指定するポートは 8080 とは限らない。
    //   ハードコードすると "Timed out" エラーの原因になる。
    // ────────────────────────────────────────────────────────
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    // ── 共有状態の初期化 ──
    let shared_state: SharedState = Arc::new(RwLock::new(AppState::default()));

    // ── CORS設定 ──
    // 開発中は Vite(localhost:5173) から API を呼べるように許可する
    // 本番（Docker）では同じオリジンなので実質不要だが、開発利便性のために残す
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    // ── APIルーター ──
    let api_router = Router::new()
        .route("/players",       get(get_players).post(add_player))
        .route("/players/:id",   put(update_player).delete(delete_player))
        .route("/config",        get(get_config).put(update_config))
        .route("/teams",         get(get_teams))
        .route("/teams/balance", post(balance_teams_handler));

    // ────────────────────────────────────────────────────────
    // 静的ファイル配信の設定（SPA対応）
    //
    // ServeDir::new("static") = ./static/ ディレクトリ以下のファイルを配信
    //
    // .not_found_service(ServeFile::new("static/index.html"))
    //   → 存在しないパス（/owner, /user など Vue Router のルート）へのアクセスに対し
    //     index.html を返す。これがSPA(シングルページアプリ)の動作に必須の設定。
    //     これがないと /owner にブラウザで直接アクセスすると 404 になる。
    //
    // Docker コンテナ内のディレクトリ構造:
    //   /app/
    //   ├── apex-team-balancer  ← Rustバイナリ（起動ディレクトリ = /app）
    //   └── static/             ← Vueのビルド成果物
    //       ├── index.html
    //       └── assets/
    //           ├── index-xxxx.js
    //           └── index-xxxx.css
    // ────────────────────────────────────────────────────────
    let spa_service = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    // ── メインルーター組み立て ──
    // 優先度（上から順に試みる）:
    //   1. GET /health             → ヘルスチェック
    //   2. /api/**                 → REST API
    //   3. それ以外               → 静的ファイル or SPA fallback (index.html)
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_router)
        // fallback_service: 上記ルートにマッチしないすべてのリクエストを処理
        .fallback_service(spa_service)
        .layer(cors)
        .with_state(shared_state);

    // ────────────────────────────────────────────────────────
    // サーバー起動
    //
    // "0.0.0.0" = すべてのネットワークインターフェースでリッスン（必須）
    // "127.0.0.1" では Docker コンテナ外（= Render のロードバランサー）から
    // 届かないため、必ず "0.0.0.0" を使うこと！
    // ────────────────────────────────────────────────────────
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🚀 起動完了: http://{}", addr);
    tracing::info!("   ヘルスチェック: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("ポートのバインドに失敗しました。PORT環境変数を確認してください");

    axum::serve(listener, app)
        .await
        .expect("サーバーの起動に失敗しました");
}

// ============================================================
// テスト
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_two_teams() {
        let players = vec![
            Player { id: "1".to_string(), name: "A".to_string(), rank: Rank::Predator, display_name: None },
            Player { id: "2".to_string(), name: "B".to_string(), rank: Rank::Master,   display_name: None },
            Player { id: "3".to_string(), name: "C".to_string(), rank: Rank::Gold,     display_name: None },
            Player { id: "4".to_string(), name: "D".to_string(), rank: Rank::Bronze,   display_name: None },
        ];
        let teams = balance_teams(&players, 2);

        assert_eq!(teams.len(), 2);
        let total: usize = teams.iter().map(|t| t.players.len()).sum();
        assert_eq!(total, 4);

        // スコア差が合計スコアの半分以下であること
        let diff = (teams[0].total_score as i32 - teams[1].total_score as i32).abs();
        let total_score: u32 = teams.iter().map(|t| t.total_score).sum();
        assert!(diff <= total_score as i32 / 2);
    }

    #[test]
    fn test_resolve_duplicates() {
        let mut players = vec![
            Player { id: "1".to_string(), name: "あかし".to_string(), rank: Rank::Gold,   display_name: None },
            Player { id: "2".to_string(), name: "たろう".to_string(), rank: Rank::Silver, display_name: None },
            Player { id: "3".to_string(), name: "あかし".to_string(), rank: Rank::Bronze, display_name: None },
        ];
        resolve_duplicate_names(&mut players);

        assert_eq!(players[0].display_name, Some("あかし(1)".to_string()));
        assert_eq!(players[1].display_name, None); // たろうは重複なし
        assert_eq!(players[2].display_name, Some("あかし(2)".to_string()));
    }

    #[test]
    fn test_rank_scores_ordered() {
        // 各ランクのスコアが昇順になっているか確認
        assert!(Rank::Bronze.score() < Rank::Silver.score());
        assert!(Rank::Silver.score() < Rank::Gold.score());
        assert!(Rank::Gold.score() < Rank::Platinum.score());
        assert!(Rank::Platinum.score() < Rank::Diamond.score());
        assert!(Rank::Diamond.score() < Rank::Master.score());
        assert!(Rank::Master.score() < Rank::Predator.score());
    }
}
