<script setup lang="ts">
import type { Ref } from 'vue'
import type { DedutyFileReader } from '~/composables/deduty/file/reader'

const { reader } = defineProps<{ reader: DedutyFileReader; extension: string }>()

const ReaderImageComponentElement: Ref<HTMLImageElement | undefined> = ref()

const readerBlob = await reader.readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const ImageEncoder = new FileReader()
ImageEncoder.onload = () => {
  if (!ReaderImageComponentElement.value)
    throw new Error('ReaderImage element is not loaded')

  const encoded = ImageEncoder.result

  if (typeof encoded !== 'string')
    throw new Error('BlobReader result is not base64 string')

  ReaderImageComponentElement.value.src = encoded
}
ImageEncoder.readAsDataURL(readerBlob)
</script>

<template>
  <div
    class="prose m-auto"
  >
    <img ref="ReaderImageComponentElement" src="">
  </div>
</template>
