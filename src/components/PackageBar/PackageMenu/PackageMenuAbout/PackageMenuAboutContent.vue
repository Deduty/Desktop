<script setup lang="ts">
const { content, extension } = defineProps<{ content: string; extension: string }>()
</script>

<template>
  <div
    h-full w-full
    m-0
    align-middle
    justify-center
  >
    {{ extension }}
    {{ content }}
  </div>
</template>
