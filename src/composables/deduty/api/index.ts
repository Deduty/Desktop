import type { DedutyWebStorageApi } from './web-storage'
import type { LectionApi } from './lection'

export interface IDedutyApi {
  webStorage: {
    lection?: DedutyWebStorageApi
    package: DedutyWebStorageApi
  }
  lections: {
    current: LectionApi
    all: LectionApi[]
  }
}
