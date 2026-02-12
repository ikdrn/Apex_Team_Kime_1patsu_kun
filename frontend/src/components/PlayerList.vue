<!--
  PlayerList.vue - プレイヤー一覧表示コンポーネント

  【役割】
  登録されているプレイヤーの一覧を表示する。
  管理者モードでは編集・削除ボタンが表示される。
  一般ユーザーは自分のエントリだけ編集できる。

  【Props】
  - isOwner: 管理者モードかどうか（true=全員操作可能）
-->
<script setup lang="ts">
import { ref } from 'vue'
import { RANK_COLORS, RANK_LIST, getDisplayName } from '@/types'
import type { Player, RankName } from '@/types'
import { usePlayersStore } from '@/stores/players'

const props = defineProps<{
  isOwner: boolean
}>()

const store = usePlayersStore()

// 編集中のプレイヤーIDを追跡（nullは編集していない状態）
const editingId = ref<string | null>(null)

// 編集フォームの入力値
const editName = ref('')
const editRank = ref<RankName>('ゴールド')

// 編集の開始（インラインフォームを展開する）
function startEdit(player: Player) {
  editingId.value = player.id
  editName.value = player.name
  editRank.value = player.rank as RankName
}

// 編集のキャンセル
function cancelEdit() {
  editingId.value = null
}

// 編集の保存
async function saveEdit() {
  if (!editingId.value || !editName.value.trim()) return
  const success = await store.updatePlayer(editingId.value, editName.value.trim(), editRank.value)
  if (success) {
    editingId.value = null
  }
}

// 削除（確認ダイアログあり）
async function handleDelete(player: Player) {
  const displayName = getDisplayName(player)
  if (!confirm(`「${displayName}」を参加者リストから削除しますか？`)) return
  await store.deletePlayer(player.id)
}

// このプレイヤーが操作可能かどうかを判定する
// 管理者は全員、一般ユーザーは自分のエントリのみ
function canEdit(player: Player): boolean {
  if (props.isOwner) return true
  return player.id === store.myPlayerId
}
</script>

<template>
  <div>
    <!-- ── セクションヘッダー ── -->
    <div class="flex items-baseline justify-between mb-2">
      <p class="text-xs font-semibold text-neutral-500 uppercase tracking-wider">
        参加者一覧
      </p>
      <span class="text-xs text-neutral-400 tabular-nums font-medium">
        {{ store.playerCount }} 名
      </span>
    </div>
    <div class="border-b border-neutral-200 mb-4"></div>

    <!-- ── プレイヤーが0人の場合のエンプティステート ── -->
    <div
      v-if="store.players.length === 0"
      class="py-10 text-center"
    >
      <div class="text-3xl font-black text-neutral-200 mb-2">0</div>
      <p class="text-sm text-neutral-400">参加者がいません</p>
    </div>

    <!-- ── プレイヤーリスト ── -->
    <!--
      TransitionGroup: v-for リストにアニメーションを付ける
    -->
    <TransitionGroup
      tag="ul"
      enter-active-class="transition-all duration-250 ease-out"
      enter-from-class="opacity-0 -translate-y-1"
      leave-active-class="transition-all duration-150 ease-in"
      leave-to-class="opacity-0 scale-95"
      class="space-y-1.5"
    >
      <li
        v-for="player in store.players"
        :key="player.id"
        class="border border-neutral-200 bg-white overflow-hidden"
        :class="{
          'border-l-4 border-l-primary-600': player.id === store.myPlayerId,
        }"
      >
        <!-- ── 通常表示行 ── -->
        <div class="flex items-center px-3 py-2.5 gap-2.5">
          <!-- ランクバッジ -->
          <span
            class="shrink-0 inline-flex items-center px-1.5 py-0.5 text-xs font-semibold border"
            :class="[
              RANK_COLORS[player.rank].bg,
              RANK_COLORS[player.rank].text,
              RANK_COLORS[player.rank].border,
            ]"
          >
            {{ player.rank }}
          </span>

          <!-- プレイヤー名 -->
          <span class="flex-1 text-sm font-medium text-neutral-800 truncate min-w-0">
            {{ getDisplayName(player) }}
            <span
              v-if="player.id === store.myPlayerId"
              class="ml-1.5 text-xs text-primary-600 font-normal"
            >
              (あなた)
            </span>
          </span>

          <!-- 操作ボタン（権限がある場合のみ） -->
          <div v-if="canEdit(player)" class="flex gap-1 shrink-0">
            <button
              @click="startEdit(player)"
              class="text-xs px-2 py-1 border border-neutral-200 text-neutral-600
                     hover:bg-neutral-50 transition-colors"
              :disabled="store.isLoading"
            >
              編集
            </button>
            <button
              v-if="isOwner"
              @click="handleDelete(player)"
              class="text-xs px-2 py-1 border border-red-200 text-red-600
                     hover:bg-red-50 transition-colors"
              :disabled="store.isLoading"
            >
              削除
            </button>
          </div>
        </div>

        <!-- ── インライン編集フォーム ── -->
        <div
          v-if="editingId === player.id"
          class="border-t border-neutral-100 bg-neutral-50 px-3 py-3"
        >
          <form
            @submit.prevent="saveEdit"
            class="flex flex-wrap gap-2 items-end"
          >
            <div class="flex-1 min-w-28">
              <label class="block text-xs text-neutral-500 mb-1">名前</label>
              <input
                v-model="editName"
                type="text"
                class="input-field text-xs py-1.5"
                :disabled="store.isLoading"
                maxlength="30"
                required
              />
            </div>
            <div class="w-32">
              <label class="block text-xs text-neutral-500 mb-1">ランク</label>
              <select
                v-model="editRank"
                class="select-field text-xs py-1.5"
                :disabled="store.isLoading"
              >
                <option v-for="r in RANK_LIST" :key="r" :value="r">{{ r }}</option>
              </select>
            </div>
            <div class="flex gap-1.5">
              <button
                type="submit"
                class="btn-primary text-xs py-1.5 px-3"
                :disabled="store.isLoading || !editName.trim()"
              >
                保存
              </button>
              <button
                type="button"
                class="btn-secondary text-xs py-1.5 px-3"
                @click="cancelEdit"
                :disabled="store.isLoading"
              >
                取消
              </button>
            </div>
          </form>
        </div>
      </li>
    </TransitionGroup>
  </div>
</template>
