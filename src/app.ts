import { defineApp } from 'iles'
import { computed } from 'vue'
import { initColorMode } from '@/utils/color-mode'
import '@/styles/global.scss'

export default defineApp({
  socialTags: false,

  head: ({ frontmatter, site, route }) => {
    const title = computed(() =>
      frontmatter.title === undefined ? site.title : `${frontmatter.title} • ${site.title}`
    )
    const seoTitle = computed(() =>
      frontmatter.seoTitle === undefined ? title.value : `${frontmatter.seoTitle} • ${site.title}`
    )
    const description = computed(() => frontmatter.description ?? site.description)
    const canonicalUrl = computed(() => new URL(route.fullPath, site.url).toString().replace(/\/+$/, ''))
    const imageUrl = computed(() => new URL(site.imageSrc, site.url).toString())

    return {
      title,

      meta: [
        { name: 'description', content: description },

        { property: 'og:site_name', content: site.title },
        { property: 'og:type', content: 'website' },
        { property: 'og:url', content: canonicalUrl },
        { property: 'og:title', content: seoTitle },
        { property: 'og:description', content: description },
        { property: 'og:image', content: imageUrl },
        { property: 'og:image:alt', content: site.imageAlt },

        { name: 'twitter:card', content: 'summary_large_image' },
        { name: 'twitter:url', content: canonicalUrl },
        { name: 'twitter:title', content: seoTitle },
        { name: 'twitter:description', content: description },
        { name: 'twitter:image', content: imageUrl },
        { name: 'twitter:image:alt', content: site.imageAlt }
      ],
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
