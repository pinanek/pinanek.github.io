<template>
  <ul>
    <slot />
  </ul>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

ul {
  margin: 1em 0;
}
</style>
