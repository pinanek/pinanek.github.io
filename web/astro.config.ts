import { defineConfig } from 'astro/config'
import constants from './src/constants'

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'

// Integrations
import solid from '@astrojs/solid-js'
import sitemap from '@astrojs/sitemap'
import htmlMinifier from '@pinanek23/astro-html-minifier'

export default defineConfig({
  outDir: '_dist',

  site: constants.site.url,

  integrations: [solid(), sitemap(), htmlMinifier()],

  vite: {
    plugins: [
      vanillaExtractPlugin({
        identifiers: 'debug'
      }) as never
    ]
  }
})
