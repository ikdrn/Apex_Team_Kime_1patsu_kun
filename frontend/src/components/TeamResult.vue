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

const teamAccents = [
  { bar: '#2563eb', label: 'text-blue-600',   light: 'bg-blue-50',   topBorder: 'border-t-blue-600'   },
  { bar: '#ea580c', label: 'text-orange-600',  light: 'bg-orange-50', topBorder: 'border-t-orange-500' },
  { bar: '#059669', label: 'text-emerald-600', light: 'bg-emerald-50',topBorder: 'border-t-emerald-600'},
  { bar: '#dc2626', label: 'text-red-600',     light: 'bg-red-50',    topBorder: 'border-t-red-600'    },
  { bar: '#7c3aed', label: 'text-violet-600',  light: 'bg-violet-50', topBorder: 'border-t-violet-600' },
  { bar: '#d97706', label: 'text-amber-600',   light: 'bg-amber-50',  topBorder: 'border-t-amber-500'  },
]

function accent(id: number) {
  return teamAccents[(id - 1) % teamAccents.length]
}

function barWidth(score: number) {
  return `${(score / maxScore.value) * 100}%`
}
</script>

<template>
  <!-- チーム分け済み -->
  <div v-if="store.hasTeams" class="p-6 space-y-6">

    <!-- ヘッダー行 -->
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-neutral-400 tracking-widest uppercase">チーム分け結果</span>
      <div class="flex items-center gap-2 text-xs">
        <span class="text-neutral-400">スコア差</span>
        <span
          class="font-mono font-semibold"
          :class="scoreDiff <= 5 ? 'text-emerald-600' : scoreDiff <= 15 ? 'text-amber-600' : 'text-red-500'"
        >{{ scoreDiff }}</span>
        <span
          class="px-1.5 py-0.5 text-[10px] font-semibold"
          :class="scoreDiff <= 5 ? 'bg-emerald-50 text-emerald-700' : scoreDiff <= 15 ? 'bg-amber-50 text-amber-700' : 'bg-red-50 text-red-600'"
        >{{ scoreDiff <= 5 ? '均衡' : scoreDiff <= 15 ? '標準' : '偏り' }}</span>
      </div>
    </div>

    <!-- チームカード グリッド -->
    <div
      class="grid gap-3"
      :class="{
        'grid-cols-2': (store.teams?.length ?? 0) <= 2,
        'grid-cols-3': (store.teams?.length ?? 0) === 3,
        'grid-cols-2 lg:grid-cols-4': (store.teams?.length ?? 0) >= 4,
      }"
    >
      <div
        v-for="team in store.teams"
        :key="team.id"
        class="bg-white border border-neutral-200 border-t-2 flex flex-col"
        :class="accent(team.id).topBorder"
      >
        <!-- チームヘッダー -->
        <div class="px-3 py-2.5 border-b border-neutral-100 flex items-center justify-between">
          <span class="text-xs font-bold tracking-widest uppercase" :class="accent(team.id).label">
            TEAM {{ team.id }}
          </span>
          <div class="flex items-center gap-2">
            <span class="text-xs font-mono font-bold text-neutral-700">{{ team.total_score }}</span>
            <span class="text-[10px] text-neutral-400">{{ team.players.length }}名</span>
          </div>
        </div>

        <!-- スコアバー -->
        <div class="px-3 py-1.5 border-b border-neutral-100">
          <div class="h-1 bg-neutral-100 overflow-hidden">
            <div
              class="h-full transition-all duration-700 ease-out"
              :style="{ width: barWidth(team.total_score), backgroundColor: accent(team.id).bar }"
            />
          </div>
        </div>

        <!-- プレイヤー一覧 -->
        <ul class="flex-1 divide-y divide-neutral-50">
          <li
            v-for="player in team.players"
            :key="player.id"
            class="flex items-center gap-2 px-3 py-2"
            :class="{ [accent(team.id).light]: player.id === store.myPlayerId }"
          >
            <span
              class="shrink-0 text-[9px] font-semibold px-1 py-0.5 border"
              :class="[RANK_COLORS[player.rank].bg, RANK_COLORS[player.rank].text, RANK_COLORS[player.rank].border]"
            >{{ player.rank.slice(0, 3) }}</span>
            <span class="flex-1 text-xs text-neutral-800 font-medium truncate">{{ getDisplayName(player) }}</span>
            <span v-if="player.id === store.myPlayerId" class="shrink-0 text-[10px] font-bold" :class="accent(team.id).label">★</span>
          </li>
        </ul>
      </div>
    </div>

    <!-- 全チームバー比較 -->
    <div class="bg-white border border-neutral-200 px-4 py-3">
      <p class="text-[10px] font-semibold text-neutral-400 tracking-widest uppercase mb-3">戦闘力バランス</p>
      <div class="space-y-2">
        <div v-for="team in store.teams" :key="team.id" class="flex items-center gap-3">
          <span class="w-12 text-[10px] font-semibold tracking-wide uppercase shrink-0" :class="accent(team.id).label">
            TEAM {{ team.id }}
          </span>
          <div class="flex-1 h-4 bg-neutral-100 overflow-hidden">
            <div
              class="h-full flex items-center justify-end pr-1.5 transition-all duration-700 ease-out"
              :style="{ width: barWidth(team.total_score), backgroundColor: accent(team.id).bar }"
            >
              <span class="text-white text-[10px] font-bold font-mono leading-none">{{ team.total_score }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 未実行 -->
  <div v-else class="flex flex-col items-center justify-center h-full min-h-64 text-center px-8">
    <p class="text-4xl font-black text-neutral-100 tracking-tighter mb-3">TEAM</p>
    <p class="text-xs text-neutral-400">チーム分けはまだ実行されていません</p>
    <p class="text-[10px] text-neutral-300 mt-1">管理者がチーム分けを実行すると結果が表示されます</p>
  </div>
</template>
