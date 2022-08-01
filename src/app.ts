import { defineApp } from 'iles'
import { computed } from 'vue'
import { initColorMode } from '@/utils/color-mode'

import Img from '@/components/content/img.vue'

import '@/styles/global.scss'

export default defineApp({
  socialTags: false,

  mdxComponents: {
    img: Img
  },

  head: ({ site, route }) => {
    const canonicalUrl = computed(() => new URL(route.fullPath, site.url).toString().replace(/\/+$/, ''))
    return {
      link: [
        { rel: 'canonical', href: canonicalUrl },
        {
          rel: 'preload',
          href: '/fonts/LexendDecaVariableFont.woff2',
          as: 'font',
          type: 'font/woff2',
          crossorigin: 'anonymous'
        }
      ],
      style: [
        {
          children:
            "@font-face{font-family:'Lexend Deca';font-style:normal;font-display:swap;font-weight:100 900;src:url('/fonts/LexendDecaVariableFont.woff2') format('woff2')}"
        }
      ],
      script: [
        {
          children: `(${initColorMode.toString()})()`
        }
      ]
    }
  }
})
