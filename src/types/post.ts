interface PostImage {
  url: string
  alt: string
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
