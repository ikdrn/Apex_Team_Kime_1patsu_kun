// ============================================================
// main.rs - チーム決め一発くん バックエンドサーバー
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
//   GET    /api/teams               - 最後に実行したチーム分け結果取得
//   GET    /health                  - ヘルスチェック（Renderのデプロイ確認用）
//   GET    /                        - Vueアプリの静的ファイル配信（本番用）
//
// 状態管理はデータベースを使わず、メモリ上の Arc<Mutex<AppState>> で行う。
// サーバーを再起動するとデータはリセットされる点に注意。
// ============================================================

// 使用するモジュールのインポート
// `use` はRustにおけるimport文に相当する
use axum::{
    extract::{Path, State},   // URLパスパラメータとアプリケーション状態を取り出す
    http::{Method, StatusCode, Uri}, // HTTPメソッド・ステータスコード・URI型
    response::{IntoResponse, Json},  // レスポンス生成ユーティリティ
    routing::{delete, get, post, put}, // HTTPメソッドに対応するルーティング関数
    Router,                            // ルーター本体
};
use serde::{Deserialize, Serialize}; // JSON変換用マクロ
use std::{
    collections::HashMap, // キーと値のペアを管理するハッシュマップ
    sync::{Arc, Mutex},   // スレッド間でデータを安全に共有するためのスマートポインタ
};
use tower_http::{
    cors::{Any, CorsLayer}, // CORS（クロスオリジン）設定
    services::ServeDir,     // 静的ファイル配信
};
use uuid::Uuid; // ユニークID生成

// ============================================================
// データ構造の定義
// ============================================================

// ───────────────────────────────────────────────────────────
// Rank: プレイヤーのランク列挙型
//
// Apex Legendsの公式ランク体系に対応。
// derive マクロで以下の機能を自動実装する:
//   - Debug: デバッグ表示（println!("{:?}", rank) で使える）
//   - Clone: .clone() でコピーを作れる
//   - Serialize/Deserialize: JSONとの相互変換
//   - PartialEq: == 演算子での比較
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
    // score() - ランクの「戦力スコア」を返すメソッド
    //
    // 【なぜこのスコアが必要か】
    // チーム分けを公平にするため、各ランクに数値を割り当てる。
    // この数値の大小でプレイヤーの強さを表現し、
    // 各チームの合計スコアが均等になるようにアルゴリズムで利用する。
    //
    // スコア設計の考え方:
    //   ブロンズ〜ダイヤモンドは線形（1〜5）
    //   マスターとプレデターは上位ランクの希少性を考慮して
    //   非線形に大きいスコアを設定することで、
    //   強いプレイヤーが同じチームに集まりにくくなる
    // ─────────────────────────────────────────────────────
    pub fn score(&self) -> u32 {
        match self {
            Rank::Bronze   => 1,  // 最初のランク
            Rank::Silver   => 2,
            Rank::Gold     => 3,
            Rank::Platinum => 5,  // プラチナから一段階差をつける
            Rank::Diamond  => 7,
            Rank::Master   => 10, // 上位ランクは大きく差をつける
            Rank::Predator => 15, // 最上位は圧倒的に高いスコア
        }
    }
}

// ───────────────────────────────────────────────────────────
// Player: プレイヤー情報を表す構造体
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// プレイヤーを一意に識別するUUID（例: "550e8400-e29b-41d4-a716-446655440000"）
    pub id: String,
    /// プレイヤー名（画面入力値）
    pub name: String,
    /// ランク（上記のRank列挙型）
    pub rank: Rank,
    /// チーム分け実行後に重複名があった場合の表示名
    /// 例: "あかし" が2人いたら "あかし(1)", "あかし(2)"
    /// 通常時は None（表示名は name と同じ）
    pub display_name: Option<String>,
}

impl Player {
    // ─────────────────────────────────────────────────────
    // display() - 実際に画面に表示すべき名前を返す
    //
    // display_name が設定されていればそちらを返し、
    // なければ通常の name を返す便利メソッド
    // ─────────────────────────────────────────────────────
    pub fn display(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name)
    }
}

// ───────────────────────────────────────────────────────────
// Team: チームを表す構造体
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// チーム番号（1始まり）
    pub id: usize,
    /// このチームに属するプレイヤーのリスト
    pub players: Vec<Player>,
    /// チームの合計スコア（均衡化の指標として利用）
    pub total_score: u32,
}

