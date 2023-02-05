<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageFormClosed'): void }>()

const packageStore = usePackageStore()

const updateComponentShowed = ref(false)

/* ============================= DELETE BUTTON ============================= */
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
/* ========================================================================= */

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
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
      <div
        icon-btn
        m-2 p-2
        flex flex-row flex-grow
        justify-between
        @click="() => updateComponentShowed = !updateComponentShowed"
      >
        <p>
          Update package
        </p>
        <div
          class="update-sign"
          :class="{ 'rotate-180': updateComponentShowed }"
          text-lg
          i-carbon:chevron-down
        />
      </div>
      <div
        m-2 mt-0
        border="~ rounded gray-200 dark:gray-700"
      />
      <div
        class="update-menu"
        :class="{ showed: updateComponentShowed }"
      >
        <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
        <div v-if="errorMessage" flex-grow>
          <Error :message="errorMessage" />
        </div>
        <Suspense v-else>
          <!-- DONE - SHOW DYNAMIC COMPONENT -->
          <template #default>
            <PackageFormSettingsUpdate :pack="pack" @package-updated="() => emit('packageFormClosed')" />
          </template>
          <!-- LOADING - SHOW LOADING ANIMATION -->
          <template #fallback>
            <Loading />
          </template>
        </Suspense>
      </div>
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
        DELETE PACKAGE
      </div>
      <div text-sm text-gray>
        Package will be deleted from local disk only if it was copied by application (that happen before exit)
      </div>
    </div>
  </div>
</template>

<style lang="sass">
.update-sign
  transition: transform 200ms ease-in-out

.update-menu
  transition: all 600ms ease-in-out
  overflow: hidden
  max-height: 0

.update-menu.showed
  transition: all 600ms ease-in-out
  max-height: 100vh

.progress
    animation: progressBar 3s ease-in-out
    animation-fill-mode: forwards

@keyframes progressBar
  0%
    background-color: rgba(165, 42, 42, 0)
  100%
    background-color: rgba(165, 42, 42, 1)
</style>
