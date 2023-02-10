import * as Commands from '~/composables/commands'

export class DedutyFileReader {
  #token: string

  constructor(token: string) {
    this.#token = token
  }

  async readAll(): Promise<Blob | undefined> {
    const content: Uint8Array[] = []
    while (true) {
      const chunked = await this.readNext()
      if (chunked)
        content.push(chunked)
      else
        break
    }
    return content.length > 0 ? new Blob(content) : undefined
  }

  async readNext(): Promise<Uint8Array | undefined> {
    return Commands.getFileChunked(this.#token)
  }
}
