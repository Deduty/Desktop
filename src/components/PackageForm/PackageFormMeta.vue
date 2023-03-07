<script setup lang="ts">
import type { DedutyPackage } from '~/composables/deduty'

const { packageObject } = defineProps<{ packageObject: DedutyPackage }>()

const { t } = useI18n()
</script>

<template>
  <div
    m-0
    h-full w-full
    flex flex-row flex-grow
  >
    <div
      m-0
      h-full w-full
      flex flex-col flex-grow
    >
      <div
        flex flex-grow flex-row
        items-baseline
      >
        <div
          p-2
          text-2xl
        >
          {{ packageObject.meta.name }}
        </div>
        <div
          m-a
          min-w-1
          w-1
        />
        <div
          text-lg
        >
          {{ `${packageObject.serviceId.charAt(0).toUpperCase()}${packageObject.serviceId.slice(1)}` }}
        </div>
      </div>
      <div
        flex flex-row
        m-0 pl-2

        class="gray-text"
        text="gray-400 dark:gray-500"
      >
        <div
          class="gray-text"
          text="sm gray-400 dark:gray-500"
        >
          {{ t('component.PackageFormMeta.Language') }}: {{ packageObject.meta.language }}
        </div>
      </div>
      <div
        flex flex-row
        m-0 pl-2

        class="gray-text"
        text="gray-400 dark:gray-500"
      >
        <div
          class="gray-text"
          text="sm gray-400 dark:gray-500"
        >
          {{ t('component.PackageFormMeta.Version') }}: {{ packageObject.meta.version }}
        </div>
        <div
          class="gray-text"
          text="sm right gray-400 dark:gray-500"
          ml-a
        >
          {{ packageObject.packageSize() }}
        </div>
      </div>
    </div>
  </div>
</template>
