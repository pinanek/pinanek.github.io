<template>
  <HeadSeo
    :title="title"
    :seo-title="seoTitle"
    :description="description"
    :image-src="image.src"
    :image-alt="image.alt"
  />
  <div class="main">
    <div class="info">
      <div class="frontmatter">
        <h1>{{ title }}</h1>
        <span class="description">{{ description }}</span>
        <span class="dates">
          <span>{{ publishedDate }}</span>
          <span v-if="lastUpdated">-</span>
          <span v-if="lastUpdated">Last updated: {{ lastUpdated }}</span>
        </span>
      </div>
      <Picture :src="image.srcSets" :alt="image.alt" :width="image.width" :height="image.height" :style="image.style" />
    </div>
    <article>
      <component :is="post" excerpt />
    </article>
  </div>
</template>

<script setup lang="ts">
import type { Document } from 'iles'
import type { PostFrontmatter } from '@/types/post'

interface Props {
  post: Document<{ frontmatter: PostFrontmatter }>
}
const props = defineProps<Props>()

// eslint-disable-next-line no-undef
const { title, seoTitle, description, image, publishedDate, lastUpdated } = $(props.post.frontmatter)
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

.main {
  margin: $spacing-20 0;
}

.info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: $spacing-8;
  transition: color $duration-default $ease-in-out;

  :deep(img) {
    border-radius: $radii-md;
    margin: auto;
  }
}

.frontmatter {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: $spacing-2;

  h1 {
    line-height: calc(1em + 0.75rem);
  }

  .description {
    font-size: 1.125rem;
  }

  .dates {
    display: flex;
    align-items: center;
    gap: $spacing-2;
  }
}
</style>
