<script setup lang="ts">
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api'

import { DedutyLection, type IDedutyLection } from '~/composables/deduty'

const properties = defineProps<{ package: string; lection: string }>()

interface ContentFile {
  content: string
  extension: string
}

const contentArray: Ref<ContentFile[]> = ref([])

invoke('getPackageLection', properties)
  .then((serialized: unknown) => {
    // TODO: UNSAFE CAST
    return DedutyLection.fromOptions(serialized as IDedutyLection)
  })
  .then(async (lection: DedutyLection) => {
    for (const file of lection.files.files) {
      contentArray.value.push({
        extension: file.extension,
        content: await invoke('getLectionFile', {
          ...properties,
          location: file.location,
        }),
      })
    }
  })
</script>

<template>
  <div
    h-full w-full
    p-2
  >
    <div
      h-full w-full
      overflow-y-auto
    >
      <div m-a>
        <div v-for="(file, index) in contentArray" :key="index">
          <Reader :content="file.content" :extension="file.extension" />
        </div>
      </div>
    </div>
  </div>
</template>
