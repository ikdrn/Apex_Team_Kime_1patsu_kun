<script setup lang="ts">
import { ref } from 'vue'
import { RANK_COLORS, RANK_LIST, RANK_SCORES, getDisplayName } from '@/types'
import type { Player, RankName } from '@/types'
import { usePlayersStore } from '@/stores/players'

const props = defineProps<{ isOwner: boolean }>()
const store = usePlayersStore()

const editingId = ref<string | null>(null)
const editName = ref('')
const editRank = ref<RankName>('ゴールド')

function startEdit(player: Player) {
  editingId.value = player.id
  editName.value = player.name
  editRank.value = player.rank as RankName
}
function cancelEdit() { editingId.value = null }

async function saveEdit() {
  if (!editingId.value || !editName.value.trim()) return
  const ok = await store.updatePlayer(editingId.value, editName.value.trim(), editRank.value)
  if (ok) editingId.value = null
}

async function handleDelete(player: Player) {
  if (!confirm(`「${getDisplayName(player)}」を削除しますか？`)) return
  await store.deletePlayer(player.id)
}

function canEdit(player: Player) {
  return props.isOwner || player.id === store.myPlayerId
}

function effectiveScore(player: Player) {
  return Math.max(1, RANK_SCORES[player.rank] + (player.score_offset ?? 0))
}
</script>

<template>
  <div>
    <!-- セクションラベル -->
    <div class="px-4 py-2.5 flex items-center justify-between border-b border-neutral-100">
      <span class="text-xs font-semibold text-neutral-400 tracking-widest uppercase">参加者</span>
      <span class="text-xs font-mono text-neutral-500">{{ store.playerCount }}</span>
    </div>

    <!-- エンプティ -->
    <div v-if="store.players.length === 0" class="px-4 py-10 text-center">
      <p class="text-xs text-neutral-300">参加者がいません</p>
    </div>

    <TransitionGroup
      tag="ul"
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0"
      leave-active-class="transition-all duration-150 ease-in"
      leave-to-class="opacity-0"
    >
      <li
        v-for="player in store.players"
        :key="player.id"
        class="border-b border-neutral-100 last:border-b-0"
        :class="{ 'bg-primary-50': player.id === store.myPlayerId }"
      >
        <!-- ── 管理者: 2行レイアウト ── -->
        <div v-if="isOwner" class="px-4 py-2">
          <!-- 行1: ランク + 名前 -->
          <div class="flex items-center gap-2 mb-1.5">
            <span
              class="shrink-0 text-[10px] font-semibold px-1.5 py-0.5 border"
              :class="[RANK_COLORS[player.rank].bg, RANK_COLORS[player.rank].text, RANK_COLORS[player.rank].border]"
            >{{ player.rank }}</span>
            <span class="text-sm font-medium text-neutral-900 truncate flex-1">
              {{ getDisplayName(player) }}
            </span>
          </div>
          <!-- 行2: 戦闘力補正 + 操作 -->
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-1.5">
              <button
                class="btn-offset"
                @click="store.adjustScoreOffset(player.id, -1)"
                :disabled="(player.score_offset ?? 0) <= -3"
              >−</button>
              <span class="text-xs tabular-nums text-neutral-600 font-mono w-16">
                <span class="font-semibold">{{ effectiveScore(player) }}</span>
                <span
                  v-if="(player.score_offset ?? 0) !== 0"
                  :class="(player.score_offset ?? 0) > 0 ? 'text-blue-500' : 'text-red-400'"
                  class="ml-0.5"
                >({{ (player.score_offset ?? 0) > 0 ? '+' : '' }}{{ player.score_offset }})</span>
              </span>
              <button
                class="btn-offset"
                @click="store.adjustScoreOffset(player.id, 1)"
                :disabled="(player.score_offset ?? 0) >= 3"
              >＋</button>
            </div>
            <div class="flex items-center gap-1">
              <button @click="startEdit(player)" class="text-[11px] px-2 py-0.5 border border-neutral-200 text-neutral-500 hover:bg-neutral-50">編集</button>
              <button @click="handleDelete(player)" class="text-[11px] px-2 py-0.5 border border-red-100 text-red-400 hover:bg-red-50">削除</button>
            </div>
          </div>
        </div>

        <!-- ── 一般: 1行レイアウト ── -->
        <div v-else class="px-4 py-2.5 flex items-center gap-2">
          <span
            class="shrink-0 text-[10px] font-semibold px-1.5 py-0.5 border"
            :class="[RANK_COLORS[player.rank].bg, RANK_COLORS[player.rank].text, RANK_COLORS[player.rank].border]"
          >{{ player.rank }}</span>
          <span class="flex-1 text-sm text-neutral-800 truncate min-w-0">
            {{ getDisplayName(player) }}
            <span v-if="player.id === store.myPlayerId" class="ml-1 text-xs text-primary-600">(あなた)</span>
          </span>
          <button v-if="canEdit(player)" @click="startEdit(player)" class="shrink-0 text-[11px] px-2 py-0.5 border border-neutral-200 text-neutral-500 hover:bg-neutral-50">編集</button>
        </div>

        <!-- インライン編集フォーム -->
        <div v-if="editingId === player.id" class="px-4 pb-3 pt-0 bg-neutral-50 border-t border-neutral-100">
          <form @submit.prevent="saveEdit" class="flex flex-wrap gap-2 items-end pt-2">
            <div class="flex-1 min-w-28">
              <label class="block text-[10px] text-neutral-400 mb-1 uppercase tracking-wide">名前</label>
              <input v-model="editName" type="text" class="input-field py-1.5 text-xs" maxlength="30" required :disabled="store.isLoading" />
            </div>
            <div class="w-28">
              <label class="block text-[10px] text-neutral-400 mb-1 uppercase tracking-wide">ランク</label>
              <select v-model="editRank" class="select-field py-1.5 text-xs" :disabled="store.isLoading">
                <option v-for="r in RANK_LIST" :key="r" :value="r">{{ r }}</option>
              </select>
            </div>
            <div class="flex gap-1.5">
              <button type="submit" class="btn-primary text-xs py-1.5 px-3" :disabled="store.isLoading || !editName.trim()">保存</button>
              <button type="button" class="btn-secondary text-xs py-1.5 px-3" @click="cancelEdit">取消</button>
            </div>
          </form>
        </div>
      </li>
    </TransitionGroup>
  </div>
</template>
