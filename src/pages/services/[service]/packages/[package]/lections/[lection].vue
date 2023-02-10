<script setup lang="ts">
const properties = defineProps<{ service: string; package: string; lection: string }>()

const packageStore = usePackageStore()

const lectionObject = (
  packageStore
    .indexedPackages
    .get(properties.service)
    ?.get(properties.package)
    ?.lections
    .find(lectionObject => lectionObject.id === properties.lection))

const errorMessage = `Lection with id \`${properties.lection}\` not found. Probably service, package or lection is not exist.`
</script>

<template>
  <div v-if="!lectionObject" flex-grow>
    <Error :message="errorMessage" />
  </div>
  <LectionReader
    v-if="lectionObject"
    :service="properties.service"
    :pack="properties.package"
    :lection="lectionObject"
    :api-enabled="true"
  />
</template>
