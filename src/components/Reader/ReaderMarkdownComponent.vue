<script setup lang="ts">
// MARKDOWN IMPORTS
import MarkdownIt from 'markdown-it'
import { escapeHtml } from 'markdown-it/lib/common/utils'
import HightLightJS from 'highlight.js'

import { getCurrentStyles } from '~/composables/highlight'
import type { DedutyFile } from '~/composables/deduty'

const { file } = defineProps<{ file: DedutyFile }>()

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

/* ========================= MARKDOWN CONFIGURATION ======================== */
const highlight = (source: string, language: string, _: string): string => {
  let compiledSource = ''

  try {
    compiledSource = HightLightJS.highlight(source, { language }).value
  }
  catch (error) {
    console.error(`Unable to highlight \`${language}\`. Unexpected error:`, error, '\nFallback...')
    compiledSource = escapeHtml(source)
  }

  return `<pre class="hljs"><code class="language-${language}">${compiledSource}</code></pre>`
}

const ConfiguredMarkdownIt = MarkdownIt({
  html: true,
  langPrefix: 'language-',
  linkify: true,
  typographer: true,
  highlight,
})
/* ========================= ======== ============= ======================== */

/* ====================== PREPARING DYNAMIC COMPONENT ====================== */
const readerBlob = await (await file.createReader()).readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const readerBuffer = await readerBlob.arrayBuffer()
const decodedContent = (new TextDecoder()).decode(readerBuffer)

const RuntimeMarkdown = {
  template: ConfiguredMarkdownIt.render(decodedContent),
}
/* ====================== ========= ======= ========= ====================== */

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
    <component :is="RuntimeMarkdown" />
  </div>
</template>
