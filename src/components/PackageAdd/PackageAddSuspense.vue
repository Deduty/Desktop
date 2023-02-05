<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { Ref } from 'vue'

import PackageRequirements from '~/components/PackageRequirements/PackageRequirements.vue'
import type { IDedutyPackage } from '~/composables/deduty'
import { DedutyPackage } from '~/composables/deduty'
import { DynamicComponent } from '~/composables/dynamic'

const emit = defineEmits<{ (event: 'packageAddSuspenseClosed'): void }>()

// { Service: { SerializationKey: SerializationType } }
const serviceRequirements: Map<string, Map<string, string>> = new Map(
  Object
    .entries(await invoke('listServiceAddRequirements') as object)
    .map(([key, value]) => [key, new Map(Object.entries(value))]))

const packageStore = usePackageStore()

class ServiceComponent extends DynamicComponent {
  constructor(
    public name: string,
    comp: any,
    prop: object = {},
    even: object = {},
    public addPackageDynamicSignal: Ref<(() => Promise<void>) | null> = ref(null),
  ) {
    super(comp, prop, even)
  }
}

const requirementSatisfied = (service: ServiceComponent, serialized: Map<string, string>) => {
  service.addPackageDynamicSignal.value = async () => {
    const package_id: string = await invoke('addPackage', { service: service.name, serialized: Object.fromEntries(serialized) })
    const options = (await invoke('getPackage', { id: package_id }) as IDedutyPackage)
    await packageStore.include(DedutyPackage.fromOptions(options))
    emit('packageAddSuspenseClosed')
  }
}

const requirementNotSatisfied = (service: ServiceComponent) => {
  service.addPackageDynamicSignal.value = null
}

const serviceComponents: ServiceComponent[]
  = [...serviceRequirements.entries()]
    .map(([key, requirements]) => {
      const component = new ServiceComponent(key, PackageRequirements, { requirements })
      component.even = {
        requirementSatisfied: (serialized: Map<string, string>) => requirementSatisfied(component, serialized),
        requirementNotSatisfied: () => requirementNotSatisfied(component),
      }
      return component
    })

const currentServiceComponent: Ref<ServiceComponent> = shallowRef(serviceComponents[0])

const stringCapitalize = (value: string) => {
  return `${value[0].toUpperCase()}${value.slice(1)}`
}

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
            w-full
            flex flex-row flex-grow
            overflow-y-auto
            scroll-smooth
            style="scrollbar-width: none;"
          >
            <div
              v-for="(component, index) in serviceComponents"
              :key="index"
            >
              <button
                :class="{ selected: component.name === currentServiceComponent.name }"
                icon-btn
                class="tab-button"
                border="~ rounded gray-200 dark:gray-700 b-none rounded-b-0"
                text-xl
                p-2
                @click="currentServiceComponent = component"
              >
                {{ stringCapitalize(component.name) }}
              </button>
            </div>
          </div>
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
              Add package
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
