<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

import Loading from '~/components/Loading.vue'
import Reader from '~/components/Reader/Reader.vue'

const { pkg } = defineProps<{ pkg: DedutyPackage | null }>()

const ComponentInstance = shallowRef(Loading)
const ComponentProperties = ref({})

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})

watchEffect(async () => {
  errorMessage.value = ''

  ComponentProperties.value = {}
  ComponentInstance.value = Loading

  if (pkg === null) {
    errorMessage.value = 'All values (include error message) are just flushed. If you see this text - frontend is little bit broken'
    return
  }

  const candidates = pkg.files.files.filter(file => file.alias === 'about')
  if (candidates.length === 0) {
    errorMessage.value = 'NO ABOUT FILE'
    return
  }

  if (candidates.length !== 1) {
    errorMessage.value = 'Package have several \'about\' alias. Expected only one for package representation'
    return
  }
  const [about] = candidates

  ComponentProperties.value = { reader: await about.createReader(), extension: about.extension }
  ComponentInstance.value = Reader
})
</script>

<template>
  <div
    h-full w-full
    m-0
  >
    <div v-if="errorMessage">
      <Error :message="errorMessage" />
    </div>
    <Suspense v-else>
      <!-- DONE - SHOW DYNAMIC COMPONENT -->
      <template #default>
        <component :is="ComponentInstance" v-bind="ComponentProperties" />
      </template>
      <!-- LOADING - SHOW LOADING ANIMATION -->
      <template #fallback>
        <Loading />
      </template>
    </Suspense>
  </div>
</template>
