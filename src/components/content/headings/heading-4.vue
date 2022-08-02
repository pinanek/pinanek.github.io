<template>
  <h3 v-bind="props">
    <HeadingLink :href="`#${id}`" />
    <slot />
  </h3>
</template>

<script setup lang="ts">
import { useAttrs } from 'vue'

const { id, ...props } = useAttrs() as { id: string; [property: string]: unknown }
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

h3 {
  position: relative;
  margin: 1em 0;
  font-size: 1.125em;
  transition: color $duration-default $ease-in-out;
}
</style>
