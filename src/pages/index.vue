<template>
  <div
    flex flex-row
    h-full w-full
    gap-4
  >
    <div
      flex flex-col
      p-2
      style="width: min(65ch, 100%)"
    >
      <PackageBar />
    </div>
    <div
      flex flex-col
      m-a p-2
    >
      <AddPackage />
    </div>
  </div>
</template>
