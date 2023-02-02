<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import { PackageSearchCriteria } from '~/composables/search'

import PackageForm from '~/components/PackageForm/PackageForm.vue'
import PackageAdd from '~/components/PackageAdd/PackageAdd.vue'

/* ================ SEARCH TO LIST ================ */

const searchCriteria = ref(new PackageSearchCriteria(''))

const searchStringUpdated = (newSearchString: string) => {
  searchCriteria.value = new PackageSearchCriteria(newSearchString)
}

/* ============= LIST TO PACKAGE FORM ============= */

class ComponentInstance {
  constructor(public comp: any, public prop: object) {}
}

const componentInstance: Ref<ComponentInstance | null> = shallowRef(null)

const dedutyDisplayChosen = (pack: DedutyPackage) => {
  componentInstance.value = new ComponentInstance(PackageForm, { pkg: pack })
}
</script>

<template>
  <!-- OVERLAY COMPONENT -->
  <div
    v-show="componentInstance"
    flex flex-row flex-grow
    h-full w-full
    left-0 right-0 top-0 bottom-0
    fixed
    class="overlay"
  >
    <div
      flex flex-grow
      justify-center items-center
      @click.self="componentInstance = null"
    >
      <component
        :is="componentInstance.comp"
        v-if="componentInstance"
        v-bind="componentInstance.prop"
      />
    </div>
  </div>
  <!-- ------- --------- -->
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
        <div
          flex flex-row
          m-0
          gap-2
        >
          <Search @search-string-updated="searchStringUpdated" />
          <button
            icon-btn
            border="~ rounded gray-200 dark:gray-700"
            @click="componentInstance = new ComponentInstance(PackageAdd, {})"
          >
            <div
              m-2
              p-4
              i-carbon-add
            />
          </button>
        </div>
        <div
          flex-grow
          overflow-hidden
          m-0
        >
          <PackageList
            :criteria="searchCriteria"
            @deduty-package-chosen="dedutyDisplayChosen"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="sass">
div.overlay
  backdrop-filter: blur(5px)
  z-index: 100
</style>
