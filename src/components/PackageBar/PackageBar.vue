<script setup lang="ts">
const searchString = ref('')

const searchStringUpdated = (newSearchString: string) => {
  searchString.value = newSearchString
}
</script>

<template>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    h-full
    p-4
    gap-4
  >
    <div m-0>
      <PackageSearch @search-string-updated="searchStringUpdated" />
    </div>
    <div
      flex-grow
      overflow-hidden
      m-0
    >
      <PackageList :search-string="searchString" />
    </div>
  </div>
</template>
