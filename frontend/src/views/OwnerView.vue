<!--
  OwnerView.vue - 管理者向けページ（/owner にアクセスした場合）

  【役割】
  管理者（オーナー）が全プレイヤーを管理し、チーム分けを実行する画面。

  【管理者ができること】
  1. 自分を参加者として追加（1回のみ）
  2. 他のプレイヤーの追加・編集・削除
  3. 各プレイヤーの戦闘力補正（±5まで）
  4. チーム数の変更（2〜6チーム）
  5. チーム分けの実行（均衡化アルゴリズム発動）
  6. チームの解散
-->
<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlayersStore } from '@/stores/players'
import PlayerForm from '@/components/PlayerForm.vue'
import PlayerList from '@/components/PlayerList.vue'
import TeamResult from '@/components/TeamResult.vue'

const store = usePlayersStore()

// チーム分け実行中のローディング状態
const isBalancing = ref(false)

// 「自分を追加」フォームの表示/非表示
const showSelfAddForm = ref(false)

// 「他人を追加」フォームの表示/非表示
const showOtherAddForm = ref(false)

// 自分が既に追加済みかどうか
const selfAdded = computed(() => store.myPlayerId !== null)

// チーム分けを実行する
async function handleBalance() {
  if (store.playerCount < 2) {
    alert('チーム分けには2人以上の参加者が必要です')
    return
  }

  isBalancing.value = true
  try {
    await store.balanceTeams()
    // 成功したら結果セクションにスムーズスクロール
    setTimeout(() => {
      document.getElementById('team-result')?.scrollIntoView({ behavior: 'smooth' })
    }, 200)
  } finally {
    isBalancing.value = false
  }
}

// チームを解散する
async function handleDisband() {
  if (!confirm('チームを解散しますか？チーム分け結果がリセットされます。')) return
  await store.disbandTeams()
}

// チーム数変更ハンドラー
async function handleTeamCountChange(e: Event) {
  const count = parseInt((e.target as HTMLSelectElement).value)
  await store.updateConfig(count)
}

// 「自分を追加」完了時
function onSelfAddDone() {
  showSelfAddForm.value = false
}

// 「他人を追加」完了時
function onOtherAddDone() {
  showOtherAddForm.value = false
}
</script>

