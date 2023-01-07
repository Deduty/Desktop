<script setup lang="ts">
import type { Ref } from 'vue'

const { content, extension } = defineProps<{ content: Uint8Array; extension: string }>()

const emit = defineEmits<{
  (event: 'error', message: string): void
  (event: 'loading'): void
  (event: 'success'): void
}>()

const ReaderImageComponentElement: Ref<HTMLImageElement | undefined> = ref()

const BlobReader = new FileReader()
BlobReader.onload = () => {
  if (!ReaderImageComponentElement.value) {
    emit('error', 'ReaderImage element is not loaded')
    throw new Error('ReaderImage element is not loaded')
  }

  const encoded = BlobReader.result

  if (typeof encoded !== 'string') {
    emit('error', 'BlobReader result is not base64 string')
    throw new Error('BlobReader result is not base64 string')
  }

  ReaderImageComponentElement.value.src = encoded
  emit('success')
}

onMounted(() => {
  BlobReader.readAsDataURL(new Blob([content]))
})
</script>

<template>
  <div
    class="prose m-auto"
  >
    <img ref="ReaderImageComponentElement" src="">
  </div>
</template>
