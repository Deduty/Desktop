<script setup lang="ts">
const { pkg } = defineProps<{ pkg: string }>()
</script>

<template>
  <div>
    MENU OVER {{ pkg }}
  </div>
</template>
