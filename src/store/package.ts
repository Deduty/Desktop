import { acceptHMRUpdate, defineStore } from 'pinia'
import { Package, PackageSize, PackageSource } from '~/composables/deduty'

export const usePackageStore = defineStore('DedutyPackage', () => {
  const packages = reactive<Package[]>([])

  function include(pkg: Package): void {
    packages.push(pkg)
    // TODO: INCLUDE IN TAURI DATABASE
  }

  function exclude(pkg: Package): void {
    const previousPackages = [...packages]

    packages.length = 0
    packages.push(
      ...previousPackages.filter(storedPackage =>
        storedPackage.name === pkg.name
        && storedPackage.version === pkg.version
        && storedPackage.source === pkg.source),
    )
    // TODO: EXCLUDE IN TAURI DATABASE
  }

  function init() {
    if (packages.length === 0)
      refresh()
  }

  function refresh() {
    // TODO: REFRESH DATA, GET TAURI DATABASE
    for (let i = 0; i < 4; i += 1) {
      packages.push(
        Package.fromOptions({
          name: `Template ${i}`,
          version: `1.${i}.${i * 2}`,
          source: [PackageSource.Git, PackageSource.Local, PackageSource.Web][i % 3],
          size: new PackageSize(1024 * (i + 1)),
          language: ['English', 'Russian'][i % 2],
        }),
      )
    }
  }

  return { packages, include, exclude, init, refresh }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(usePackageStore, import.meta.hot))
