<script setup lang="ts">
import type { Package } from '~/composables/deduty'

const { pkg } = defineProps<{ pkg: Package | null }>()
</script>

<template>
  <div
    m-0
    flex flex-row
  >
    <!-- META -->
    <div
      flex flex-col flex-grow
    >
      <!-- TITLE -->
      <div
        mr-a p-2

        text-2xl
      >
        {{ pkg?.meta.name }}
      </div>
      <!-- VERSION \ SOURCE -->
      <div
        flex flex-row
        m-0 pl-2 pr-2

        class="gray-text"
        text="gray-400 dark:gray-500"
      >
        <div mr-a>
          Version: {{ pkg?.meta.version }}
        </div>
        <div>Local</div>
      </div>
    </div>
    <!-- META END -->
    <div
      ml-2 mr-2
      border="~ rounded gray-200 dark:gray-700"
      class="vertical-line"
    />
    <!-- MINI SIDE MENU -->
    <div
      p-2
      gap-2
      flex flex-col
    >
      <div
        text-3xl
        border-rounded
        class="package form button"
      >
        <div i-carbon-book />
      </div>
      <div m-a />
      <div
        text-3xl
        border-rounded
        class="package form button"
      >
        <div i-carbon-settings />
      </div>
    </div>
    <!-- MINI SIDE MENU END -->
  </div>
</template>

<style scoped lang="sass">
div.package.form.button
  cursor: pointer
  transition: all 200ms ease-in-out

div.package.form.button:hover
  // background-color: darkcyan
  color: darkcyan
  transition: all 200ms ease-in-out
</style>
