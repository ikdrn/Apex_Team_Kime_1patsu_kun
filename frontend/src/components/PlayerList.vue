<!--
  PlayerList.vue - プレイヤー一覧表示コンポーネント

  管理者モードでは2行レイアウト（名前行 + コントロール行）を使用し、
  名前が表示されなくなるレイアウト崩れを防ぐ。
-->
<script setup lang="ts">
import { ref } from 'vue'
import { RANK_COLORS, RANK_LIST, RANK_SCORES, getDisplayName } from '@/types'
import type { Player, RankName } from '@/types'
import { usePlayersStore } from '@/stores/players'

const props = defineProps<{
  isOwner: boolean
}>()

const store = usePlayersStore()

const editingId = ref<string | null>(null)
const editName = ref('')
const editRank = ref<RankName>('ゴールド')

function startEdit(player: Player) {
  editingId.value = player.id
  editName.value = player.name
  editRank.value = player.rank as RankName
}

function cancelEdit() {
  editingId.value = null
}

async function saveEdit() {
  if (!editingId.value || !editName.value.trim()) return
  const success = await store.updatePlayer(editingId.value, editName.value.trim(), editRank.value)
  if (success) editingId.value = null
}

async function handleDelete(player: Player) {
  const displayName = getDisplayName(player)
  if (!confirm(`「${displayName}」を参加者リストから削除しますか？`)) return
  await store.deletePlayer(player.id)
}

function canEdit(player: Player): boolean {
  if (props.isOwner) return true
  return player.id === store.myPlayerId
}

function effectiveScore(player: Player): number {
  return Math.max(1, RANK_SCORES[player.rank] + (player.score_offset ?? 0))
}

function offsetLabel(player: Player): string {
  const offset = player.score_offset ?? 0
  if (offset === 0) return ''
  return offset > 0 ? `+${offset}` : `${offset}`
}
</script>

<template>
  <div>
    <!-- セクションヘッダー -->
    <div class="flex items-baseline justify-between mb-2">
      <p class="text-xs font-semibold text-neutral-500 uppercase tracking-wider">
        参加者一覧
      </p>
      <span class="text-xs text-neutral-400 tabular-nums font-medium">
        {{ store.playerCount }} 名
      </span>
    </div>
    <div class="border-b border-neutral-200 mb-4"></div>

    <!-- エンプティステート -->
    <div v-if="store.players.length === 0" class="py-10 text-center">
      <div class="text-3xl font-black text-neutral-200 mb-2">0</div>
      <p class="text-sm text-neutral-400">参加者がいません</p>
    </div>

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
        :class="{ 'border-l-4 border-l-primary-600': player.id === store.myPlayerId }"
      >
        <!-- ══ 管理者向け 2行レイアウト ══ -->
        <div v-if="isOwner" class="px-3 pt-2.5 pb-2">
          <!-- 行1: ランクバッジ + プレイヤー名 -->
          <div class="flex items-center gap-2 mb-1.5">
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
            <span class="text-sm font-medium text-neutral-800 truncate">
              {{ getDisplayName(player) }}
              <span v-if="player.id === store.myPlayerId" class="ml-1 text-xs text-primary-600 font-normal">
                (あなた)
              </span>
            </span>
          </div>

          <!-- 行2: 戦闘力補正 + 編集/削除ボタン -->
          <div class="flex items-center justify-between gap-2">
            <!-- 戦闘力補正コントロール -->
            <div class="flex items-center gap-1">
              <!-- − ボタン -->
              <button
                @click="store.adjustScoreOffset(player.id, -1)"
                class="w-6 h-6 flex items-center justify-center border border-neutral-200
                       text-neutral-500 hover:bg-red-50 hover:border-red-200 hover:text-red-600
                       transition-colors text-sm leading-none disabled:opacity-30 disabled:cursor-not-allowed"
                :disabled="(player.score_offset ?? 0) <= -5"
                title="戦闘力を-1"
              >−</button>

              <!-- スコア表示 -->
              <span class="text-xs tabular-nums text-center w-12">
                <span class="font-semibold text-neutral-800">戦闘力 {{ effectiveScore(player) }}</span>
                <span
                  v-if="(player.score_offset ?? 0) !== 0"
                  class="ml-0.5"
                  :class="(player.score_offset ?? 0) > 0 ? 'text-blue-500' : 'text-red-400'"
                >({{ offsetLabel(player) }})</span>
              </span>

              <!-- + ボタン -->
              <button
                @click="store.adjustScoreOffset(player.id, 1)"
                class="w-6 h-6 flex items-center justify-center border border-neutral-200
                       text-neutral-500 hover:bg-blue-50 hover:border-blue-200 hover:text-blue-600
                       transition-colors text-sm leading-none disabled:opacity-30 disabled:cursor-not-allowed"
                :disabled="(player.score_offset ?? 0) >= 5"
                title="戦闘力を+1"
              >＋</button>
            </div>

            <!-- 編集/削除ボタン -->
            <div class="flex gap-1">
              <button
                @click="startEdit(player)"
                class="text-xs px-2 py-1 border border-neutral-200 text-neutral-600
                       hover:bg-neutral-50 transition-colors"
                :disabled="store.isLoading"
              >編集</button>
              <button
                @click="handleDelete(player)"
                class="text-xs px-2 py-1 border border-red-200 text-red-600
                       hover:bg-red-50 transition-colors"
                :disabled="store.isLoading"
              >削除</button>
            </div>
          </div>
        </div>

        <!-- ══ 一般ユーザー向け 1行レイアウト ══ -->
        <div v-else class="flex items-center px-3 py-2.5 gap-2.5">
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
          <span class="flex-1 text-sm font-medium text-neutral-800 truncate min-w-0">
            {{ getDisplayName(player) }}
            <span v-if="player.id === store.myPlayerId" class="ml-1.5 text-xs text-primary-600 font-normal">
              (あなた)
            </span>
          </span>
          <div v-if="canEdit(player)" class="flex gap-1 shrink-0">
            <button
              @click="startEdit(player)"
              class="text-xs px-2 py-1 border border-neutral-200 text-neutral-600
                     hover:bg-neutral-50 transition-colors"
              :disabled="store.isLoading"
            >編集</button>
          </div>
        </div>

        <!-- ── インライン編集フォーム ── -->
        <div
          v-if="editingId === player.id"
          class="border-t border-neutral-100 bg-neutral-50 px-3 py-3"
        >
          <form @submit.prevent="saveEdit" class="flex flex-wrap gap-2 items-end">
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
              >保存</button>
              <button
                type="button"
                class="btn-secondary text-xs py-1.5 px-3"
                @click="cancelEdit"
                :disabled="store.isLoading"
              >取消</button>
            </div>
          </form>
        </div>
      </li>
    </TransitionGroup>
  </div>
</template>
