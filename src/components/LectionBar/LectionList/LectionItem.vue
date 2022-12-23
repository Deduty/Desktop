<script setup lang="ts">
import type { DedutyLection } from '~/composables/deduty'

const { lection } = defineProps<{ lection: DedutyLection }>()
</script>

<template>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    class="package item"
    gap-2
    m-2 p-2
  >
    <div
      flex flex-row
      items-baseline
    >
      <div
        class="black-text"
        text-lg
      >
        {{ lection.meta.name }} - {{ lection.meta.order }}
      </div>
      <div
        m-a
        min-w-1
        w-1
      />
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
