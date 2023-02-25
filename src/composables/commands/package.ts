import { invoke } from '@tauri-apps/api'
import type { IDedutyPackage } from '../deduty'

export const addPackage = (serviceId: string, serialized: string): Promise<string> => {
  return invoke('addPackage', { service: serviceId, serialized })
    .then((value) => {
      if (typeof value !== 'string')
        throw new TypeError(`Internal error: \`addPackage\` has returned \`${value}\` but \`string\` was expected`)

      return value
    })
}

export const getPackage = (serviceId: string, packageId: string): Promise<IDedutyPackage> => {
  return invoke('getPackage', { service: serviceId, package: packageId })
    .then((value) => {
      console.warn('TODO: Make dev only full check')

      return value as IDedutyPackage
    })
}

export const listPackages = (serviceId: string): Promise<string[]> => {
  return invoke('listPackages', { service: serviceId })
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

export const subPackage = (serviceId: string, packageId: string): Promise<void> => {
  return invoke('subPackage', { service: serviceId, package: packageId })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`subPackage\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}

export const updatePackage = (serviceId: string, packageId: string, serialized: string): Promise<void> => {
  return invoke('updatePackage', { service: serviceId, package: packageId, serialized })
    .then((value) => {
      if (value)
        throw new TypeError(`Internal error: \`subPackage\` has returned \`${value}\` but \`undefined | null\` was expected`)
    })
}
