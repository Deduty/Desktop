<script setup lang="ts">
// COMPONENT IMPORTS
import Error from '../Error.vue'

import ReaderHtmlComponent from './ReaderHtmlComponent.vue'
import ReaderImageComponent from './ReaderImageComponent.vue'
import ReaderMarkdownComponent from './ReaderMarkdownComponent.vue'

import { DynamicComponent } from '~/composables/dynamic'
import type { DedutyFile } from '~/composables/deduty'

const { file, scriptsAllowed } = defineProps<{ file: DedutyFile; scriptsAllowed: boolean }>()

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

  createComponent(): DynamicComponent {
    if (this.isHtml())
      return new DynamicComponent(ReaderHtmlComponent, { file, scriptsAllowed })

    if (this.isImage())
      return new DynamicComponent(ReaderImageComponent, { file })

    if (this.isMarkdown())
      return new DynamicComponent(ReaderMarkdownComponent, { file })

    return new DynamicComponent(Error, { message: `File extension \`${this.origin}\` is not supported` })
  }
}

const componentInstance = (new Extension(file.ext)).createComponent()
const ErrorMessage = ref('')

onErrorCaptured((error) => {
  ErrorMessage.value = error.message
})
</script>

<template>
  <div
    :id="file.meta.name"

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
          <component
            :is="componentInstance.comp"
            v-bind="componentInstance?.prop"
            v-on="componentInstance?.even"
          />
        </template>
        <!-- LOADING - SHOW LOADING ANIMATION -->
        <template #fallback>
          <Loading />
        </template>
      </Suspense>
    </div>
  </div>
</template>
