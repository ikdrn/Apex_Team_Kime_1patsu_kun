// ============================================================
// types/index.ts - TypeScript型定義ファイル
//
// 【役割】
// バックエンドのRust構造体と対応するTypeScript型を定義する。
// 型定義を1か所にまとめることで、変更時の修正箇所を最小化できる。
// ============================================================

// ───────────────────────────────────────────────────────────
// RankName: ランクの文字列型
//
// Rustの Rank 列挙型の serde(rename) に対応した日本語文字列。
// TypeScriptでは文字列リテラルの合算型として表現する。
// ───────────────────────────────────────────────────────────
export type RankName =
  | 'ブロンズ'
  | 'シルバー'
  | 'ゴールド'
  | 'プラチナ'
  | 'ダイヤモンド'
  | 'マスター'
  | 'プレデター'

// ランクの一覧（ドロップダウンの選択肢に使用）
// as const でリテラル型の配列として確定させる
export const RANK_LIST: RankName[] = [
  'ブロンズ',
  'シルバー',
  'ゴールド',
  'プラチナ',
  'ダイヤモンド',
  'マスター',
  'プレデター',
]

// ランクのスコア（表示用・フロントエンドでの参照用）
// バックエンドと一致させること
export const RANK_SCORES: Record<RankName, number> = {
  'ブロンズ':     10,
  'シルバー':     20,
  'ゴールド':     30,
  'プラチナ':     40,
  'ダイヤモンド': 50,
  'マスター':     60,
  'プレデター':   70,
}

// ランクに対応するCSSカラークラス（表示用バッジに使用）
export const RANK_COLORS: Record<RankName, { bg: string; text: string; border: string }> = {
  'ブロンズ':     { bg: 'bg-amber-100',  text: 'text-amber-800',  border: 'border-amber-300'  },
  'シルバー':     { bg: 'bg-slate-100',  text: 'text-slate-700',  border: 'border-slate-300'  },
  'ゴールド':     { bg: 'bg-yellow-100', text: 'text-yellow-800', border: 'border-yellow-400' },
  'プラチナ':     { bg: 'bg-cyan-100',   text: 'text-cyan-800',   border: 'border-cyan-300'   },
  'ダイヤモンド': { bg: 'bg-blue-100',   text: 'text-blue-800',   border: 'border-blue-300'   },
  'マスター':     { bg: 'bg-purple-100', text: 'text-purple-800', border: 'border-purple-400' },
  'プレデター':   { bg: 'bg-red-100',    text: 'text-red-800',    border: 'border-red-400'    },
}

// ───────────────────────────────────────────────────────────
// Player: プレイヤー情報（バックエンドの Player 構造体に対応）
// ───────────────────────────────────────────────────────────
export interface Player {
  /** バックエンドが生成したUUID（プレイヤーの一意識別子） */
  id: string
  /** プレイヤー名（入力値） */
  name: string
  /** ランク */
  rank: RankName
  /**
   * スコア補正値（−5〜+5）
   * 管理者が±1ずつ調整可能。デフォルト0。
   */
  score_offset: number
  /**
   * 重複名解決後の表示名。
   * チーム分け実行後に同名プレイヤーがいた場合、
   * バックエンドが "名前(1)", "名前(2)" 形式でセットする。
   * undefined/null のときは name をそのまま表示する。
   */
  display_name?: string | null
}

// プレイヤーの表示名を返す便利関数
export function getDisplayName(player: Player): string {
  return player.display_name ?? player.name
}

// ───────────────────────────────────────────────────────────
// Team: チーム情報（バックエンドの Team 構造体に対応）
// ───────────────────────────────────────────────────────────
export interface Team {
  /** チーム番号（1始まり） */
  id: number
  /** このチームに所属するプレイヤーのリスト */
  players: Player[]
  /** チームの合計スコア（戦力値の合計） */
  total_score: number
}

// ───────────────────────────────────────────────────────────
// AppConfig: アプリケーション設定（バックエンドの AppConfig に対応）
// ───────────────────────────────────────────────────────────
export interface AppConfig {
  /** 作るチームの数（デフォルト: 2） */
  team_count: number
}

// ───────────────────────────────────────────────────────────
// BalanceResult: チーム分け実行結果
// ───────────────────────────────────────────────────────────
export interface BalanceResult {
  /** 分けられたチームのリスト */
  teams: Team[]
  /** 重複名が解決済みのプレイヤーリスト */
  players: Player[]
}

// ───────────────────────────────────────────────────────────
// APIError: エラーレスポンスの型
// ───────────────────────────────────────────────────────────
export interface ApiError {
  error: string
}
