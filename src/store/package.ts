import { invoke } from '@tauri-apps/api'
import { acceptHMRUpdate, defineStore } from 'pinia'
import type { Ref } from 'vue'

import type { IDedutyPackage } from '~/composables/deduty'
import { DedutyPackage } from '~/composables/deduty'

export const usePackageStore = defineStore('DedutyPackage', () => {
  const packages: Ref<DedutyPackage[]> = ref([])

  async function include(pkg: DedutyPackage): Promise<void> {
    packages.value.push(pkg)
  }

  async function exclude(pkg: DedutyPackage): Promise<void> {
    packages.value = [...packages.value.filter(storedPackage => storedPackage.id !== pkg.id)]
  }

  async function refresh(totally = false) {
    if (totally)
      packages.value = []

    const updated: Set<string> = new Set(await invoke('listPackages'))
    packages.value
      .map(pack => pack.id)
      .forEach(updated.delete)

    for (const uuid of updated) {
      try {
        const serialized: IDedutyPackage = await invoke('getPackage', { id: uuid })
        if (!serialized)
          continue

        packages.value.push(DedutyPackage.fromOptions(serialized))
      }
      catch (error) {
        console.error(`Internal error: Unable to fetch Package '${uuid}' due to: ${error}`)
      }
    }
  }

  refresh()
    .catch(error => console.error(`Internal error: Unable to init Frontend Package storage due to: ${error}`))

  return { packages, include, exclude, refresh }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(usePackageStore, import.meta.hot))
