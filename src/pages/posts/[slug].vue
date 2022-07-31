<template layout="post">
  <HeadSeo
    :title="post.frontmatter.title"
    :seo-title="post.frontmatter.seoTitle"
    :description="post.frontmatter.description"
    :image-src="post.frontmatter.image.url"
    :image-alt="post.frontmatter.image.alt"
  />
  <component :is="post" excerpt />
</template>

<script setup lang="ts">
import type { Document } from 'iles'
import type { PostFrontmatter } from '@/types/post'

interface Props {
  post: Document<{ frontmatter: PostFrontmatter }>
}

defineProps<Props>()
</script>

<script lang="ts">
// eslint-disable-next-line no-undef
export default definePageComponent({
  async getStaticPaths() {
    // eslint-disable-next-line no-undef
    const posts = useDocuments<{ frontmatter: PostFrontmatter }>('content/posts').value

    return posts.map((post) => ({
      params: {
        slug: post.filename
          .replace(/\/index.mdx?$/, '')
          .split(/[/\\]/)
          .pop() as string
      },
      props: { post }
    }))
  }
})
</script>
