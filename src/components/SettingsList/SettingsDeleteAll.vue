<script setup lang="ts">
import type { Ref } from 'vue'

import * as Commands from '~/composables/commands'

const { t } = useI18n()

const packageStore = usePackageStore()

/* ============================= DELETE BUTTON ============================= */
const deleteButtonHolden = ref(false)
const deleteTimeout: Ref<any> = ref(null)

watch(deleteButtonHolden, (isHolden) => {
  if (isHolden) {
    deleteTimeout.value = setTimeout(
      () => {
        for (const packageObject of packageStore.storedPackages) {
          packageStore.exclude(packageObject)
          Commands.subPackage(packageObject.serviceId, packageObject.id)
        }
        deleteButtonHolden.value = false
      },
      2400,
    )
  }
  else {
    clearTimeout(deleteTimeout.value)
  }
})
</script>

<template>
  <div
    border="~ rounded gray-200 dark:gray-700"
    cursor-pointer
    p-4
    :class="{ progress: deleteButtonHolden }"
    @mousedown="() => deleteButtonHolden = true"
    @mouseup="() => deleteButtonHolden = false"
  >
    <div>
      {{ t('component.SettingsDeleteAll.DELETE ALL PACKAGES') }}
    </div>
    <div text-sm text-gray>
      {{ t('component.SettingsDeleteAll.DELETE-PACKAGE-HINT') }}
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
