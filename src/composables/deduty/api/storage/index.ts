import type { DedutyLectionStorageApi } from './lection'
import type { DedutyPackageStorageApi } from './package'

export { DedutyStorageApi } from './abstract'
export { DedutyLectionStorageApi } from './lection'
export { DedutyPackageStorageApi } from './package'

export interface IDedutyStorageApi {
  lection?: DedutyLectionStorageApi
  package: DedutyPackageStorageApi
}
