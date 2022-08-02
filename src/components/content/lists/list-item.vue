<template>
  <li><slot /></li>
</template>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

li:not(li li) {
  line-height: $line-height-content-paragraph;
  transition: color $duration-default $ease-in-out;
}
</style>
