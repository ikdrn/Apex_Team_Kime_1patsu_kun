// ============================================================
// main.rs - チーム決め一発くん バックエンドサーバー（v4）
//
// 【v4 変更点】
//   1. score_offset フィールド追加（プレイヤーごとの戦闘力補正値 -5〜+5）
//   2. ランダム性付与（同スコア帯でシャッフル → 毎回違うチーム編成）
//   3. PATCH /api/players/:id/offset エンドポイント追加（±1ずつ調整）
//   4. DELETE /api/teams エンドポイント追加（チーム解散）
// ============================================================

use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, patch, post, put},
    Router,
};
use rand::seq::SliceRandom; // シャッフル用
use rand::thread_rng;       // 乱数生成器
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use uuid::Uuid;

// ============================================================
// データ構造
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rank {
    #[serde(rename = "ブロンズ")]   Bronze,
    #[serde(rename = "シルバー")]   Silver,
    #[serde(rename = "ゴールド")]   Gold,
    #[serde(rename = "プラチナ")]   Platinum,
    #[serde(rename = "ダイヤモンド")] Diamond,
    #[serde(rename = "マスター")]   Master,
    #[serde(rename = "プレデター")] Predator,
}

impl Rank {
    // ランクの基本スコア
    // ブロンズ(1)〜プレデター(15) の非線形スケール
    pub fn base_score(&self) -> i32 {
        match self {
            Rank::Bronze   => 10,
            Rank::Silver   => 20,
            Rank::Gold     => 30,
            Rank::Platinum => 40,
            Rank::Diamond  => 50,
            Rank::Master   => 60,
            Rank::Predator => 70,
        }
    }
}

// ───────────────────────────────────────────────────────────
// Player 構造体
//
// 【score_offset について】
//   同じランクでも個人差があるため、管理者が ±1 ずつ調整できる補正値。
//   範囲: -3〜+3（ランク間のスコア差は10なので ±3 で十分な表現力）
//   デフォルト: 0（補正なし）
// ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub rank: Rank,
    /// 個人差補正値（管理者が ±1 ずつ調整, 範囲: -3〜+3）
    #[serde(default)] // JSONに含まれない場合は 0 として扱う
    pub score_offset: i32,
    pub display_name: Option<String>,
}

