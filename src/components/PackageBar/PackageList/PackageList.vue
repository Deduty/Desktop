<script setup lang="ts">
import type { Ref } from 'vue'

import { Package, PackageSize, PackageSource } from '~/composables/deduty'

const currentPackage: Ref<Package | null> = ref(null)
const packageList: Ref<Package[]> = ref([])

const packageMenuElement = ref<HTMLElement>()

onMounted(() => {
  for (let i = 0; i < 1000; i += 1) {
    packageList.value.push(
      Package.fromOptions({
        name: `Template ${i}`,
        version: `1.${i}.${i * 2}`,
        source: Object.values(PackageSource)[i % 3] as PackageSource,
        size: new PackageSize(1024 * (i + 1)),
        language: ['English', 'Russian'][i % 2],
      }))
  }
})
</script>

<template>
  <div
    v-show="currentPackage !== null"
    flex flex-row
    h-full w-full
    left-0 right-0 top-0 bottom-0
    fixed
    class="overlay"
    @click="currentPackage = null"
  >
    <div
      flex flex-grow
      justify-center items-center
    >
      <div
        ref="packageMenuElement"
        class="overlay box"
      >
        <PackageMenu :pkg="currentPackage" />
      </div>
    </div>
  </div>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    h-full
    m-0 p-2
  >
    <ul overflow-y-scroll>
      <li
        v-for="(pkg, index) in packageList" :key="index"
        @click="currentPackage = pkg"
      >
        <PackageItem :pkg="pkg" />
      </li>
    </ul>
  </div>
</template>

<style scoped lang="sass">
div.overlay
  backdrop-filter: blur(5px)
  z-index: 100

div.overlay.box
  width: 40%
  height: 90%
</style>
