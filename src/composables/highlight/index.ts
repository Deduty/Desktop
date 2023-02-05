import DedutyDark from './deduty-dark'
import DedutyLight from './deduty-light'

export interface IStyleObject {
  [className: string]: {
    [styleValue: string]: string
  }
}

const SelfToSelfTemplate = (value: string) => value

export class StyleObject {
  constructor(private mapping: IStyleObject) {}

  public styleSheet({
    classNameTemplate = SelfToSelfTemplate,
    styleKeyTemplate = SelfToSelfTemplate,
    styleValueTemplate = SelfToSelfTemplate,
  }): string {
    return Object
      .entries(this.mapping)
      .map(([className, styleMapping]) => {
        const styleBody = Object
          .entries(styleMapping)
          .map(([styleKey, styleValue]) => `\t${styleKeyTemplate(styleKey)}: ${styleValueTemplate(styleValue)};`)

        return [`${classNameTemplate(className)} {`, ...styleBody, '}\n'].join('\n')
      })
      .join('\n')
  }
}

export const getCurrentStyles = async (): Promise<{ dark: StyleObject; light: StyleObject }> => {
  return {
    dark: new StyleObject(DedutyDark as IStyleObject),
    light: new StyleObject(DedutyLight as IStyleObject),
  }
}
