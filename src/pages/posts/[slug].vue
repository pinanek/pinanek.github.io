<template layout="post">
  <Post :post="post" />
</template>

<script setup lang="ts">
import type { Document } from 'iles'
import type { PostFrontmatter } from '@/types/post'
import { getSlugByFileName } from '@/utils/post.js'

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
    const posts = $(useDocuments<{ frontmatter: PostFrontmatter }>('content/posts'))

    return posts.map((post) => ({
      params: { slug: getSlugByFileName(post.filename) },
      props: { post }
    }))
  }
})
</script>
