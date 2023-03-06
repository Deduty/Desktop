<script setup lang="ts">
import { dialog } from '@tauri-apps/api'

import type { DedutyPackage } from '~/composables/deduty'
import * as Commands from '~/composables/commands'

const { packageObject } = defineProps<{ packageObject: DedutyPackage }>()

const webStorageShowed = ref(false)

const exportWebStorage = async () => {
  const path = await dialog.save({ title: 'Save export file', defaultPath: 'export' })
  if (!path)
    return

  if (typeof path !== 'string')
    throw new Error(`Internal error: Path is not an array, but ${path}`)

  Commands.webStorageExport(packageObject.serviceId, packageObject.id, path)
}

const importWebStorage = async () => {
  const path = await dialog.open({ title: 'Select import file' })
  if (!path)
    return

  if (typeof path !== 'string')
    throw new Error(`Internal error: Path is not an array, but ${path}`)

  Commands.webStorageImport(packageObject.serviceId, packageObject.id, path)
}

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})
</script>

<template>
  <div
    icon-btn
    m-2 p-2
    flex flex-row flex-grow
    justify-between
    @click="webStorageShowed = !webStorageShowed"
  >
    <p>
      Web storage
    </p>
    <div
      class="web-sign"
      :class="{ 'rotate-180': webStorageShowed }"
      text-lg
      i-carbon:chevron-down
    />
  </div>
  <div
    m-2 mt-0
    border="~ rounded gray-200 dark:gray-700"
  />
  <div
    class="web-menu"
    :class="{ showed: webStorageShowed }"
  >
    <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
    <div v-if="errorMessage" flex-grow>
      <Error :message="errorMessage" />
    </div>
    <div
      v-else
      flex flex-row
      gap-2
      p-4
      border="~ rounded gray-200 dark:gray-700"
    >
      <button
        border="~ rounded gray-200 dark:gray-700"
        class="confirm-button"
        icon-btn
        p-2
        @click="exportWebStorage"
      >
        Export
      </button>
      <button
        border="~ rounded gray-200 dark:gray-700"
        class="confirm-button"
        icon-btn
        p-2
        @click="importWebStorage"
      >
        Import
      </button>
      <button
        border="~ rounded gray-200 dark:gray-700"
        class="confirm-button"
        icon-btn
        p-2 ml-a
        @click="Commands.webStorageClear(packageObject.serviceId, packageObject.id)"
      >
        Clear
      </button>
    </div>
  </div>
</template>

<style lang="sass">
html.dark .confirm-button[disabled]
  color: rgb(55, 65, 81)

html.dark .confirm-button[disabled]:hover
  color: rgb(55, 65, 81)
  opacity: 0.75

.confirm-button[disabled]
  color: rgb(229, 231, 235)

.confirm-button[disabled]:hover
  color: rgb(229, 231, 235)
  opacity: 0.75

.web-sign
  transition: transform 200ms ease-in-out

.web-menu
  transition: all 600ms ease-in-out
  overflow: hidden
  max-height: 0

.web-menu.showed
  transition: all 600ms ease-in-out
  max-height: 100vh
</style>
