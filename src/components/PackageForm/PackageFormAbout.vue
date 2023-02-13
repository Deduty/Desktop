<script setup lang="ts">
import type { Ref } from 'vue'

import type { DedutyPackage } from '~/composables/deduty'
import { DynamicComponent } from '~/composables/dynamic'

import Loading from '~/components/Loading.vue'
import Message from '~/components/Message.vue'
import LectionReader from '~/components/LectionReader.vue'

const { pack } = defineProps<{ pack: DedutyPackage }>()

const componentInstance: Ref<DynamicComponent> = shallowRef(
  new DynamicComponent(Message, { message: 'About file not represented' }))

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})

const aboutLection = pack.lections.find(lection => lection.id === 'about')
if (aboutLection) {
  componentInstance.value
    = new DynamicComponent(LectionReader, { service: pack.service, pack: pack.id, lection: aboutLection, apiEnabled: true })
}
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
