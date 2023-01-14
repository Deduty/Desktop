import { invoke } from '@tauri-apps/api'

import { DedutyFileReader } from './reader'
import type { IDedutyFile, IDedutyFileCollection } from './scheme'

export class DedutyFile implements IDedutyFile {
  constructor(
    public alias: string | undefined,
    public extension: string,
    public id: string,
  ) {}

  async createReader(): Promise<DedutyFileReader> {
    const token: string = await invoke('openFileChunked', { id: this.id })
    return new DedutyFileReader(token)
  }

  static fromOptions({ alias, extension, id }: IDedutyFile): DedutyFile {
    return new DedutyFile(alias, extension, id)
  }
}

export class DedutyFileCollection implements IDedutyFileCollection {
  constructor(
    public files: DedutyFile[],
  ) {}

  static fromOptions({ files }: IDedutyFileCollection): DedutyFileCollection {
    return new DedutyFileCollection(files.map(DedutyFile.fromOptions))
  }
}
