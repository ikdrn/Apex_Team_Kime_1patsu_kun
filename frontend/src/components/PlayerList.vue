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

function startEdit(p: Player) {
  editingId.value = p.id
  editName.value = p.name
  editRank.value = p.rank as RankName
}
function cancelEdit() { editingId.value = null }

async function saveEdit() {
  if (!editingId.value || !editName.value.trim()) return
  const ok = await store.updatePlayer(editingId.value, editName.value.trim(), editRank.value)
  if (ok) editingId.value = null
}

async function handleDelete(p: Player) {
  if (!confirm(`「${getDisplayName(p)}」を削除しますか？`)) return
  await store.deletePlayer(p.id)
}

function effectiveScore(p: Player) {
  return Math.max(1, RANK_SCORES[p.rank] + (p.score_offset ?? 0))
}
</script>

<template>
  <div>
    <!-- ヘッダー -->
    <div class="px-4 py-3 flex items-center justify-between border-b border-neutral-200 bg-neutral-50">
      <span class="text-xs font-semibold tracking-widest uppercase text-neutral-500">参加者</span>
      <span class="text-sm font-semibold font-mono text-neutral-700">{{ store.playerCount }}</span>
    </div>

    <!-- エンプティ -->
    <div v-if="store.players.length === 0" class="py-16 text-center">
      <p class="text-sm text-neutral-400">まだ誰も参加していません</p>
    </div>

    <TransitionGroup tag="ul" enter-active-class="transition-all duration-200" enter-from-class="opacity-0" leave-active-class="transition-all duration-150" leave-to-class="opacity-0">
      <li
        v-for="player in store.players"
        :key="player.id"
        class="border-b border-neutral-100 last:border-b-0 group"
      >
        <!-- ── 管理者: 2行レイアウト ── -->
        <div v-if="isOwner" class="px-4 py-3 hover:bg-neutral-50 transition-colors">
          <!-- 行1: ランクバッジ + 名前 -->
          <div class="flex items-center gap-2 mb-2">
            <span
              class="shrink-0 text-xs font-bold px-2 py-0.5 border leading-tight"
              :class="[RANK_COLORS[player.rank].bg, RANK_COLORS[player.rank].text, RANK_COLORS[player.rank].border]"
            >{{ player.rank }}</span>
            <span class="text-sm font-medium text-neutral-900 truncate">{{ getDisplayName(player) }}</span>
            <span v-if="player.id === store.myPlayerId" class="ml-auto shrink-0 text-xs text-primary-600 font-medium">(自分)</span>
          </div>
          <!-- 行2: 戦闘力補正 + 操作 -->
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <button class="btn-nudge" @click="store.adjustScoreOffset(player.id, -1)" :disabled="(player.score_offset ?? 0) <= -3">−</button>
              <div class="text-sm font-mono text-center min-w-[56px]">
                <span class="font-bold text-neutral-800">{{ effectiveScore(player) }}</span>
                <span v-if="(player.score_offset ?? 0) !== 0" class="text-xs ml-0.5" :class="(player.score_offset ?? 0) > 0 ? 'text-blue-500' : 'text-red-400'">
                  ({{ (player.score_offset ?? 0) > 0 ? '+' : '' }}{{ player.score_offset }})
                </span>
              </div>
              <button class="btn-nudge" @click="store.adjustScoreOffset(player.id, 1)" :disabled="(player.score_offset ?? 0) >= 3">＋</button>
            </div>
            <div class="flex gap-1.5">
              <button @click="startEdit(player)" class="btn-ghost text-xs py-1">編集</button>
              <button @click="handleDelete(player)" class="btn-ghost text-xs py-1 text-red-500 hover:bg-red-50">削除</button>
            </div>
          </div>
        </div>

        <!-- ── 一般ユーザー: 1行 ── -->
        <div v-else class="px-4 py-3 flex items-center gap-3 hover:bg-neutral-50 transition-colors">
          <span
            class="shrink-0 text-xs font-bold px-2 py-0.5 border leading-tight"
            :class="[RANK_COLORS[player.rank].bg, RANK_COLORS[player.rank].text, RANK_COLORS[player.rank].border]"
          >{{ player.rank }}</span>
          <span class="flex-1 text-sm text-neutral-800 truncate min-w-0">{{ getDisplayName(player) }}</span>
          <span v-if="player.id === store.myPlayerId" class="shrink-0 text-xs text-primary-600 font-medium">(自分)</span>
          <button
            v-if="props.isOwner || player.id === store.myPlayerId"
            @click="startEdit(player)"
            class="btn-ghost text-xs py-1 opacity-0 group-hover:opacity-100 shrink-0"
          >編集</button>
        </div>

        <!-- インライン編集フォーム -->
        <div v-if="editingId === player.id" class="px-4 pb-4 pt-2 bg-neutral-50 border-t border-neutral-100">
          <form @submit.prevent="saveEdit" class="flex flex-wrap gap-2 items-end">
            <div class="flex-1 min-w-[100px]">
              <label class="block text-xs text-neutral-500 mb-1">名前</label>
              <input v-model="editName" type="text" class="input-field" maxlength="30" required :disabled="store.isLoading" />
            </div>
            <div class="w-[110px]">
              <label class="block text-xs text-neutral-500 mb-1">ランク</label>
              <select v-model="editRank" class="select-field" :disabled="store.isLoading">
                <option v-for="r in RANK_LIST" :key="r" :value="r">{{ r }}</option>
              </select>
            </div>
            <div class="flex gap-1.5">
              <button type="submit" class="btn-primary text-xs py-2 px-3" :disabled="store.isLoading || !editName.trim()">保存</button>
              <button type="button" class="btn-secondary text-xs py-2" @click="cancelEdit">取消</button>
            </div>
          </form>
        </div>
      </li>
    </TransitionGroup>
  </div>
</template>
