<template>
  <Picture :src="src" v-bind="props" />
</template>

<script setup lang="ts">
import { useAttrs } from 'vue'

const { src, ...props } = useAttrs() as { src: string; [x: string]: unknown }
</script>

<script lang="ts">
export default {
  inheritAttrs: false
}
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

:deep(img) {
  border-radius: $radii-md;
}
</style>