<template>
  <div class="min-h-screen bg-neutral-50">
    <!-- ══════════════════════════════════════════════════════════
         ヘッダー
         ══════════════════════════════════════════════════════════ -->
    <header class="bg-neutral-900 text-white">
      <div class="max-w-5xl mx-auto px-4 sm:px-6 py-4">
        <div class="flex items-center justify-between">
          <div>
            <div class="flex items-center gap-3">
              <h1 class="text-xl font-bold tracking-tight">
                チーム決め<span class="text-accent-400">一発</span>くん
              </h1>
              <!-- 管理者バッジ -->
              <span class="px-2 py-0.5 bg-accent-600 text-white text-xs font-semibold uppercase tracking-wider">
                OWNER
              </span>
            </div>
          </div>

          <!-- 統計サマリー -->
          <div class="hidden sm:flex gap-6 text-right">
            <div>
              <div class="text-2xl font-bold tabular-nums">{{ store.playerCount }}</div>
              <div class="text-xs text-neutral-400">参加者</div>
            </div>
            <div>
              <div class="text-2xl font-bold tabular-nums">{{ store.config.team_count }}</div>
              <div class="text-xs text-neutral-400">チーム数</div>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- ── 管理者コントロールバー ── -->
    <div class="bg-white border-b border-neutral-200 sticky top-0 z-10 shadow-sm">
      <div class="max-w-5xl mx-auto px-4 sm:px-6 py-3 flex flex-wrap items-center gap-3">

        <!-- チーム数セレクター -->
        <div class="flex items-center gap-2">
          <label class="text-xs font-semibold text-neutral-500 whitespace-nowrap">
            チーム数
          </label>
          <select
            :value="store.config.team_count"
            @change="handleTeamCountChange"
            class="select-field w-20 text-sm py-1.5"
            :disabled="store.isLoading"
          >
            <option value="2">2</option>
            <option value="3">3</option>
            <option value="4">4</option>
            <option value="5">5</option>
            <option value="6">6</option>
          </select>
        </div>

        <!-- 区切り線 -->
        <div class="hidden sm:block w-px h-6 bg-neutral-200"></div>

        <!-- 自分を追加ボタン（登録済みなら非活性） -->
        <button
          @click="showSelfAddForm = !showSelfAddForm; showOtherAddForm = false"
          class="btn-secondary text-xs py-1.5 px-3"
          :disabled="selfAdded"
          :class="{ 'opacity-40 cursor-not-allowed': selfAdded }"
          :title="selfAdded ? '自分は既に追加済みです' : ''"
        >
          {{ showSelfAddForm ? '▲ 閉じる' : '自分を追加' }}
          <span v-if="selfAdded" class="ml-1 text-emerald-600">✓</span>
        </button>

        <!-- 他人を追加ボタン -->
        <button
          @click="showOtherAddForm = !showOtherAddForm; showSelfAddForm = false"
          class="btn-secondary text-xs py-1.5 px-3"
        >
          {{ showOtherAddForm ? '▲ 閉じる' : '他人を追加' }}
        </button>

        <!-- スペーサー -->
        <div class="flex-1"></div>

        <!-- チーム解散ボタン（チーム分け済みの場合のみ表示） -->
        <button
          v-if="store.hasTeams"
          @click="handleDisband"
          :disabled="store.isLoading"
          class="text-xs py-1.5 px-3 border border-neutral-300 text-neutral-600
                 hover:bg-neutral-100 transition-colors"
        >
          チーム解散
        </button>

        <!-- チーム分け実行ボタン（最も目立つ位置に配置） -->
        <button
          @click="handleBalance"
          :disabled="isBalancing || store.isLoading || store.playerCount < 2"
          class="btn-accent gap-2"
        >
          <!-- 実行中はスピナーを表示 -->
          <svg
            v-if="isBalancing"
            class="animate-spin h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          {{ isBalancing ? 'チーム分け中...' : 'チーム分けを実行' }}
        </button>

      </div>

      <!-- ── 自分を追加フォーム ── -->
      <Transition
        enter-active-class="transition-all duration-200 ease-out overflow-hidden"
        enter-from-class="opacity-0 max-h-0"
        enter-to-class="opacity-100 max-h-96"
        leave-active-class="transition-all duration-150 ease-in overflow-hidden"
        leave-from-class="opacity-100 max-h-96"
        leave-to-class="opacity-0 max-h-0"
      >
        <div
          v-if="showSelfAddForm"
          class="border-t border-neutral-100 bg-neutral-50 px-4 sm:px-6 py-4"
        >
          <div class="max-w-md">
            <p class="text-xs text-neutral-500 mb-3">自分の名前とランクを登録します。登録後は非活性になります。</p>
            <PlayerForm
              :isOwner="false"
              :editTarget="null"
              @done="onSelfAddDone"
              @cancel="() => { showSelfAddForm = false }"
            />
          </div>
        </div>
      </Transition>

      <!-- ── 他人を追加フォーム ── -->
      <Transition
        enter-active-class="transition-all duration-200 ease-out overflow-hidden"
        enter-from-class="opacity-0 max-h-0"
        enter-to-class="opacity-100 max-h-96"
        leave-active-class="transition-all duration-150 ease-in overflow-hidden"
        leave-from-class="opacity-100 max-h-96"
        leave-to-class="opacity-0 max-h-0"
      >
        <div
          v-if="showOtherAddForm"
          class="border-t border-neutral-100 bg-neutral-50 px-4 sm:px-6 py-4"
        >
          <div class="max-w-md">
            <PlayerForm
              :isOwner="true"
              :editTarget="null"
              @done="onOtherAddDone"
              @cancel="() => { showOtherAddForm = false }"
            />
          </div>
        </div>
      </Transition>
    </div>

    <!-- ══════════════════════════════════════════════════════════
         メインコンテンツ
         ══════════════════════════════════════════════════════════ -->
    <main class="max-w-5xl mx-auto px-4 sm:px-6 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">

        <!-- ────────────────────────────────────────────────────
             左カラム: 参加者管理リスト
             ──────────────────────────────────────────────────── -->
        <div class="lg:col-span-1">
          <div class="card">
            <!-- プレイヤー数が少ない場合の注意メッセージ -->
            <div
              v-if="store.playerCount < store.config.team_count && store.playerCount > 0"
              class="mb-4 px-3 py-2 bg-amber-50 border border-amber-200 text-xs text-amber-800"
            >
              ⚠ チーム数（{{ store.config.team_count }}）より参加者（{{ store.playerCount }}名）が少ないです。
              チーム数を減らすか、参加者を追加してください。
            </div>

            <PlayerList :isOwner="true" />
          </div>
        </div>

        <!-- ────────────────────────────────────────────────────
             右カラム: チーム分け結果
             ──────────────────────────────────────────────────── -->
        <div id="team-result" class="lg:col-span-2">
          <div class="card">
            <!-- チーム分け前のガイダンス -->
            <div
              v-if="!store.hasTeams && store.playerCount > 0"
              class="mb-5 px-4 py-3 bg-primary-50 border border-primary-200"
            >
              <p class="text-sm font-medium text-primary-800">
                準備完了 — {{ store.playerCount }}名が登録されています
              </p>
              <p class="text-xs text-primary-600 mt-1">
                上の「チーム分けを実行」ボタンを押すと、
                戦闘力均衡化アルゴリズムで {{ store.config.team_count }}チームに分けます。
              </p>
            </div>

            <TeamResult />
          </div>
        </div>

      </div>
    </main>

    <!-- ── フッター ── -->
    <footer class="mt-16 py-6 border-t border-neutral-200">
      <div class="max-w-5xl mx-auto px-4 sm:px-6 flex items-center justify-between">
        <p class="text-xs text-neutral-300">
          チーム決め一発くん — 管理者コンソール
        </p>
        <!-- 一般ユーザー画面へのリンク -->
        <a href="/" class="text-xs text-neutral-400 hover:text-neutral-600 transition-colors">
          一般ユーザー画面 →
        </a>
      </div>
    </footer>
  </div>
</template>
