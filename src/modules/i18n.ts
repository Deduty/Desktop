import { createI18n } from 'vue-i18n'
import { type UserModule } from '~/types'

// Import i18n resources
// https://vitejs.dev/guide/features.html#glob-import
//
// Don't need this? Try vitesse-lite: https://github.com/antfu/vitesse-lite
const messages = Object.fromEntries(
  Object.entries(
    import.meta.glob<{ default: any }>('../../locales/*.y(a)?ml', { eager: true }))
    .map(([key, value]) => {
      const yaml = key.endsWith('.yaml')
      return [key.slice(14, yaml ? -5 : -4), value.default]
    }),
)

export const install: UserModule = ({ app }) => {
  localStorage['i18n-language'] = localStorage['i18n-language'] || 'en'

  const i18n = createI18n({
    legacy: false,
    locale: localStorage['i18n-language'],
    fallbackLocale: 'en',
    messages,
  })

  app.use(i18n)
}
