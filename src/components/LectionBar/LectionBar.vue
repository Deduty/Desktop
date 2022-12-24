<script setup lang="ts">
const { pkg } = defineProps<{ pkg: string }>()

const searchString = ref('')

const searchStringUpdated = (newSearchString: string) => {
  searchString.value = newSearchString
}
</script>

<template>
  <div
    flex flex-col
    h-full
    m-0 p-4
    gap-4
  >
    <div m-0 p-0>
      <LectionMenu :pkg="pkg" @search-string-updated="searchStringUpdated" />
    </div>
    <div m-0 p-0>
      <LectionList :pkg="pkg" :search-string="searchString" />
    </div>
  </div>
</template>
