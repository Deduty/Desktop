<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyFile } from '~/composables/deduty'

const { file } = defineProps<{ file: DedutyFile }>()

const ReaderImageComponentElement: Ref<HTMLImageElement | undefined> = ref()

const readerBlob = await (await file.createReader()).readAll()
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
    <img ref="ReaderImageComponentElement" class="rounded" src="">
  </div>
</template>
