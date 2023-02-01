import { acceptHMRUpdate, defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api'
import type { IDedutyPackage } from '~/composables/deduty'
import { DedutyPackage } from '~/composables/deduty'

export const usePackageStore = defineStore('DedutyPackage', () => {
  const packages = reactive<DedutyPackage[]>([])

  async function include(pkg: DedutyPackage): Promise<void> {
    packages.push(pkg)
  }

  async function exclude(pkg: DedutyPackage): Promise<void> {
    const previousPackages = [...packages]

    packages.length = 0
    packages.push(...previousPackages.filter(storedPackage => storedPackage.id !== pkg.id))
  }

  async function refresh(totally = false) {
    if (totally)
      packages.length = 0

    const updated: Set<string> = new Set(await invoke('listPackages'))
    const stored: Set<string> = new Set(packages.map(pkg => pkg.id))

    for (const uuid of stored)
      updated.delete(uuid)

    for (const uuid of updated) {
      try {
        const serialized: IDedutyPackage = await invoke('getPackage', { id: uuid })
        if (!serialized)
          continue

        packages.push(DedutyPackage.fromOptions(serialized))
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
