<script setup lang="ts">
import { ref, watch } from 'vue'
import { RANK_LIST } from '@/types'
import type { Player, RankName } from '@/types'
import { usePlayersStore } from '@/stores/players'

const props = defineProps<{ addForOther: boolean; editTarget: Player | null }>()
const emit = defineEmits<{ done: []; cancel: [] }>()
const store = usePlayersStore()

const nameInput = ref('')
const rankInput = ref<RankName>('ゴールド')
const isSubmitting = ref(false)

watch(() => props.editTarget, (t) => {
  nameInput.value = t ? t.name : ''
  rankInput.value = t ? t.rank : 'ゴールド'
}, { immediate: true })

async function handleSubmit() {
  const name = nameInput.value.trim()
  if (!name) return
  isSubmitting.value = true
  try {
    if (props.editTarget) {
      const ok = await store.updatePlayer(props.editTarget.id, name, rankInput.value)
      if (ok) emit('done')
    } else {
      const player = await store.addPlayer(name, rankInput.value, !props.addForOther)
      if (player) { nameInput.value = ''; rankInput.value = 'ゴールド'; emit('done') }
    }
  } finally { isSubmitting.value = false }
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="space-y-3">
    <div>
      <label class="block text-xs font-semibold text-neutral-500 mb-1">プレイヤー名</label>
      <input v-model="nameInput" type="text" placeholder="名前を入力" class="input-field" maxlength="30" required :disabled="isSubmitting" />
    </div>
    <div>
      <label class="block text-xs font-semibold text-neutral-500 mb-1">ランク</label>
      <select v-model="rankInput" class="select-field" :disabled="isSubmitting">
        <option v-for="r in RANK_LIST" :key="r" :value="r">{{ r }}</option>
      </select>
    </div>
    <div class="flex gap-2 pt-1">
      <button type="submit" class="btn-primary flex-1" :disabled="isSubmitting || !nameInput.trim()">
        <svg v-if="isSubmitting" class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
        </svg>
        {{ editTarget ? '更新する' : '登録する' }}
      </button>
      <button v-if="editTarget" type="button" class="btn-secondary" @click="emit('cancel')" :disabled="isSubmitting">取消</button>
    </div>
  </form>
</template>
