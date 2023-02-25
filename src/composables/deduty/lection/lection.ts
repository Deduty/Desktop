import { DedutyFile } from '../file'
import type { IDedutyLection, IDedutyLectionMeta } from './scheme'
import { updateValues } from '~/composables/utils'

export class DedutyLection {
  constructor(
    public serviceId: string,
    public packageId: string,
    public id: string,
    public meta: IDedutyLectionMeta,
    public files: DedutyFile[],
    public size?: number,
  ) {}

  static fromOptions(serviceId: string, packageId: string, { id, meta, files, size }: IDedutyLection): DedutyLection {
    const optionMeta: IDedutyLectionMeta = updateValues({ name: id, hidden: false }, JSON.parse(meta))
    const objectFiles = files.map(file => DedutyFile.fromOptions(serviceId, packageId, id, file))

    return new DedutyLection(serviceId, packageId, id, optionMeta, objectFiles, size)
  }
}
