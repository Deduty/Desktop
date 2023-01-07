<script setup lang="ts">
import type { Ref } from 'vue'

const { content, extension } = defineProps<{ content: Uint8Array; extension: string }>()

const ReaderImageComponentElement: Ref<HTMLImageElement | undefined> = ref()

const BlobReader = new FileReader()
BlobReader.onload = () => {
  if (!ReaderImageComponentElement.value)
    throw new Error('ReaderImage element is not loaded')

  const encoded = BlobReader.result

  if (typeof encoded !== 'string')
    throw new Error('BlobReader result is not base64 string')

  ReaderImageComponentElement.value.src = encoded
}

onMounted(() => {
  BlobReader.readAsDataURL(new Blob([content]))
})
</script>

<template>
  <div
    class="prose m-auto"
  >
    <h2>{{ extension }}</h2>
    <img ref="ReaderImageComponentElement" src="">
  </div>
</template>
