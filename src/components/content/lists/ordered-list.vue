<template>
  <ol>
    <slot />
  </ol>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

ol {
  margin: 1em 0;

  & :deep(li::marker) {
    font-weight: $font-weight-bold;
  }
}
</style>
