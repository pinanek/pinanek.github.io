import { defineConfig } from 'astro/config'

import solid from '@astrojs/solid-js'
import image from '@astrojs/image'
import sitemap from '@astrojs/sitemap'
import mdx from '@astrojs/mdx'
import htmlMinifier from './integrations/astro-html-minifier.mjs'

import remarkMdxImages from 'remark-mdx-images'
import remarkUnwrapImages from 'remark-unwrap-images'

import rehypeSlug from 'rehype-slug'

export default defineConfig({
  site: 'http://pinanek23.pages.dev/',

  integrations: [
    solid(),
    image(),
    mdx({
      remarkPlugins: {
        extends: [remarkUnwrapImages, remarkMdxImages]
      },
      rehypePlugins: [rehypeSlug]
    }),
    sitemap(),
    htmlMinifier()
  ]
})
