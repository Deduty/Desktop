import { invoke } from '@tauri-apps/api'

export const closeFileChunked = (token: string): Promise<void> => {
  return invoke('closeFileChunked', { token })
    .then((value) => {
      if (typeof value !== undefined)
        throw new TypeError(`Internal error: \`closeFileChunked\` has returned \`${value}\` but \`undefined\` was expected`)
    })
}

export const openFileChunked = (service: string, pack: string, lection: string, file: string): Promise<string> => {
  return invoke('openFileChunked', { service, package: pack, lection, file })
    // Ensure its string
    .then((value) => {
      if (typeof value !== 'string')
        throw new TypeError(`Internal error: \`openFileChunked\` has returned \`${value}\` but \`string\` was expected`)

      return value
    })
}

export const getFileChunked = (token: string, chunk = 8 * 1024 * 1024): Promise<Uint8Array | undefined> => {
  if (chunk <= 0 || !Number.isInteger(chunk))
    throw new Error(`Internal error: Invalid value \`chunk = ${chunk}\`. \`chunk\` must be positive integer.`)

  return invoke('getFileChunked', { token, chunk })
    .then((value) => {
      if (!value)
        return undefined

      if (!Array.isArray(value))
        throw new TypeError(`Internal error: \`getFileChunked\` has returned \`${value}\` but \`number[]\` was expected`)

      // Heavy per item check
      if (import.meta.env.DEV) {
        for (const item of value) {
          if (typeof item !== 'number') {
            console.error('Bug caught: Not all values are strings in array', value)
            throw new TypeError(`Internal error: \`getFileChunked\` has returned \`${value}\` but \`number[]\` was expected`)
          }
        }
      }

      console.warn('chunked.ts', typeof value, typeof new Uint8Array(chunk))
      return new Uint8Array(value)
    })
}
