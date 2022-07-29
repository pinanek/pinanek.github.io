import { defineConfig } from 'astro/config'

import solid from '@astrojs/solid-js'
import image from '@astrojs/image'
import sitemap from '@astrojs/sitemap'
import mdx from '@astrojs/mdx'
import htmlMinifier from './integrations/astro-html-minifier.mjs'

import remarkGfm from 'remark-gfm'
import remarkSmartypants from 'remark-smartypants'
import remarkMdxImages from 'remark-mdx-images'
import remarkUnwrapImages from 'remark-unwrap-images'

import rehypeSlug from 'rehype-slug'

export default defineConfig({
  site: 'http://pinanek23.pages.dev/',

  integrations: [
    mdx({
      remarkPlugins: [remarkSmartypants, remarkGfm, remarkUnwrapImages, remarkMdxImages],
      rehypePlugins: [rehypeSlug]
    }),
    solid(),
    image(),
    sitemap(),
    htmlMinifier()
  ]
})
