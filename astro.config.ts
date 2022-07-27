import { defineConfig } from 'astro/config'

import solid from '@astrojs/solid-js'
import sitemap from '@astrojs/sitemap'
import astroHtmlMinifier from './integrations/astro-html-minifier'

export default defineConfig({
  site: 'http://pinanek23.pages.dev/',

  integrations: [solid(), sitemap(), astroHtmlMinifier()],

  legacy: {
    astroFlavoredMarkdown: true
  }
})