// ───────────────────────────────────────────────────────────
// AppConfig: アプリケーション設定
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 分けるチームの数（デフォルト: 2）
    pub team_count: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { team_count: 2 }
    }
}

// ───────────────────────────────────────────────────────────
// AppState: サーバーが保持するすべての状態
//
// Arc<Mutex<AppState>> でラップして複数スレッドから安全にアクセスする。
// - Arc  = 参照カウント付きスマートポインタ（複数の所有者を持てる）
// - Mutex = 同時アクセスを防ぐロック機構
// ───────────────────────────────────────────────────────────
#[derive(Debug)]
pub struct AppState {
    /// 現在登録されているプレイヤーのリスト
    pub players: Vec<Player>,
    /// 最後に実行したチーム分けの結果（まだ実行していなければNone）
    pub teams: Option<Vec<Team>>,
    /// アプリケーション設定
    pub config: AppConfig,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            teams: None,
            config: AppConfig::default(),
        }
    }
}

// ============================================================
// APIリクエスト/レスポンスの型定義
// ============================================================

/// プレイヤー追加リクエストのボディ
#[derive(Debug, Deserialize)]
pub struct AddPlayerRequest {
    pub name: String,
    pub rank: Rank,
}

/// プレイヤー更新リクエストのボディ
#[derive(Debug, Deserialize)]
pub struct UpdatePlayerRequest {
    pub name: String,
    pub rank: Rank,
}

/// 設定更新リクエストのボディ
#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub team_count: usize,
}

/// チーム分け実行結果のレスポンス
#[derive(Debug, Serialize)]
pub struct BalanceResult {
    /// 分けられたチームのリスト
    pub teams: Vec<Team>,
    /// 重複名解決済みのプレイヤーリスト（画面側で名前を更新するために返す）
    pub players: Vec<Player>,
}

// ============================================================
// チーム均衡化アルゴリズム
// ============================================================

// ───────────────────────────────────────────────────────────
// resolve_duplicate_names - 重複した名前にサフィックスを付ける
//
// 【動作例】
//   入力: ["あかし", "たろう", "あかし", "じろう", "たろう", "たろう"]
//   出力: ["あかし(1)", "たろう(1)", "あかし(2)", "じろう", "たろう(2)", "たろう(3)"]
//
// 【なぜ必要か】
//   同じ名前のプレイヤーが複数いる場合、チーム表示で誰が誰か分からなくなる。
//   自動的に番号を付けることで視覚的に区別できるようにする。
// ───────────────────────────────────────────────────────────
fn resolve_duplicate_names(players: &mut Vec<Player>) {
    // まず各名前が何回出現するかカウントする
    let mut name_count: HashMap<String, usize> = HashMap::new();
    for player in players.iter() {
        *name_count.entry(player.name.clone()).or_insert(0) += 1;
    }

    // 重複する名前にインデックスを付けるためのカウンター
    let mut name_index: HashMap<String, usize> = HashMap::new();

    for player in players.iter_mut() {
        if name_count[&player.name] > 1 {
            // 同名プレイヤーが2人以上いる場合は (1), (2) ... を付ける
            let idx = name_index.entry(player.name.clone()).or_insert(0);
            *idx += 1;
            player.display_name = Some(format!("{}({})", player.name, idx));
        } else {
            // 名前が一意の場合は display_name をリセット
            player.display_name = None;
        }
    }
}

