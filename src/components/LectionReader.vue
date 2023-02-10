<script setup lang="ts">
import type { DedutyLection } from '~/composables/deduty'

import type { IDedutyApi } from '~/composables/deduty/api'
import { DedutyWebStorageApi } from '~/composables/deduty/api/web-storage'

const { service, pack, lection, apiEnabled } = defineProps<{ service: string; pack: string; lection: DedutyLection; apiEnabled: boolean }>()

const Deduty: IDedutyApi = {
  webStorage: {
    lection: new DedutyWebStorageApi(service, pack, lection.id),
    package: new DedutyWebStorageApi(service, pack),
  },
}

// @ts-expect-error: The `document` object is being used for passing DedutyApi into lection
document.Deduty = Deduty
</script>

<template>
  <!--
    Note: This hack is required for setting a global DedutyApi object for lection interaction
          Also note that next line removed link to the object in document that was set before
  -->
  <!-- eslint-disable -->
  <component is="script" v-if="apiEnabled">
    const Deduty = document.Deduty;
    delete document.Deduty;
  </component>
  <!-- eslint-enable -->
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
        <div v-for="(file, index) in lection.files" :key="index">
          <Reader :file="file" />
        </div>
      </div>
    </div>
  </div>
</template>
