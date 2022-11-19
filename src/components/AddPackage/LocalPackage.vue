<script setup lang="ts">
import { dialog } from '@tauri-apps/api'

const pathString = ref('')
const selectPath = () => {
  dialog.open({
    directory: true,
    multiple: false,
    title: 'Select folder',
  })
    .then((value) => {
      if (typeof value === 'string')
        pathString.value = value
      else if (Array.isArray(value))
        pathString.value = value[0]
      else
        pathString.value = ''
    })
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