// ───────────────────────────────────────────────────────────
// balance_teams - チーム均衡化のコアアルゴリズム
//
// 【アルゴリズムの概要】
// "Greedy Number Partitioning"（貪欲法による数値分割）を応用。
//
// 手順:
//   1. プレイヤーをスコア降順（強い順）でソートする
//      → 強いプレイヤーから割り当てることで、チーム間の偏りを最小化する
//
//   2. 各プレイヤーを「現在の合計スコアが最も低いチーム」に追加する
//      → 最小ヒープ的な発想。常に最も弱いチームに強いプレイヤーを補充する
//
// 【なぜこの方法が有効か】
//   例: プレデター(15), マスター(10), マスター(10), ゴールド(3) を2チームに分ける場合
//
//   Step1: プレデター(15) → チームA [15]  チームB [0]   ← Bが最低
//   Step2: マスター(10)  → チームB [10]  チームA [15]  ← Aが最低
//   Step3: マスター(10)  → チームA [25]  チームB [10]  ← Bが最低
//   Step4: ゴールド(3)   → チームB [13]  チームA [25]  ← ...
//   結果: A=25, B=13 → 差12
//
//   もしランダムに分けると: A=[15+10]=25, B=[10+3]=13 or A=[15+3]=18, B=[10+10]=20
//   貪欲法は最適に近い分け方を自動で見つける
//
// 【パラメータ】
//   players    - 分けるプレイヤーのリスト（このリスト自体は変更しない）
//   team_count - 作るチームの数
//
// 【戻り値】
//   Vec<Team> - 均衡化されたチームのリスト
// ───────────────────────────────────────────────────────────
fn balance_teams(players: &[Player], team_count: usize) -> Vec<Team> {
    // チーム数が0や1の場合のガード処理（異常値が来ても壊れないように）
    let team_count = team_count.max(1);

    // プレイヤーが0人の場合は空のチームリストを返す
    if players.is_empty() {
        return (1..=team_count)
            .map(|i| Team {
                id: i,
                players: Vec::new(),
                total_score: 0,
            })
            .collect();
    }

    // ── Step 1: プレイヤーをスコア降順（強い順）でソート ──
    // players は &[Player]（不変参照）なので、一度クローンしてから並べ替える
    let mut sorted_players: Vec<Player> = players.to_vec();
    sorted_players.sort_by(|a, b| {
        // スコアで降順比較。同スコアの場合は名前でアルファベット順（安定ソート）
        b.rank.score().cmp(&a.rank.score())
    });

    // ── Step 2: チームの合計スコアと所属プレイヤーを追跡する構造を初期化 ──
    // team_scores[i] = チーム(i+1)の現在の合計スコア
    let mut team_scores: Vec<u32> = vec![0; team_count];
    // team_players[i] = チーム(i+1)に割り当てられたプレイヤーのリスト
    let mut team_players: Vec<Vec<Player>> = vec![Vec::new(); team_count];

    // ── Step 3: 各プレイヤーを最もスコアが低いチームに割り当て ──
    for player in sorted_players {
        // 現在スコアが最も低いチームのインデックスを探す
        // enumerate() でインデックスと値のペアにしてから min_by_key で最小を取る
        let min_team_idx = team_scores
            .iter()
            .enumerate()
            .min_by_key(|(_, &score)| score)
            .map(|(idx, _)| idx)
            .unwrap_or(0); // チームが存在しない場合の安全弁（実際には起こらない）

        // そのチームにプレイヤーを追加し、合計スコアを更新
        let player_score = player.rank.score();
        team_players[min_team_idx].push(player);
        team_scores[min_team_idx] += player_score;
    }

    // ── Step 4: 結果を Team 構造体にまとめて返す ──
    team_players
        .into_iter()
        .enumerate()
        .map(|(i, players)| {
            let total_score = players.iter().map(|p| p.rank.score()).sum();
            Team {
                id: i + 1, // チーム番号は1始まり
                players,
                total_score,
            }
        })
        .collect()
}

// ============================================================
// APIハンドラー関数
// ============================================================
// 各関数はHTTPリクエストを受け取り、JSONレスポンスを返す。
// State(state) でサーバーの共有状態にアクセスできる。
// ============================================================

/// GET /api/players - 全プレイヤーを返す
async fn get_players(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    // Mutex をロックして状態にアクセス
    // lock() が失敗するのは別スレッドがパニックした場合のみ（実用上ほぼ起きない）
    let state = state.lock().unwrap();
    Json(serde_json::json!({
        "players": state.players
    }))
}

/// POST /api/players - プレイヤーを追加する
async fn add_player(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(req): Json<AddPlayerRequest>, // リクエストボディをJSONから自動変換
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // 新しいプレイヤーを作成（IDはUUIDv4でランダム生成）
    let new_player = Player {
        id: Uuid::new_v4().to_string(),
        name: req.name.trim().to_string(), // 前後の空白を除去
        rank: req.rank,
        display_name: None,
    };

    // 追加したプレイヤーをクローン（後でレスポンスに使うため）
    let added = new_player.clone();
    state.players.push(new_player);

    // チーム分け結果をリセット（プレイヤーが変わったので古い結果は無効）
    state.teams = None;

    // 201 Created ステータスと追加されたプレイヤーを返す
    (StatusCode::CREATED, Json(serde_json::json!({ "player": added })))
}

