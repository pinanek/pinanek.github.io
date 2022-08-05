<page>
path: /rss.xml
</page>

<template>
  <RenderFeed format="atom" :options="options" :items="items" />
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

  const publishedDate = post.publishedDate !== undefined ? new Date(post.publishedDate) : new Date()
  const lastUpdated = post.lastUpdated !== undefined ? new Date(post.lastUpdated) : publishedDate

  return {
    link: new URL(`posts/${slug}`, site.url).toString(),
    title: post.seoTitle,
    description: post.description,
    published: publishedDate,
    date: lastUpdated,
    image: new URL(post.image.src, site.url).toString(),
    author: [
      {
        name: 'Pinanek23',
        link: site.url
      }
    ]
  }
})
</script>
