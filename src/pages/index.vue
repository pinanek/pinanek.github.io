<template layout="home">
  <div class="greeting-container">
    <p class="hello">Hello, I am</p>
    <p class="name">Pinanek23</p>
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

.name {
  font-weight: $font-weight-extra-bold;
  background: #0466c8;
  background: linear-gradient(to right top, #0466c8 20%, #a45cba 70%, #a45cba 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  font-size: 3rem;
  line-height: calc(1em + 1rem);

  @media ($breakpoint-sm) {
    font-size: 4rem;
    line-height: calc(1em + 1.25rem);
  }

  @media ($breakpoint-md) {
    font-size: 6rem;
    line-height: calc(1em + 1.5rem);
  }
}

.name::selection {
  -webkit-text-fill-color: var(--color-text-inverse);
}
</style>
