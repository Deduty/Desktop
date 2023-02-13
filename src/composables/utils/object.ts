/**
 * Updated target from values and returns its. Mutates target without new keys applied.
 * Have basic type check via `typeof`
 *
 * @param target Object that will be updated
 * @param values Object which data will be used for target update
 * @returns target for handly using
 */
export const updateValues = <T extends object>(target: T, values: object): T => {
  for (const [key, value] of Object.entries(values)) {
    if (
      Object.hasOwn(target, key)
      && value
      && (typeof value === typeof (target as any)[key])
    )
      (target as any)[key] = value
  }

  return target
}
