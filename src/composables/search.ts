import type { DedutyPackage } from './deduty'

export class SearchCriteria {
  private regexQuery: RegExp

  constructor(query: string) {
    this.regexQuery = new RegExp(query, 'iu')
  }

  public match(pack: DedutyPackage): boolean {
    return this.regexQuery.test(pack.meta.name)
  }
}
