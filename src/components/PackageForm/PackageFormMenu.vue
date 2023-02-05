<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage | null }>()
const emit = defineEmits<{ (event: 'toggleSettingsClicked'): void }>()

const isSettingsToggled = ref(false)

const router = useRouter()
</script>

<template>
  <div
    m-0
    flex flex-row
  >
    <!-- META -->
    <div
      flex flex-col flex-grow
    >
      <!-- TITLE -->
      <div
        mr-a p-2

        text-2xl
      >
        {{ pack?.meta.name }}
      </div>
      <!-- VERSION \ SOURCE -->
      <div
        flex flex-row
        m-0 pl-2 pr-2

        class="gray-text"
        text="gray-400 dark:gray-500"
      >
        <div mr-a>
          Version: {{ pack?.meta.version }}
        </div>
        <div>Local</div>
      </div>
    </div>
    <!-- META END -->
    <div
      ml-2 mr-2
      border="~ rounded gray-200 dark:gray-700"
    />
    <!-- MINI SIDE MENU -->
    <div
      p-2
      gap-2
      flex flex-col
    >
      <div
        text-3xl
        border-rounded
        class="package form button"
        @click="router.push(`/package/${pack?.id}`)"
      >
        <div i-carbon-book />
      </div>
      <div m-a />
      <div
        v-show="!isSettingsToggled"
        text-3xl
        border-rounded
        class="package form button"
        @click="emit('toggleSettingsClicked'); isSettingsToggled = true"
      >
        <div i-carbon-settings />
      </div>
      <div
        v-show="isSettingsToggled"
        text-3xl
        border-rounded
        class="package form button"
        @click="emit('toggleSettingsClicked'); isSettingsToggled = false"
      >
        <div i-carbon-account />
      </div>
    </div>
    <!-- MINI SIDE MENU END -->
  </div>
</template>

<style scoped lang="sass">
div.package.form.button
  cursor: pointer
  transition: all 200ms ease-in-out

div.package.form.button:hover
  // background-color: darkcyan
  color: darkcyan
  transition: all 200ms ease-in-out
</style>
