import { defineConfig } from 'iles'
import { getHighlighter } from 'shiki'

import images, { widthPreset } from '@islands/images'

import site from './src/site'
import htmlMinifier from './scripts/html-minifier'
import xmlMinifier from './scripts/xml-minifier'

import remarkGfm from 'remark-gfm'
import remarkImageSize from './plugins/remark/remark-image-size'

import rehypeInlineCode from './plugins/rehype/rehype-inline-code'
import rehypeCodeBlock from './plugins/rehype/rehype-code-block'

// Get shiki highlighter
const highlighter = await getHighlighter({
  theme: 'dark-plus',
  langs: ['py', 'cpp', 'csharp', 'php', 'js', 'ts', 'tsx', 'bash', 'diff', 'json', 'jsonc', 'yaml']
})

export default defineConfig({
  siteUrl: site.url,

  modules: [
    images({
      markdown: widthPreset({
        loading: 'lazy',
        decoding: 'async',
        widths: ['original'],
        formats: {
          webp: { quality: 75 },
          png: { quality: 75 }
        }
      })
    })
  ],

  markdown: {
    withImageSrc(src) {
      if (!/\.\.?\//.test(src)) src = `./${src}`
      if (!src.includes('?')) return `${src}?preset=markdown`
    },
    remarkPlugins: [remarkGfm, remarkImageSize],
    rehypePlugins: [
      [rehypeInlineCode, { highlighter }],
      [rehypeCodeBlock, { highlighter }]
    ]
  },

  ssg: {
    onSiteRendered: async () => {
      await Promise.all([htmlMinifier(), xmlMinifier()])
    }
  },

  jsx: 'solid',

  vite: {
    optimizeDeps: {
      include: ['solid-js']
    }
  }
})
