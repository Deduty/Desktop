import type { DedutyLection } from '~/composables/deduty'

export class LectionRouter {
  #service: string
  #package: string
  #lection: string

  constructor(service: string, pack: string, lection: string) {
    this.#service = service
    this.#package = pack
    this.#lection = lection
  }

  public go() {
    window.location.assign(`/services/${this.#service}/packages/${this.#package}/lections/${this.#lection}`)
  }
}

export class LectionApi {
  #lectionRouter?: LectionRouter
  #lectionObject: DedutyLection

  constructor(lectionObject: DedutyLection, lectonRouter?: LectionRouter) {
    this.#lectionObject = lectionObject
    this.#lectionRouter = lectonRouter
  }

  get id(): string {
    return this.#lectionObject.id
  }

  get name(): string {
    return this.#lectionObject.meta.name
  }

  get router(): LectionRouter | undefined {
    return this.#lectionRouter
  }
}
