import type { IDedutyFileCollection } from '../file'

export interface IDedutyLectionMeta {
  name: string
  order: number
}

export interface IDedutyLection {
  id: string
  meta: IDedutyLectionMeta
  files: IDedutyFileCollection
}
