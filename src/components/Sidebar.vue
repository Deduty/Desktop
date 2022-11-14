<script setup lang="ts">
const isHomePage = ref(true)
</script>

<template>
  <div
    flex flex-row
    h-full
    m-0
  >
    <div
      border="~ rounded gray-200 dark:gray-700"
      class="vertical-line"
    />
    <nav p-2>
      <a>
        <div i-carbon-account />
      </a>
      <a>
        <div i-carbon-settings />
      </a>
      <a mt-a>
        <div
          v-show="!isHomePage"
          i-carbon-arrow-left
        />
        <div
          v-show="isHomePage"
          i-carbon-home
        />
      </a>
    </nav>
  </div>
</template>

<style scoped lang="sass">
div.vertical-line
  height: calc(100% - 1rem)
  margin-block: auto

nav
  display: grid
  grid-template-rows: max-content
  gap: 1rem

  a
    div
      font-size: 2.25rem
</style>