/// PUT /api/players/:id - 指定IDのプレイヤーを更新する
async fn update_player(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<String>, // URLの :id 部分を取り出す
    Json(req): Json<UpdatePlayerRequest>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // IDでプレイヤーを検索し、見つかれば更新する
    if let Some(player) = state.players.iter_mut().find(|p| p.id == id) {
        player.name = req.name.trim().to_string();
        player.rank = req.rank;
        player.display_name = None; // 更新時は表示名をリセット

        let updated = player.clone();
        state.teams = None; // チーム結果もリセット
        return (StatusCode::OK, Json(serde_json::json!({ "player": updated })));
    }

    // 見つからなかった場合は404エラー
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": "プレイヤーが見つかりません" })),
    )
}

/// DELETE /api/players/:id - 指定IDのプレイヤーを削除する
async fn delete_player(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // retain() は条件が true の要素だけ残す（falseの要素を削除するフィルタ）
    let before_len = state.players.len();
    state.players.retain(|p| p.id != id);
    let after_len = state.players.len();

    if before_len != after_len {
        // 実際に削除された場合
        state.teams = None; // チーム結果をリセット
        StatusCode::NO_CONTENT.into_response()
    } else {
        // 対象IDが存在しなかった場合
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "プレイヤーが見つかりません" })),
        )
            .into_response()
    }
}

/// GET /api/config - 現在の設定を返す
async fn get_config(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let state = state.lock().unwrap();
    Json(serde_json::json!({ "config": state.config }))
}

/// PUT /api/config - 設定を更新する（管理者のみが使う想定）
async fn update_config(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(req): Json<UpdateConfigRequest>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // チーム数の最小値は2、最大値はプレイヤー数（最低でも2）
    let team_count = req.team_count.max(2);
    state.config.team_count = team_count;

    // チーム数が変わったので既存のチーム結果をリセット
    state.teams = None;

    Json(serde_json::json!({ "config": state.config }))
}

/// POST /api/teams/balance - チーム分けを実行する
///
/// これがこのアプリの核心機能。
/// 1. 重複名を解決する
/// 2. 均衡化アルゴリズムでチームに振り分ける
/// 3. 結果を保存してレスポンスで返す
async fn balance_teams_handler(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // プレイヤーが誰もいない場合はエラー
    if state.players.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤーが登録されていません" })),
        )
            .into_response();
    }

    // チーム数がプレイヤー数を超えないように制限する
    // （プレイヤー3人を4チームに分けることはできない）
    let team_count = state.config.team_count.min(state.players.len());

    // ── 重複名の解決 ──
    // players のクローンを作り、そちらで display_name を更新する
    // （元の state.players も同期して更新する）
    let mut players_with_resolved_names = state.players.clone();
    resolve_duplicate_names(&mut players_with_resolved_names);

    // 解決した display_name を元のプレイヤーリストにも反映する
    for resolved in &players_with_resolved_names {
        if let Some(orig) = state.players.iter_mut().find(|p| p.id == resolved.id) {
            orig.display_name = resolved.display_name.clone();
        }
    }

    // ── チーム均衡化の実行 ──
    let teams = balance_teams(&players_with_resolved_names, team_count);

    // 結果を状態として保存（GET /api/teams で後から取得できるように）
    state.teams = Some(teams.clone());

    let result = BalanceResult {
        teams,
        players: players_with_resolved_names,
    };

    Json(serde_json::json!(result)).into_response()
}

/// GET /api/teams - 最後に実行したチーム分け結果を返す
async fn get_teams(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let state = state.lock().unwrap();
    match &state.teams {
        Some(teams) => Json(serde_json::json!({ "teams": teams })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "チーム分けがまだ実行されていません" })),
        )
            .into_response(),
    }
}

/// GET /health - ヘルスチェックエンドポイント
///
/// Renderがサービスの起動確認に使う。
/// 200 OK を返すだけのシンプルなエンドポイント。
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

// ============================================================
// SPAフォールバックハンドラー
// ============================================================
// Vue RouterはSPA（シングルページアプリケーション）なので、
// /owner のような存在しないサーバーサイドパスへのアクセスも
// index.html を返す必要がある。
// ============================================================
async fn spa_fallback() -> impl IntoResponse {
    // staticディレクトリのindex.htmlを読み込んで返す
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => axum::response::Html(content).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            "フロントエンドのビルドファイルが見つかりません。先にVueをビルドしてください。",
        )
            .into_response(),
    }
}

