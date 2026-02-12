<!--
  PlayerForm.vue - プレイヤー追加・編集フォームコンポーネント

  【役割】
  名前とランクを入力してプレイヤーを登録・更新するフォーム。
  管理者（isOwner=true）の場合は任意のプレイヤーを編集できる。
  一般ユーザー（isOwner=false）の場合は自分のエントリのみ操作できる。

  【Propsの説明】
  - isOwner: 管理者モードかどうか
  - editTarget: 編集対象のプレイヤー（新規追加の場合はnull）
-->
<script setup lang="ts">
import { ref, watch } from 'vue'
import { RANK_LIST, RANK_COLORS } from '@/types'
import type { Player, RankName } from '@/types'
import { usePlayersStore } from '@/stores/players'

// ── Propsの定義 ──
// defineProps でこのコンポーネントが受け取るデータを宣言する
const props = defineProps<{
  isOwner: boolean
  editTarget: Player | null // null = 新規追加モード
}>()

// ── Emitsの定義 ──
// 親コンポーネントに通知するイベントを宣言する
const emit = defineEmits<{
  done: [] // フォーム送信完了時に発火
  cancel: [] // キャンセル時に発火
}>()

const store = usePlayersStore()

// ── フォームの状態 ──
const nameInput = ref('')
const rankInput = ref<RankName>('ゴールド')
const isSubmitting = ref(false)

// 編集対象が変わったらフォームの値を更新する
// watch: ref の値が変化したときに実行される
// immediate: true で初回レンダリング時にも実行される
watch(
  () => props.editTarget,
  (target) => {
    if (target) {
      // 編集モード: 対象プレイヤーの現在値をフォームに入れる
      nameInput.value = target.name
      rankInput.value = target.rank
    } else {
      // 新規追加モード: フォームをリセット
      nameInput.value = ''
      rankInput.value = 'ゴールド'
    }
  },
  { immediate: true }
)

// ── フォームの送信処理 ──
async function handleSubmit() {
  // 名前が空の場合は送信しない
  const trimmedName = nameInput.value.trim()
  if (!trimmedName) return

  isSubmitting.value = true

  try {
    if (props.editTarget) {
      // === 編集モード ===
      const success = await store.updatePlayer(
        props.editTarget.id,
        trimmedName,
        rankInput.value
      )
      if (success) emit('done')
    } else {
      // === 新規追加モード ===
      // 一般ユーザーは自分のプレイヤーとして登録する（saveAsMyPlayer = !isOwner）
      const newPlayer = await store.addPlayer(
        trimmedName,
        rankInput.value,
        !props.isOwner // 管理者でない場合は自分のプレイヤーとして保存
      )
      if (newPlayer) {
        // 成功したらフォームをリセット
        nameInput.value = ''
        rankInput.value = 'ゴールド'
        emit('done')
      }
    }
  } finally {
    isSubmitting.value = false
  }
}

function handleCancel() {
  emit('cancel')
}
</script>

<template>
  <!--
    フォームコンテナ
    border-l-4 で左端にアクセントカラーのボーダーを付けるエディトリアルスタイル
  -->
  <form
    @submit.prevent="handleSubmit"
    class="card border-l-4 border-l-primary-700"
  >
    <!-- フォームタイトル -->
    <h3 class="text-sm font-semibold text-neutral-700 mb-4">
      {{ editTarget ? 'プレイヤー情報を編集' : '参加者を登録' }}
    </h3>

    <div class="space-y-4">
      <!-- ── 名前入力 ── -->
      <div>
        <label for="player-name" class="block text-xs font-semibold text-neutral-500 mb-1.5 uppercase tracking-wide">
          プレイヤー名
        </label>
        <input
          id="player-name"
          v-model="nameInput"
          type="text"
          placeholder="名前を入力"
          class="input-field"
          maxlength="30"
          required
          :disabled="isSubmitting"
        />
      </div>

      <!-- ── ランク選択 ── -->
      <div>
        <label for="player-rank" class="block text-xs font-semibold text-neutral-500 mb-1.5 uppercase tracking-wide">
          ランク
        </label>
        <select
          id="player-rank"
          v-model="rankInput"
          class="select-field"
          :disabled="isSubmitting"
        >
          <!--
            RANK_LIST を v-for でループして選択肢を生成する
            :key は Vue がリスト要素を効率的に更新するために必要な一意のキー
          -->
          <option
            v-for="rank in RANK_LIST"
            :key="rank"
            :value="rank"
          >
            {{ rank }}
          </option>
        </select>

        <!-- ランクプレビューバッジ（選択中のランクをビジュアルで確認） -->
        <div class="mt-2 flex items-center gap-2">
          <span
            class="inline-flex items-center px-2.5 py-0.5 text-xs font-medium border"
            :class="[
              RANK_COLORS[rankInput].bg,
              RANK_COLORS[rankInput].text,
              RANK_COLORS[rankInput].border,
            ]"
          >
            {{ rankInput }}
          </span>
        </div>
      </div>

      <!-- ── ボタン群 ── -->
      <div class="flex gap-2 pt-2">
        <!-- 送信ボタン -->
        <button
          type="submit"
          class="btn-primary flex-1"
          :disabled="isSubmitting || !nameInput.trim()"
        >
          <!-- isSubmitting 中はローディングスピナーを表示 -->
          <svg
            v-if="isSubmitting"
            class="animate-spin -ml-1 mr-2 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          {{ editTarget ? '更新する' : '登録する' }}
        </button>

        <!-- キャンセルボタン（編集モードのときのみ表示） -->
        <button
          v-if="editTarget"
          type="button"
          class="btn-secondary"
          @click="handleCancel"
          :disabled="isSubmitting"
        >
          キャンセル
        </button>
      </div>
    </div>
  </form>
</template>
