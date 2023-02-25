import { invoke } from '@tauri-apps/api'

export const webStorageExport = (serviceId: string, packageId: string, path: string): Promise<void> => {
  return invoke('webStorageExport', { service: serviceId, package: packageId, path })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const webStorageImport = (serviceId: string, packageId: string, path: string): Promise<void> => {
  return invoke('webStorageImport', { service: serviceId, package: packageId, path })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const webStorageClear = (serviceId: string, packageId: string): Promise<void> => {
  return invoke('webStorageClear', { service: serviceId, package: packageId })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const webStorageDelete = (serviceId: string, packageId: string, lectionId: string | undefined, key: string): Promise<string | undefined> => {
  return invoke('webStorageDelete', { service: serviceId, package: packageId, lection: lectionId, key })
    .then((value) => {
      if (typeof value === 'string')
        return value

      if (!value)
        return undefined

      throw new TypeError(`Internal error: \`webStorageDelete\` has returned \`${value}\` but \`string\` was expected`)
    })
}

export const webStorageGet = (serviceId: string, packageId: string, lectionId: string | undefined, key: string, fallback: string | undefined): Promise<string | undefined> => {
  return invoke('webStorageGet', { service: serviceId, package: packageId, lection: lectionId, key, fallback })
    .then((value) => {
      if (typeof value === 'string')
        return value

      if (!value)
        return undefined

      throw new TypeError(`Internal error: \`webStorageGet\` has returned \`${value}\` but \`string\` was expected`)
    })
}

export const webStorageSet = (serviceId: string, packageId: string, lectionId: string | undefined, key: string, value: string, replaced: boolean): Promise<string | undefined> => {
  return invoke('webStorageSet', { service: serviceId, package: packageId, lection: lectionId, key, value, replaced })
    .then((value) => {
      if (typeof value === 'string')
        return value

      if (!value)
        return undefined

      throw new TypeError(`Internal error: \`webStorageSet\` has returned \`${value}\` but \`string\` was expected`)
    })
}
