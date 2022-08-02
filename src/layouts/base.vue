<script client:load lang="ts">
import { handleColorMode } from '@/utils/color-mode'
handleColorMode()
</script>

<template>
  <HeadSeo />
  <slot />
</template>
