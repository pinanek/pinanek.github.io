import { defineConfig } from 'iles'

import imagesPlugins, { widthPreset } from '@islands/images'
import feed from '@islands/feed'

import site from './src/site'
import { markdownConfig, extendMarkdownFrontmatter } from './src/markdown/config'
import xmlMinifier from './scripts/xml-minifier'
import htmlMinifier from './scripts/html-minifier'

const images = imagesPlugins({
  postThumbnail: widthPreset({
    loading: 'eager',
    decoding: 'async',
    widths: ['original'],
    formats: {
      webp: { quality: 75 },
      png: { quality: 75 }
    }
  }),
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

export default defineConfig({
  siteUrl: site.url,

  modules: [images, feed()],

  extendFrontmatter: extendMarkdownFrontmatter(images),
  markdown: markdownConfig,

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
