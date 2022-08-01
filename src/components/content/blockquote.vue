<template>
  <blockquote><slot /></blockquote>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

blockquote {
  margin: 1em 0;
  border-left: $spacing-1 solid var(--color-content-blockquote-indicator);
  padding-left: $spacing-4;
  transition: border-left-color $duration-default $ease-in-out;
}
</style>
