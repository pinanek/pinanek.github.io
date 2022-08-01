<template>
  <h2 v-bind="props">
    <HeadingLink :href="`#${id}`" />
    <slot />
  </h2>
</template>

<script setup lang="ts">
import { useAttrs } from 'vue'

const { id, ...props } = useAttrs() as { id: string; [x: string]: unknown }
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

h2 {
  position: relative;
  margin: 1.5em 0 0.5em;
  font-size: 1.75em;
  transition: color $duration-default $ease-in-out;
}
</style>
