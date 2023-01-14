<script setup lang="ts">
// COMPONENT IMPORTS
import ReaderHtmlComponent from './ReaderHtmlComponent.vue'
import ReaderImageComponent from './ReaderImageComponent.vue'
import ReaderMarkdownComponent from './ReaderMarkdownComponent.vue'

import type { DedutyFileReader } from '~/composables/deduty/file/reader'

const { reader, extension } = defineProps<{ reader: DedutyFileReader; extension: string }>()

class Extension {
  constructor(public origin: string) {}

  isHtml(): boolean {
    return this.origin === 'html'
  }

  isImage(): boolean {
    return ['png', 'jpeg', 'jpg', 'bmp', 'gif'].includes(this.origin)
  }

  isMarkdown(): boolean {
    return this.origin === 'md'
  }
}

class ReaderComponent {
  private m_component
  private m_properties: object

  constructor(ext: Extension) {
    if (ext.isHtml()) {
      this.m_component = ReaderHtmlComponent
      this.m_properties = { reader }
    }
    else if (ext.isImage()) {
      this.m_component = ReaderImageComponent
      this.m_properties = { reader, extension }
    }
    else if (ext.isMarkdown()) {
      this.m_component = ReaderMarkdownComponent
      this.m_properties = { reader }
    }
    else {
      this.m_component = Error
      this.m_properties = { message: `File extension \`${ext.origin}\` is not supported` }
    }
  }

  get component() {
    return this.m_component
  }

  get properties() {
    return this.m_properties
  }
}

const ComponentInstance = new ReaderComponent(new Extension(extension))
const ErrorMessage = ref('')

onErrorCaptured((error) => {
  ErrorMessage.value = error.message
})
</script>

<template>
  <div
    h-full w-full
    m-0
    align-middle
    justify-center
  >
    <div
      prose m-auto
    >
      <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
      <div v-if="ErrorMessage">
        <Error :message="ErrorMessage" />
      </div>
      <Suspense v-else>
        <!-- DONE - SHOW DYNAMIC COMPONENT -->
        <template #default>
          <component :is="ComponentInstance.component" v-bind="ComponentInstance.properties" />
        </template>
        <!-- LOADING - SHOW LOADING ANIMATION -->
        <template #fallback>
          <Loading />
        </template>
      </Suspense>
    </div>
  </div>
</template>
