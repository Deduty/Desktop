<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyLection, DedutyPackage } from '~/composables/deduty'

import type { LectionSearchCriteria } from '~/composables/search'

const { pack, criteria } = defineProps<{ pack: DedutyPackage; criteria: LectionSearchCriteria }>()

const router = useRouter()
const lectionDisplayItems: Ref<{ lection: DedutyLection; showed: boolean }[]> = ref([])

const filterSearchCriteria = (criteria: LectionSearchCriteria) => {
  for (const pair of lectionDisplayItems.value)
    pair.showed = criteria.match(pair.lection) && !pair.lection.meta.hidden
}

watch(() => criteria, filterSearchCriteria, { deep: true })

for (const lection of pack.lections) {
  lectionDisplayItems.value.push({
    lection,
    showed: criteria.match(lection) && !lection.meta.hidden,
  })
}
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
        v-for="(pair, index) in lectionDisplayItems" v-show="pair.showed"
        :key="index"
        @click="router.push(`/services/${pack.service}/packages/${pack.id}/lections/${pair.lection.id}`)"
      >
        <LectionItem :lection="pair.lection" />
      </li>
    </ul>
  </div>
</template>
