<script setup lang="ts">
import { type Ref, compile } from 'vue'

const { content } = defineProps<{ content: Uint8Array }>()

const emit = defineEmits<{
  (event: 'error', message: string): void
  (event: 'loading'): void
  (event: 'success'): void
}>()

const RuntimeHtml = compile((new TextDecoder()).decode(content))

const HtmlElement: Ref<HTMLElement | undefined> = ref()

onMounted(() => {
  emit('loading')

  if (!HtmlElement.value) {
    emit('error', 'Unable to get static html element')
    throw new Error('Unable to get static html element')
  }

  emit('success')
})
</script>

<template>
  <div
    ref="HtmlElement"
    class="shiki text-left"
  >
    <RuntimeHtml />
  </div>
</template>
