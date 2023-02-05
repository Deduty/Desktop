<script setup lang="ts">
const { message } = defineProps<{ message: string }>()
</script>

<template>
  <div
    h-full w-full
    align-middle
    justify-center
    flex flex-grow
    border="~ rounded red-200 dark:red-700"
    m-0 p-4
  >
    {{ message }}
  </div>
</template>
