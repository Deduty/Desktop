import * as Commands from '~/composables/commands'

export class DedutyWebStorageApi {
  #service: string
  #package: string
  #lection?: string

  constructor(service: string, pack: string, lection?: string) {
    this.#service = service
    this.#package = pack
    this.#lection = lection
  }

  public async delete(key: string): Promise<string | undefined> {
    return Commands.webStorageDelete(this.#service, this.#package, this.#lection, key)
  }

  public async get(key: string, fallback?: string): Promise<string | undefined > {
    return Commands.webStorageGet(this.#service, this.#package, this.#lection, key, fallback)
  }

  public async set(key: string, value: string, replace?: boolean): Promise<string | undefined> {
    return Commands.webStorageSet(this.#service, this.#package, this.#lection, key, value, replace || false)
  }
}
