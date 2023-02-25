<script setup lang="ts">
// IMPORTANT: Since this layout is used by any page. Its router is general for all components
//            This script provides guard for redeclaration case of lection reading.
//
// EXAMPLE: Each time when reader is loaded, scripts reevaluated (which is expect).
//          But user scripts can't have same var names so all code logic is breaks.
//
// NOTE: Possible solution is to have Deduty.setWorker('UniqueName', Worker)
//       But is much difficult for user then just use code. So it's easier to just reload page WHEN
//       it is required without chaning code base
//
// REQUIREMENTS: Implement this guard on each layout. But for now it's unique
//
useRouter().beforeEach((to, from) => {
  // Force reload when accessing lection
  if (to.name === 'services-serviceId-packages-packageId-lections-lectionId')
    window.location.assign(to.path)

  // Force reload when return from lection to application
  // Theoretically can reduce errors but we still have problem with about lection
  if (from.name === 'services-serviceId-packages-packageId-lections-lectionId')
    window.location.assign(to.path)
})
</script>

<template>
  <div
    flex flex-row
    h-full w-full
    m-a
  >
    <div
      flex flex-row
      h-full w-full
      m-0
    >
      <RouterView />
    </div>
    <div
      left-a
    >
      <Sidebar />
    </div>
  </div>
</template>
