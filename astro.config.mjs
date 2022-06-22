import { defineConfig } from 'astro/config'

import { astroImageTools } from 'astro-imagetools'
import solid from '@astrojs/solid-js'

import autoprefixer from 'autoprefixer'
import cssnano from 'cssnano'

import { s } from 'hastscript'

const headingLinkIcon = s(
  'svg',
  {
    viewBox: '0 0 24 24',
    xlmns: 'http://www.w3.org/2000/svg'
  },
  [
    s('path', {
      d: 'M6.25 3A3.25 3.25 0 0 0 3 6.25v6a3.25 3.25 0 0 0 3.25 3.25H7.5V14H6.25a1.75 1.75 0 0 1-1.75-1.75v-6c0-.966.784-1.75 1.75-1.75h6c.966 0 1.75.784 1.75 1.75v6A1.75 1.75 0 0 1 12.25 14H11v1.5h1.25a3.25 3.25 0 0 0 3.25-3.25v-6A3.25 3.25 0 0 0 12.25 3h-6Z'
    }),
    s('path', {
      d: 'M10 11.75c0-.966.784-1.75 1.75-1.75h1.248V8.5H11.75a3.25 3.25 0 0 0-3.25 3.25v6A3.25 3.25 0 0 0 11.75 21h6A3.25 3.25 0 0 0 21 17.75v-6a3.25 3.25 0 0 0-3.25-3.25H16.5V10h1.25c.966 0 1.75.784 1.75 1.75v6a1.75 1.75 0 0 1-1.75 1.75h-6A1.75 1.75 0 0 1 10 17.75v-6Z'
    })
  ]
)

export default defineConfig({
  integrations: [solid(), astroImageTools],

  experimental: {
    integrations: true
  },

  markdown: {
    remarkPlugins: ['remark-gfm', ['remark-smartypants', { dashes: false }]],
    rehypePlugins: [
      'rehype-external-links',
      'rehype-slug',
      [
        'rehype-autolink-headings',
        {
          properties: {
            class: 'heading-link',
            'aria-hidden': 'true',
            tabindex: -1
          },
          content: [headingLinkIcon]
        }
      ]
    ],

    shikiConfig: {
      theme: 'dark-plus'
    }
  },

  vite: {
    css: {
      postcss: {
        plugins: [autoprefixer(), cssnano()]
      }
    }
  }
})
