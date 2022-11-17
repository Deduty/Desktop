<script setup lang="ts">
const inputString = ref('')

const placeHolderString = ref('')
const placeHolderVariants = [
  'Tutorial',
  'Mathematics',
  'Physics',
  'Geometry',
  'English',
  'Easter egg',
]

onMounted(() => {
  let placeHolderIndex: number
  let isPlaceHolderAddingAnimation = true

  setInterval(() => {
    switch (placeHolderString.value.length) {
      case 0:
        // Index inits here
        placeHolderIndex = Math.floor(Math.random() * placeHolderVariants.length)
        isPlaceHolderAddingAnimation = true
        break

      case placeHolderVariants[placeHolderIndex].length:
        isPlaceHolderAddingAnimation = false
        break
    }

    placeHolderString.value = placeHolderVariants[placeHolderIndex]
      // 1 => 1, 0 => -1 || y = 2x - 1
      .slice(0, placeHolderString.value.length + 2 * Number(isPlaceHolderAddingAnimation) - 1)
  },
  200 /* ms */,
  )
})
</script>

<template>
  <div
    flex flex-row
    border="~ rounded gray-200 dark:gray-700"
    class="box"
  >
    <input
      v-model="inputString"
      :placeholder="placeHolderString"
      w-full
      p-2
      outline-0
    >
    <button
      class="remove"
      :class="inputString === '' ? '' : 'some'"
      p-2
      @click="inputString = ''"
    >
      <div
        i-carbon-filter-remove
        text-2xl
      />
    </button>
    <button
      class="search"
      p-2
    >
      <div
        i-carbon-search
        text-2xl
      />
    </button>
  </div>
  <div />
</template>

<style scoped lang="sass">
// ================================ INPUT BAR =================================
html.dark input
  background-color: #121212

// ================================ CONTAINER =================================
// ---------------------------------- REMOVE ----------------------------------
div.box:hover > button.remove
  opacity: 0%
  transition: all 200ms ease-in-out

div.box:hover > button.remove:hover
  opacity: 0%
  transition: all 200ms ease-in-out

// ------------------------------ REMOVE.SOME ---------------------------------
div.box:hover > button.remove.some
  opacity: 60%
  transition: all 200ms ease-in-out

div.box:hover > button.remove.some:hover
  opacity: 90%
  transition: all 200ms ease-in-out

// ---------------------------------- SEARCH ----------------------------------
div.box:hover > button.search
  background-color: cadetblue
  transition: all 200ms ease-in-out

div.box:hover > button.search:hover
  background-color: darkcyan
  transition: all 200ms ease-in-out
// ================================== REMOVE ==================================
button.remove
  cursor: default
  opacity: 0%
  transition: all 200ms ease-in-out

button.remove.some
  cursor: pointer
  opacity: 0%
  transition: all 200ms ease-in-out

// ================================== SEARCH ==================================
button.search
  border-top-right-radius: 0.25rem
  border-bottom-right-radius: 0.25rem
</style>
