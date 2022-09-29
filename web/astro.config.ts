import constants from './src/constants'
import type { AstroUserConfig } from 'astro'

// Integrations
import solid from '@astrojs/solid-js'
import mdx from '@astrojs/mdx'
import image from '@astrojs/image'
import sitemap from '@astrojs/sitemap'
import htmlMinifier from '@pinanek23/astro-html-minifier'

// Vite
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'

// Remark
import remarkGfm from 'remark-gfm'
import remarkMdxImages from 'remark-mdx-images'
import remarkCodeBlock from '@pinanek23/remark-code-block'
import remarkInlineCode from '@pinanek23/remark-inline-code'

// Rehype
import rehypeInfoBar from '@pinanek23/rehype-info-bar'

// Markdown highlighter
import { Highlighter } from '@pinanek23/highlighter'

const highlighter = new Highlighter({ theme: 'kenanip' })

const config: AstroUserConfig = {
  outDir: '_dist',

  site: constants.site.url,

  markdown: {
    syntaxHighlight: false,
    remarkPlugins: [
      remarkGfm,
      remarkMdxImages,
      [remarkCodeBlock, { highlighter }],
      [remarkInlineCode, { highlighter }]
    ],
    rehypePlugins: [rehypeInfoBar]
  },

  integrations: [
    solid(),
    mdx(),
    image({
      serviceEntryPoint: '@astrojs/image/sharp'
    }),
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
}

export default config