impl Player {
    // 実際に使われる戦闘力スコア = 基本スコア + 個人差補正
    // 最低値は 1（0以下にはならない）
    pub fn effective_score(&self) -> i32 {
        (self.rank.base_score() + self.score_offset).max(1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: usize,
    pub players: Vec<Player>,
    /// チームの合計戦闘力スコア（effective_score の合計）
    pub total_score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub team_count: usize,
}

impl Default for AppConfig {
    fn default() -> Self { Self { team_count: 2 } }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub players: Vec<Player>,
    pub teams: Option<Vec<Team>>,
    pub config: AppConfig,
}

type SharedState = Arc<RwLock<AppState>>;

// ============================================================
// APIリクエスト型
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

/// PATCH /api/players/:id/offset のボディ
/// delta = +1 または -1 のみ許可（範囲外は clamp で ±1 に正規化される）
#[derive(Debug, Deserialize)]
pub struct AdjustOffsetRequest {
    pub delta: i32,
}

#[derive(Debug, Serialize)]
pub struct BalanceResult {
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
}

// ============================================================
// アルゴリズム
// ============================================================

fn resolve_duplicate_names(players: &mut Vec<Player>) {
    let mut name_count: HashMap<String, usize> = HashMap::new();
    for p in players.iter() {
        *name_count.entry(p.name.clone()).or_insert(0) += 1;
    }
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
// balance_teams - 戦闘力均衡化チーム分けアルゴリズム（マルチトライ方式）
//
// 【ランダム性と均衡性の両立】
//   30回の試行を行い、その中で最もスコア差が小さいチーム分けを採用する。
//
//   各試行:
//     1. プレイヤーを完全にシャッフル（全員の順序をランダム化）
//     2. effective_score 降順でソート（同スコアは1の結果でランダムな順）
//     3. 貪欲法でチームに割り当て（最も合計スコアの低いチームへ）
//
//   この方式により:
//   - 同じランク構成でも毎回異なるチーム編成になる
//   - 30回の試行から最良のバランスを選ぶので大きな偏りが生まれない
// ───────────────────────────────────────────────────────────
fn balance_teams(players: &[Player], team_count: usize) -> Vec<Team> {
    let team_count = team_count.max(1).min(players.len().max(1));

    if players.is_empty() {
        return (1..=team_count)
            .map(|i| Team { id: i, players: vec![], total_score: 0 })
            .collect();
    }

    let mut rng = thread_rng();
    let mut best_teams: Option<Vec<Team>> = None;
    let mut best_diff = i32::MAX;

    // 30回試行して最もスコア差の小さい結果を採用
    for _ in 0..30 {
        // Step 1: 完全ランダムシャッフル
        let mut shuffled: Vec<Player> = players.to_vec();
        shuffled.shuffle(&mut rng);

        // Step 2: 降順ソート（同スコアはシャッフル済みなのでランダムな順になる）
        shuffled.sort_by(|a, b| b.effective_score().cmp(&a.effective_score()));

        // Step 3: 貪欲法でチームに割り当て
        let mut team_scores: Vec<i32> = vec![0; team_count];
        let mut team_players: Vec<Vec<Player>> = vec![vec![]; team_count];

        for player in &shuffled {
            let min_idx = team_scores
                .iter()
                .enumerate()
                .min_by_key(|(_, &s)| s)
                .map(|(i, _)| i)
                .unwrap_or(0);
            team_scores[min_idx] += player.effective_score();
            team_players[min_idx].push(player.clone());
        }

        // Step 4: スコア差を計算
        let diff = team_scores.iter().max().unwrap_or(&0)
            - team_scores.iter().min().unwrap_or(&0);

        if diff < best_diff {
            best_diff = diff;
            best_teams = Some(
                team_players
                    .into_iter()
                    .enumerate()
                    .map(|(i, pls)| {
                        let total: i32 = pls.iter().map(|p| p.effective_score()).sum();
                        Team { id: i + 1, players: pls, total_score: total }
                    })
                    .collect(),
            );
        }
    }

    best_teams.unwrap_or_default()
}

// ============================================================
// APIハンドラー
// ============================================================

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

async fn get_players(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    Json(serde_json::json!({ "players": state.players }))
}

async fn add_player(
    State(state): State<SharedState>,
    Json(req): Json<AddPlayerRequest>,
) -> impl IntoResponse {
    let name = req.name.trim().to_string();
    // 名前バリデーション: 1文字以上30文字以下
    if name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤー名を入力してください" })),
        ).into_response();
    }
    if name.chars().count() > 30 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤー名は30文字以内で入力してください" })),
        ).into_response();
    }
    let mut state = state.write().unwrap();
    let new_player = Player {
        id: Uuid::new_v4().to_string(),
        name,
        rank: req.rank,
        score_offset: 0,
        display_name: None,
    };
    let added = new_player.clone();
    state.players.push(new_player);
    state.teams = None;
    (StatusCode::CREATED, Json(serde_json::json!({ "player": added }))).into_response()
}

async fn update_player(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePlayerRequest>,
) -> impl IntoResponse {
    let name = req.name.trim().to_string();
    // 名前バリデーション: 1文字以上30文字以下
    if name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤー名を入力してください" })),
        ).into_response();
    }
    if name.chars().count() > 30 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤー名は30文字以内で入力してください" })),
        ).into_response();
    }
    let mut state = state.write().unwrap();
    if let Some(player) = state.players.iter_mut().find(|p| p.id == id) {
        player.name = name;
        player.rank = req.rank;
        player.display_name = None;
        let updated = player.clone();
        state.teams = None;
        return (StatusCode::OK, Json(serde_json::json!({ "player": updated })));
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "プレイヤーが見つかりません" })))
}

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
        (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "プレイヤーが見つかりません" }))).into_response()
    }
}

// ───────────────────────────────────────────────────────────
// PATCH /api/players/:id/offset - 戦闘力補正値を ±1 調整する
//
// 【設計】
//   delta = +1 または -1 のみ受け付ける。
//   範囲: -3〜+3 でクランプ（それ以上は変化なし）。
//   チーム分け結果は無効化される（補正値が変わったため）。
// ───────────────────────────────────────────────────────────
async fn adjust_offset(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<AdjustOffsetRequest>,
) -> impl IntoResponse {
    // delta は +1 か -1 のみ許可
    let delta = req.delta.clamp(-1, 1);

    let mut state = state.write().unwrap();
    if let Some(player) = state.players.iter_mut().find(|p| p.id == id) {
        // -3〜+3 の範囲でクランプ
        player.score_offset = (player.score_offset + delta).clamp(-3, 3);
        let updated = player.clone();
        state.teams = None;
        return (StatusCode::OK, Json(serde_json::json!({ "player": updated })));
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "プレイヤーが見つかりません" })))
}

