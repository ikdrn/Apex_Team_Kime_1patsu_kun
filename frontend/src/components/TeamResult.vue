<script setup lang="ts">
import { computed } from 'vue'
import { usePlayersStore } from '@/stores/players'
import { RANK_COLORS, getDisplayName } from '@/types'

const store = usePlayersStore()

const maxScore = computed(() => {
  if (!store.teams?.length) return 1
  return Math.max(...store.teams.map(t => t.total_score), 1)
})

const scoreDiff = computed(() => {
  if (!store.teams || store.teams.length < 2) return 0
  const s = store.teams.map(t => t.total_score)
  return Math.max(...s) - Math.min(...s)
})

const teamStyles = [
  { color: '#2563eb', labelClass: 'text-blue-600',    softBg: 'bg-blue-50',   bar: 'bg-blue-500',   ring: 'ring-blue-200'   },
  { color: '#ea580c', labelClass: 'text-orange-600',  softBg: 'bg-orange-50', bar: 'bg-orange-500', ring: 'ring-orange-200' },
  { color: '#059669', labelClass: 'text-emerald-600', softBg: 'bg-emerald-50',bar: 'bg-emerald-500',ring: 'ring-emerald-200'},
  { color: '#dc2626', labelClass: 'text-red-600',     softBg: 'bg-red-50',    bar: 'bg-red-500',    ring: 'ring-red-200'    },
  { color: '#7c3aed', labelClass: 'text-violet-600',  softBg: 'bg-violet-50', bar: 'bg-violet-500', ring: 'ring-violet-200' },
  { color: '#d97706', labelClass: 'text-amber-600',   softBg: 'bg-amber-50',  bar: 'bg-amber-500',  ring: 'ring-amber-200'  },
]

function ts(id: number) { return teamStyles[(id - 1) % teamStyles.length] }
function barPct(score: number) { return `${Math.round((score / maxScore.value) * 100)}%` }
</script>

<template>
  <!-- チーム分け済み -->
  <div v-if="store.hasTeams" class="p-5 space-y-5">

    <!-- ステータス行 -->
    <div class="flex items-center gap-3">
      <h2 class="text-xs font-semibold tracking-widest uppercase text-neutral-500">チーム分け結果</h2>
      <div class="flex-1 h-px bg-neutral-200" />
      <div class="flex items-center gap-2 text-sm">
        <span class="text-neutral-500">スコア差</span>
        <span class="font-mono font-bold" :class="scoreDiff <= 5 ? 'text-emerald-600' : scoreDiff <= 15 ? 'text-amber-500' : 'text-red-500'">
          {{ scoreDiff }}
        </span>
        <span
          class="text-xs font-semibold px-2 py-0.5 rounded-full"
          :class="scoreDiff <= 5 ? 'bg-emerald-100 text-emerald-700' : scoreDiff <= 15 ? 'bg-amber-100 text-amber-700' : 'bg-red-100 text-red-600'"
        >{{ scoreDiff <= 5 ? '均衡' : scoreDiff <= 15 ? '標準' : '偏り' }}</span>
      </div>
    </div>

    <!-- チームカード -->
    <div
      class="grid gap-4"
      :class="{
        'grid-cols-2': (store.teams?.length ?? 0) === 2,
        'grid-cols-3': (store.teams?.length ?? 0) === 3,
        'grid-cols-2 lg:grid-cols-4': (store.teams?.length ?? 0) === 4,
        'grid-cols-3 lg:grid-cols-5': (store.teams?.length ?? 0) === 5,
        'grid-cols-3 lg:grid-cols-6': (store.teams?.length ?? 0) >= 6,
      }"
    >
      <div
        v-for="team in store.teams"
        :key="team.id"
        class="bg-white border border-neutral-200 overflow-hidden shadow-sm"
      >
        <!-- カラーバー -->
        <div class="h-1.5" :class="ts(team.id).bar" />

        <!-- チームヘッダー -->
        <div class="px-4 py-3 border-b border-neutral-100">
          <div class="flex items-start justify-between mb-2">
            <span class="text-sm font-bold tracking-wider" :class="ts(team.id).labelClass">TEAM {{ team.id }}</span>
            <div class="text-right">
              <div class="text-xl font-black font-mono leading-none text-neutral-900">{{ team.total_score }}</div>
              <div class="text-xs text-neutral-400 mt-0.5">{{ team.players.length }}名</div>
            </div>
          </div>
          <!-- スコアバー -->
          <div class="h-1.5 bg-neutral-100 overflow-hidden rounded-full">
            <div
              class="h-full transition-all duration-700 ease-out rounded-full"
              :class="ts(team.id).bar"
              :style="{ width: barPct(team.total_score) }"
            />
          </div>
        </div>

        <!-- プレイヤーリスト -->
        <ul>
          <li
            v-for="player in team.players"
            :key="player.id"
            class="flex items-center gap-2.5 px-4 py-2.5 border-b border-neutral-50 last:border-b-0"
            :class="{ [ts(team.id).softBg]: player.id === store.myPlayerId }"
          >
            <span
              class="shrink-0 text-xs font-bold px-1.5 py-0.5 border leading-tight"
              :class="[RANK_COLORS[player.rank].bg, RANK_COLORS[player.rank].text, RANK_COLORS[player.rank].border]"
            >{{ player.rank }}</span>
            <span class="flex-1 text-sm text-neutral-800 font-medium truncate">{{ getDisplayName(player) }}</span>
            <span v-if="player.id === store.myPlayerId" class="shrink-0 text-sm" :class="ts(team.id).labelClass">★</span>
          </li>
        </ul>
      </div>
    </div>

    <!-- バランス比較バー -->
    <div class="bg-white border border-neutral-200 shadow-sm px-5 py-4">
      <p class="text-xs font-semibold tracking-widest uppercase text-neutral-400 mb-4">戦闘力バランス</p>
      <div class="space-y-3">
        <div v-for="team in store.teams" :key="team.id" class="flex items-center gap-3">
          <span class="w-16 text-xs font-semibold shrink-0" :class="ts(team.id).labelClass">TEAM {{ team.id }}</span>
          <div class="flex-1 h-5 bg-neutral-100 overflow-hidden rounded">
            <div
              class="h-full flex items-center justify-end pr-2 transition-all duration-700 ease-out rounded"
              :class="ts(team.id).bar"
              :style="{ width: barPct(team.total_score) }"
            >
              <span class="text-white text-xs font-bold font-mono leading-none">{{ team.total_score }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 未実行 -->
  <div v-else class="h-full flex flex-col items-center justify-center min-h-[300px] text-center px-8">
    <div class="text-5xl font-black text-neutral-150 tracking-tighter mb-4 select-none" style="color:#e5e7eb">TEAM</div>
    <p class="text-sm text-neutral-400 font-medium">チーム分けがまだ実行されていません</p>
    <p class="text-xs text-neutral-300 mt-1.5">管理者がチーム分けを実行すると結果が表示されます</p>
  </div>
</template>
