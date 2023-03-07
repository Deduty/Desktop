<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import * as Commands from '~/composables/commands'

const { packageObject } = defineProps<{ packageObject: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageFormClosed'): void }>()

const { t } = useI18n()

const packageStore = usePackageStore()

/* ============================= DELETE BUTTON ============================= */
const deleteButtonHolden = ref(false)
const deleteTimeout: Ref<any> = ref(null)

watch(deleteButtonHolden, (isHolden) => {
  if (isHolden) {
    deleteTimeout.value = setTimeout(
      () => {
        emit('packageFormClosed')
        packageStore.exclude(packageObject)
        Commands.subPackage(packageObject.serviceId, packageObject.id)
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
    flex flex-col
    h-full
    gap-2
  >
    <div
      w-full
      flex flex-col
    >
      <PackageFormSettingsUpdate
        :package-object="packageObject"
        @package-form-closed="() => emit('packageFormClosed')"
      />
    </div>
    <div
      w-full
      flex flex-col
    >
      <PackageFormSettingsWebStorage :package-object="packageObject" />
    </div>
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
        {{ t('component.PackageFormSettings.DELETE PACKAGE') }}
      </div>
      <div text-sm text-gray>
        {{ t('component.PackageFormSettings.DELETE-PACKAGE-HINT') }}
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
