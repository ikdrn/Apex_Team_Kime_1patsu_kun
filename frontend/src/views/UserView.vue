<!--
  UserView.vue - 一般ユーザー向けページ（/ にアクセスした場合）

  【役割】
  一般プレイヤーが自分の名前とランクを登録する画面。
  - 既に登録済みの場合は自分のエントリを編集できる
  - まだ登録していない場合は新規登録フォームを表示
  - チーム分け結果（管理者が実行後）を表示する

  【権限制限】
  - チーム分け実行ボタンは表示しない
  - 他の人のエントリは編集・削除できない
-->
<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlayersStore } from '@/stores/players'
import PlayerForm from '@/components/PlayerForm.vue'
import PlayerList from '@/components/PlayerList.vue'
import TeamResult from '@/components/TeamResult.vue'

const store = usePlayersStore()

// 登録済みの場合に編集フォームを表示するかどうか
const showEditForm = ref(false)

// 自分がまだ登録していないかどうか
const isNotRegistered = computed(() => !store.myPlayer)

// 登録完了コールバック
function onRegistered() {
  showEditForm.value = false
}
</script>

<template>
  <div class="min-h-screen bg-neutral-50">
    <!-- ══════════════════════════════════════════════════════════
         ヘッダー
         ══════════════════════════════════════════════════════════ -->
    <header class="bg-white border-b border-neutral-200">
      <div class="max-w-5xl mx-auto px-4 sm:px-6 py-4 flex items-center justify-between">
        <div>
          <!-- ロゴ/タイトル部分 -->
          <h1 class="text-xl font-bold tracking-tight text-neutral-900">
            チーム決め<span class="text-accent-600">一発</span>くん
          </h1>
        </div>

        <!-- 参加者数バッジ -->
        <div class="text-right">
          <div class="text-2xl font-bold tabular-nums text-neutral-900">
            {{ store.playerCount }}
          </div>
          <div class="text-xs text-neutral-400">参加者</div>
        </div>
      </div>
    </header>

    <!-- ══════════════════════════════════════════════════════════
         メインコンテンツ
         ══════════════════════════════════════════════════════════ -->
    <main class="max-w-5xl mx-auto px-4 sm:px-6 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">

        <!-- ────────────────────────────────────────────────────
             左カラム: 登録フォーム + 参加者リスト
             ──────────────────────────────────────────────────── -->
        <div class="lg:col-span-1 space-y-6">

          <!-- ── 自分の登録状態に応じてフォームを切り替え ── -->

          <!-- ケース1: 未登録 → 新規登録フォームを表示 -->
          <div v-if="isNotRegistered">
            <div class="mb-3">
              <p class="text-xs font-semibold text-neutral-500 uppercase tracking-wider">
                参加登録
              </p>
              <p class="text-xs text-neutral-400 mt-1">
                あなたの名前とランクを登録してください
              </p>
            </div>
            <PlayerForm
              :isOwner="false"
              :editTarget="null"
              @done="onRegistered"
              @cancel="() => {}"
            />
          </div>

          <!-- ケース2: 登録済み → 自分のエントリを表示、編集ボタン -->
          <div v-else>
            <div class="card border-l-4 border-l-emerald-500">
              <div class="flex items-start justify-between gap-3">
                <div>
                  <p class="text-xs text-neutral-500 mb-1 font-medium">あなたの登録情報</p>
                  <p class="font-semibold text-neutral-900">
                    {{ store.myPlayer?.name }}
                  </p>
                  <p class="text-sm text-neutral-600 mt-0.5">
                    {{ store.myPlayer?.rank }}
                  </p>
                </div>
                <!-- 登録済みの緑チェックアイコン -->
                <div class="shrink-0 w-8 h-8 bg-emerald-100 flex items-center justify-center text-emerald-600 text-sm font-bold">
                  ✓
                </div>
              </div>
              <!-- 編集ボタン -->
              <button
                @click="showEditForm = !showEditForm"
                class="mt-3 w-full btn-secondary text-xs py-1.5"
              >
                {{ showEditForm ? 'キャンセル' : '情報を変更する' }}
              </button>
            </div>

            <!-- 編集フォーム（「情報を変更する」押下時に展開） -->
            <Transition
              enter-active-class="transition-all duration-200 ease-out"
              enter-from-class="opacity-0 -translate-y-1"
              leave-active-class="transition-all duration-150 ease-in"
              leave-to-class="opacity-0 -translate-y-1"
            >
              <div v-if="showEditForm" class="mt-3">
                <PlayerForm
                  :isOwner="false"
                  :editTarget="store.myPlayer ?? null"
                  @done="() => { showEditForm = false }"
                  @cancel="() => { showEditForm = false }"
                />
              </div>
            </Transition>
          </div>

          <!-- ── 参加者リスト ── -->
          <div class="card">
            <PlayerList :isOwner="false" />
          </div>
        </div>

        <!-- ────────────────────────────────────────────────────
             右カラム: チーム分け結果
             ──────────────────────────────────────────────────── -->
        <div class="lg:col-span-2">
          <div class="card">
            <TeamResult />
          </div>

          <!-- ローディングオーバーレイ -->
          <div
            v-if="store.isLoading"
            class="mt-4 flex items-center gap-2 text-sm text-neutral-500"
          >
            <svg class="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
            読み込み中...
          </div>
        </div>

      </div>
    </main>

    <!-- ── フッター ── -->
    <footer class="mt-16 py-6 border-t border-neutral-200">
      <div class="max-w-5xl mx-auto px-4 sm:px-6 text-center">
        <p class="text-xs text-neutral-300">
          チーム決め一発くん
        </p>
      </div>
    </footer>
  </div>
</template>
