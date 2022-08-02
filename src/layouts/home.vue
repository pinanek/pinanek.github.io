<template layout="base">
  <div class="header-top-spacing" />
  <Header />
  <div class="header-bottom-spacing" />
  <main>
    <slot />
  </main>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;
.header-top-spacing {
  height: $spacing-16;
}

.header-bottom-spacing {
  height: $spacing-12;
}
</style>
