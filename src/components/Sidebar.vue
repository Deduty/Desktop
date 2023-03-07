<script setup lang="ts">
import type { RouteLocationNormalized } from 'vue-router'

const router = useRouter()

const haveHistory = ref(false)
const isHomePage = ref(true)

const specialBackAddress = ref('')

const routerBackWithFallback = () => {
  if (specialBackAddress.value)
    router.push(specialBackAddress.value)

  else
    router.back()
}

const hookRouterPath = (to: RouteLocationNormalized) => {
  // Clear alternative back address
  specialBackAddress.value = ''

  // Detect case when force reload on lection page
  // In that case router is empty
  if (
    to.name === 'services-serviceId-packages-packageId-lections-lectionId'
    && window.history.state.back === null
  ) {
    // In case when no history, fallback to lections
    specialBackAddress.value = `/services/${to.params.serviceId}/packages/${to.params.packageId}/lections`
  }

  if (
    to.name === 'services-serviceId-packages-packageId-lections'
    && window.history.state.back === null
  ) {
    // In case when no history, fallback to lections
    specialBackAddress.value = '/'
  }

  haveHistory.value = window.history.state.back !== null || specialBackAddress.value !== ''
  isHomePage.value = to.path === '/'
}

router.afterEach(hookRouterPath)

onMounted(() => hookRouterPath(router.currentRoute.value))
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
        @click="router.push('/settings')"
      >
        <div i-carbon-settings />
      </button>
      <button
        class="button"
        mt-a
        :disabled="!haveHistory"
        @click="routerBackWithFallback"
      >
        <div
          i-carbon-arrow-left
        />
      </button>
      <button
        class="button"
        :disabled="isHomePage"
        @click="router.push('/')"
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
