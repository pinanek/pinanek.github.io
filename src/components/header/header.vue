<template>
  <div class="wrapper">
    <div class="page-sizing page-spacing container">
      <a href="/" class="logo" aria-label="Go to home">
        <Logo />
      </a>
      <div class="items">
        <div class="header-items">
          <HeaderItem href="/posts">--posts</HeaderItem>
          <HeaderItem href="/categories">--categories</HeaderItem>
          <HeaderItem href="/about">--about-me-🌻</HeaderItem>
        </div>
        <div>
          <ThemeButton client:visible />
        </div>
      </div>
      <MenuButton client:visible />
    </div>
  </div>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

.wrapper {
  position: sticky;
  top: 0;
  z-index: 100;
  background-color: var(--color-page-background);
  transition: background-color $duration-default $ease-in-out;
}

.container {
  display: flex;
  justify-content: space-between;
  gap: $spacing-6;
  flex-wrap: wrap;
  padding-top: $spacing-6;
  padding-bottom: $spacing-6;

  @media ($breakpoint-md) {
    gap: $spacing-12;
    align-items: center;
  }
}

.logo {
  color: var(--color-text-default);
  text-decoration: none;
  transition: color $duration-default $ease-in-out;
}

.items {
  display: none;
  flex-direction: column;
  justify-content: space-between;
  flex-grow: 1;
  flex-basis: 100%;
  order: 1;
  gap: $spacing-6;
  padding-left: $spacing-2;

  .mobile-header-menu-open & {
    display: flex;
  }

  @media ($breakpoint-md) {
    order: 0;
    display: flex;
    flex-basis: 0;
    padding-left: auto;
    align-items: center;
    flex-direction: row;
  }
}

.header-items {
  display: flex;
  flex-direction: column;
  gap: $spacing-6;

  @media ($breakpoint-md) {
    align-items: center;
    flex-direction: row;
  }
}

// Menu button
.container > :deep(ile-root) {
  display: inline-block;

  @media ($breakpoint-md) {
    display: none !important;
  }
}
</style>
