<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import type { SearchCriteria } from '~/composables/search'
import { usePackageStore } from '~/store/package'

const { criteria } = defineProps<{ criteria: SearchCriteria }>()
const emit = defineEmits<{ (event: 'dedutyPackageChosen', pack: DedutyPackage): void }>()

interface PackageDisplayItem {
  package: DedutyPackage
  showed: boolean
}

const packageStore = usePackageStore()

const packageDisplayItems: Ref<PackageDisplayItem[]> = ref([])
const replaceDisplayItems = (packages: DedutyPackage[]) => {
  packageDisplayItems.value = packages.map(pkg =>
    ({ package: pkg, showed: true } as PackageDisplayItem))
}

watch(
  () => criteria,
  (criteria: SearchCriteria) => {
    for (const pair of packageDisplayItems.value)
      pair.showed = criteria.match(pair.package)
  },
  { deep: true },
)

watch(() => packageStore.packages, replaceDisplayItems)
onMounted(() => replaceDisplayItems(packageStore.packages))
</script>

<template>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    h-full
    m-0 p-2
  >
    <ul overflow-y-auto>
      <li
        v-for="(pair, index) in packageDisplayItems" v-show="pair.showed"
        :key="index"
        @click="emit('dedutyPackageChosen', pair.package)"
      >
        <PackageItem :pack="pair.package" />
      </li>
    </ul>
  </div>
</template>
