import type { IDedutyFile } from '../file'

export interface IDedutyLectionMeta {
  name: string
  hidden: boolean
}

export interface IDedutyLection {
  id: string
  meta: string
  files: IDedutyFile[]
  size?: number
}
