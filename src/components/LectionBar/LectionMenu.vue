<script setup lang="ts">
const { pkg } = defineProps<{ pkg: string }>()
const emit = defineEmits<{ (event: 'searchStringUpdated', newSearchString: string): void }>()
const searchStringUpdated = (newSearchString: string) => emit('searchStringUpdated', newSearchString)
</script>

<template>
  <div
    flex flex-row
    w-full
    gap-4
  >
    <div flex-grow>
      <LectionSearch @search-string-updated="searchStringUpdated" />
    </div>
    <div mr-a>
      Continue / Start reading
    </div>
  </div>
</template>
