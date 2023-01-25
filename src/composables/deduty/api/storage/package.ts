import { invoke } from '@tauri-apps/api'
import { DedutyStorageApi } from './abstract'
import { ValueAsBoolean, ValueAsStringOrNull } from './utils'

export class DedutyPackageStorageApi extends DedutyStorageApi {
  #package: string

  constructor(pkg: string) {
    super(
      (key: string) =>
        invoke('packageStorageDelete', { package: this.#package, key })
          .then(ValueAsBoolean),
      (key: string) =>
        invoke('packageStorageGet', { package: this.#package, key })
          .then(ValueAsStringOrNull),
      (key: string, value: string, replace: boolean) =>
        invoke('packageStorageSet', { package: this.#package, key, value, replace })
          .then(ValueAsBoolean),
    )

    this.#package = pkg
  }
}
