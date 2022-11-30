import { acceptHMRUpdate, defineStore } from 'pinia'
import { Package } from '~/composables/deduty'

export const usePackageStore = defineStore('DedutyPackage', () => {
  const packages = reactive<Package[]>([])

  function include(pkg: Package): void {
    packages.push(pkg)
    // TODO: INCLUDE IN TAURI DATABASE
  }

  function exclude(pkg: Package): void {
    const previousPackages = [...packages]

    packages.length = 0
    packages.push(...previousPackages.filter(storedPackage => storedPackage.id !== pkg.id))
    // TODO: EXCLUDE IN TAURI DATABASE
  }

  function init() {
    if (packages.length === 0)
      refresh()
  }

  function refresh() {
    // TODO: REFRESH DATA, GET TAURI DATABASE
    const meta = { language: 'wrong-lang', name: 'wrong-name', tags: ['wrong', 'package'], version: '1.0.0-rc1' }
    const files = { files: [{ extension: 'md', location: 'nowhere.md', alias: 'about' }] }
    packages.push(Package.fromOptions({ id: 'wrong-id', meta, files }))
  }

  return { packages, include, exclude, init, refresh }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(usePackageStore, import.meta.hot))
