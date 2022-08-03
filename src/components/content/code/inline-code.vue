<template>
  <code>
    <slot />
  </code>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

code {
  padding: $spacing-1 $spacing-2;
  font-family: $font-mono;
  border-radius: $radii-sm;
  transition: background-color $duration-default $ease-in-out;

  :deep(span.line > span) {
    transition-property: color, background-color;
    transition-duration: $duration-default;
    transition-timing-function: $ease-in-out;
  }
}
</style>
