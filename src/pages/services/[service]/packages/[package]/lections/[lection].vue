<script setup lang="ts">
import type { Ref } from 'vue'
import type { DedutyLection } from '~/composables/deduty'

const properties = defineProps<{ service: string; package: string; lection: string }>()

const packageStore = usePackageStore()

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})

const lectionObject: Ref<DedutyLection | undefined> = ref()

onMounted(async () => {
  await Promise.all([packageStore.initialRefreshPromise])
  lectionObject.value = (
    packageStore
      .indexedPackages
      .get(properties.service)
      ?.get(properties.package)
      ?.lections
      .find(lectionObject => lectionObject.id === properties.lection)
  )

  if (!lectionObject.value)
    errorMessage.value = `Lection with id \`${properties.lection}\` not found. Probably service, package or lection is not exist.`
})
</script>

<template>
  <div v-if="errorMessage" flex-grow>
    <Error :message="errorMessage" />
  </div>
  <LectionReader
    v-if="lectionObject"
    :service="properties.service"
    :pack="properties.package"
    :lection="lectionObject"
    :api-enabled="true"
  />
  <Loading v-else />
</template>
