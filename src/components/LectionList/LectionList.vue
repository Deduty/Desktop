<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Ref } from 'vue'

import type { IDedutyLection } from '~/composables/deduty'
import { DedutyLection } from '~/composables/deduty'
import type { LectionSearchCriteria } from '~/composables/search'

const { packageId, criteria } = defineProps<{ packageId: string; criteria: LectionSearchCriteria }>()

interface LectionDisplayItem {
  lection: DedutyLection
  showed: boolean
}

const router = useRouter()
const lectionDisplayItems: Ref<LectionDisplayItem[]> = ref([])

const filterSearchCriteria = (criteria: LectionSearchCriteria) => {
  for (const pair of lectionDisplayItems.value)
    pair.showed = criteria.match(pair.lection)
}

watch(() => criteria, filterSearchCriteria, { deep: true })

invoke('listPackageLections', { id: packageId })
  .then((lections: unknown) => {
    if (Array.isArray(lections))
      return lections as string[]
    throw new TypeError(`Internal error: Lections returned from backend must be \`string[]\`, not \`${typeof lections}\``)
  })
  .then(async (lections: string[]) => {
    for (const lectionId of lections) {
      const options = await invoke('getPackageLection', { package: packageId, lection: lectionId }) as IDedutyLection
      const lection = DedutyLection.fromOptions(packageId, options)
      lectionDisplayItems.value.push({ lection, showed: criteria.match(lection) } as LectionDisplayItem)
    }
  })
  .catch(console.error)
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
        @click="router.push(`/package/${packageId}/lection/${pair.lection.id}`)"
      >
        <LectionItem :lection="pair.lection" />
      </li>
    </ul>
  </div>
</template>
