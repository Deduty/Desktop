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
    class="box"
  >
    {{ pkg?.name }}
  </div>
</template>

<style scoped lang="sass">
html div.box
  background-color: white

html.dark div.box
  background-color: #121212
</style>
