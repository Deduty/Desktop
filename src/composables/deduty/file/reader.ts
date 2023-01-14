import { invoke } from '@tauri-apps/api'

export class DedutyFileReader {
  private token: string

  constructor(token: string) {
    this.token = token
  }

  async readAll(): Promise<Blob | null> {
    const content: Uint8Array[] = []
    while (true) {
      const chunked = await this.readNext()
      if (chunked)
        content.push(chunked)
      else
        break
    }
    return content.length > 0 ? new Blob(content) : null
  }

  async readNext(): Promise<Uint8Array | null> {
    // Invoke chunk of file id with size 8 bit * 1024 kb * 1024 mb == 1 mb
    try {
      const chunk = await invoke('getFileChunked', { id: this.token, chunk: 8 * 1024 * 1024 })
      return Array.isArray(chunk) ? new Uint8Array(chunk) : null
    }
    catch (error) {
      return null
    }
  }
}
