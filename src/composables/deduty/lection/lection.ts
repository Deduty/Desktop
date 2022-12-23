import { DedutyFileCollection } from '../file'
import type { IDedutyLection, IDedutyLectionMeta } from './scheme'

export class DedutyLectionMeta implements IDedutyLectionMeta {
  constructor(
    public name: string,
    public order: number,
  ) {}

  static fromOptions({ name, order }: IDedutyLectionMeta): DedutyLectionMeta {
    return new DedutyLectionMeta(name, order)
  }
}

export class DedutyLection implements IDedutyLection {
  constructor(
    public id: string,
    public meta: DedutyLectionMeta,
    public files: DedutyFileCollection,
  ) {}

  static fromOptions({ id, meta, files }: IDedutyLection): DedutyLection {
    return new DedutyLection(id, DedutyLectionMeta.fromOptions(meta), DedutyFileCollection.fromOptions(files))
  }
}
