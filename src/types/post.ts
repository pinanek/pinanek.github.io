import type { MarkdownInstance } from 'astro'

interface PostImage {
  url: string
  alt: string
}

interface PostHeadings {
  id: string
  level: number
  content: string
}

interface PostFrontMatter {
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

type Post = MarkdownInstance<PostFrontMatter>

type PostHeaders = Awaited<ReturnType<Post['getHeadings']>>

export type { Post, PostHeaders, PostFrontMatter }
