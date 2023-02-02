<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageFormClosed'): void }>()

const packageStore = usePackageStore()

const deleteButtonHolden = ref(false)
const deleteTimeout: Ref<any> = ref(null)

watch(deleteButtonHolden, (isHolden) => {
  if (isHolden) {
    deleteTimeout.value = setTimeout(
      () => {
        emit('packageFormClosed')
        packageStore.exclude(pack)
        invoke('subPackage', { id: pack.id })
      },
      3000,
    )
  }
  else {
    clearTimeout(deleteTimeout.value)
  }
})
</script>

<template>
  <div
    flex flex-col
    h-full
  >
    <div mt-a />
    <div
      border="~ rounded gray-200 dark:gray-700"
      cursor-pointer
      p-4
      :class="{ progress: deleteButtonHolden }"
      @mousedown="() => deleteButtonHolden = true"
      @mouseup="() => deleteButtonHolden = false"
    >
      <div>
        DELETE PACKAGE
      </div>
      <div text-sm text-gray>
        Package will be deleted from local disk only if it was copied by application (that happen before exit)
      </div>
    </div>
  </div>
</template>

<style lang="sass">
.progress
    animation: progressBar 3s ease-in-out
    animation-fill-mode: forwards

@keyframes progressBar
  0%
    background-color: rgba(165, 42, 42, 0)
  100%
    background-color: rgba(165, 42, 42, 1)
</style>
