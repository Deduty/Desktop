import { DedutyFileReader } from './reader'
import type { IDedutyFile } from './scheme'
import * as Commands from '~/composables/commands'
import { updateValues } from '~/composables/utils'

export interface IDedutyFileMeta {
  name: string
}

export class DedutyFile {
  constructor(
    public service: string,
    public pack: string,
    public lection: string,
    public id: string,
    public ext: string,
    public meta: IDedutyFileMeta,
    public size?: number,
  ) {}

  async createReader(): Promise<DedutyFileReader> {
    const token = await Commands.openFileChunked(this.service, this.pack, this.lection, this.id)
    return new DedutyFileReader(token)
  }

  static fromOptions(service: string, pack: string, lection: string, { id, ext, size, meta }: IDedutyFile): DedutyFile {
    const optionMeta: IDedutyFileMeta = updateValues({ name: id }, JSON.parse(meta))

    return new DedutyFile(service, pack, lection, id, ext, optionMeta, size)
  }
}
