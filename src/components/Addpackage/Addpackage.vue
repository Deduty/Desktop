<template>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    w-xs
    p-4
    gap-4
  >
    <div
      left
    >
      Add package
    </div>
    <div
      flex flex-col
      gap-1
    >
      <button
        border="~ rounded gray-200 dark:gray-700"
        p-2
        text-left
      >
        Git
      </button>
      <button
        border="~ rounded gray-200 dark:gray-700"
        p-2
        text-left
      >
        Local
      </button>
      <button
        border="~ rounded gray-200 dark:gray-700"
        p-2
        text-left
      >
        Create
      </button>
    </div>
  </div>
</template>

<style scoped lang="sass">
button
  transition: all 200ms ease-in-out

button:hover
  background-color: cadetblue
  transition: all 200ms ease-in-out
</style>
