<script setup lang="ts">
// MARKDOWN IMPORTS
import MarkdownIt from 'markdown-it'
import type { DedutyFileReader } from '~/composables/deduty/file/reader'
// import LinkAttributes from 'markdown-it-link-attributes'
// import Shiki from 'markdown-it-shiki'

const { reader } = defineProps<{ reader: DedutyFileReader }>()

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

const readerBlob = await reader.readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const readerBuffer = await readerBlob.arrayBuffer()
const decodedContent = (new TextDecoder()).decode(readerBuffer)

const RuntimeMarkdown = {
  template: ConfiguredMarkdownIt.render(decodedContent),
}
</script>

<template>
  <div
    class="shiki text-left"
  >
    <component :is="RuntimeMarkdown" />
  </div>
</template>
