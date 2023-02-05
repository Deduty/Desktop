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
    public size: number | undefined,
    public service: string,
    public meta: DedutyPackageMeta,
    public files: DedutyFileCollection,
  ) {}

  static fromOptions({ id, size, service, meta, files }: IDedutyPackage): DedutyPackage {
    return new DedutyPackage(
      id,
      size,
      service,
      DedutyPackageMeta.fromOptions(meta),
      DedutyFileCollection.fromOptions(id, files),
    )
  }

  packageSize(): string {
    if (!this.size)
      return 'Unknown'

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
      if (shrinkedSize < 1024)
        return `${shrinkedSize.toFixed(2).replace(/(\.0+0)/g, '')} ${prefix}`
    }
    return '> 1 PiB'
  }
}
