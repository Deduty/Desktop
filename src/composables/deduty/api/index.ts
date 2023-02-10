import type { DedutyWebStorageApi } from './web-storage'

export interface IDedutyApi {
  webStorage: {
    lection?: DedutyWebStorageApi
    package: DedutyWebStorageApi
  }
}
