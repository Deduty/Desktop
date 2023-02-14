<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
</script>

<template>
  <div
    m-0
    h-full w-full
    flex flex-row flex-grow
  >
    <div
      m-0
      h-full w-full
      flex flex-col flex-grow
    >
      <div
        flex flex-grow flex-row
        items-baseline
      >
        <div
          p-2
          text-2xl
        >
          {{ pack.meta.name }}
        </div>
        <div
          m-a
          min-w-1
          w-1
        />
        <div
          text-lg
        >
          {{ `${pack.service.charAt(0).toUpperCase()}${pack.service.slice(1)}` }}
        </div>
      </div>
      <div
        flex flex-row
        mt-a
        m-0 pl-2

        class="gray-text"
        text="gray-400 dark:gray-500"
      >
        <div
          class="gray-text"
          text="sm gray-400 dark:gray-500"
        >
          Version: {{ pack.meta.version }}
        </div>
        <div
          class="gray-text"
          text="sm right gray-400 dark:gray-500"
          ml-a
        >
          {{ pack.packageSize() }}
        </div>
      </div>
    </div>
  </div>
</template>
