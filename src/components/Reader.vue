<script setup lang="ts">
// MARKDOWN IMPORTS
import MarkdownIt from 'markdown-it'
// import LinkAttributes from 'markdown-it-link-attributes'
// import Shiki from 'markdown-it-shiki'

const { content, extension } = defineProps<{ content: Uint8Array; extension: string }>()

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

// const RuntimeImage = computed(() => {
//   // ElementAttrs<ImgHTMLAttributes>
//   const RuntimeImageElement: Ref<ImgHTMLAttributes> = $ref()
//   RuntimeImageElement
// })
</script>

<template>
  <div
    h-full w-full
    m-0
    align-middle
    justify-center
  >
    <div
      v-if="['png', 'jpeg', 'jpg'].includes(extension)"
      class="prose prose-sm m-auto"
    >
      <img id="RuntimeImageElement" src="">
    </div>

    <div
      v-if="extension === 'md'"
      class="prose shiki prose-sm m-auto text-left"
    >
      <component :is="RuntimeMarkdown" />
    </div>
  </div>
</template>
