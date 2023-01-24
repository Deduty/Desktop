import { invoke } from '@tauri-apps/api'
import { DedutyStorageApi } from './abstract'
import { ValueAsBoolean, ValueAsStringOrNull } from './utils'

export class DedutyPackageStorageApi extends DedutyStorageApi {
  constructor(private pkg: string) {
    super(
      (key: string) =>
        invoke('lectionStorageDelete', { package: this.pkg, key })
          .then(ValueAsBoolean),
      (key: string) =>
        invoke('packageStorageGet', { package: this.pkg, key })
          .then(ValueAsStringOrNull),
      (key: string, value: string, replace: boolean) =>
        invoke('packageStorageSet', { package: this.pkg, key, value, replace })
          .then(ValueAsBoolean),
    )
  }
}
