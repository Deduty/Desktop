<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageFormClosed'): void }>()

const isSettingsToggled = ref(false)
const toggleSettingsClicked = () => {
  isSettingsToggled.value = !isSettingsToggled.value
}

const packageFormClosed = () => {
  emit('packageFormClosed')
}
</script>

<template>
  <div
    h-full w-full
    border="~ rounded gray-200 dark:gray-700"
    bg-op-0
    m-0 p-4
    flex flex-col
    class="box"
    gap-4
  >
    <div>
      <PackageFormMenu
        :pack="pack"
        @toggle-settings-clicked="toggleSettingsClicked"
      />
    </div>
    <div
      flex-grow
      overflow-auto
    >
      <PackageFormAbout
        v-show="!isSettingsToggled"
        :pack="pack"
      />
      <PackageFormSettings
        v-show="isSettingsToggled"
        :pack="pack"
        @package-form-closed="packageFormClosed"
      />
    </div>
  </div>
</template>

<style scoped lang="sass">
div.box
  width: 65ch
  height: 90%

html div.box
  background-color: white

html.dark div.box
  background-color: #121212
</style>
