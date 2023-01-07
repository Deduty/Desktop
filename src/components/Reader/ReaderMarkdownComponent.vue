<script setup lang="ts">
// MARKDOWN IMPORTS
import MarkdownIt from 'markdown-it'
// import LinkAttributes from 'markdown-it-link-attributes'
// import Shiki from 'markdown-it-shiki'

const { content } = defineProps<{ content: Uint8Array }>()

// MARKDOWN SETUP
const ConfiguredMarkdownIt = MarkdownIt()
// .use(Shiki, {
//   theme: {
//     light: 'vitesse-light',
//     dark: 'vitesse-dark',
//   },
// })
// .use(LinkAttributes, {
//   matcher: (link: string) => /^https?:\/\//.test(link),
//   attrs: {
//     target: '_blank',
//     rel: 'noopener',
//   },
// })

const RuntimeMarkdown = computed(() => {
  return {
    template: ConfiguredMarkdownIt.render(
      (new TextDecoder()).decode(content.buffer),
    ),
  }
})
</script>

<template>
  <div
    class="prose shiki prose-sm m-auto text-left"
  >
    <component :is="RuntimeMarkdown" />
  </div>
</template>
