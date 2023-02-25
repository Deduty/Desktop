import { DedutyWebStorageApi } from './web-storage'
import type { DedutyPackage } from '~/composables/deduty'

export interface IPackageMeta {
  id: string
  name: string
}

export class PackageApi {
  #package: DedutyPackage
  #webStorage: DedutyWebStorageApi

  constructor(packageObject: DedutyPackage) {
    this.#package = packageObject
    this.#webStorage = new DedutyWebStorageApi(packageObject.serviceId, packageObject.id)
  }

  get meta(): IPackageMeta {
    return { id: this.#package.id, name: this.#package.meta.name }
  }

  get webStorage(): DedutyWebStorageApi {
    return this.#webStorage
  }
}
