<template layout="home">
  <div class="greeting-container">
    <span class="hello">Hello, I am</span>
    <a href="/about" class="name-link">
      <span class="name">Pinanek23</span>
      <span class="indicator"></span>
    </a>
  </div>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

.greeting-container {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  text-align: right;
}

.hello {
  transition: color $duration-default $ease-in-out;
  font-size: 1.5rem;

  @media ($breakpoint-md) {
    font-size: 2.125rem;
  }
}

.name-link {
  position: relative;
  font-size: 3rem;
  line-height: calc(1em + 1rem);
  text-decoration: none;

  @media ($breakpoint-sm) {
    font-size: 4rem;
    line-height: calc(1em + 1.25rem);
  }

  @media ($breakpoint-md) {
    font-size: 6rem;
    line-height: calc(1em + 1.5rem);
  }
}

.name {
  font-weight: $font-weight-extra-bold;
  background: #0466c8;
  background: linear-gradient(to right top, #0466c8 20%, #a45cba 70%, #a45cba 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.indicator {
  position: absolute;
  width: 0;
  left: 0;
  bottom: 0.075em;
  height: 0.075em;
  background: #0466c8;
  background: linear-gradient(to right top, #0466c8 20%, #a45cba 70%, #a45cba 100%);
  transition: width $duration-default $ease-in-out;

  .name-link:hover & {
    width: 100%;
  }
}

.name::selection {
  -webkit-text-fill-color: var(--color-text-default);
}
</style>
