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

enum ReaderState {
  Error = 'Error',
  Loading = 'Loading',
  Success = 'Success',
}

/* STATE KEEPERS */
const currentState = ref(ReaderState.Loading)
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
    <div v-show="currentState === ReaderState.Success">
      <!-- READER SUB-COMPONENTS -->
      <div v-if="ExtensionInstance.isImage()">
        <ReaderImageComponent
          :content="content"
          :extension="extension"
          @error="handleErrorState"
          @loading="handleLoadingState"
          @success="handleSuccessState"
        />
      </div>
      <div v-if="ExtensionInstance.isMarkdown()">
        <ReaderMarkdownComponent
          :content="content"
          @error="handleErrorState"
          @loading="handleLoadingState"
          @success="handleSuccessState"
        />
      </div>
    </div>
    <!-- OTHER SCREENS -->
    <div v-show="currentState === ReaderState.Loading">
      <Loading />
      Textjlkasjfl;kj asl;kfdjasl;kfjasl;kd
    </div>
    <div v-show="currentState === ReaderState.Error">
      <Error :message="errorMessage" />
    </div>
  </div>
</template>
