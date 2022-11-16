<script setup lang="ts">
enum PackageSource {
  Git,
  Local,
  Web,
}

interface Package {
  name: string
  version: string
  source: PackageSource
  size: number /* bytes */
  location: string
  language: string
}

const sizeToString = (size: number /* bytes */): string => {
  const sizes = [
    { prefix: 'B', division: 1 },
    { prefix: 'KiB', division: 8 },
    { prefix: 'MiB', division: 1024 },
    { prefix: 'GiB', division: 1024 },
    { prefix: 'TiB', division: 1024 },
  ]

  let shrinkedSize = size
  for (const { prefix, division } of sizes) {
    shrinkedSize = shrinkedSize / division
    if (shrinkedSize < 1024)
      return `${shrinkedSize.toFixed(2).replace(/(\.0+|0+)/g, '')} ${prefix}`
  }
  return '> 1 PiB'
}

const packageList = ref<Package[]>([])

onMounted(() => {
  for (let i = 0; i < 1000; i += 1) {
    packageList.value.push({
      name: `Template ${i}`,
      version: `1.${i}.${i * 2}`,
      source: Object.values(PackageSource)[i % 3] as PackageSource,
      size: 1024 * (i + 1),
      location: '',
      language: ['English', 'Russian'][i % 2],
    })
  }
})
</script>

<template>
  <div
    flex flex-col
    border="~ rounded gray-200 dark:gray-700"
    class="container"
    h-full
    m-0 p-2
  >
    <ul overflow-y-scroll>
      <li v-for="(pkg, index) in packageList" :key="index">
        <div
          flex flex-row
          border="~ rounded gray-200 dark:gray-700"
          class="package item"
          m-2 p-2
        >
          <div
            flex flex-col
            gap-2
          >
            <div
              class="black-text"
              text-lg
            >
              {{ pkg.name }}
            </div>
            <div m-a />
            <div
              class="gray-text"
              text="sm gray-400 dark:gray-500"
            >
              Version: {{ pkg.version }}
            </div>
          </div>
          <div m-a />
          <div
            flex flex-col
            gap-2
          >
            <div
              class="black-text"
              text="sm right"
            >
              {{ sizeToString(pkg.size) }}
            </div>
            <div m-a />
            <div
              class="gray-text"
              text="sm right gray-400 dark:gray-500"
            >
              {{ pkg.source }}
            </div>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped lang="sass">
div.package.item
  cursor: pointer
  transition: all 200ms ease-in-out

div.package.item:hover
  background-color: darkcyan
  transition: all 200ms ease-in-out

div.package.item:hover .black-text
  color: white
  transition: all 200ms ease-in-out

div.package.item:hover .gray-text
  color: black
  transition: all 200ms ease-in-out
</style>
