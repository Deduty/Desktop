<script setup lang="ts">
import { LectionSearchCriteria } from '~/composables/search'

const properties = defineProps<{ package: string }>()

/* ================ SEARCH TO LIST ================ */
const searchCriteria = ref(new LectionSearchCriteria(''))

const searchStringUpdated = (newSearchString: string) => {
  searchCriteria.value = new LectionSearchCriteria(newSearchString)
}
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
        <div
          flex-grow
          overflow-hidden
          m-0
        >
          <LectionList
            :package-id="properties.package"
            :criteria="searchCriteria"
          />
        </div>
      </div>
    </div>
  </div>
</template>
