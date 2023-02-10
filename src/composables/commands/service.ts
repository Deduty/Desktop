import { invoke } from '@tauri-apps/api'

export const getServiceAddRequirements = (service: string): Promise<string> => {
  return invoke('getServiceAddRequirements', { service })
    .then((value) => {
      if (typeof value !== 'string')
        throw new TypeError(`Internal error: \`getServiceAddRequirements\` has returned \`${value}\` but \`string\` was expected`)

      return value
    })
}

export const getServiceUpdateRequirements = (service: string): Promise<string> => {
  return invoke('getServiceUpdateRequirements', { service })
    .then((value) => {
      if (typeof value !== 'string')
        throw new TypeError(`Internal error: \`getServiceUpdateRequirements\` has returned \`${value}\` but \`string\` was expected`)

      return value
    })
}

export const listServices = (): Promise<string[]> => {
  return invoke('listServices')
    .then((value) => {
      if (!Array.isArray(value))
        throw new TypeError(`Internal error: \`listServices\` has returned \`${value}\` but \`string[]\` was expected`)

      // Heavy per item check
      if (import.meta.env.DEV) {
        for (const item of value) {
          if (typeof item !== 'string') {
            console.error('Bug caught: Not all values are strings in array', value)
            throw new TypeError(`Internal error: \`listServices\` has returned \`${value}\` but \`string[]\` was expected`)
          }
        }
      }

      return value as string[]
    })
}
