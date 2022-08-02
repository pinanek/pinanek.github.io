import { defineApp } from 'iles'
import { computed } from 'vue'
import { initColorMode } from '@/utils/color-mode'

import Heading2 from '@/components/content/headings/heading-2.vue'
import Heading3 from '@/components/content/headings/heading-3.vue'
import Heading4 from '@/components/content/headings/heading-4.vue'
import Paragraph from '@/components/content/paragraph.vue'
import Link from '@/components/content/link.vue'
import Blockquote from '@/components/content/blockquote.vue'
import Image from '@/components/content/image.vue'
import InlineCode from '@/components/content/code/inline-code.vue'
import CodeBlock from '@/components/content/code/code-block.vue'
import InfoBar from '@/components/content/info-bar/info-bar.vue'
import ListItem from '@/components/content/lists/list-item.vue'
import OrderedList from '@/components/content/lists/ordered-list.vue'
import UnorderedList from '@/components/content/lists/unordered-list.vue'

import '@/styles/global.scss'

export default defineApp({
  socialTags: false,

  mdxComponents: {
    h2: Heading2,
    h3: Heading3,
    h4: Heading4,
    p: Paragraph,
    a: Link,
    blockquote: Blockquote,
    img: Image,
    inlineCode: InlineCode,
    pre: CodeBlock,
    infoBar: InfoBar,
    li: ListItem,
    ol: OrderedList,
    ul: UnorderedList
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
