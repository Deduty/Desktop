<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageFormClosed'): void }>()

const updateComponentShowed = ref(false)

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})
</script>

<template>
  <div
    icon-btn
    m-2 p-2
    flex flex-row flex-grow
    justify-between
    @click="() => updateComponentShowed = !updateComponentShowed"
  >
    <p>
      Update package
    </p>
    <div
      class="update-sign"
      :class="{ 'rotate-180': updateComponentShowed }"
      text-lg
      i-carbon:chevron-down
    />
  </div>
  <div
    m-2 mt-0
    border="~ rounded gray-200 dark:gray-700"
  />
  <div
    class="update-menu"
    :class="{ showed: updateComponentShowed }"
  >
    <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
    <div v-if="errorMessage" flex-grow>
      <Error :message="errorMessage" />
    </div>
    <Suspense v-else>
      <!-- DONE - SHOW DYNAMIC COMPONENT -->
      <template #default>
        <PackageFormSettingsUpdateSuspense
          :pack="pack"
          @package-updated="() => emit('packageFormClosed')"
        />
      </template>
      <!-- LOADING - SHOW LOADING ANIMATION -->
      <template #fallback>
        <Loading />
      </template>
    </Suspense>
  </div>
</template>

<style lang="sass">
.update-sign
  transition: transform 200ms ease-in-out

.update-menu
  transition: all 600ms ease-in-out
  overflow: hidden
  max-height: 0

.update-menu.showed
  transition: all 600ms ease-in-out
  max-height: 100vh
</style>
