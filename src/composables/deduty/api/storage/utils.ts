export function ValueAsStringOrNull(value: unknown): string | null {
  if (!value)
    return null
  if (typeof value === 'string')
    return value as string
  throw new TypeError(`Internal error: Lection storage got value must be string, not ${typeof value}`)
}

export function ValueAsBoolean(value: unknown): boolean {
  if (typeof value === 'boolean')
    return value
  throw new TypeError(`Internal error: Lection storage set result must be boolean, not ${typeof value}`)
}
