<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Package } from '~/composables/deduty'

const { pkg } = defineProps<{ pkg: Package | null }>()

enum AboutScreen {
  Content = 'Content',
  Error = 'Error',
  Loading = 'Loading',
  Nothing = 'Nothing',
}

const aboutScreen = ref(AboutScreen.Loading)
const contentExtension = ref('')
const contentString = ref('')
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
      if (typeof value !== 'string')
        throw new Error(`Internal error: Application provides wrong type '${typeof value}' when 'string' was expected`)
      return value as string
    })
    .then((content: string) => {
      aboutScreen.value = AboutScreen.Content
      contentExtension.value = about.extension
      contentString.value = content
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
    <PackageMenuAboutContent
      v-if="aboutScreen === AboutScreen.Content"
      :content="contentString"
      :extension="contentExtension"
    />
    <!-- NOTHING SCREEN (When no file with 'about' alias) -->
    <div
      v-if="aboutScreen === AboutScreen.Nothing"
      m-0
      align-middle
      justify-center
    >
      About is not represented
    </div>
  </div>
</template>
