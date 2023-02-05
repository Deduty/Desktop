<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import { DynamicComponent } from '~/composables/dynamic'

import Loading from '~/components/Loading.vue'
import Message from '~/components/Message.vue'
import Reader from '~/components/Reader/Reader.vue'

const { pack } = defineProps<{ pack: DedutyPackage }>()

const componentInstance: Ref<DynamicComponent> = ref(new DynamicComponent(Loading))

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})

onMounted(async () => {
  const candidates = pack.files.files.filter(file => file.alias === 'about')
  if (candidates.length === 0) {
    componentInstance.value = new DynamicComponent(Message, { message: 'About file not represented' })
    return
  }

  if (candidates.length !== 1) {
    errorMessage.value = 'Package have several \'about\' alias. Expected only one for package representation'
    return
  }
  const [about] = candidates

  componentInstance.value = new DynamicComponent(Reader, { reader: await about.createReader(), extension: about.extension })
})
</script>

<template>
  <div
    h-full w-full
    flex flex-grow
    m-0
    p-2
  >
    <div v-if="errorMessage">
      <Error :message="errorMessage" />
    </div>
    <Suspense v-else>
      <!-- DONE - SHOW DYNAMIC COMPONENT -->
      <template #default>
        <component
          :is="componentInstance.comp"
          v-if="componentInstance"
          v-bind="componentInstance.prop"
          v-on="componentInstance?.even"
        />
      </template>
      <!-- LOADING - SHOW LOADING ANIMATION -->
      <template #fallback>
        <Loading flex flex-grow />
      </template>
    </Suspense>
  </div>
</template>
