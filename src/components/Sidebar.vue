<script setup lang="ts">
const Router = useRouter()

const haveHistory = ref(false)
const isHomePage = ref(true)

const hookRouterPath = (to: { path: String }) => {
  haveHistory.value = window.history.state.back !== null
  isHomePage.value = to.path === '/'
}

Router.afterEach(hookRouterPath)

onMounted(() => hookRouterPath({ path: window.location.pathname }))
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
      gap-4
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
        @click="Router.push('/')"
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
  color: darkcyan
  transition: all 200ms ease-in-out

.button:disabled
  cursor: default
  opacity: 0.5

div.vertical-line
  height: calc(100% - 1rem)
  margin-block: auto

nav
  button
    height: fit-content
    width: fit-content

    div
      font-size: 2.25rem
</style>
