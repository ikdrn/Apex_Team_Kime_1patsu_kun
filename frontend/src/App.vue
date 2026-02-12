<!--
  App.vue - アプリケーションのルートコンポーネント

  【役割】
  - Vue Routerの <RouterView> を置くレイアウトの土台
  - エラートースト（一時的なエラーメッセージ）の表示
  - アプリ起動時のデータ初期化
-->
<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { RouterView } from 'vue-router'
import { usePlayersStore } from '@/stores/players'

const store = usePlayersStore()

// ポーリングタイマーID
let pollTimer: ReturnType<typeof setInterval> | null = null

// アプリ起動時にサーバーからデータを読み込み、3秒ごとに自動更新する
onMounted(async () => {
  await store.initialize()

  // 3秒ごとにプレイヤーリストとチーム結果をバックグラウンド更新
  // （ユーザーが操作中でもリアルタイムに反映させるため）
  pollTimer = setInterval(async () => {
    // ローディング中・エラー表示中は更新を一時停止して UX を乱さない
    if (store.isLoading) return
    await Promise.all([
      store.fetchPlayers(),
      store.fetchTeams(),
    ])
  }, 3000)
})

onUnmounted(() => {
  if (pollTimer !== null) {
    clearInterval(pollTimer)
  }
})
</script>

<template>
  <!--
    アプリ全体のラッパー
    min-h-screen: 最低でも画面の高さ分を占有する
  -->
  <div class="min-h-screen bg-neutral-50">
    <!--
      RouterView: 現在のURLに対応するコンポーネントがここに表示される
      / → UserView.vue
      /owner → OwnerView.vue
    -->
    <RouterView />

    <!--
      エラートースト（Transitionで滑らかに表示/非表示）
      errorMessage がある場合のみ表示される
    -->
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 translate-y-2"
      leave-active-class="transition-all duration-200 ease-in"
      leave-to-class="opacity-0 translate-y-2"
    >
      <div
        v-if="store.errorMessage"
        class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50
               flex items-center gap-3
               bg-neutral-900 text-white
               px-5 py-3 text-sm font-medium
               shadow-lg"
        role="alert"
      >
        <!-- エラーアイコン（シンプルな × 文字） -->
        <span class="text-red-400 font-bold text-base leading-none">!</span>
        {{ store.errorMessage }}
        <button
          @click="store.clearError()"
          class="ml-2 text-neutral-400 hover:text-white transition-colors"
          aria-label="エラーを閉じる"
        >
          ✕
        </button>
      </div>
    </Transition>
  </div>
</template>
