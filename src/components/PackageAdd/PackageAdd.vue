<script setup lang="ts">
const emit = defineEmits<{ (event: 'packageAddClosed'): void }>()

const { t } = useI18n()

const packageAddClosed = () => {
  emit('packageAddClosed')
}

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})
</script>

<template>
  <div
    h-full w-full
    border="~ rounded gray-200 dark:gray-700"
    m-0 p-4
    flex flex-col
    class="box"
    gap-4
    w-prose h-min
  >
    <div text-2xl>
      {{ t('component.PackageAdd.Add package') }}
    </div>
    <div
      h-full w-full
      m-a
      flex flex-grow
    >
      <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
      <div v-if="errorMessage" flex-grow>
        <Error :message="errorMessage" />
      </div>
      <Suspense v-else>
        <!-- DONE - SHOW DYNAMIC COMPONENT -->
        <template #default>
          <PackageAddSuspense @package-add-suspense-closed="packageAddClosed" />
        </template>
        <!-- LOADING - SHOW LOADING ANIMATION -->
        <template #fallback>
          <Loading />
        </template>
      </Suspense>
    </div>
  </div>
</template>

<style scoped lang="sass">
html div.box
  background-color: white

html.dark div.box
  background-color: #121212
</style>
