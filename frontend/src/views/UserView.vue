<!--
  UserView.vue - 一般ユーザー画面
  レイアウト: h-screen 固定（スクロールなし）
    [ヘッダー 48px]
    [ボディ flex-1]
      [左パネル 320px: 登録 + 参加者リスト]
      [右パネル flex-1: チーム分け結果]
-->
<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlayersStore } from '@/stores/players'
import PlayerForm from '@/components/PlayerForm.vue'
import PlayerList from '@/components/PlayerList.vue'
import TeamResult from '@/components/TeamResult.vue'

const store = usePlayersStore()

const showEditForm = ref(false)
const isNotRegistered = computed(() => !store.myPlayer)
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden bg-neutral-50">

    <!-- ══ ヘッダー 48px ══ -->
    <header class="flex-none h-12 bg-white border-b border-neutral-200 flex items-center px-4">
      <h1 class="text-sm font-semibold text-neutral-900 tracking-tight">
        チーム決め<span class="text-accent-600">一発</span>くん
      </h1>
      <div class="flex-1" />
      <div class="flex items-center gap-1.5 text-xs text-neutral-400">
        <span class="font-mono font-semibold text-neutral-900 text-base">{{ store.playerCount }}</span>
        名参加中
      </div>
    </header>

    <!-- ══ ボディ ══ -->
    <div class="flex-1 flex overflow-hidden">

      <!-- ── 左パネル: 登録 + 参加者リスト ── -->
      <aside class="w-80 flex-none flex flex-col border-r border-neutral-200 bg-white overflow-hidden">

        <!-- 登録状態に応じて切り替え -->
        <div class="flex-none border-b border-neutral-100">

          <!-- 未登録: 登録フォーム -->
          <div v-if="isNotRegistered" class="px-4 py-4">
            <p class="text-xs text-neutral-500 mb-3">名前とランクを入力して参加登録してください</p>
            <PlayerForm
              :isOwner="false"
              :editTarget="null"
              @done="() => {}"
              @cancel="() => {}"
            />
          </div>

          <!-- 登録済み: 自分のエントリ表示 -->
          <div v-else class="px-4 py-3">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-xs text-neutral-400 mb-0.5">登録済み</p>
                <p class="text-sm font-semibold text-neutral-900">{{ store.myPlayer?.name }}</p>
                <p class="text-xs text-neutral-500 mt-0.5">{{ store.myPlayer?.rank }}</p>
              </div>
              <div class="flex items-center gap-2">
                <span class="w-7 h-7 flex items-center justify-center bg-emerald-100 text-emerald-600 text-xs font-bold">✓</span>
                <button
                  @click="showEditForm = !showEditForm"
                  class="btn-secondary text-xs px-2 py-1 h-7"
                >
                  {{ showEditForm ? '閉じる' : '変更' }}
                </button>
              </div>
            </div>

            <!-- 編集フォーム -->
            <div v-if="showEditForm" class="mt-3 pt-3 border-t border-neutral-100">
              <PlayerForm
                :isOwner="false"
                :editTarget="store.myPlayer ?? null"
                @done="() => { showEditForm = false }"
                @cancel="() => { showEditForm = false }"
              />
            </div>
          </div>
        </div>

        <!-- 参加者リスト（スクロール） -->
        <div class="flex-1 overflow-y-auto">
          <PlayerList :isOwner="false" />
        </div>
      </aside>

      <!-- ── 右パネル: チーム分け結果 ── -->
      <main class="flex-1 overflow-y-auto bg-neutral-50">
        <TeamResult />
      </main>
    </div>

  </div>
</template>
