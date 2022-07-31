<template>
  <Head>
    <title>{{ title }}</title>

    <meta name="description" :content="description" />

    <meta property="og:site_name" content="Pinanek23" />
    <meta property="og:type" content="website" />
    <meta property="og:url" :content="canonicalUrl" />
    <meta property="og:title" :content="seoTitle" />
    <meta property="og:description" :content="description" />
    <meta property="og:image" :content="imageUrl" />
    <meta property="og:image:alt" :content="imageAlt" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:url" :content="canonicalUrl" />
    <meta name="twitter:title" :content="seoTitle" />
    <meta name="twitter:description" :content="description" />
    <meta name="twitter:image" :content="imageUrl" />
    <meta name="twitter:image:alt" :content="imageAlt" />
  </Head>
</template>

<script setup lang="ts">
import { useRoute } from 'iles'
import site from '@/site'

interface Props {
  title?: string
  seoTitle?: string
  description?: string
  imageSrc?: string
  imageAlt?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: undefined,
  seoTitle: undefined,
  description: 'My little garden 🌻 with knowledge and experience of what I have gained 😁',
  imageSrc: 'default-logo.png',
  imageAlt: 'Logo of Pinanek23'
})

const route = useRoute()

const title = props.title === undefined ? site.title : `${props.title} • ${site.title}`
const seoTitle = props.seoTitle === undefined ? title : `${props.seoTitle} • ${site.title}`
const description = props.description ?? site.description
const canonicalUrl = new URL(route.fullPath, site.url).toString().replace(/\/+$/, '')
const imageUrl = new URL(site.imageSrc, site.url).toString()
</script>
