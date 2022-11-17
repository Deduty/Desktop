export enum PackageSource {
  Git,
  Local,
  Web,
}

export class PackageSize {
  private sizeString = ''

  /**
     * @param size bytes
    */
  constructor(readonly size: number) {}

  toString(): string {
    if (this.sizeString !== '')
      return this.sizeString

    const sizes = [
      { prefix: 'B', division: 1 },
      { prefix: 'KiB', division: 8 },
      { prefix: 'MiB', division: 1024 },
      { prefix: 'GiB', division: 1024 },
      { prefix: 'TiB', division: 1024 },
    ]

    let shrinkedSize = this.size
    for (const { prefix, division } of sizes) {
      shrinkedSize = shrinkedSize / division
      if (shrinkedSize < 1024) {
        this.sizeString = `${shrinkedSize.toFixed(2).replace(/(\.0+|0+)/g, '')} ${prefix}`
        return this.sizeString
      }
    }
    this.sizeString = '> 1 PiB'
    return this.sizeString
  }
}

export interface IPackage {
  name: string
  version: string
  source: PackageSource
  size: PackageSize
  language: string
}

export class Package implements IPackage {
  constructor(
    public name: string,
    public version: string,
    public source: PackageSource,
    public size: PackageSize,
    public language: string,
  ) {}

  static fromOptions(options: IPackage): Package {
    return new Package(...Object.values(options))
  }
}
