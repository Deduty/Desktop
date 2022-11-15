<script setup lang="ts">
const Router = useRouter()

const haveHistory = ref(false)
const isHomePage = ref(true)

watch(Router, (router) => {
  isHomePage.value = router.currentRoute.value.path === '/'
  haveHistory.value = window.history.length > 1
})
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
    <nav
      flex flex-col
      p-2
    >
      <button
        class="button"
      >
        <div i-carbon-account />
      </button>
      <button
        class="button"
      >
        <div i-carbon-settings />
      </button>
      <button
        class="button"
        mt-a
        :disabled="!haveHistory"
        @click="Router.back()"
      >
        <div
          i-carbon-arrow-left
        />
      </button>
      <button
        class="button"
        :disabled="isHomePage"
      >
        <div
          i-carbon-home
        />
      </button>
    </nav>
  </div>
</template>

<style scoped lang="sass">
.button
  cursor: pointer
  transition: all 200ms ease-in-out

.button:hover
  color: green
  transition: all 200ms ease-in-out

.button:disabled
  cursor: default
  opacity: 0.5

div.vertical-line
  height: calc(100% - 1rem)
  margin-block: auto

nav
  gap: 1rem

  button
    height: fit-content
    width: fit-content

    div
      font-size: 2.25rem
</style>
