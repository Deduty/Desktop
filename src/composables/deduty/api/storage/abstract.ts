export declare function DeleteValueFunction(key: string): Promise<boolean>
export declare function GetValueFunction(key: string): Promise<string | null>
export declare function SetValueFunction(key: string, value: string, replace: boolean): Promise<boolean>

export class DedutyStorageApi {
  constructor(
    readonly deleteValue: typeof DeleteValueFunction,
    readonly getValue: typeof GetValueFunction,
    readonly setValue: typeof SetValueFunction,
  ) {}

  public async delete(key: string): Promise<boolean> {
    return this.deleteValue(key)
  }

  public async get(key: string): Promise<string | null > {
    return this.getValue(key)
  }

  public async set(key: string, value: string, replace?: boolean): Promise<boolean> {
    return this.setValue(key, value, replace || false)
  }
}
