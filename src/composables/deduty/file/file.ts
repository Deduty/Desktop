import { invoke } from '@tauri-apps/api'

import { DedutyFileReader } from './reader'
import type { IDedutyFile, IDedutyFileCollection } from './scheme'

export class DedutyFile implements IDedutyFile {
  constructor(
    public alias: string | undefined,
    public extension: string,
    public pkg: string,
    public id: string,
  ) {}

  async createReader(): Promise<DedutyFileReader> {
    const token: string = await invoke('openFileChunked', { package: this.pkg, id: this.id })
    return new DedutyFileReader(token)
  }

  static fromOptions(pkg: string, { alias, extension, id }: IDedutyFile): DedutyFile {
    return new DedutyFile(alias, extension, pkg, id)
  }
}

export class DedutyFileCollection implements IDedutyFileCollection {
  constructor(
    public files: DedutyFile[],
  ) {}

  static fromOptions(pkg: string, { files }: IDedutyFileCollection): DedutyFileCollection {
    return new DedutyFileCollection(files.map(file => DedutyFile.fromOptions(pkg, file)))
  }
}
