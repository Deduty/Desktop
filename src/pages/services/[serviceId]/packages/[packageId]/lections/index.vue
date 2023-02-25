<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import { LectionSearchCriteria } from '~/composables/search'

const { serviceId, packageId } = defineProps<{ serviceId: string; packageId: string }>()

const searchCriteria = ref(new LectionSearchCriteria(''))
const searchStringUpdated = (newSearchString: string) => {
  searchCriteria.value = new LectionSearchCriteria(newSearchString)
}

const packageStore = usePackageStore()

const packageObject: Ref<DedutyPackage | null> = ref(null)
const errorMessage = ref('')

onMounted(async () => {
  packageObject.value = packageStore.indexedPackages.get(serviceId)?.get(packageId) || null
  if (!packageObject.value) {
    await packageStore.refresh(false, [serviceId])
    packageObject.value = packageStore.indexedPackages.get(serviceId)?.get(packageId) || null
  }
  if (!packageObject.value)
    errorMessage.value = `Package with id \`${packageId}\` not found. Probably service or package is not exist.`
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
            :package-object="packageObject"
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
