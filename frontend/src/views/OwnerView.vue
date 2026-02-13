<!--
  OwnerView.vue - 管理者画面
  レイアウト: h-screen 固定（スクロールなし）
    [ヘッダー 48px]
    [ツールバー 48px]
    [ボディ flex-1]
      [左パネル 288px: 参加者管理]
      [右パネル flex-1: チーム分け結果]
-->
<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlayersStore } from '@/stores/players'
import PlayerForm from '@/components/PlayerForm.vue'
import PlayerList from '@/components/PlayerList.vue'
import TeamResult from '@/components/TeamResult.vue'

const store = usePlayersStore()

const isBalancing = ref(false)
const showSelfAddForm = ref(false)
const showOtherAddForm = ref(false)

const selfAdded = computed(() => store.myPlayerId !== null)

async function handleBalance() {
  if (store.playerCount < 2) {
    alert('チーム分けには2人以上の参加者が必要です')
    return
  }
  isBalancing.value = true
  try {
    await store.balanceTeams()
  } finally {
    isBalancing.value = false
  }
}

async function handleDisband() {
  if (!confirm('チームを解散しますか？')) return
  await store.disbandTeams()
}

async function handleTeamCountChange(e: Event) {
  const count = parseInt((e.target as HTMLSelectElement).value)
  await store.updateConfig(count)
}

function toggleSelfForm() {
  showSelfAddForm.value = !showSelfAddForm.value
  showOtherAddForm.value = false
}

function toggleOtherForm() {
  showOtherAddForm.value = !showOtherAddForm.value
  showSelfAddForm.value = false
}
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden bg-neutral-50">

    <!-- ══ ヘッダー 48px ══ -->
    <header class="flex-none h-12 bg-neutral-950 flex items-center px-4 gap-3">
      <h1 class="text-sm font-semibold text-white tracking-tight">
        チーム決め<span class="text-accent-400">一発</span>くん
      </h1>
      <span class="text-[10px] font-semibold tracking-widest uppercase text-neutral-400 border border-neutral-700 px-1.5 py-0.5">
        OWNER
      </span>
      <div class="flex-1" />
      <!-- サマリー -->
      <div class="flex items-center gap-4 text-right">
        <div>
          <span class="text-lg font-bold tabular-nums text-white font-mono">{{ store.playerCount }}</span>
          <span class="text-xs text-neutral-500 ml-1">人</span>
        </div>
        <div>
          <span class="text-lg font-bold tabular-nums text-white font-mono">{{ store.config.team_count }}</span>
          <span class="text-xs text-neutral-500 ml-1">チーム</span>
        </div>
      </div>
    </header>

    <!-- ══ ツールバー 48px ══ -->
    <div class="flex-none h-12 bg-white border-b border-neutral-200 flex items-center px-4 gap-2">
      <!-- チーム数 -->
      <label class="text-xs text-neutral-500 whitespace-nowrap">チーム数</label>
      <select
        :value="store.config.team_count"
        @change="handleTeamCountChange"
        class="select-field w-16 text-xs py-1"
        :disabled="store.isLoading"
      >
        <option v-for="n in [2,3,4,5,6]" :key="n" :value="n">{{ n }}</option>
      </select>

      <div class="w-px h-5 bg-neutral-200 mx-1" />

      <!-- 参加者追加ボタン群 -->
      <button
        @click="toggleSelfForm"
        class="btn-secondary text-xs px-3 py-1.5 h-8"
        :disabled="selfAdded"
        :class="{ 'opacity-40 cursor-not-allowed': selfAdded }"
        :title="selfAdded ? '登録済み' : '自分を追加'"
      >
        自分を追加
        <span v-if="selfAdded" class="text-emerald-600 ml-0.5">✓</span>
        <span v-else-if="showSelfAddForm" class="text-neutral-400 ml-0.5">▲</span>
      </button>

      <button
        @click="toggleOtherForm"
        class="btn-secondary text-xs px-3 py-1.5 h-8"
      >
        他人を追加
        <span v-if="showOtherAddForm" class="text-neutral-400 ml-0.5">▲</span>
      </button>

      <div class="flex-1" />

      <!-- チーム解散 -->
      <button
        v-if="store.hasTeams"
        @click="handleDisband"
        class="btn-secondary text-xs px-3 py-1.5 h-8 text-neutral-500"
        :disabled="store.isLoading"
      >
        解散
      </button>

      <!-- チーム分け実行 -->
      <button
        @click="handleBalance"
        :disabled="isBalancing || store.isLoading || store.playerCount < 2"
        class="btn-accent text-xs px-4 py-1.5 h-8"
      >
        <svg v-if="isBalancing" class="animate-spin w-3.5 h-3.5" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
        </svg>
        {{ isBalancing ? '実行中...' : 'チーム分け' }}
      </button>
    </div>

    <!-- ══ ボディ: 左パネル + 右パネル ══ -->
    <div class="flex-1 flex overflow-hidden">

      <!-- ── 左パネル: 参加者管理 ── -->
      <aside class="w-72 flex-none flex flex-col border-r border-neutral-200 bg-white overflow-hidden">

        <!-- 追加フォームエリア（折りたたみ） -->
        <div
          v-if="showSelfAddForm || showOtherAddForm"
          class="flex-none border-b border-neutral-100 bg-neutral-50 px-4 py-3"
        >
          <p class="text-xs text-neutral-500 mb-2">
            {{ showSelfAddForm ? '自分の情報を登録（登録後はボタンが非活性になります）' : '参加者を追加' }}
          </p>
          <PlayerForm
            :isOwner="showOtherAddForm"
            :editTarget="null"
            @done="() => { showSelfAddForm = false; showOtherAddForm = false }"
            @cancel="() => { showSelfAddForm = false; showOtherAddForm = false }"
          />
        </div>

        <!-- 参加者リスト（スクロール） -->
        <div class="flex-1 overflow-y-auto">
          <!-- 警告バナー -->
          <div
            v-if="store.playerCount > 0 && store.playerCount < store.config.team_count"
            class="px-4 py-2 bg-amber-50 border-b border-amber-100 text-xs text-amber-700 flex items-start gap-1.5"
          >
            <span class="shrink-0">⚠</span>
            <span>参加者（{{ store.playerCount }}名）がチーム数（{{ store.config.team_count }}）より少ないです</span>
          </div>
          <PlayerList :isOwner="true" />
        </div>
      </aside>

      <!-- ── 右パネル: チーム分け結果 ── -->
      <main class="flex-1 overflow-y-auto bg-neutral-50">
        <!-- 待機状態のガイダンス -->
        <div
          v-if="!store.hasTeams && store.playerCount >= 2"
          class="m-6 px-4 py-3 bg-white border border-neutral-200 text-sm"
        >
          <p class="font-medium text-neutral-800">
            {{ store.playerCount }} 名登録済み — ツールバーの「チーム分け」を実行してください
          </p>
        </div>

        <TeamResult />
      </main>
    </div>

  </div>

  <!-- エラー/ローディングインジケーター（右下固定） -->
  <div
    v-if="store.isLoading"
    class="fixed bottom-4 right-4 flex items-center gap-2 bg-neutral-900 text-white text-xs px-3 py-2"
  >
    <svg class="animate-spin w-3 h-3" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
    </svg>
    同期中
  </div>
</template>
