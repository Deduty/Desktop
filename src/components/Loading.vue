<!--
  This loading animation were fistly published by Supah
    Dribbble: https://dribbble.com/supahfunk
    Twitter: https://twitter.com/supahfunk
    Codepen: https://codepen.io/supah/

  Thanks!
-->

<template>
  <div
    h-full w-full
    flex flex-grow
    align-middle
    content-center
    justify-center
  >
    <svg class="spinner" viewBox="0 0 50 50">
      <circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="5" />
    </svg>
  </div>
</template>

<style scoped lang="sass">
.spinner
  animation: rotate 2s linear infinite
  width: 50px
  height: 50px

  & .path
    stroke: cadetblue
    stroke-linecap: round
    animation: dash 1.5s ease-in-out infinite

@keyframes rotate
  100%
    transform: rotate(360deg)

@keyframes dash
  0%
    stroke-dasharray: 1, 150
    stroke-dashoffset: 0

  50%
    stroke-dasharray: 90, 150
    stroke-dashoffset: -35

  100%
    stroke-dasharray: 90, 150
    stroke-dashoffset: -124
</style>
