<script setup lang="ts">
import type { Ref } from 'vue'

import PackageRequirements from '~/components/PackageRequirements/PackageRequirements.vue'
import { DynamicComponent } from '~/composables/dynamic'
import { DedutyPackage } from '~/composables/deduty'
import * as Commands from '~/composables/commands'

const emit = defineEmits<{ (event: 'packageAddSuspenseClosed'): void }>()

const { t } = useI18n()

// { Service: { SerializationKey: SerializationType } }
const serviceRequirements: Map<string, Map<string, string>> = new Map()
for (const service of await Commands.listServices()) {
  const requirements = JSON.parse(await Commands.getServiceAddRequirements(service)) as object
  serviceRequirements.set(service, new Map(Object.entries(requirements)))
}

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
    const pack = await Commands.addPackage(service.name, JSON.stringify(Object.fromEntries(serialized.entries())))
    const packageOptions = await Commands.getPackage(service.name, pack)
    await packageStore.include(DedutyPackage.fromOptions(service.name, packageOptions))
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

const currentServiceComponent: Ref<ServiceComponent | undefined> = shallowRef(serviceComponents[0])

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
    <div
      v-else-if="!currentServiceComponent"
      flex flex-grow
    >
      <Message message="Services are not installed" />
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
              v-on="currentServiceComponent?.even"
            />
            <button
              class="confirm-button"
              icon-btn
              border="~ rounded gray-200 dark:gray-700"
              ml-a m-4 mt-0
              p-2
              :disabled="currentServiceComponent.addPackageDynamicSignal.value === null"
              @click="currentServiceComponent?.addPackageDynamicSignal.value"
            >
              {{ t('component.PackageAddSuspense.Add package') }}
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
