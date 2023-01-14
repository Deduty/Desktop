export interface IDedutyFile {
  alias?: string
  extension: string
  id: string
}

export interface IDedutyFileCollection {
  files: IDedutyFile[]
}
