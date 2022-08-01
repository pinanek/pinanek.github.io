import { defineConfig } from 'iles'

import images, { widthPreset } from '@islands/images'

import site from './src/site'
import htmlMinifier from './scripts/html-minifier'
import xmlMinifier from './scripts/xml-minifier'

import remarkImageSize from './plugins/remark/remark-image-size'

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
    remarkPlugins: [remarkImageSize]
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
