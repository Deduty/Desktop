<script setup lang="ts">
import type { Ref } from 'vue'
import type { DedutyFile } from '~/composables/deduty'

const { file } = defineProps<{ file: DedutyFile }>()

const readerBlob = await (await file.createReader()).readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const readerBuffer = await readerBlob.arrayBuffer()
const readerContent = (new TextDecoder()).decode(readerBuffer)
const readerContentElement: Ref<HTMLElement | undefined> = ref()

onMounted(() => {
  if (readerContentElement.value) {
    const scripts = readerContentElement.value.getElementsByTagName('script')

    for (const script of scripts) {
      const inserted = document.createElement('script')
      inserted.textContent = script.textContent
      document.head.appendChild(inserted)
    }
  }
})
</script>

<template>
  <div
    ref="HtmlElement"
    class="shiki text-left"
  >
    <div ref="readerContentElement" v-html="readerContent" />
  </div>
</template>
