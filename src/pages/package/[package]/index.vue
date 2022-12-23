<script setup lang="ts">
const properties = defineProps<{ package: string }>()
</script>

<template>
  <div
    h-full w-full
  >
    <LectionBar :pkg="properties.package" />
  </div>
</template>
