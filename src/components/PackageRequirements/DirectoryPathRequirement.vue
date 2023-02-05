<script setup lang="ts">
import { dialog } from '@tauri-apps/api'

const emit = defineEmits<{
  (event: 'requirementSatisfied', serialized: string): void
  (event: 'requirementNotSatisfied'): void
}>()

const errorMessage = ref('')
const placeHolderText = ref('')

const pathString = ref('')
const selectPath = () => {
  dialog.open({
    directory: true,
    multiple: false,
    title: 'Select folder',
  })
    .then((path: string[] | string | null) => {
      placeHolderText.value = ''

      if (!path)
        throw new Error('SKIP')

      if (typeof path !== 'string')
        throw new Error(`Internal error: Path is not an array, but ${path}`)

      placeHolderText.value = path
      emit('requirementSatisfied', path)
    })
    .catch((error) => {
      emit('requirementNotSatisfied')

      if (error.message !== 'SKIP')
        errorMessage.value = error.message
    })
}
</script>

<template>
  <div
    flex flex-col flex-grow
    gap-2
  >
    <div
      flex flex-col flex-grow
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
        flex flex-row flex-grow
        class="box select"
        border="~ rounded gray-200 dark:gray-700"
      >
        <input
          v-model="pathString"
          :placeholder="placeHolderText || 'Press the button -->'"
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
    <div v-if="errorMessage">
      <Error :message="errorMessage" />
    </div>
  </div>
</template>

<style scoped lang="sass">
// ============================== BOX BACKGROUND ==============================
// html div.box
//   background-color: white

// html.dark div.box
//   background-color: #121212

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
