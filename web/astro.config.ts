import { defineConfig } from 'astro/config'
import constants from './src/constants'

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'

// Integrations
import solid from '@astrojs/solid-js'
import mdx from '@astrojs/mdx'
import image from '@astrojs/image'
import sitemap from '@astrojs/sitemap'
import htmlMinifier from '@pinanek23/astro-html-minifier'

// Remark
import remarkGfm from 'remark-gfm'
import remarkMdxImages from 'remark-mdx-images'
import remarkCodeBlock from '@pinanek23/remark-code-block'
import remarkInlineCode from '@pinanek23/remark-inline-code'

// Markdown highlighter
import { Highlighter } from '@pinanek23/highlighter'

const highlighter = new Highlighter({ theme: 'kenanip' })

// Config
export default defineConfig({
  outDir: '_dist',

  site: constants.site.url,

  markdown: {
    syntaxHighlight: false
  },

  integrations: [
    solid(),
    mdx({
      remarkPlugins: [
        remarkGfm,
        remarkMdxImages,
        [remarkCodeBlock, { highlighter }],
        [remarkInlineCode, { highlighter }]
      ]
    }),
    image(),
    sitemap(),
    htmlMinifier()
  ],

  vite: {
    plugins: [
      vanillaExtractPlugin({
        identifiers: 'debug'
      }) as never
    ]
  }
})
