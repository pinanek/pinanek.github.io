<template>
  <a :href="`/categories/${category.raw}`">
    <span class="background"></span>
    <span>{{ category.name }}</span>
  </a>
</template>

<script setup lang="ts">
import type { PostCategory } from '@/types/post'

interface Props {
  category: PostCategory
}

defineProps<Props>()
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

a {
  position: relative;
  color: var(--color-accent-default);
  padding: 0.25rem 0.75rem;
  text-decoration: none;
  text-align: center;
  transition: color $duration-default $ease-in-out;
}

.background {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  margin: auto;
  z-index: -1;
  width: 100%;
  height: 100%;
  background-color: var(--color-category-item-background);
  border-radius: $radii-md;
  transition-property: transform, background-color;
  transition-duration: $duration-default;
  transition-timing-function: $ease-in-out;

  a:hover & {
    transform: scale(1.1);
  }
}
</style>
