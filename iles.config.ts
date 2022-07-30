import { defineConfig } from 'iles'
import site from './src/site'
import htmlMinifier from './scripts/html-minifier'
import xmlMinifier from './scripts/xml-minifier'

export default defineConfig({
  siteUrl: site.url,
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
