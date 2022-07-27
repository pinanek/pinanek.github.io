import { defineConfig } from 'astro/config'

import solid from '@astrojs/solid-js'
import sitemap from '@astrojs/sitemap'
import mdx from '@astrojs/mdx'
import htmlMinifier from './integrations/astro-html-minifier.mjs'
import mdxImprovement from './integrations/astro-mdx-improvement.mjs'

export default defineConfig({
  site: 'http://pinanek23.pages.dev/',

  integrations: [solid(), mdx(), mdxImprovement(), sitemap(), htmlMinifier()]
})