async fn get_config(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    Json(serde_json::json!({ "config": state.config }))
}

async fn update_config(
    State(state): State<SharedState>,
    Json(req): Json<UpdateConfigRequest>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    state.config.team_count = req.team_count.max(2);
    state.teams = None;
    Json(serde_json::json!({ "config": state.config }))
}

async fn balance_teams_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    if state.players.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "プレイヤーが登録されていません" })),
        ).into_response();
    }
    let team_count = state.config.team_count.min(state.players.len());

    let mut resolved = state.players.clone();
    resolve_duplicate_names(&mut resolved);
    for r in &resolved {
        if let Some(orig) = state.players.iter_mut().find(|p| p.id == r.id) {
            orig.display_name = r.display_name.clone();
        }
    }

    let teams = balance_teams(&resolved, team_count);
    state.teams = Some(teams.clone());
    Json(serde_json::json!(BalanceResult { teams, players: resolved })).into_response()
}

async fn get_teams(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    match &state.teams {
        Some(teams) => Json(serde_json::json!({ "teams": teams })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "チーム分けがまだ実行されていません" })),
        ).into_response(),
    }
}

// ───────────────────────────────────────────────────────────
// DELETE /api/teams - チームを解散する（管理者専用）
//
// チーム分け結果をリセットする。プレイヤーはそのまま残る。
// ───────────────────────────────────────────────────────────
async fn disband_teams(State(state): State<SharedState>) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    state.teams = None;
    StatusCode::NO_CONTENT
}

// ============================================================
// メイン関数
// ============================================================
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let shared_state: SharedState = Arc::new(RwLock::new(AppState::default()));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let api_router = Router::new()
        .route("/players",              get(get_players).post(add_player))
        .route("/players/:id",          put(update_player).delete(delete_player))
        .route("/players/:id/offset",   patch(adjust_offset))   // ← 新規: 戦闘力補正
        .route("/config",               get(get_config).put(update_config))
        .route("/teams",                get(get_teams).delete(disband_teams)) // ← DELETE追加
        .route("/teams/balance",        post(balance_teams_handler));

    let spa_service = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_router)
        .fallback_service(spa_service)
        .layer(cors)
        .with_state(shared_state);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🚀 起動: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await
        .expect("ポートバインド失敗");
    axum::serve(listener, app).await.expect("サーバー起動失敗");
}

// ============================================================
// テスト
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_score_with_offset() {
        let p = Player { id: "1".into(), name: "A".into(), rank: Rank::Gold,
                         score_offset: 2, display_name: None };
        assert_eq!(p.effective_score(), 32); // Gold(30) + 2 = 32

        let p2 = Player { id: "2".into(), name: "B".into(), rank: Rank::Bronze,
                          score_offset: -3, display_name: None };
        assert_eq!(p2.effective_score(), 7); // Bronze(10) - 3 = 7（clamp は不要）

        // clamp 動作確認: Bronze(10) + offset が極端に負でも最低値1になること
        let p3 = Player { id: "3".into(), name: "C".into(), rank: Rank::Bronze,
                          score_offset: -10, display_name: None };
        assert_eq!(p3.effective_score(), 1); // Bronze(10) - 10 = 0 → clamp to 1
    }

    #[test]
    fn test_balance_produces_all_players() {
        let players = vec![
            Player { id: "1".into(), name: "A".into(), rank: Rank::Predator, score_offset: 0, display_name: None },
            Player { id: "2".into(), name: "B".into(), rank: Rank::Master,   score_offset: 0, display_name: None },
            Player { id: "3".into(), name: "C".into(), rank: Rank::Gold,     score_offset: 0, display_name: None },
            Player { id: "4".into(), name: "D".into(), rank: Rank::Bronze,   score_offset: 0, display_name: None },
        ];
        let teams = balance_teams(&players, 2);
        assert_eq!(teams.len(), 2);
        let total: usize = teams.iter().map(|t| t.players.len()).sum();
        assert_eq!(total, 4);
    }

    #[test]
    fn test_resolve_duplicates() {
        let mut players = vec![
            Player { id: "1".into(), name: "あかし".into(), rank: Rank::Gold,   score_offset: 0, display_name: None },
            Player { id: "2".into(), name: "あかし".into(), rank: Rank::Bronze, score_offset: 0, display_name: None },
        ];
        resolve_duplicate_names(&mut players);
        assert_eq!(players[0].display_name, Some("あかし(1)".to_string()));
        assert_eq!(players[1].display_name, Some("あかし(2)".to_string()));
    }
}
