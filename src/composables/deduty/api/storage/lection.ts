import { invoke } from '@tauri-apps/api'
import { DedutyStorageApi } from './abstract'
import { ValueAsBoolean, ValueAsStringOrNull } from './utils'

export class DedutyLectionStorageApi extends DedutyStorageApi {
  constructor(private pkg: string, private lection: string) {
    super(
      (key: string) =>
        invoke('lectionStorageDelete', { package: this.pkg, lection: this.lection, key })
          .then(ValueAsBoolean),
      (key: string) =>
        invoke('lectionStorageGet', { package: this.pkg, lection: this.lection, key })
          .then(ValueAsStringOrNull),
      (key: string, value: string, replace: boolean) =>
        invoke('lectionStorageSet', { package: this.pkg, lection: this.lection, key, value, replace })
          .then(ValueAsBoolean),
    )
  }
}
