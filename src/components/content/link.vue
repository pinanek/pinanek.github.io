<template>
  <a><slot /></a>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

a {
  color: var(--color-accent-default);
  text-decoration: none;
  transition: color $duration-default $ease-in-out;

  &:hover {
    text-decoration: underline;
  }
}
</style>
