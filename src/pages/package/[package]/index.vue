<script setup lang="ts">
const properties = defineProps<{ package: string }>()
</script>

<template>
  <div m-a text-3xl>
    PACKAGE: {{ properties.package }}
  </div>
</template>
