<script setup lang="ts">
import type { Ref } from 'vue'
import { Package, PackageSize, PackageSource } from '~/composables/deduty'

const { searchString } = defineProps<{ searchString: string }>()

interface PackageSearchItem {
  package: Package
  showed: boolean
}

const packageSearchList: Ref<PackageSearchItem[]> = ref([])
const currentPackage: Ref<Package | null> = ref(null)

const packageMenuElement = ref<HTMLElement>()

onMounted(() => {
  for (let i = 0; i < 1000; i += 1) {
    packageSearchList.value.push(
      {
        package: Package.fromOptions({
          name: `Template ${i}`,
          version: `1.${i}.${i * 2}`,
          source: Object.values(PackageSource)[i % 3] as PackageSource,
          size: new PackageSize(1024 * (i + 1)),
          language: ['English', 'Russian'][i % 2],
        }),
        showed: true,
      },
    )
  }
})

watch(
  () => searchString,
  (searchString) => {
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
  },
)
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
