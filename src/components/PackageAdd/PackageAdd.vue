<template>
  <div
    h-full w-full
    border="~ rounded gray-200 dark:gray-700"
    m-0 p-4
    flex flex-col
    class="box"
    gap-4
    w-prose h-min
  >
    <LocalPackageAdd />
  </div>
</template>

<style scoped lang="sass">
html div.box
  background-color: white

html.dark div.box
  background-color: #121212
</style>
