// ============================================================
// stores/players.ts - Piniaストア（グローバル状態管理）
//
// 【Piniaとは】
// VueアプリのグローバルなState（状態）を管理するライブラリ。
// Reactのnpmパッケージ redux/zustand に相当する。
// コンポーネント間でデータを共有したいとき、propsではなくここを使う。
//
// 【このストアで管理するもの】
// - プレイヤーリスト
// - チーム分け結果
// - アプリ設定（チーム数）
// - ローディング・エラー状態
// - 自分のプレイヤーID（セッション中のみ保持・localStorage不使用）
// ============================================================
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import axios from 'axios'
import type { Player, Team, AppConfig, BalanceResult } from '@/types'

// ── axios インスタンスの設定 ──
// ベースURLを設定することで、各API呼び出しで毎回書かなくて済む
const api = axios.create({
  baseURL: '/api', // Viteのプロキシ設定によりバックエンドに転送される
  headers: {
    'Content-Type': 'application/json',
  },
})

// ───────────────────────────────────────────────────────────
// usePlayersStore - プレイヤー・チーム管理ストア
//
// defineStore の第1引数はストアの識別名（デバッグ時に表示される）
// 第2引数は Setup関数（Composition APIスタイル）
// ───────────────────────────────────────────────────────────
export const usePlayersStore = defineStore('players', () => {
  // ══════════════════════════════════════════════════════════
  // State（状態）
  // ref() でリアクティブな変数を作成する。
  // 値が変わると、それを参照しているコンポーネントが自動で再レンダリングされる。
  // ══════════════════════════════════════════════════════════

  /** 全プレイヤーリスト */
  const players = ref<Player[]>([])

  /** チーム分け結果（未実行時はnull） */
  const teams = ref<Team[] | null>(null)

  /** アプリ設定 */
  const config = ref<AppConfig>({ team_count: 2 })

  /** APIリクエスト中かどうか（ローディング表示に使用） */
  const isLoading = ref(false)

  /** エラーメッセージ（なければnull） */
  const errorMessage = ref<string | null>(null)

  /**
   * 自分のプレイヤーID（セッション中のみ保持）
   * ページをリロードするとリセットされる（localStorage不使用）
   */
  const myPlayerId = ref<string | null>(null)

  // ══════════════════════════════════════════════════════════
  // Getters（算出プロパティ）
  // computed() でリアクティブな算出値を作成する。
  // 依存するデータが変わると自動で再計算される。
  // ══════════════════════════════════════════════════════════

  /** 自分のプレイヤーオブジェクト（登録済みの場合）*/
  const myPlayer = computed<Player | undefined>(() =>
    myPlayerId.value
      ? players.value.find(p => p.id === myPlayerId.value)
      : undefined
  )

  /** チーム分けが実行済みかどうか */
  const hasTeams = computed(() => teams.value !== null && teams.value.length > 0)

  /** プレイヤー総数 */
  const playerCount = computed(() => players.value.length)

  // ══════════════════════════════════════════════════════════
  // Actions（副作用のある操作・API呼び出し）
  // ══════════════════════════════════════════════════════════

  // ─────────────────────────────────────────────────────────
  // エラーを設定するヘルパー
  // ─────────────────────────────────────────────────────────
  function setError(message: string) {
    errorMessage.value = message
    // 5秒後にエラーを自動クリア
    setTimeout(() => { errorMessage.value = null }, 5000)
  }

  function clearError() {
    errorMessage.value = null
  }

  // ─────────────────────────────────────────────────────────
  // fetchPlayers - サーバーから全プレイヤーを取得する
  // ─────────────────────────────────────────────────────────
  async function fetchPlayers() {
    try {
      isLoading.value = true
      const res = await api.get<{ players: Player[] }>('/players')
      players.value = res.data.players
    } catch (e) {
      setError('プレイヤー一覧の取得に失敗しました')
      console.error('[fetchPlayers]', e)
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────
  // fetchConfig - 設定を取得する
  // ─────────────────────────────────────────────────────────
  async function fetchConfig() {
    try {
      const res = await api.get<{ config: AppConfig }>('/config')
      config.value = res.data.config
    } catch (e) {
      console.error('[fetchConfig]', e)
    }
  }

  // ─────────────────────────────────────────────────────────
  // fetchTeams - 最後のチーム分け結果を取得する（ページリロード後の復元用）
  // ─────────────────────────────────────────────────────────
  async function fetchTeams() {
    try {
      const res = await api.get<{ teams: Team[] }>('/teams')
      teams.value = res.data.teams
    } catch (e: any) {
      // 404はチーム分け未実行を意味するため、エラーとして扱わない
      if (e.response?.status !== 404) {
        console.error('[fetchTeams]', e)
      }
    }
  }

  // ─────────────────────────────────────────────────────────
  // addPlayer - 新しいプレイヤーを追加する
  //
  // 【パラメータ】
  //   name  - プレイヤー名
  //   rank  - ランク
  //   saveAsMyPlayer - true のとき、追加したプレイヤーIDを「自分」として保存する
  // ─────────────────────────────────────────────────────────
  async function addPlayer(
    name: string,
    rank: string,
    saveAsMyPlayer = false
  ): Promise<Player | null> {
    try {
      isLoading.value = true
      clearError()

      const res = await api.post<{ player: Player }>('/players', { name, rank })
      const newPlayer = res.data.player

      // ローカルの状態を更新（再取得しなくてもUIに即反映される）
      players.value.push(newPlayer)

      // 一般ユーザーが自分のプレイヤーとして登録する場合
      if (saveAsMyPlayer) {
        myPlayerId.value = newPlayer.id
      }

      // チーム分け結果をリセット（プレイヤーが変わったため）
      teams.value = null

      return newPlayer
    } catch (e: any) {
      setError(e.response?.data?.error ?? 'プレイヤーの追加に失敗しました')
      console.error('[addPlayer]', e)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────
  // updatePlayer - 既存プレイヤーの情報を更新する
  // ─────────────────────────────────────────────────────────
  async function updatePlayer(id: string, name: string, rank: string): Promise<boolean> {
    try {
      isLoading.value = true
      clearError()

      const res = await api.put<{ player: Player }>(`/players/${id}`, { name, rank })
      const updated = res.data.player

      // ローカルリストの該当プレイヤーを更新
      const idx = players.value.findIndex(p => p.id === id)
      if (idx !== -1) {
        players.value[idx] = updated
      }

      teams.value = null
      return true
    } catch (e: any) {
      setError(e.response?.data?.error ?? 'プレイヤーの更新に失敗しました')
      console.error('[updatePlayer]', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────
  // deletePlayer - プレイヤーを削除する（管理者のみ使用）
  // ─────────────────────────────────────────────────────────
  async function deletePlayer(id: string): Promise<boolean> {
    try {
      isLoading.value = true
      clearError()

      await api.delete(`/players/${id}`)

      // ローカルリストから削除
      players.value = players.value.filter(p => p.id !== id)

      // 削除したプレイヤーが「自分」だった場合、IDをクリア
      if (myPlayerId.value === id) {
        myPlayerId.value = null
      }

      teams.value = null
      return true
    } catch (e: any) {
      setError(e.response?.data?.error ?? 'プレイヤーの削除に失敗しました')
      console.error('[deletePlayer]', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────
  // adjustScoreOffset - プレイヤーのスコア補正値を±1調整する（管理者のみ使用）
  //
  // バックエンドでは −5〜+5 にクランプされる
  // ─────────────────────────────────────────────────────────
  async function adjustScoreOffset(id: string, delta: number): Promise<boolean> {
    try {
      const res = await api.patch<{ player: Player }>(`/players/${id}/offset`, { delta })
      const updated = res.data.player

      const idx = players.value.findIndex(p => p.id === id)
      if (idx !== -1) {
        players.value[idx] = updated
      }

      teams.value = null
      return true
    } catch (e: any) {
      setError(e.response?.data?.error ?? 'スコア補正の変更に失敗しました')
      console.error('[adjustScoreOffset]', e)
      return false
    }
  }

  // ─────────────────────────────────────────────────────────
  // updateConfig - 設定を更新する（管理者のみ使用）
  // ─────────────────────────────────────────────────────────
  async function updateConfig(teamCount: number): Promise<boolean> {
    try {
      clearError()
      const res = await api.put<{ config: AppConfig }>('/config', { team_count: teamCount })
      config.value = res.data.config
      teams.value = null
      return true
    } catch (e: any) {
      setError(e.response?.data?.error ?? '設定の更新に失敗しました')
      console.error('[updateConfig]', e)
      return false
    }
  }

  // ─────────────────────────────────────────────────────────
  // balanceTeams - チーム分けを実行する（管理者のみ使用）
  //
  // バックエンドの均衡化アルゴリズムを呼び出し、
  // 結果をローカルストアに保存する。
  // また、重複名解決済みのプレイヤーリストで players を更新する。
  // ─────────────────────────────────────────────────────────
  async function balanceTeams(): Promise<boolean> {
    try {
      isLoading.value = true
      clearError()

      const res = await api.post<BalanceResult>('/teams/balance')
      const result = res.data

      // チーム結果を保存
      teams.value = result.teams

      // バックエンドが重複名を解決した結果でプレイヤーリストを更新
      players.value = result.players

      return true
    } catch (e: any) {
      setError(e.response?.data?.error ?? 'チーム分けの実行に失敗しました')
      console.error('[balanceTeams]', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────
  // disbandTeams - チームを解散する（管理者のみ使用）
  // ─────────────────────────────────────────────────────────
  async function disbandTeams(): Promise<boolean> {
    try {
      clearError()
      await api.delete('/teams')
      teams.value = null
      return true
    } catch (e: any) {
      setError(e.response?.data?.error ?? 'チームの解散に失敗しました')
      console.error('[disbandTeams]', e)
      return false
    }
  }

  // ─────────────────────────────────────────────────────────
  // initialize - アプリ起動時に呼ぶ初期化関数
  // 全データをサーバーから取得する
  // ─────────────────────────────────────────────────────────
  async function initialize() {
    // 並行してAPIを呼び出す（Promise.all で同時実行して待ち時間を短縮）
    await Promise.all([
      fetchPlayers(),
      fetchConfig(),
      fetchTeams(),
    ])
  }

  // ── ストアが外部に公開するものをまとめて return する ──
  return {
    // State
    players,
    teams,
    config,
    isLoading,
    errorMessage,
    myPlayerId,
    // Getters
    myPlayer,
    hasTeams,
    playerCount,
    // Actions
    initialize,
    fetchPlayers,
    fetchConfig,
    fetchTeams,
    addPlayer,
    updatePlayer,
    deletePlayer,
    adjustScoreOffset,
    updateConfig,
    balanceTeams,
    disbandTeams,
    clearError,
  }
})
