<script setup lang="ts">
import type { Ref } from 'vue'
import type { Package } from '~/composables/deduty'
import { usePackageStore } from '~/store/package'

const { searchString } = defineProps<{ searchString: string }>()

interface PackageSearchItem {
  package: Package
  showed: boolean
}

const packageStore = usePackageStore()

const packageSearchList: Ref<PackageSearchItem[]> = ref([])
const currentPackage: Ref<Package | null> = ref(null)

const filterShowedPackages = (searchString: string) => {
  packageSearchList.value.forEach((pair) => {
    const searchResult = pair.package.name.match(searchString)
    pair.showed = (
      (
        searchResult !== null
        && searchResult.length > 0
      )
      || pair.package.name.includes(searchString)
    )
  })
}

watch(packageStore.packages, () => {
  packageSearchList.value = packageStore.packages.map(pkg =>
    ({ package: pkg, showed: true } as PackageSearchItem))

  if (searchString !== '')
    filterShowedPackages(searchString)
})

watch(() => searchString, filterShowedPackages)

onMounted(() => packageStore.init())
</script>

<template>
  <div
    v-show="currentPackage !== null"
    flex flex-row
    h-full w-full
    left-0 right-0 top-0 bottom-0
    fixed
    class="overlay"
  >
    <div
      flex flex-grow
      justify-center items-center
      @click.self="currentPackage = null"
    >
      <div
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
    <ul overflow-y-auto>
      <li
        v-for="(pair, index) in packageSearchList" v-show="pair.showed"
        :key="index"
        @click="currentPackage = pair.package"
      >
        <PackageItem :pkg="pair.package" />
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
