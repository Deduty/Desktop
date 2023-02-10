import type { IDedutyLection } from '../lection'

export interface IDedutyPackageMeta {
  name: string
  version: string
  language: string
  tags: string[]
}

export interface IDedutyPackage {
  id: string
  meta: string
  lections: IDedutyLection[]
  size?: number
}
