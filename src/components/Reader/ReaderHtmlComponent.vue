<script setup lang="ts">
import type { Ref } from 'vue'
import type { DedutyFile } from '~/composables/deduty'

const { file, scriptsAllowed } = defineProps<{ file: DedutyFile; scriptsAllowed: boolean }>()

const readerBlob = await (await file.createReader()).readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const readerBuffer = await readerBlob.arrayBuffer()
const readerContent = (new TextDecoder()).decode(readerBuffer)
const readerContentElement: Ref<HTMLElement | undefined> = ref()

onMounted(() => {
  if (readerContentElement.value && scriptsAllowed) {
    for (const script of readerContentElement.value.getElementsByTagName('script')) {
      const reloadedScript = document.createElement('script')
      reloadedScript.textContent = script.textContent

      script.parentNode?.appendChild(reloadedScript)
      script.remove()
    }
  }
})
</script>

<template>
  <div ref="readerContentElement" v-html="readerContent" />
</template>
