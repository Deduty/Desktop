<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Ref } from 'vue'

import PackageRequirements from '~/components/PackageRequirements/PackageRequirements.vue'
import type { DedutyPackage } from '~/composables/deduty'

const { pack } = defineProps<{ pack: DedutyPackage }>()
const emit = defineEmits<{ (event: 'packageUpdated'): void }>()

// { Service: { SerializationKey: SerializationType } }
const packageService = await invoke('getService', { package: pack.id }) as string
const allRequirements = await invoke('listServiceUpdateRequirements') as any
const serviceRequirements: Map<string, string> = new Map(Object.entries(allRequirements[packageService]))

const packageStore = usePackageStore()

class ServiceComponent {
  constructor(
    public name: string,
    public comp: any,
    public prop: object = {},
    public even: object = {},
    public addPackageDynamicSignal: Ref<(() => Promise<void>) | null> = ref(null),
  ) {}
}

const requirementSatisfied = (service: ServiceComponent, serialized: Map<string, string>) => {
  service.addPackageDynamicSignal.value = async () => {
    await invoke('updatePackage', { service: service.name, id: pack.id, serialized: Object.fromEntries(serialized) })
    await packageStore.exclude(pack)
    await packageStore.refresh()
    emit('packageUpdated')
  }
}

const requirementNotSatisfied = (service: ServiceComponent) => {
  service.addPackageDynamicSignal.value = null
}

const currentServiceComponent: Ref<ServiceComponent> = shallowRef(
  new ServiceComponent(
    packageService,
    PackageRequirements,
    {
      requirements: serviceRequirements,
    },
    {
      requirementSatisfied: (serialized: Map<string, string>) => requirementSatisfied(currentServiceComponent.value, serialized),
      requirementNotSatisfied: () => requirementNotSatisfied(currentServiceComponent.value),
    },
  ))

const errorMessage = ref('')
onErrorCaptured((error) => {
  errorMessage.value = error.message
})
</script>

<template>
  <div
    h-full w-full
    flex flex-grow
  >
    <!-- ERROR - SHOW ERROR WHEN CHILD COMPONENT ERROR MESSAGE CAUGHT -->
    <div v-if="errorMessage" flex-grow>
      <Error :message="errorMessage" />
    </div>
    <Suspense v-else>
      <!-- DONE - SHOW DYNAMIC COMPONENT -->
      <template #default>
        <div
          h-full w-full
          flex flex-col flex-grow
        >
          <div
            h-full w-full
            flex flex-col flex-grow
            border="~ rounded gray-200 dark:gray-700"
          >
            <component
              :is="currentServiceComponent.comp"
              v-bind="currentServiceComponent.prop"
              v-on="currentServiceComponent.even"
            />
            <button
              class="confirm-button"
              icon-btn
              border="~ rounded gray-200 dark:gray-700"
              ml-a m-4 mt-0
              p-2
              :disabled="currentServiceComponent.addPackageDynamicSignal.value === null"
              @click="currentServiceComponent.addPackageDynamicSignal.value"
            >
              Update package
            </button>
          </div>
        </div>
      </template>
      <!-- LOADING - SHOW LOADING ANIMATION -->
      <template #fallback>
        <Loading />
      </template>
    </Suspense>
  </div>
</template>

<style scoped lang="sass">
html.dark .confirm-button[disabled]
  color: rgb(55, 65, 81)

html.dark .confirm-button[disabled]:hover
  color: rgb(55, 65, 81)
  opacity: 0.75

.confirm-button[disabled]
  color: rgb(229, 231, 235)

.confirm-button[disabled]:hover
  color: rgb(229, 231, 235)
  opacity: 0.75

.tab-button
  transition: all 200ms ease-in-out

.tab-button.selected
  color: cadetblue
</style>
