<script setup lang="ts">
import { isDark, preferredDark, toggleDark } from '~/composables/dark'

const isAutoUseDark = ref(localStorage['vueuse-color-scheme'] === 'auto')

const useDarkToggle = () => {
  isAutoUseDark.value = false
  toggleDark()
}

const useSystemThemeToggle = () => {
  if (isAutoUseDark.value) {
    localStorage['vueuse-color-scheme'] = isDark.value ? 'dark' : 'light'
    isAutoUseDark.value = false
  }
  else {
    localStorage['vueuse-color-scheme'] = 'auto'
    toggleDark(preferredDark.value)
    isAutoUseDark.value = true
  }
}
</script>

<template>
  <div flex flex-col gap-2>
    <div text-2xl>
      Theme
    </div>
    <div
      flex flex-row
      p-2
      border="~ rounded gray-200 dark:gray-700"
    >
      <div flex mr-a>
        Change theme
      </div>
      <div
        icon-btn
        @click="useDarkToggle()"
      >
        <div
          v-if="!isDark"
          flex flex-row
          gap-2
        >
          <div>Day</div>
          <div m-a i-carbon-sun />
        </div>
        <div
          v-if="isDark"
          flex flex-row
          gap-2
        >
          <div>Night</div>
          <div m-a i-carbon-moon />
        </div>
      </div>
    </div>
    <div
      flex flex-row
      p-2
      border="~ rounded gray-200 dark:gray-700"
    >
      <div flex mr-a>
        Use system settings
      </div>
      <div flex>
        <input
          m-a
          type="checkbox"
          :checked="isAutoUseDark"
          @click="useSystemThemeToggle()"
        >
      </div>
    </div>
  </div>
</template>
