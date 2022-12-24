<script setup lang="ts">
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { DedutyLection, type IDedutyLection } from '~/composables/deduty'

const { pkg, searchString } = defineProps<{ pkg: string; searchString: string }>()

const router = useRouter()

interface DedutyLectionItem {
  lection: DedutyLection
  showed: boolean
}

const lections: Ref<DedutyLectionItem[]> = ref([])

const testSearchString = (searchString: string, lection: DedutyLection): boolean => {
  const searchResult = lection.meta.name.match(searchString)

  return (
    searchResult !== null
    && searchResult.length > 0
  )
  || searchString === ''
  || lection.meta.name.includes(searchString)
}

const filterShowedLections = (searchString: string) => {
  lections.value.forEach((pair) => {
    pair.showed = testSearchString(searchString, pair.lection)
  })
}

invoke('listPackageLections', { package: pkg })
  .then((lections: unknown) => {
    if (!Array.isArray(lections))
      throw new Error(`Internal error: \`listPackageLections\` returns non string array but \`${lections}\``)
    return lections as string[]
  })
  // TODO: Limit by visible x2 on screen?
  .then(async (ids: string[]) => {
    for (const id of ids) {
      const lection: IDedutyLection = await invoke('getPackageLection', { package: pkg, lection: id })
      lections.value.push({ lection: DedutyLection.fromOptions(lection), showed: searchString === '' })
    }
  })

watch(() => searchString, (searchString: string) => filterShowedLections(searchString))
</script>

<template>
  <div
    flex flex-col
    h-full
    m-0 p-0
  >
    <ul overflow-y-auto flex flex-col gap-2>
      <li
        v-for="(pair, index) in lections" v-show="pair.showed"
        :key="index"
        @click="router.push(`/package/${pkg}/lection/${pair.lection.id}`)"
      >
        <LectionItem :lection="pair.lection" />
      </li>
    </ul>
  </div>
</template>
