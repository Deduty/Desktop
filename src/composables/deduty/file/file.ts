import { DedutyFileReader } from './reader'
import type { IDedutyFile } from './scheme'
import * as Commands from '~/composables/commands'
import { updateValues } from '~/composables/utils'

export interface IDedutyFileMeta {
  name: string
}

export class DedutyFile {
  constructor(
    public serviceId: string,
    public packageId: string,
    public lectionId: string,
    public id: string,
    public ext: string,
    public meta: IDedutyFileMeta,
    public size?: number,
  ) {}

  async createReader(): Promise<DedutyFileReader> {
    const token = await Commands.openFileChunked(this.serviceId, this.packageId, this.lectionId, this.id)
    return new DedutyFileReader(token)
  }

  static fromOptions(serviceId: string, packageId: string, lectionId: string, { id, ext, size, meta }: IDedutyFile): DedutyFile {
    const optionMeta: IDedutyFileMeta = updateValues({ name: id }, JSON.parse(meta))

    return new DedutyFile(serviceId, packageId, lectionId, id, ext, optionMeta, size)
  }
}
