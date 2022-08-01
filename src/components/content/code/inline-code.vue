<template>
  <code v-bind="$attrs">
    <slot />
  </code>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

code {
  padding: $spacing-0_5 $spacing-1_5;
  font-family: $font-mono;
  border-radius: $radii-sm;
}
</style>
