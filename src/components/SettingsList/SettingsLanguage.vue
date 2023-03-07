<script setup lang="ts">
const { availableLocales, locale, t } = useI18n()

const toggleLanguage = () => {
  locale.value = availableLocales[
    (availableLocales.indexOf(locale.value) + 1) % availableLocales.length]

  localStorage['i18n-language'] = locale.value
}
</script>

<template>
  <div flex flex-col gap-2>
    <div text-2xl>
      {{ t('component.SettingsLanguage.Language') }}
    </div>
    <div
      flex flex-row
      p-2
      border="~ rounded gray-200 dark:gray-700"
    >
      <div flex mr-a>
        {{ t('component.SettingsLanguage.Change language') }}
      </div>
      <div
        icon-btn
        @click="toggleLanguage()"
      >
        {{ t('language.name') }}
      </div>
    </div>
  </div>
</template>
