import type { IDedutyFile, IDedutyFileCollection, IDedutyPackage, IDedutyPackageMeta } from './scheme'

export class PackageFile implements IDedutyFile {
  constructor(
    public location: string,
    public extension: string,
    public alias?: string,
  ) {}

  static fromOptions({ alias, location, extension }: IDedutyFile): PackageFile {
    return new PackageFile(location, extension, alias)
  }
}

export class PackageFiles implements IDedutyFileCollection {
  constructor(
    public files: PackageFile[],
  ) {}

  static fromOptions({ files }: IDedutyFileCollection): PackageFiles {
    return new PackageFiles(files.map(file => PackageFile.fromOptions(file)))
  }
}

export class PackageMeta implements IDedutyPackageMeta {
  constructor(
    public name: string,
    public version: string,
    public language: string,
    public tags: string[],
  ) {}

  static fromOptions({ name, version, language, tags }: IDedutyPackageMeta): PackageMeta {
    return new PackageMeta(name, version, language, tags)
  }
}

export class Package implements IDedutyPackage {
  constructor(
    public id: string,
    public meta: PackageMeta,
    public files: PackageFiles,
  ) {}

  static fromOptions({ id, meta, files }: IDedutyPackage): Package {
    return new Package(id, PackageMeta.fromOptions(meta), PackageFiles.fromOptions(files))
  }
}

// export class PackageSize {
//   private sizeString = ''

//   /**
//      * @param size bytes
//     */
//   constructor(readonly size: number) {}

//   toString(): string {
//     if (this.sizeString !== '')
//       return this.sizeString

//     const sizes = [
//       { prefix: 'B', division: 1 },
//       { prefix: 'KiB', division: 8 },
//       { prefix: 'MiB', division: 1024 },
//       { prefix: 'GiB', division: 1024 },
//       { prefix: 'TiB', division: 1024 },
//     ]

//     let shrinkedSize = this.size
//     for (const { prefix, division } of sizes) {
//       shrinkedSize = shrinkedSize / division
//       if (shrinkedSize < 1024) {
//         this.sizeString = `${shrinkedSize.toFixed(2).replace(/(\.0+0)/g, '')} ${prefix}`
//         return this.sizeString
//       }
//     }
//     this.sizeString = '> 1 PiB'
//     return this.sizeString
//   }
// }
