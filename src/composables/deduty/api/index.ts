import { LectionApi } from './lection'
import { PackageApi } from './package'
import type { DedutyLection, DedutyPackage } from '~/composables/deduty'

export interface IDedutyApi {
  package: PackageApi
  lections: LectionApi[]
  lection: LectionApi
}

export class DedutyApi implements IDedutyApi {
  package: PackageApi
  lections: LectionApi[]
  lection: LectionApi

  constructor(packageObject: DedutyPackage, lectionObject: DedutyLection) {
    this.package = new PackageApi(packageObject)
    this.lections = packageObject.lections.map(lectionObject => new LectionApi(lectionObject))

    const currentLection = packageObject.lections.find(lection => lection.id === lectionObject.id)
    if (!currentLection)
      throw new Error(`Internal error: Unable to build DedutyApi: Lection with id \`${lectionObject.id}\` not found`)

    this.lection = new LectionApi(currentLection)
  }
}
