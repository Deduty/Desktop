<script setup lang="ts">
const { message } = defineProps<{ message: string }>()
</script>

<template>
  <div
    grid
    align-middle
    justify-center
    m-a p-4
    flex-grow
  >
    {{ message }}
  </div>
</template>
