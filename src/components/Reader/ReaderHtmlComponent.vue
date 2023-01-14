<script setup lang="ts">
import { compile } from 'vue'
import type { DedutyFileReader } from '~/composables/deduty/file/reader'

const { reader } = defineProps<{ reader: DedutyFileReader }>()

const readerBlob = await reader.readAll()
if (!readerBlob)
  throw new Error('Reader return null value. Probably file empty or already was read. Try to reload page')

const readerBuffer = await readerBlob.arrayBuffer()

const RuntimeHtml = compile((new TextDecoder()).decode(readerBuffer))
</script>

<template>
  <div
    ref="HtmlElement"
    class="shiki text-left"
  >
    <RuntimeHtml />
  </div>
</template>
