import remarkGfm from 'remark-gfm'
import remarkImageSize from './remark/remark-image-size'
import rehypeInlineCode from './rehype/rehype-inline-code'
import rehypeCodeBlock from './rehype/rehype-code-block'

import { highlighter } from './highlighter'

import type { UserConfig } from 'iles'

const config: UserConfig['markdown'] = {
  withImageSrc(src) {
    if (!/\.\.?\//.test(src)) src = `./${src}`
    if (!src.includes('?')) return `${src}?preset=markdown`
  },
  remarkPlugins: [remarkGfm, remarkImageSize],
  rehypePlugins: [
    [rehypeInlineCode, { highlighter }],
    [rehypeCodeBlock, { highlighter }]
  ]
}

export default config
