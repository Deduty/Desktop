import { acceptHMRUpdate, defineStore } from 'pinia'
import type { Ref } from 'vue'

import type { IDedutyPackage } from '~/composables/deduty'
import { DedutyPackage } from '~/composables/deduty'
import * as Commands from '~/composables/commands'

export const usePackageStore = defineStore('DedutyPackage', () => {
  const storedPackages: Ref<DedutyPackage[]> = ref([])
  const indexedPackages: Ref<Map<string, Map<string, DedutyPackage>>> = ref(new Map())

  async function include(packageObject: DedutyPackage): Promise<void> {
    storedPackages.value.push(packageObject)

    if (!indexedPackages.value.get(packageObject.serviceId))
      indexedPackages.value.set(packageObject.serviceId, new Map())

    indexedPackages.value.get(packageObject.serviceId)!.set(packageObject.id, packageObject)
  }

  async function exclude(packageObject: DedutyPackage): Promise<void> {
    if (!indexedPackages.value.get(packageObject.serviceId) || indexedPackages.value.get(packageObject.serviceId)!.get(packageObject.id) === undefined) {
      console.warn('Package is not contained by frontend', packageObject)
      return
    }

    // Must be contained according to check above
    indexedPackages.value.get(packageObject.serviceId)!.delete(packageObject.id)

    const newStoredPackages: DedutyPackage[] = []
    for (const packages of indexedPackages.value.values())
      newStoredPackages.push(...packages.values())

    storedPackages.value = newStoredPackages
  }

  async function refresh(totally = false, services: string[] = []) {
    if (totally && services.length !== 0)
      throw new Error('Refresh cannot be applied on total store but for some services. This can leads to miss some packages')

    if (totally) {
      storedPackages.value = []
      indexedPackages.value.clear()
    }

    if (services.length === 0)
      services.push(...await Commands.listServices())

    const updatedServices: Map<string, Set<string>> = new Map()
    for (const service of services) {
      try {
        updatedServices.set(service, new Set(await Commands.listPackages(service)))
      }
      catch (error) {
        console.error(`Unable to receive packages ids from service\`${service}\`\n`, error)
      }
    }

    for (const service of updatedServices.keys())
      indexedPackages.value.set(service, new Map())

    for (const [service, packages] of updatedServices) {
      for (const pack of packages) {
        const packageOptions: IDedutyPackage = await Commands.getPackage(service, pack)
        const objectPackage = DedutyPackage.fromOptions(service, packageOptions)

        indexedPackages.value.get(service)!.set(pack, objectPackage)
        storedPackages.value.push(objectPackage)
      }
    }
  }

  const initialRefreshPromise = refresh()
    .catch(error => console.error(`Internal error: Unable to init Frontend Package storage due to: ${error}`))

  return { storedPackages, indexedPackages, include, exclude, refresh, initialRefreshPromise }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(usePackageStore, import.meta.hot))
