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
        mr-a p-2
        text-2xl
      >
        {{ pack.meta.name }}
      </div>
      <div
        flex flex-row
        mt-a
        m-0 pl-2 pr-2

        class="gray-text"
        text="gray-400 dark:gray-500"
      >
        <div mr-a>
          Version: {{ pack.meta.version }}
        </div>
        <div>Local</div>
      </div>
    </div>
  </div>
</template>
