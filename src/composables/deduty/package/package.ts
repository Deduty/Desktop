import { DedutyFileCollection } from '../file'
import type { IDedutyPackage, IDedutyPackageMeta } from './scheme'

export class DedutyPackageMeta implements IDedutyPackageMeta {
  constructor(
    public name: string,
    public version: string,
    public language: string,
    public tags: string[],
  ) {}

  static fromOptions({ name, version, language, tags }: IDedutyPackageMeta): DedutyPackageMeta {
    return new DedutyPackageMeta(name, version, language, tags)
  }
}

export class DedutyPackage implements IDedutyPackage {
  constructor(
    public id: string,
    public meta: DedutyPackageMeta,
    public files: DedutyFileCollection,
  ) {}

  static fromOptions({ id, meta, files }: IDedutyPackage): DedutyPackage {
    return new DedutyPackage(id, DedutyPackageMeta.fromOptions(meta), DedutyFileCollection.fromOptions(id, files))
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
