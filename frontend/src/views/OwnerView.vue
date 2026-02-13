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
  if (store.playerCount < 2) { alert('2人以上必要です'); return }
  isBalancing.value = true
  try { await store.balanceTeams() } finally { isBalancing.value = false }
}

async function handleDisband() {
  if (!confirm('チームを解散しますか？')) return
  await store.disbandTeams()
}

async function handleTeamCountChange(e: Event) {
  await store.updateConfig(parseInt((e.target as HTMLSelectElement).value))
}

function toggleSelf() { showSelfAddForm.value = !showSelfAddForm.value; showOtherAddForm.value = false }
function toggleOther() { showOtherAddForm.value = !showOtherAddForm.value; showSelfAddForm.value = false }
function closeForm() { showSelfAddForm.value = false; showOtherAddForm.value = false }
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden bg-neutral-100">

    <!-- ══ ヘッダー ══ -->
    <header class="flex-none h-14 bg-neutral-900 flex items-center px-5 gap-3 shadow-sm">
      <h1 class="text-base font-bold text-white tracking-tight">
        チーム決め<span class="text-accent-400">一発</span>くん
      </h1>
      <span class="text-[10px] font-semibold tracking-widest uppercase text-neutral-400 border border-neutral-700 px-1.5 py-0.5 leading-tight">
        OWNER
      </span>
      <div class="flex-1" />
      <div class="flex items-center gap-5">
        <div class="text-center">
          <div class="text-xl font-bold font-mono text-white leading-none">{{ store.playerCount }}</div>
          <div class="text-[10px] text-neutral-500 mt-0.5">参加者</div>
        </div>
        <div class="text-center">
          <div class="text-xl font-bold font-mono text-white leading-none">{{ store.config.team_count }}</div>
          <div class="text-[10px] text-neutral-500 mt-0.5">チーム</div>
        </div>
      </div>
    </header>

    <!-- ══ ツールバー ══ -->
    <div class="flex-none h-12 bg-white border-b border-neutral-200 flex items-center px-5 gap-2.5 shadow-sm">
      <label class="text-xs text-neutral-500 shrink-0">チーム数</label>
      <select :value="store.config.team_count" @change="handleTeamCountChange" class="select-field w-16 py-1 text-sm" :disabled="store.isLoading">
        <option v-for="n in [2,3,4,5,6]" :key="n" :value="n">{{ n }}</option>
      </select>

      <div class="w-px h-5 bg-neutral-200 mx-0.5" />

      <button @click="toggleSelf" class="btn-secondary" :disabled="selfAdded" :class="{ 'opacity-40 cursor-not-allowed': selfAdded }">
        自分を追加 <span v-if="selfAdded" class="text-emerald-600">✓</span>
      </button>
      <button @click="toggleOther" class="btn-secondary">他人を追加</button>

      <div class="flex-1" />

      <button v-if="store.hasTeams" @click="handleDisband" class="btn-secondary text-neutral-500" :disabled="store.isLoading">
        チーム解散
      </button>
      <button @click="handleBalance" :disabled="isBalancing || store.isLoading || store.playerCount < 2" class="btn-accent">
        <svg v-if="isBalancing" class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
        </svg>
        {{ isBalancing ? '実行中...' : 'チーム分けを実行' }}
      </button>
    </div>

    <!-- ══ ボディ ══ -->
    <div class="flex-1 flex overflow-hidden">

      <!-- 左パネル -->
      <aside class="w-[300px] flex-none flex flex-col bg-white border-r border-neutral-200 overflow-hidden">

        <!-- 追加フォーム（インライン） -->
        <Transition
          enter-active-class="transition-all duration-200 ease-out"
          enter-from-class="opacity-0 -translate-y-1"
          leave-active-class="transition-all duration-150 ease-in"
          leave-to-class="opacity-0 -translate-y-1"
        >
          <div v-if="showSelfAddForm || showOtherAddForm" class="flex-none border-b border-neutral-200 bg-neutral-50 px-4 py-4">
            <div class="flex items-center justify-between mb-3">
              <p class="text-xs font-semibold text-neutral-600">
                {{ showSelfAddForm ? '自分を追加（登録後は非活性になります）' : '参加者を追加' }}
              </p>
              <button @click="closeForm" class="text-neutral-400 hover:text-neutral-700 text-lg leading-none">×</button>
            </div>
            <PlayerForm
              :isOwner="showOtherAddForm"
              :editTarget="null"
              @done="closeForm"
              @cancel="closeForm"
            />
          </div>
        </Transition>

        <!-- 警告 -->
        <div
          v-if="store.playerCount > 0 && store.playerCount < store.config.team_count"
          class="flex-none px-4 py-2.5 bg-amber-50 border-b border-amber-200 text-xs text-amber-700 flex gap-2"
        >
          <span>⚠</span>
          <span>参加者({{ store.playerCount }})がチーム数({{ store.config.team_count }})より少ない</span>
        </div>

        <!-- リスト（スクロール） -->
        <div class="flex-1 overflow-y-auto">
          <PlayerList :isOwner="true" />
        </div>
      </aside>

      <!-- 右パネル -->
      <main class="flex-1 overflow-y-auto">
        <div v-if="!store.hasTeams && store.playerCount >= 2" class="m-5 p-4 bg-white border border-neutral-200 shadow-sm">
          <p class="text-sm font-semibold text-neutral-700">{{ store.playerCount }} 名が登録されました</p>
          <p class="text-xs text-neutral-400 mt-1">ツールバーの「チーム分けを実行」を押すと {{ store.config.team_count }} チームに分けます</p>
        </div>
        <TeamResult />
      </main>
    </div>

  </div>

  <!-- ローディングインジケーター -->
  <Transition enter-active-class="transition-opacity duration-150" leave-active-class="transition-opacity duration-150" enter-from-class="opacity-0" leave-to-class="opacity-0">
    <div v-if="store.isLoading" class="fixed bottom-4 right-4 flex items-center gap-2 bg-neutral-900/90 text-white text-xs px-3 py-2 shadow-lg backdrop-blur-sm">
      <svg class="animate-spin w-3 h-3" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
      </svg>
      同期中
    </div>
  </Transition>
</template>
