import { DedutyLection } from '../lection'
import type { IDedutyPackage, IDedutyPackageMeta } from './scheme'
import { updateValues } from '~/composables/utils'

export class DedutyPackage {
  constructor(
    public serviceId: string,
    public id: string,
    public meta: IDedutyPackageMeta,
    public lections: DedutyLection[],
    public size?: number,
  ) {}

  static fromOptions(serviceId: string, { id, meta, lections, size }: IDedutyPackage): DedutyPackage {
    const defaultOptionMeta: IDedutyPackageMeta = { name: id, version: 'Unknown', language: 'Unknown', tags: ['Deprecated'] }
    const optionMeta: IDedutyPackageMeta = updateValues(defaultOptionMeta, JSON.parse(meta))

    const objectLections = lections.map(lection => DedutyLection.fromOptions(serviceId, id, lection))

    return new DedutyPackage(serviceId, id, optionMeta, objectLections, size)
  }

  packageSize(): string {
    if (!this.size)
      return 'Unknown'

    const sizes = [
      // Size must be bytes, not bites
      { prefix: 'B', division: 1 },
      { prefix: 'KiB', division: 1024 },
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
