import { DedutyFile } from '../file'
import type { IDedutyLection, IDedutyLectionMeta } from './scheme'
import { updateValues } from '~/composables/utils'

export class DedutyLection {
  constructor(
    public id: string,
    public meta: IDedutyLectionMeta,
    public files: DedutyFile[],
    public size?: number,
  ) {}

  static fromOptions(service: string, pack: string, { id, meta, files, size }: IDedutyLection): DedutyLection {
    const optionMeta: IDedutyLectionMeta = updateValues({ name: id, hidden: false }, JSON.parse(meta))
    const objectFiles = files.map(file => DedutyFile.fromOptions(service, pack, id, file))

    return new DedutyLection(id, optionMeta, objectFiles, size)
  }
}
