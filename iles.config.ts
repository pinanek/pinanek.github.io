import { defineConfig } from 'iles'

import images, { widthPreset } from '@islands/images'

import site from './src/site'
import htmlMinifier from './scripts/html-minifier'
import xmlMinifier from './scripts/xml-minifier'

import markdownConfig from './src/markdown/config'

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
