<script setup lang="ts">
enum ChildComponents {
  CreatePackage,
  GitPackage,
  LocalPackage,
}

const activeChildComponent = ref<ChildComponents | null>(null)
</script>

<template>
  <div
    v-show="activeChildComponent !== null"
    flex flex-row
    h-full w-full
    left-0 right-0 top-0 bottom-0
    fixed
    class="overlay"
  >
    <div
      flex flex-grow
      justify-center items-center
      @click.self="activeChildComponent = null"
    >
      <div
        class="overlay box"
      >
        <div
          v-show="activeChildComponent === ChildComponents.CreatePackage"
          h-full w-full
          m-0
        >
          <CreatePackage />
        </div>
        <div
          v-show="activeChildComponent === ChildComponents.GitPackage"
          h-full w-full
          m-0
        >
          <GitPackage />
        </div>
        <div
          v-show="activeChildComponent === ChildComponents.LocalPackage"
          h-full w-full
          m-0
        >
          <LocalPackage />
        </div>
      </div>
    </div>
  </div>

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
        @click="activeChildComponent = ChildComponents.CreatePackage"
      >
        Create
      </button>
      <button
        border="~ rounded gray-200 dark:gray-700"
        p-2
        text-left
        @click="activeChildComponent = ChildComponents.GitPackage"
      >
        Git
      </button>
      <button
        border="~ rounded gray-200 dark:gray-700"
        p-2
        text-left
        @click="activeChildComponent = ChildComponents.LocalPackage"
      >
        Local
      </button>
    </div>
  </div>
</template>

<style scoped lang="sass">
div.overlay
  backdrop-filter: blur(5px)
  z-index: 100

div.overlay.box
  width: 80%
  height: 80%

button
  transition: all 200ms ease-in-out

button:hover
  background-color: darkcyan
  transition: all 200ms ease-in-out
</style>
