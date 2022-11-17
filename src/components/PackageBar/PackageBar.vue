<template>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    h-full
    p-4
    gap-4
  >
    <div m-0>
      <PackageSearch />
    </div>
    <div
      flex-grow
      overflow-hidden
      m-0
    >
      <PackageList />
    </div>
  </div>
</template>
