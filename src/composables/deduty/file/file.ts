import type { IDedutyFile, IDedutyFileCollection } from './scheme'

export class DedutyFile implements IDedutyFile {
  constructor(
    public location: string,
    public extension: string,
    public alias?: string,
  ) {}

  static fromOptions({ alias, location, extension }: IDedutyFile): DedutyFile {
    return new DedutyFile(location, extension, alias)
  }
}

export class DedutyFileCollection implements IDedutyFileCollection {
  constructor(
    public files: DedutyFile[],
  ) {}

  static fromOptions({ files }: IDedutyFileCollection): DedutyFileCollection {
    return new DedutyFileCollection(files.map(DedutyFile.fromOptions))
  }
}
