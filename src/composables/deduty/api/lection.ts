import { DedutyWebStorageApi } from './web-storage'
import type { DedutyLection } from '~/composables/deduty'

export interface ILectionMeta {
  id: string
  name: string
}

export class LectionApi {
  #lection: DedutyLection
  #webStorage: DedutyWebStorageApi

  constructor(lectionObject: DedutyLection) {
    this.#lection = lectionObject
    this.#webStorage = new DedutyWebStorageApi(lectionObject.serviceId, lectionObject.packageId, lectionObject.id)
  }

  get meta(): ILectionMeta {
    return { id: this.#lection.id, name: this.#lection.meta.name }
  }

  public go() {
    window.location.assign(`/services/${this.#lection.serviceId}/packages/${this.#lection.packageId}/lections/${this.#lection.id}`)
  }

  get webStorage(): DedutyWebStorageApi {
    return this.#webStorage
  }
}
