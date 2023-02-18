<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import { LectionSearchCriteria } from '~/composables/search'

const properties = defineProps<{ service: string; package: string }>()

const searchCriteria = ref(new LectionSearchCriteria(''))
const searchStringUpdated = (newSearchString: string) => {
  searchCriteria.value = new LectionSearchCriteria(newSearchString)
}

const packageStore = usePackageStore()

const packageObject: Ref<DedutyPackage | null> = ref(null)
const errorMessage = ref('')

onMounted(async () => {
  packageObject.value = packageStore.indexedPackages.get(properties.service)?.get(properties.package) || null
  if (!packageObject.value) {
    await packageStore.refresh(false, [properties.service])
    packageObject.value = packageStore.indexedPackages.get(properties.service)?.get(properties.package) || null
  }
  if (!packageObject.value)
    errorMessage.value = `Package with id \`${properties.package}\` not found. Probably service or package is not exist.`
})
</script>

<template>
  <div
    flex flex-row
    h-full w-full
    justify-center
    gap-4
  >
    <div
      flex flex-col
      p-2
      style="width: min(65ch, 100%)"
    >
      <div
        flex flex-col
        border="~ rounded gray-200 dark:gray-700"
        h-full
        p-4
        gap-4
      >
        <div>
          <Search
            :place-holder-variants="[
              'Introduction',
              'Diffusion',
              'Group theory',
              'Ring theory',
              'Easter egg',
            ]" @search-string-updated="searchStringUpdated"
          />
        </div>
        <div v-if="errorMessage">
          <Error :message="errorMessage" />
        </div>
        <div
          v-if="packageObject"
          flex-grow
          overflow-hidden
          m-0
        >
          <LectionList
            :pack="packageObject"
            :criteria="searchCriteria"
          />
        </div>
        <div
          v-else
          flex-grow m-0
        >
          <Loading flex flex-grow />
        </div>
      </div>
    </div>
  </div>
</template>
