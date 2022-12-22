<script setup lang="ts">
const properties = defineProps<{ package: string; lection: string }>()
</script>

<template>
  <div m-a text-3xl>
    PACKAGE: {{ properties.package }}
    LECTION: {{ properties.lection }}
  </div>
</template>
