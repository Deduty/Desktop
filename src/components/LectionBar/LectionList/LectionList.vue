<script setup lang="ts">
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { DedutyLection, type IDedutyLection } from '~/composables/deduty'

const { pkg } = defineProps<{ pkg: string }>()
const lections: Ref<DedutyLection[]> = ref([])
const router = useRouter()

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
      lections.value.push(DedutyLection.fromOptions(lection))
    }
  })
</script>

<template>
  <div
    flex flex-col
    h-full
    m-0 p-2
  >
    <ul overflow-y-auto>
      <li
        v-for="(lection, index) in lections" :key="index"
        @click="router.push(`/package/${pkg}/lection/${lection.id}`)"
      >
        <LectionItem :lection="lection" />
      </li>
    </ul>
  </div>
</template>
