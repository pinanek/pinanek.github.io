<template>
  <Picture :src="src" v-bind="props" />
</template>

<script setup lang="ts">
import { useAttrs } from 'vue'

const { src, ...props } = useAttrs() as { src: string; [property: string]: unknown }
</script>

<script lang="ts">
export default {
  inheritAttrs: false
}
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

picture {
  margin: 1em 0;
}

:deep(img) {
  border-radius: $radii-md;
  margin: auto;
}
</style>
