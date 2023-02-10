<script setup lang="ts">
import { LectionSearchCriteria } from '~/composables/search'

const properties = defineProps<{ service: string; package: string }>()

const searchCriteria = ref(new LectionSearchCriteria(''))
const searchStringUpdated = (newSearchString: string) => {
  searchCriteria.value = new LectionSearchCriteria(newSearchString)
}

const packageStore = usePackageStore()

const packageObject = (
  packageStore
    .indexedPackages
    .get(properties.service)
    ?.get(properties.package))

const errorMessage = `Package with id \`${properties.package}\` not found. Probably service or package is not exist.`
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
        <div v-if="!packageObject">
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
      </div>
    </div>
  </div>
</template>
