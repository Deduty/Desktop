<script setup lang="ts">
const { pkg } = defineProps<{ pkg: string }>()
</script>

<template>
  <div
    flex flex-col
    h-full
    m-0 p-2
  >
    <div>
      <LectionMenu :pkg="pkg" />
    </div>
    <div>
      <LectionList :pkg="pkg" />
    </div>
  </div>
</template>
