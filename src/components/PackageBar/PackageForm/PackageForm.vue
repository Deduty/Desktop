<script setup lang="ts">
import type { Package } from '~/composables/deduty'

const { pkg } = defineProps<{ pkg: Package | null }>()
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
      <PackageFormMenu :pkg="pkg" />
    </div>
    <div
      flex-grow
      overflow-auto
    >
      <PackageFormAbout :pkg="pkg" />
    </div>
  </div>
</template>

<style scoped lang="sass">
html div.box
  background-color: white

html.dark div.box
  background-color: #121212
</style>
