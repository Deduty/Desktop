<script setup lang="ts">
const { content, extension } = defineProps<{ content: Uint8Array; extension: string }>()

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

enum ReaderState {
  Error = 'Error',
  Loading = 'Loading',
  Success = 'Success',
}

/* STATE KEEPERS */
const currentState = ref(ReaderState.Success)
const errorMessage = ref('')

/* READER STATE HANDLERS */
const handleErrorState = (message: string) => {
  errorMessage.value = message
  currentState.value = ReaderState.Error
}

const handleLoadingState = () => {
  currentState.value = ReaderState.Loading
  errorMessage.value = ''
}

const handleSuccessState = () => {
  currentState.value = ReaderState.Success
  errorMessage.value = ''
}
/* --------------------- */

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
      prose m-auto
    >
      <div v-show="currentState === ReaderState.Success">
        <!-- READER SUB-COMPONENTS -->
        <div v-if="ExtensionInstance.isHtml()">
          <ReaderHtmlComponent
            :content="content"
            @error="handleErrorState"
            @loading="handleLoadingState"
            @success="handleSuccessState"
          />
        </div>
        <div v-else-if="ExtensionInstance.isImage()">
          <ReaderImageComponent
            :content="content"
            :extension="extension"
            @error="handleErrorState"
            @loading="handleLoadingState"
            @success="handleSuccessState"
          />
        </div>
        <div v-else-if="ExtensionInstance.isMarkdown()">
          <ReaderMarkdownComponent
            :content="content"
            @error="handleErrorState"
            @loading="handleLoadingState"
            @success="handleSuccessState"
          />
        </div>
        <div v-else>
          <Error :message="`The following file format is not supported (may be not yet): ${extension}`" />
        </div>
      </div>
      <!-- OTHER SCREENS -->
      <div v-show="currentState === ReaderState.Loading">
        <div m-auto prose>
          <Loading />
        </div>
      </div>
      <div v-show="currentState === ReaderState.Error">
        <div m-auto prose>
          <Error :message="errorMessage" />
        </div>
      </div>
    </div>
  </div>
</template>
