<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyLection } from '~/composables/deduty'
import type { IDedutyApi } from '~/composables/deduty/api'
import { DedutyWebStorageApi } from '~/composables/deduty/api/web-storage'

interface Target {
  service: string
  package: string
  lection: string
}

const { target, apiEnabled, lectionChangingAllowed } = defineProps<{
  target: Target
  apiEnabled: boolean
  lectionChangingAllowed: boolean
}>()

const packageStore = usePackageStore()

const packageLections: Ref<DedutyLection[]> = ref([])
const currentLection: Ref<number> = ref(0)

// Getting valid lection index or throw error
{
  await Promise.all([packageStore.initialRefreshPromise])
  const service = packageStore.indexedPackages.get(target.service)
  if (!service)
    throw new Error(`Service with id \`${target.service}\` not found`)

  const pack = service.get(target.package)
  if (!pack)
    throw new Error(`Package with id \`${target.package}\` not found`)

  let rawLectionId: number | undefined

  packageLections.value = [...pack.lections.filter(
    // Lection is hidden or lection is hidden but it's our TARGET
    lection => !lection.meta.hidden || lection.id === target.lection)]

  for (const [index, lection] of packageLections.value.entries()) {
    if (lection.id === target.lection) {
      rawLectionId = index
      break
    }
  }

  if (rawLectionId === undefined)
    throw new Error(`Lection with id \`${target.lection}\` not found`)

  currentLection.value = rawLectionId
}

const Deduty: IDedutyApi = {
  webStorage: {
    lection: new DedutyWebStorageApi(target.service, target.package, target.lection),
    package: new DedutyWebStorageApi(target.service, target.package),
  },
}

watch(currentLection, (currentLection) => {
  Deduty.webStorage.lection = new DedutyWebStorageApi(
    target.service,
    target.package,
    packageLections.value[currentLection].id,
  )
})

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
    let Deduty = document.Deduty;
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
        <div v-for="file in packageLections[currentLection].files" :key="file.id">
          <Reader :file="file" />
        </div>
      </div>
      <div
        v-if="lectionChangingAllowed"
        p-4 m-a prose
        flex flex-row gap-6
      >
        <div
          v-if="currentLection > 0"
          class="icon-btn"
          @click="currentLection -= 1"
        >
          Read previous: {{ packageLections[currentLection - 1]?.meta.name }}
        </div>
        <div
          v-if="currentLection < packageLections.length - 1"
          class="icon-btn"
          @click="currentLection += 1"
        >
          Read next: {{ packageLections[currentLection + 1]?.meta.name }}
        </div>
      </div>
    </div>
  </div>
</template>
