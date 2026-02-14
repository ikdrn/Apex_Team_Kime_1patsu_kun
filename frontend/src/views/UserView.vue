<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { usePlayersStore } from '@/stores/players'
import PlayerForm from '@/components/PlayerForm.vue'
import PlayerList from '@/components/PlayerList.vue'
import TeamResult from '@/components/TeamResult.vue'

const store = usePlayersStore()
const showEditForm = ref(false)
const isNotRegistered = computed(() => !store.myPlayer)

onMounted(() => store.startPolling())
onUnmounted(() => store.stopPolling())
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden bg-neutral-100">

    <!-- ══ ヘッダー ══ -->
    <header class="flex-none h-14 bg-white border-b border-neutral-200 flex items-center px-5 shadow-sm">
      <h1 class="text-base font-bold text-neutral-900 tracking-tight">
        チーム決め<span class="text-accent-600">一発</span>くん
      </h1>
      <div class="flex-1" />
      <div class="flex items-center gap-1.5">
        <span class="text-2xl font-bold font-mono text-neutral-900 leading-none">{{ store.playerCount }}</span>
        <span class="text-sm text-neutral-400">名参加中</span>
      </div>
    </header>

    <!-- ══ ボディ ══ -->
    <div class="flex-1 flex overflow-hidden">

      <!-- 左パネル -->
      <aside class="w-[300px] flex-none flex flex-col bg-white border-r border-neutral-200 overflow-hidden">

        <!-- 登録エリア -->
        <div class="flex-none border-b border-neutral-200">
          <!-- 未登録 -->
          <div v-if="isNotRegistered" class="px-4 py-4">
            <p class="text-xs font-semibold text-neutral-500 mb-3">名前とランクを入力して参加登録してください</p>
            <PlayerForm :addForOther="false" :editTarget="null" @done="() => {}" @cancel="() => {}" />
          </div>
          <!-- 登録済み -->
          <div v-else class="px-4 py-3">
            <div class="flex items-center gap-3">
              <div class="flex-1 min-w-0">
                <p class="text-xs text-neutral-400 mb-0.5 font-medium">登録済み</p>
                <p class="text-sm font-bold text-neutral-900 truncate">{{ store.myPlayer?.name }}</p>
                <p class="text-xs text-neutral-500 mt-0.5">{{ store.myPlayer?.rank }}</p>
              </div>
              <div class="flex items-center gap-2 shrink-0">
                <span class="w-8 h-8 flex items-center justify-center bg-emerald-100 text-emerald-600 text-sm font-bold">✓</span>
                <button @click="showEditForm = !showEditForm" class="btn-secondary text-xs">
                  {{ showEditForm ? '閉じる' : '変更' }}
                </button>
              </div>
            </div>
            <Transition enter-active-class="transition-all duration-200" enter-from-class="opacity-0" leave-active-class="transition-all duration-150" leave-to-class="opacity-0">
              <div v-if="showEditForm" class="mt-3 pt-3 border-t border-neutral-100">
                <PlayerForm :addForOther="false" :editTarget="store.myPlayer ?? null" @done="() => { showEditForm = false }" @cancel="() => { showEditForm = false }" />
              </div>
            </Transition>
          </div>
        </div>

        <!-- 参加者リスト（スクロール） -->
        <div class="flex-1 overflow-y-auto">
          <PlayerList :isOwner="false" />
        </div>
      </aside>

      <!-- 右パネル -->
      <main class="flex-1 overflow-y-auto">
        <TeamResult />
      </main>
    </div>

  </div>
</template>
