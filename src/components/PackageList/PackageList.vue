<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import type { PackageSearchCriteria } from '~/composables/search'
import { usePackageStore } from '~/store/package'

const { criteria } = defineProps<{ criteria: PackageSearchCriteria }>()
const emit = defineEmits<{ (event: 'dedutyPackageChosen', packageObject: DedutyPackage): void }>()

interface PackageDisplayItem {
  package: DedutyPackage
  showed: boolean
}

const packageStore = usePackageStore()

const packageDisplayItems: Ref<PackageDisplayItem[]> = ref([])

const replaceDisplayItems = (packages: DedutyPackage[]) => {
  packageDisplayItems.value = packages.map(pack =>
    ({ package: pack, showed: criteria.match(pack) } as PackageDisplayItem))
}

const filterSearchCriteria = (criteria: PackageSearchCriteria) => {
  for (const pair of packageDisplayItems.value)
    pair.showed = criteria.match(pair.package)
}

watch(() => criteria, filterSearchCriteria, { deep: true })
watch(() => packageStore.storedPackages, replaceDisplayItems, { deep: true })
onMounted(() => replaceDisplayItems(packageStore.storedPackages))
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
        <PackageItem :package-object="pair.package" />
      </li>
    </ul>
  </div>
</template>
