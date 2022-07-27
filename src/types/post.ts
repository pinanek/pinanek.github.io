import type { JSX } from 'solid-js/jsx-runtime'

interface PostImage {
  url: string
  alt: string
}

interface PostHeadings {
  id: string
  level: number
  content: string
}

interface PostFrontmatter {
  title: string
  seoTitle: string
  description: string
  categories: string[]
  image: PostImage
  headings: PostHeadings[]
  isEnablingToc?: boolean
  publishedDate: string
  lastUpdated?: string
}

type Post = {
  frontmatter: PostFrontmatter
  default: JSX.FunctionElement
  file: string
}

export type { Post, PostFrontmatter }
