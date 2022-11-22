<script setup lang="ts">
import { dialog, invoke } from '@tauri-apps/api'
import { Package, PackageSize, PackageSource } from '~/composables/deduty'

interface ISerializedDedutyPackage {
  name: string
  version: string
  language: string
}

const packageStore = usePackageStore()

const pathString = ref('')
const selectPath = () => {
  dialog.open({
    directory: true,
    multiple: false,
    title: 'Select folder',
  })
    .then((path: string[] | string | null) => {
      if (typeof path !== 'string')
        throw new Error(`Internal error: Path is not an array, but ${path}`)

      return invoke('addLocalPackage', { path })
    })
    .then((pkg: unknown) => {
      if (typeof pkg !== 'object' || !pkg)
        throw new Error('Internal error: Serialized package must be an object')

      // TODO: Unsafe cast
      return pkg as ISerializedDedutyPackage
    })
    .then((pkg: ISerializedDedutyPackage) => {
      packageStore.include(
        new Package(
          pkg.name,
          pkg.version,
          PackageSource.Local,
          new PackageSize(0),
          pkg.language,
        ),
      )
    })
    .catch(console.error)
}
</script>

<template>
  <div
    class="box"
    h-full w-full
    m-0 p-4
    border="~ rounded gray-200 dark:gray-700"
    bg-op-0
  >
    <div
      flex flex-col
    >
      <div text-xl mb-2>
        Add local package
      </div>
      <div
        flex flex-col
        border="~ rounded gray-200 dark:gray-700"
        gap-1
        p-2
      >
        <span
          text-lg
        >
          Select folder
        </span>
        <div
          flex flex-row
          class="box select"
          border="~ rounded gray-200 dark:gray-700"
        >
          <input
            v-model="pathString"
            placeholder="Press the button -->"
            disabled
            w-full
            p-2
            outline-0
          >
          <button
            class="select"
            text-2xl
            p-2
            @click="selectPath()"
          >
            . . .
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="sass">
// ============================== BOX BACKGROUND ==============================
html div.box
  background-color: white

html.dark div.box
  background-color: #121212

// ================================ INPUT BAR =================================
html input
  background-color: white

html.dark input
  background-color: #121212

// ================================= BOX PATH =================================
// ---------------------------------- SELECT ----------------------------------
div.box.select:hover > button.select
  background-color: cadetblue
  transition: all 200ms ease-in-out

div.box.select:hover > button.select:hover
  background-color: darkcyan
  transition: all 200ms ease-in-out
// ================================== SELECT ==================================
button.select
  min-width: fit-content
  width: 1%

  border-top-right-radius: 0.25rem
  border-bottom-right-radius: 0.25rem
</style>
