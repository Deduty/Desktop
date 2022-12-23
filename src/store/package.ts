import { acceptHMRUpdate, defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api'
import type { IDedutyPackage } from '~/composables/deduty'
import { DedutyPackage } from '~/composables/deduty'

export const usePackageStore = defineStore('DedutyPackage', () => {
  const packages = reactive<DedutyPackage[]>([])

  async function include(pkg: DedutyPackage): Promise<void> {
    packages.push(pkg)
    // TODO: INCLUDE IN TAURI DATABASE
  }

  async function exclude(pkg: DedutyPackage): Promise<void> {
    const previousPackages = [...packages]

    packages.length = 0
    packages.push(...previousPackages.filter(storedPackage => storedPackage.id !== pkg.id))
    // TODO: EXCLUDE IN TAURI DATABASE
  }

  async function refresh(totally = false) {
    if (totally)
      packages.length = 0

    const updated: Set<string> = new Set(await invoke('listLocalPackage'))
    const stored: Set<string> = new Set(packages.map(pkg => pkg.id))

    for (const uuid of stored)
      updated.delete(uuid)

    for (const uuid of updated) {
      try {
        const serialized: IDedutyPackage = await invoke('getLocalPackage', { id: uuid })
        if (!serialized)
          continue

        packages.push(DedutyPackage.fromOptions(serialized))
      }
      catch (error) {
        console.error(`Internal error: Unable to fetch Package '${uuid}' due to: ${error}`)
      }
    }

    // WRONG PACKAGE FOR DEV NEEDS
    const meta = { language: 'wrong-lang', name: 'wrong-name', tags: ['wrong', 'package'], version: '1.0.0-rc1' }
    const files = { files: [{ extension: 'md', location: 'nowhere.md', alias: 'about' }] }
    packages.push(DedutyPackage.fromOptions({ id: 'wrong-id', meta, files }))
  }

  refresh()
    .catch(error => console.error(`Internal error: Unable to init Frontend Package storage due to: ${error}`))

  return { packages, include, exclude, refresh }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(usePackageStore, import.meta.hot))
