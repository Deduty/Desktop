<script setup lang="ts">
import MarkdownIt from 'markdown-it'

const { content, extension } = defineProps<{ content: string; extension: string }>()

const ConfiguredMarkdownIt = MarkdownIt()

const RuntimeMarkdown = computed(() => {
  return {
    template: ConfiguredMarkdownIt.render(content),
  }
})
</script>

<template>
  <div
    h-full w-full
    m-0
    align-middle
    justify-center
  >
    <div
      class="prose shiki prose-sm m-auto text-left"
    >
      <component :is="RuntimeMarkdown" />
    </div>
  </div>
</template>
