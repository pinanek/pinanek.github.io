import type { ImageAttrs } from '@islands/images'

interface PostImage {
  src: string
  alt: string
  srcSets: ImageAttrs[]
  width: number
  height: number
  style: string
}

interface PostFrontmatter {
  title: string
  seoTitle: string
  description: string
  categories: string[]
  image: PostImage
  isEnablingToc?: boolean
  publishedDate: string
  lastUpdated?: string
}

export type { PostFrontmatter }
