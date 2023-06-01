<script setup lang="ts">
// Unified
import { unified } from 'unified'

// Unified - Remark
import remarkParse from 'remark-parse'
import remarkMath from 'remark-math'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'

// Unified - Rehype
import rehypeKatex from 'rehype-katex'
import rehypeHighlight from 'rehype-highlight'
import rehypeStringify from 'rehype-stringify'

// Highlight styles
import { getCurrentStyles } from '~/composables/highlight'

// Properties
import type { DedutyFile } from '~/composables/deduty'

const { file } = defineProps<{ file: DedutyFile }>()

const parsedContent = ref('')

const readerBlob = await (await file.createReader()).readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const readerBuffer = await readerBlob.arrayBuffer()
const readerContent = (new TextDecoder()).decode(readerBuffer)

await unified()
  // Remark
  .use(remarkParse)
  .use(remarkMath, { singleDollarTextMath: true })
  .use(remarkGfm)
  .use(remarkRehype, { allowDangerousHtml: true })
  // Rehype
  .use(rehypeKatex, { displayMode: true })
  .use(rehypeHighlight, { ignoreMissing: true })
  .use(rehypeStringify, { allowDangerousHtml: true })
  // Conclusion
  .process(readerContent)
  .then((file) => {
    parsedContent.value = file.toString()
  })

/* ========================= DARK DYNAMIC SUPPORT ========================== */
const updateMarkdownHighlight = (isDark: boolean) => {
  if (isDark) {
    document
      .querySelectorAll('pre, pre > code, pre > code > *')
      .forEach(element => element.classList.add('dark'))
  }
  else {
    document
      .querySelectorAll('pre, pre > code, pre > code > *')
      .forEach(element => element.classList.remove('dark'))
  }
}

const isDark = useDark()
watch(isDark, updateMarkdownHighlight)
/* ========================= ==== ======= ======= ========================== */

onMounted(async () => {
  updateMarkdownHighlight(isDark.value)

  const CURRENT_HIGHLIGHT_STYLE = 'CURRENT_HIGHLIGHT_STYLE'

  // When highlight set for entire page
  if (document.getElementById(CURRENT_HIGHLIGHT_STYLE))
    return

  const elementStyle = document.createElement('style')
  elementStyle.id = CURRENT_HIGHLIGHT_STYLE

  const { dark, light } = await getCurrentStyles()

  elementStyle.textContent = [
    dark.styleSheet({ classNameTemplate: name => `html.dark .${name}` }),
    light.styleSheet({ classNameTemplate: name => `.${name}` }),
  ].join('\n')
  document.body.appendChild(elementStyle)
})
</script>

<template>
  <div
    class="shiki text-left"
  >
    <div v-html="parsedContent" />
  </div>
</template>

<style lang="sass">
.katex-html
  display: none
</style>
