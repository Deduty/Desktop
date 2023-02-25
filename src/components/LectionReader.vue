<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyLection, DedutyPackage } from '~/composables/deduty'
import { DedutyApi } from '~/composables/deduty/api'

const { target, apiEnabled, lectionChangingAllowed } = defineProps<{
  target: ITarget
  apiEnabled: boolean
  lectionChangingAllowed: boolean
}>()

interface ITarget {
  serviceId: string
  packageId: string
  lectionId: string
}

const router = useRouter()

class SmartLection {
  constructor(public lection: DedutyLection) {}

  go() {
    window.location.assign(`/services/${this.lection.serviceId}/packages/${this.lection.packageId}/lections/${this.lection.id}`)
  }

  get name(): string {
    return this.lection.meta.name
  }
}

const packageStore = usePackageStore()

const packageObject: Ref<DedutyPackage | undefined> = ref()
const lectionObject: Ref<DedutyLection | undefined> = ref()

const previousLection: Ref<SmartLection | undefined> = ref()
const nextLection: Ref<SmartLection | undefined> = ref()

// Getting valid lection index or throw error
{
  await Promise.all([packageStore.initialRefreshPromise])

  const servicePackages = packageStore.indexedPackages.get(target.serviceId)
  if (!servicePackages)
    throw new Error(`Service with id \`${target.serviceId}\` not found`)

  packageObject.value = servicePackages.get(target.packageId)
  if (!packageObject.value)
    throw new Error(`Package with id \`${target.packageId}\` not found`)

  for (const lection of packageObject.value?.lections) {
    if (lection.id === target.lectionId) {
      lectionObject.value = lection
      continue
    }

    if (lection.meta.hidden)
      continue

    if (lectionObject.value) {
      nextLection.value = new SmartLection(lection)
      break
    }

    previousLection.value = new SmartLection(lection)
  }

  if (!lectionObject.value)
    throw new Error(`Lection with id \`${target.packageId}\` not found`)
}

const Deduty: DedutyApi = new DedutyApi(packageObject.value, lectionObject.value)

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
        <div v-for="file in lectionObject!!.files" :key="file.id">
          <Reader :file="file" />
        </div>
      </div>
      <div
        v-if="lectionChangingAllowed && !lectionObject?.meta.hidden"
        p-4 m-a prose
        flex flex-row gap-6
      >
        <div
          v-if="previousLection"
          class="icon-btn"
          @click="router.push(`/services/${previousLection!.lection.serviceId}/packages/${previousLection!.lection.packageId}/lections/${previousLection!.lection.id}`)"
        >
          Read previous: {{ previousLection.name }}
        </div>
        <div
          v-if="nextLection"
          class="icon-btn"
          @click="nextLection!!.go()"
        >
          Read next: {{ nextLection.name }}
        </div>
      </div>
    </div>
  </div>
</template>
