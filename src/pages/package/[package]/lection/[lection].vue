<script setup lang="ts">
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api'

import { DedutyLection, type IDedutyLection } from '~/composables/deduty'
import type { DedutyFileReader } from '~/composables/deduty/file/reader'

const properties = defineProps<{ package: string; lection: string }>()

interface ReaderFile {
  reader: DedutyFileReader
  extension: string
}

const readerArray: Ref<ReaderFile[]> = ref([])

invoke('getPackageLection', properties)
  .then((serialized: unknown) => {
    // TODO: UNSAFE CAST
    return DedutyLection.fromOptions(serialized as IDedutyLection)
  })
  .then(async (lection: DedutyLection) => {
    for (const file of lection.files.files) {
      readerArray.value.push({
        extension: file.extension,
        reader: await file.createReader(),
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
      <div
        flex flex-col
        m-a
        gap-4
      >
        <div v-for="(file, index) in readerArray" :key="index">
          <Reader :reader="file.reader" :extension="file.extension" />
        </div>
      </div>
    </div>
  </div>
</template>
