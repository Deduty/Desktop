<script setup lang="ts">
const properties = defineProps<{ service: string; package: string; lection: string }>()

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})
</script>

<template>
  <div h-full w-full flex flex-col>
    <div v-if="errorMessage">
      <Error :message="errorMessage" />
    </div>
    <Suspense v-else>
      <!-- DONE - SHOW DYNAMIC COMPONENT -->
      <template #default>
        <LectionReader
          :target="properties"
          :api-enabled="true"
          :lection-changing-allowed="true"
        />
      </template>
      <!-- LOADING - SHOW LOADING ANIMATION -->
      <template #fallback>
        <Loading flex flex-grow />
      </template>
    </Suspense>
  </div>
</template>
