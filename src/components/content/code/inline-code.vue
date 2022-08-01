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
}
</style>
