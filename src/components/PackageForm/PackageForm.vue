<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageFormClosed'): void }>()

const router = useRouter()

const currentFormTab: Ref<'About' | 'Settings'> = ref('About')

const packageFormClosed = () => {
  emit('packageFormClosed')
}
</script>

<template>
  <div
    h-full w-full
    border="~ rounded gray-200 dark:gray-700"
    bg-op-0
    m-0 p-4
    flex flex-col
    class="box"
    gap-4
  >
    <div flex flex-row>
      <PackageFormMeta :pack="pack" />
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
          v-show="currentFormTab !== 'Settings'"
          text-3xl
          border-rounded
          class="package form button"
          @click="currentFormTab = 'Settings'"
        >
          <div i-carbon-settings />
        </div>
        <div
          v-show="currentFormTab !== 'About'"
          text-3xl
          border-rounded
          class="package form button"
          @click="currentFormTab = 'About'"
        >
          <div i-carbon-account />
        </div>
      </div>
    </div>
    <div
      flex-grow
      overflow-auto
    >
      <PackageFormAbout
        v-show="currentFormTab === 'About'"
        :pack="pack"
      />
      <PackageFormSettings
        v-show="currentFormTab === 'Settings'"
        :pack="pack"
        @package-form-closed="packageFormClosed"
      />
    </div>
  </div>
</template>

<style scoped lang="sass">
div.box
  width: 65ch
  height: 90%

html div.box
  background-color: white

html.dark div.box
  background-color: #121212

div.package.form.button
  cursor: pointer
  transition: all 200ms ease-in-out

div.package.form.button:hover
  // background-color: darkcyan
  color: darkcyan
  transition: all 200ms ease-in-out
</style>
