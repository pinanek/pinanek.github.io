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

interface Post
  extends Omit<MarkdownInstance<PostFrontmatter>, 'default' | 'url' | 'compiledContent' | 'rawContent' | 'Content'> {
  url: string
  default: ({ components }: { components?: Record<string, unknown> }) => astroHTML.JSX.Element
}

export type { Post, PostFrontmatter }
