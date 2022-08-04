<page>
path: /rss.xml
</page>

<template>
  <RenderFeed format="rss" :options="options" :items="items" />
</template>

<script setup lang="ts">
import { getSlugByFileName } from '@/utils/post'
import type { FeedOptions, FeedItem } from '@islands/feed'
import type { PostFrontmatter } from '@/types/post'

// eslint-disable-next-line no-undef
const { site } = usePage()

// eslint-disable-next-line no-undef
const posts = $(useDocuments<PostFrontmatter>('content/posts'))

const options: FeedOptions = {
  title: site.title,
  description: site.description,
  id: site.url,
  link: site.url,
  language: 'en',
  image: new URL(site.imageSrc, site.url).toString(),
  copyright: 'Copyright (c) 2022 Ngo Duc Hoang Son'
}

const items = posts.map<FeedItem>((post) => {
  const slug = getSlugByFileName(post.filename)

  return {
    link: new URL(`posts/${slug}`, site.url).toString(),
    date: post.date,
    title: post.seoTitle,
    description: post.description,
    published: post.publishedDate !== undefined ? new Date(post.publishedDate) : undefined,
    image: new URL(post.image.src, site.url).toString()
  }
})
</script>
