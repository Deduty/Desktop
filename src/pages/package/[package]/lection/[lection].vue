<script setup lang="ts">
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api'

import { DedutyLection, type IDedutyLection } from '~/composables/deduty'
import type { DedutyFileReader } from '~/composables/deduty/file/reader'

import type { IDedutyApi } from '~/composables/deduty/api'
import { DedutyLectionStorageApi, DedutyPackageStorageApi } from '~/composables/deduty/api/storage'

const properties = defineProps<{ package: string; lection: string }>()

interface ReaderFile {
  reader: DedutyFileReader
  extension: string
}

const readerArray: Ref<ReaderFile[]> = ref([])

invoke('getPackageLection', properties)
  .then((serialized: unknown) => {
    // TODO: UNSAFE CAST
    return DedutyLection.fromOptions(properties.package, serialized as IDedutyLection)
  })
  .then(async (lection: DedutyLection) => {
    for (const file of lection.files.files) {
      readerArray.value.push({
        extension: file.extension,
        reader: await file.createReader(),
      })
    }
  })

const Deduty: IDedutyApi = {
  storage: {
    lection: new DedutyLectionStorageApi(properties.package, properties.lection),
    package: new DedutyPackageStorageApi(properties.package),
  },
}

document.Deduty = Deduty
</script>

<template>
  <component is="script">
    const Deduty = document.Deduty;
    delete document.Deduty;
  </component>
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