// ============================================================
// メイン関数 - サーバーの起動
// ============================================================
// #[tokio::main] マクロが main を非同期関数として実行できるようにする
// ============================================================
#[tokio::main]
async fn main() {
    // ── ログの初期化 ──
    // RUST_LOG 環境変数でログレベルを制御できる（例: RUST_LOG=debug）
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    // ── アプリケーション状態の初期化 ──
    // Arc<Mutex<>> でラップして複数のルートハンドラーから共有できるようにする
    let shared_state = Arc::new(Mutex::new(AppState::default()));

    // ── CORSの設定 ──
    // 開発環境（Vite dev server）からのAPIリクエストを許可する。
    // 本番環境ではVueとRustが同じオリジンで動作するため不要だが、
    // 開発中の利便性のために設定しておく。
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(Any);

    // ── APIルーターの定義 ──
    // /api プレフィックスを持つすべてのAPIエンドポイントをまとめる
    let api_router = Router::new()
        .route("/players",        get(get_players).post(add_player))
        .route("/players/:id",    put(update_player).delete(delete_player))
        .route("/config",         get(get_config).put(update_config))
        .route("/teams",          get(get_teams))
        .route("/teams/balance",  post(balance_teams_handler));

    // ── ポート設定 ──
    // Renderでは PORT 環境変数でポートが渡される。
    // ローカル開発時のデフォルトは 8080。
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // ── メインルーターの組み立て ──
    // 優先度: /health → /api/* → 静的ファイル → SPAフォールバック
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_router)
        // Vueのビルド成果物（JS/CSS/画像など）を配信
        // Docker内では /app/static に配置される
        .nest_service("/assets", ServeDir::new("static/assets"))
        // その他のリクエスト（/, /owner など）はSPAのindex.htmlを返す
        .fallback(spa_fallback)
        .layer(cors)
        // アプリケーション状態を全ルートで共有できるように登録
        .with_state(shared_state);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🚀 サーバー起動: http://{}", addr);

    // ── サーバーの起動 ──
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ============================================================
// テスト
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    // balance_teams のテスト
    #[test]
    fn test_balance_teams_two_teams() {
        // テスト用プレイヤーを作成
        let players = vec![
            Player { id: "1".to_string(), name: "A".to_string(), rank: Rank::Predator, display_name: None },
            Player { id: "2".to_string(), name: "B".to_string(), rank: Rank::Master,   display_name: None },
            Player { id: "3".to_string(), name: "C".to_string(), rank: Rank::Gold,     display_name: None },
            Player { id: "4".to_string(), name: "D".to_string(), rank: Rank::Bronze,   display_name: None },
        ];

        let teams = balance_teams(&players, 2);

        // 2チームに分かれているか確認
        assert_eq!(teams.len(), 2);
        // 全プレイヤーがどこかのチームに入っているか確認
        let total_players: usize = teams.iter().map(|t| t.players.len()).sum();
        assert_eq!(total_players, 4);

        // チーム1のスコアとチーム2のスコアの差が適切な範囲内か確認
        // プレデター(15)+ゴールド(3)=18 vs マスター(10)+ブロンズ(1)=11 → 差7
        // これが貪欲法の結果（最適解に近い）
        let score_diff = (teams[0].total_score as i32 - teams[1].total_score as i32).abs();
        println!("スコア差: {}", score_diff);
        assert!(score_diff <= 15, "スコア差が大きすぎます: {}", score_diff);
    }

    // 重複名解決のテスト
    #[test]
    fn test_resolve_duplicate_names() {
        let mut players = vec![
            Player { id: "1".to_string(), name: "あかし".to_string(), rank: Rank::Gold, display_name: None },
            Player { id: "2".to_string(), name: "たろう".to_string(), rank: Rank::Silver, display_name: None },
            Player { id: "3".to_string(), name: "あかし".to_string(), rank: Rank::Bronze, display_name: None },
        ];

        resolve_duplicate_names(&mut players);

        // 重複している "あかし" に番号が付いているか
        assert_eq!(players[0].display_name, Some("あかし(1)".to_string()));
        assert_eq!(players[2].display_name, Some("あかし(2)".to_string()));
        // 重複していない "たろう" は変更なし
        assert_eq!(players[1].display_name, None);
    }
}
