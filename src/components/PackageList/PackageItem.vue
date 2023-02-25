<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

const { packageObject } = defineProps<{ packageObject: DedutyPackage }>()
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
        {{ packageObject.meta.name }}
      </div>
      <div
        m-a
        min-w-1
        w-1
      />
      <div
        class="black-text"
        text="sm right"
      >
        {{ `${packageObject.serviceId.charAt(0).toUpperCase()}${packageObject.serviceId.slice(1)}` }}
      </div>
    </div>
    <div
      flex flex-row
      items-baseline
    >
      <div
        class="gray-text"
        text="sm gray-400 dark:gray-500"
      >
        Version: {{ packageObject.meta.version }}
      </div>
      <div m-a />
      <div
        class="gray-text"
        text="sm right gray-400 dark:gray-500"
      >
        {{ packageObject.packageSize() }}
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
