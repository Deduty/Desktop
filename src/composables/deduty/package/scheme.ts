import type { IDedutyFileCollection } from '../file'

export interface IDedutyPackageMeta {
  name: string
  version: string
  language: string
  tags: string[]
}

export interface IDedutyPackage {
  id: string
  size?: number
  service: string
  meta: IDedutyPackageMeta
  files: IDedutyFileCollection
}
