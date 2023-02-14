import { invoke } from '@tauri-apps/api'

export const webStorageExport = (service: string, pack: string, path: string): Promise<void> => {
  return invoke('webStorageExport', { service, package: pack, path })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const webStorageImport = (service: string, pack: string, path: string): Promise<void> => {
  return invoke('webStorageImport', { service, package: pack, path })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const webStorageClear = (service: string, pack: string): Promise<void> => {
  return invoke('webStorageClear', { service, package: pack })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const webStorageDelete = (service: string, pack: string, lection: string | undefined, key: string): Promise<string | undefined> => {
  return invoke('webStorageDelete', { service, package: pack, lection, key })
    .then((value) => {
      if (!value)
        return undefined

      if (typeof value === 'string')
        return value

      throw new TypeError(`Internal error: \`webStorageDelete\` has returned \`${value}\` but \`string\` was expected`)
    })
}

export const webStorageGet = (service: string, pack: string, lection: string | undefined, key: string, fallback: string | undefined): Promise<string | undefined> => {
  return invoke('webStorageGet', { service, package: pack, lection, key, fallback })
    .then((value) => {
      if (!value)
        return undefined

      if (typeof value === 'string')
        return value

      throw new TypeError(`Internal error: \`webStorageGet\` has returned \`${value}\` but \`string\` was expected`)
    })
}

export const webStorageSet = (service: string, pack: string, lection: string | undefined, key: string, value: string, replaced: boolean): Promise<string | undefined> => {
  return invoke('webStorageSet', { service, package: pack, lection, key, value, replaced })
    .then((value) => {
      if (!value)
        return undefined

      if (typeof value === 'string')
        return value

      throw new TypeError(`Internal error: \`webStorageSet\` has returned \`${value}\` but \`string\` was expected`)
    })
}
