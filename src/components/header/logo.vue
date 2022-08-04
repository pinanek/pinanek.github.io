<template>
  <span>./pinanek23</span>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

span {
  font-size: 1.5rem;
  font-weight: $font-weight-extra-bold;

  @media ($breakpoint-sm) {
    font-size: 1.75rem;
  }

  @media ($breakpoint-md) {
    font-size: 2rem;
  }
}
</style>
