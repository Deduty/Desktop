<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Ref } from 'vue'
import type { DedutyPackage } from '~/composables/deduty'

const { pkg } = defineProps<{ pkg: DedutyPackage | null }>()

enum AboutScreen {
  Content = 'Content',
  Error = 'Error',
  Loading = 'Loading',
  Nothing = 'Nothing',
}

const aboutScreen = ref(AboutScreen.Loading)
const contentExtension = ref('')
const contentArray: Ref<Uint8Array> = ref(new Uint8Array())
const errorMessage = ref('')

watchEffect(async () => {
  errorMessage.value = 'All values (include error message) are just flushed. If you see this text - frontend is little bit broken'
  aboutScreen.value = AboutScreen.Loading

  if (pkg === null) {
    errorMessage.value = 'Package not set for this window. You never suppose to see this'
    aboutScreen.value = AboutScreen.Error
    return
  }

  const files = pkg.files.files.filter(file => file.alias === 'about')
  if (files.length === 0) {
    aboutScreen.value = AboutScreen.Nothing
    return
  }
  if (files.length !== 1) {
    errorMessage.value = 'Package have several \'about\' alias. Expected only one for package representation'
    aboutScreen.value = AboutScreen.Error
    return
  }

  const [about] = files

  invoke('getPackageFile', { id: pkg.id, location: about.location })
    .then((value: unknown) => {
      if (!Array.isArray(value))
        throw new Error(`Internal error: Application provides wrong type '${typeof value}' when 'string' was expected`)
      return new Uint8Array(value)
    })
    .then((content: Uint8Array) => {
      aboutScreen.value = AboutScreen.Content
      contentExtension.value = about.extension
      contentArray.value = content
    })
    .catch((error) => {
      errorMessage.value = `Unable to get about file: ${error}`
      aboutScreen.value = AboutScreen.Error
    })
})
</script>

<template>
  <div
    h-full w-full
    m-0
  >
    <!-- DEFAULT SCREENS -->
    <Error v-if="aboutScreen === AboutScreen.Error" :message="errorMessage" />
    <Loading v-if="aboutScreen === AboutScreen.Loading" />
    <!-- CONTENT SCREEN -->
    <Reader
      v-if="aboutScreen === AboutScreen.Content"
      :content="contentArray"
      :extension="contentExtension"
    />
    <!-- NOTHING SCREEN (When no file with 'about' alias) -->
    <div
      v-if="aboutScreen === AboutScreen.Nothing"
      h-full w-full
      m-0
      grid
    >
      <div
        m-a
        grid-cols-none
        grid-rows-none
        text-2xl
      >
        About is not represented
      </div>
    </div>
  </div>
</template>
