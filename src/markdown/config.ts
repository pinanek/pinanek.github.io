import * as path from 'path'
import imageSize from 'image-size'

import remarkGfm from 'remark-gfm'
import remarkImageSize from './remark/remark-image-size'

import rehypeExternalLinks from 'rehype-external-links'
import rehypeInlineCode from './rehype/rehype-inline-code'
import rehypeCodeBlock from './rehype/rehype-code-block'
import rehypeInfoBar from './rehype/rehype-info-bar'

import { highlighter } from './highlighter'
import { getPostDateString } from '../utils/date'

import type { default as imagesPlugins, ImageAttrs } from '@islands/images'
import type { UserConfig } from 'iles'

const baseDir = process.cwd()

const markdownConfig: UserConfig['markdown'] = {
  withImageSrc(src) {
    if (!/\.\.?\//.test(src)) src = `./${src}`
    if (!src.includes('?')) return `${src}?preset=markdown`
  },
  remarkPlugins: [remarkGfm, remarkImageSize],
  rehypePlugins: [
    [rehypeExternalLinks, { target: '_blank', rel: ['nofollow', 'noopener', 'noreferrer'] }],
    [rehypeInlineCode, { highlighter }],
    [rehypeCodeBlock, { highlighter }],
    rehypeInfoBar
  ]
}

function extendMarkdownFrontmatter(images: ReturnType<typeof imagesPlugins>): UserConfig['extendFrontmatter'] {
  return async function extendFrontmatter(frontmatter, filename) {
    if (frontmatter.image !== undefined && frontmatter.image.src !== undefined) {
      const imageDir = path.dirname(filename)
      const src = path.join(imageDir, frontmatter.image.src)

      const { width, height } = imageSize(path.join(baseDir, src))

      frontmatter.image.srcSets = (await images.api.resolveImage(src, { preset: 'postThumbnail' })) as ImageAttrs[]
      frontmatter.image.src = (await images.api.resolveImage(src, { preset: 'postThumbnail', src: true })) as string
      frontmatter.image.width = width
      frontmatter.image.height = height
      frontmatter.image.style = `max-width: 100%; height: auto; aspect-ratio: ${width}/${height}`

      frontmatter.publishedDate = getPostDateString(frontmatter.publishedDate)

      if (frontmatter.lastUpdated !== undefined) {
        frontmatter.lastUpdated = getPostDateString(frontmatter.lastUpdated)
      }
    }
  }
}

export { markdownConfig, extendMarkdownFrontmatter }
