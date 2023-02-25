import type { DedutyLection, DedutyPackage } from './deduty'

export class PackageSearchCriteria {
  public regexQuery: RegExp

  constructor(query: string) {
    this.regexQuery = new RegExp(query, 'iu')
  }

  public match(packageObject: DedutyPackage): boolean {
    return this.regexQuery.test(packageObject.meta.name)
  }
}

export class LectionSearchCriteria {
  public regexQuery: RegExp

  constructor(query: string) {
    this.regexQuery = new RegExp(query, 'iu')
  }

  public match(lection: DedutyLection): boolean {
    return this.regexQuery.test(lection.meta.name)
  }
}
