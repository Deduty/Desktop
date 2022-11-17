<script setup lang="ts">
import type { Package } from '~/composables/deduty'

const { pkg } = defineProps<{ pkg: Package }>()
</script>

<template>
  <div
    flex flex-row
    border="~ rounded gray-200 dark:gray-700"
    class="package item"
    m-2 p-2
  >
    <div
      flex flex-col
      gap-2
    >
      <div
        class="black-text"
        text-lg
      >
        {{ pkg.name }}
      </div>
      <div m-a />
      <div
        class="gray-text"
        text="sm gray-400 dark:gray-500"
      >
        Version: {{ pkg.version }}
      </div>
    </div>
    <div m-a />
    <div
      flex flex-col
      gap-2
    >
      <div
        class="black-text"
        text="sm right"
      >
        {{ pkg.size.toString() }}
      </div>
      <div m-a />
      <div
        class="gray-text"
        text="sm right gray-400 dark:gray-500"
      >
        {{ pkg.source }}
      </div>
    </div>
  </div>
</template>

<style scoped lang="sass">
div.package.item
  cursor: pointer
  transition: all 200ms ease-in-out

div.package.item:hover
  background-color: darkcyan
  transition: all 200ms ease-in-out

div.package.item:hover .black-text
  color: white
  transition: all 200ms ease-in-out

div.package.item:hover .gray-text
  color: black
  transition: all 200ms ease-in-out
</style>
