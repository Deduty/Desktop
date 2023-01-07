<script setup lang="ts">
const { content, extension } = defineProps<{ content: Uint8Array; extension: string }>()

class Extension {
  constructor(public origin: string) {}

  isImage(): boolean {
    return ['png', 'jpeg', 'jpg', 'bmp', 'gif'].includes(this.origin)
  }

  isMarkdown(): boolean {
    return this.origin === 'md'
  }
}

const ExtensionInstance = new Extension(extension)
</script>

<template>
  <div
    h-full w-full
    m-0
    align-middle
    justify-center
  >
    <div
      v-if="ExtensionInstance.isImage()"
    >
      <ReaderImageComponent :content="content" :extension="extension" />
    </div>

    <div v-if="ExtensionInstance.isMarkdown()">
      <ReaderMarkdownComponent :content="content" />
    </div>
  </div>
</template>
