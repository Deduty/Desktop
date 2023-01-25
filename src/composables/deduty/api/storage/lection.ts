import { invoke } from '@tauri-apps/api'
import { DedutyStorageApi } from './abstract'
import { ValueAsBoolean, ValueAsStringOrNull } from './utils'

export class DedutyLectionStorageApi extends DedutyStorageApi {
  #package: string
  #lection: string

  constructor(pkg: string, lec: string) {
    super(
      (key: string) =>
        invoke('lectionStorageDelete', { package: this.#package, lection: this.#lection, key })
          .then(ValueAsBoolean),
      (key: string) =>
        invoke('lectionStorageGet', { package: this.#package, lection: this.#lection, key })
          .then(ValueAsStringOrNull),
      (key: string, value: string, replace: boolean) =>
        invoke('lectionStorageSet', { package: this.#package, lection: this.#lection, key, value, replace })
          .then(ValueAsBoolean),
    )

    this.#package = pkg
    this.#lection = lec
  }
}
