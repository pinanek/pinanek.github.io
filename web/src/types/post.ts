import type { MDXInstance } from 'astro'

interface PostFrontmatter {
  title: string
  seoTitle: string
  description: string
  categories: string[]
  imageSrc: string
  imageAlt: string
  isEnablingToc?: boolean
  publishedDate: string
  lastUpdated?: string
}

type PostData = MDXInstance<PostFrontmatter>

export type { PostData, PostFrontmatter }
