<!--
  TeamResult.vue - チーム分け結果表示コンポーネント

  【役割】
  バランシング実行後の結果（チーム構成・スコア）を視覚的に表示する。

  【デザインの意図】
  - チームを横に並べるグリッドレイアウト
  - 各チームのスコアバーでバランスを一目で確認できる
  - チーム間のスコア差も表示
-->
<script setup lang="ts">
import { computed } from 'vue'
import { usePlayersStore } from '@/stores/players'
import { RANK_COLORS, getDisplayName } from '@/types'

const store = usePlayersStore()

// ── チームごとの最大スコアを計算（スコアバーの描画に使用）──
const maxScore = computed(() => {
  if (!store.teams || store.teams.length === 0) return 1
  return Math.max(...store.teams.map(t => t.total_score), 1)
})

// ── スコアの最大差（均衡度の指標）──
const scoreDiff = computed(() => {
  if (!store.teams || store.teams.length < 2) return 0
  const scores = store.teams.map(t => t.total_score)
  return Math.max(...scores) - Math.min(...scores)
})

// ── チームカラーの定義（チームIDに対応した色）──
// チームを視覚的に区別するためのカラーリスト
const teamColors = [
  { border: 'border-t-blue-600',    bg: 'bg-blue-600',    text: 'text-blue-700',    barBg: '#2563eb' },
  { border: 'border-t-orange-500',  bg: 'bg-orange-500',  text: 'text-orange-700',  barBg: '#f97316' },
  { border: 'border-t-emerald-600', bg: 'bg-emerald-600', text: 'text-emerald-700', barBg: '#059669' },
  { border: 'border-t-rose-600',    bg: 'bg-rose-600',    text: 'text-rose-700',    barBg: '#e11d48' },
  { border: 'border-t-violet-600',  bg: 'bg-violet-600',  text: 'text-violet-700',  barBg: '#7c3aed' },
  { border: 'border-t-amber-500',   bg: 'bg-amber-500',   text: 'text-amber-700',   barBg: '#f59e0b' },
]

function getTeamColor(teamId: number) {
  return teamColors[(teamId - 1) % teamColors.length]
}

// スコアバーの幅をパーセントで計算する
function getScoreBarWidth(score: number): string {
  return `${(score / maxScore.value) * 100}%`
}
</script>

<template>
  <div v-if="store.hasTeams" class="animate-slide-up">
    <!-- ── セクションヘッダー ── -->
    <div class="flex items-baseline justify-between mb-4">
      <h2 class="section-heading mb-0 pb-0 border-0">
        チーム分け結果
      </h2>
      <!-- スコア差の表示（均衡度インジケーター） -->
      <div class="flex items-center gap-2 text-xs text-neutral-500">
        <span>スコア差:</span>
        <span
          class="font-semibold"
          :class="scoreDiff <= 3 ? 'text-emerald-600' : scoreDiff <= 7 ? 'text-amber-600' : 'text-red-600'"
        >
          {{ scoreDiff }}
        </span>
        <span
          class="px-1.5 py-0.5 text-xs font-medium"
          :class="scoreDiff <= 3 ? 'bg-emerald-50 text-emerald-700' : scoreDiff <= 7 ? 'bg-amber-50 text-amber-700' : 'bg-red-50 text-red-700'"
        >
          {{ scoreDiff <= 3 ? '均衡' : scoreDiff <= 7 ? '標準' : '偏り' }}
        </span>
      </div>
    </div>
    <div class="border-b border-neutral-200 mb-5"></div>

    <!--
      チームグリッド
      グリッド列数はチーム数に応じて動的に変更
      2チーム以下→2列、3チーム以上→3列
    -->
    <div
      class="grid gap-4"
      :class="{
        'grid-cols-1 sm:grid-cols-2': (store.teams?.length ?? 0) <= 2,
        'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3': (store.teams?.length ?? 0) > 2,
      }"
    >
      <div
        v-for="team in store.teams"
        :key="team.id"
        class="bg-white border border-neutral-200 border-t-4"
        :class="getTeamColor(team.id).border"
      >
        <!-- チームヘッダー -->
        <div class="px-4 pt-4 pb-3 border-b border-neutral-100">
          <div class="flex items-center justify-between">
            <h3 class="font-bold text-neutral-900 tracking-tight">
              TEAM {{ team.id }}
            </h3>
            <span class="text-xs text-neutral-500 tabular-nums">
              {{ team.players.length }} 名
            </span>
          </div>

          <!-- スコアバー -->
          <div class="mt-3">
            <div class="flex items-center justify-between text-xs text-neutral-500 mb-1">
              <span>戦闘力</span>
              <span
                class="font-semibold tabular-nums"
                :class="getTeamColor(team.id).text"
              >
                {{ team.total_score }}
              </span>
            </div>
            <!-- バー本体 -->
            <div class="h-1.5 bg-neutral-100 overflow-hidden">
              <div
                class="h-full transition-all duration-700 ease-out"
                :style="{
                  width: getScoreBarWidth(team.total_score),
                  backgroundColor: getTeamColor(team.id).barBg
                }"
              />
            </div>
          </div>
        </div>

        <!-- プレイヤーリスト -->
        <ul class="divide-y divide-neutral-50">
          <li
            v-for="player in team.players"
            :key="player.id"
            class="flex items-center gap-3 px-4 py-2.5"
          >
            <!-- ランクバッジ（小サイズ・2文字表示） -->
            <span
              class="shrink-0 inline-flex items-center px-1.5 py-0.5 text-xs font-medium border"
              :class="[
                RANK_COLORS[player.rank].bg,
                RANK_COLORS[player.rank].text,
                RANK_COLORS[player.rank].border,
              ]"
            >
              {{ player.rank.slice(0, 2) }}
            </span>
            <!-- プレイヤー名 -->
            <span class="flex-1 text-sm text-neutral-800 font-medium truncate">
              {{ getDisplayName(player) }}
            </span>
            <!-- 自分のプレイヤーに星印を付ける -->
            <span
              v-if="player.id === store.myPlayerId"
              class="shrink-0 text-xs text-primary-600 font-bold"
            >
              ★
            </span>
          </li>
        </ul>
      </div>
    </div>

    <!-- ── 全チームのスコア比較バー ── -->
    <div class="mt-6 p-4 bg-neutral-50 border border-neutral-200">
      <p class="text-xs font-semibold text-neutral-500 uppercase tracking-wider mb-3">
        戦闘力バランス比較
      </p>
      <div class="space-y-2.5">
        <div
          v-for="team in store.teams"
          :key="team.id"
          class="flex items-center gap-3"
        >
          <span class="w-14 text-xs font-medium text-neutral-600 shrink-0">
            TEAM {{ team.id }}
          </span>
          <div class="flex-1 h-5 bg-neutral-200 overflow-hidden">
            <div
              class="h-full transition-all duration-700 ease-out flex items-center justify-end pr-1.5"
              :style="{
                width: getScoreBarWidth(team.total_score),
                backgroundColor: getTeamColor(team.id).barBg
              }"
            >
              <span class="text-white text-xs font-bold tabular-nums leading-none">
                {{ team.total_score }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- チーム分け未実行時のプレースホルダー -->
  <div
    v-else
    class="py-16 text-center border border-dashed border-neutral-200"
  >
    <div class="text-5xl mb-4 font-black tracking-tighter text-neutral-200">
      TEAM
    </div>
    <p class="text-sm text-neutral-400">
      チーム分けはまだ実行されていません
    </p>
    <p class="text-xs text-neutral-300 mt-1">
      管理者がチーム分けを実行すると結果が表示されます
    </p>
  </div>
</template>
