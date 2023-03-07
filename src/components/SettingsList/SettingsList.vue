<template>
  <div
    flex flex-col
    h-full
    m-0 p-2
  >
    <div
      text-4xl
      p-2
    >
      Settings
    </div>
    <div overflow-y-auto p-2>
      <SettingsTheme pb-8 />
      <div text-2xl p-2 pl-0>
        Other
      </div>
      <SettingsDeleteAll />
    </div>
  </div>
</template>
