import * as path from 'path'
import imageSize from 'image-size'

import remarkGfm from 'remark-gfm'
import remarkImageSize from './remark/remark-image-size'

import rehypeExternalLinks from 'rehype-external-links'
import rehypeInlineCode from './rehype/rehype-inline-code'
import rehypeCodeBlock from './rehype/rehype-code-block'
import rehypeInfoBar from './rehype/rehype-info-bar'

import { highlighter } from './highlighter'
import { categoryNames } from './categories'
import { getPostDateString, isPostValidDate } from '../utils/date'

import type { default as imagesPlugins, ImageAttrs } from '@islands/images'
import type { RawPageMatter, UserConfig } from 'iles'
import type { PostCategory } from '../types/post'

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
    // Only extend frontmatter of markdown files 🥴
    if (!filename.endsWith('.md') || Object.keys(frontmatter).length === 3) return

    validateFrontmatterFields(frontmatter, filename)

    // Extend frontmatter
    await updateImageFrontmatter(images, frontmatter, filename)
    updateDatesFrontmatter(frontmatter)
    updateCategoriesFrontmatter(frontmatter)
  }
}

function validateFrontmatterFields(frontmatter: RawPageMatter, filename: string) {
  const invalidFields: string[] = []

  if (frontmatter.title === undefined) invalidFields.push('title')
  if (frontmatter.seoTitle === undefined) invalidFields.push('seoTitle')
  if (frontmatter.description === undefined) invalidFields.push('description')

  // Validate categories
  if (
    frontmatter.categories === undefined ||
    !Array.isArray(frontmatter.categories) ||
    frontmatter.categories.length === 0
  ) {
    invalidFields.push('categories')
  } else {
    for (const category of frontmatter.categories) {
      if (categoryNames[category] === undefined) {
        invalidFields.push(`categories[${category}]`)
      }
    }
  }

  // Validate image
  if (frontmatter.image === undefined) {
    invalidFields.push('image')
  } else {
    if (frontmatter.image.src === undefined) invalidFields.push('image.src')
    if (frontmatter.image.alt === undefined) invalidFields.push('image.alt')
  }

  // Validate dates
  if (!frontmatter.draft) {
    if (frontmatter.publishedDate === undefined || !isPostValidDate(frontmatter.publishedDate))
      invalidFields.push('publishedDate')

    if (frontmatter.lastUpdated !== undefined && !isPostValidDate(frontmatter.lastUpdated))
      invalidFields.push('lastUpdated')
  }

  if (invalidFields.length > 0) throw new Error(`${filename}: Invalid fields (${invalidFields.join(', ')})`)
}

async function updateImageFrontmatter(
  images: ReturnType<typeof imagesPlugins>,
  frontmatter: RawPageMatter,
  filename: string
) {
  const imageDir = path.dirname(filename)
  const src = path.join(imageDir, frontmatter.image.src)

  const { width, height } = imageSize(path.join(baseDir, src))

  frontmatter.image.srcSets = (await images.api.resolveImage(src, { preset: 'postThumbnail' })) as ImageAttrs[]
  frontmatter.image.src = (await images.api.resolveImage(src, { preset: 'postThumbnail', src: true })) as string
  frontmatter.image.width = width
  frontmatter.image.height = height
  frontmatter.image.style = `max-width: 100%; height: auto; aspect-ratio: ${width}/${height}`

  return frontmatter
}

function updateDatesFrontmatter(frontmatter: RawPageMatter) {
  // Date
  if (frontmatter.publishedDate !== undefined) {
    frontmatter.publishedDate = getPostDateString(frontmatter.publishedDate)
  }

  if (frontmatter.lastUpdated !== undefined) {
    frontmatter.lastUpdated = getPostDateString(frontmatter.lastUpdated)
  }
}

function updateCategoriesFrontmatter(frontmatter: RawPageMatter) {
  const newCategories: PostCategory[] = []

  for (const category of frontmatter.categories) {
    newCategories.push({
      raw: category,
      name: categoryNames[category]
    })
  }

  frontmatter.categories = newCategories
}

export { markdownConfig, extendMarkdownFrontmatter }
