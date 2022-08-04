<template layout="base">
  <div class="header-top-spacing" />
  <Header />
  <div class="header-bottom-spacing" />
  <main class="page-spacing page-sizing">
    <slot />
  </main>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

.header-top-spacing {
  height: $spacing-8;

  @media ($breakpoint-sm) {
    height: $spacing-12;
  }

  @media ($breakpoint-md) {
    height: $spacing-16;
  }
}

.header-bottom-spacing {
  height: $spacing-4;

  @media ($breakpoint-sm) {
    height: $spacing-8;
  }

  @media ($breakpoint-md) {
    height: $spacing-12;
  }
}
</style>
