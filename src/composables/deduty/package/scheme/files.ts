export interface IDedutyFile {
  alias?: string
  location: string
  extension: string
}

export interface IDedutyFileCollection {
  files: IDedutyFile[]
}
