import { invoke } from '@tauri-apps/api'
import type { IDedutyPackage } from '../deduty'

export const addPackage = (service: string, serialized: string): Promise<string> => {
  return invoke('addPackage', { service, serialized })
    .then((value) => {
      if (typeof value !== 'string')
        throw new TypeError(`Internal error: \`addPackage\` has returned \`${value}\` but \`string\` was expected`)

      return value
    })
}

export const getPackage = (service: string, pack: string): Promise<IDedutyPackage> => {
  return invoke('getPackage', { service, package: pack })
    .then((value) => {
      console.warn('TODO: Make dev only full check')

      return value as IDedutyPackage
    })
}

export const listPackages = (service: string): Promise<string[]> => {
  return invoke('listPackages', { service })
    .then((value) => {
      if (!Array.isArray(value))
        throw new TypeError(`Internal error: \`listPackages\` has returned \`${value}\` but \`string[]\` was expected`)

      // Heavy per item check
      if (import.meta.env.DEV) {
        for (const item of value) {
          if (typeof item !== 'string') {
            console.error('Bug caught: Not all values are strings in array', value)
            throw new TypeError(`Internal error: \`listPackages\` has returned \`${value}\` but \`string[]\` was expected`)
          }
        }
      }

      return value as string[]
    })
}

export const subPackage = (service: string, pack: string): Promise<void> => {
  return invoke('subPackage', { service, package: pack })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`subPackage\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const updatePackage = (service: string, pack: string, serialized: string): Promise<void> => {
  return invoke('updatePackage', { service, package: pack, serialized })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`subPackage\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}
