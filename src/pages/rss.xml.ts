import rss from '@astrojs/rss'
import type { APIRoute } from 'astro'
import type { Post } from '@/types/post'

const postsImportResult = import.meta.glob<Post>('/src/content/posts/{*,**/index}.mdx', { eager: true })
const posts = Object.values(postsImportResult)

export const get: APIRoute = () =>
  rss({
    title: "Pinanek23's blog",
    description: 'My little garden 🌻 with knowledge and experience of what I have gained 😁',
    site: import.meta.env.SITE,
    items: posts.map(({ url, frontmatter: { seoTitle, description, publishedDate } }) => {
      const slug = url
        .replace(/\/index.mdx$/, '')
        .split(/[/\\]/)
        .pop()

      return {
        link: `/posts/${slug}/`,
        description: description,
        title: seoTitle,
        pubDate: new Date(publishedDate)
      }
    })
  })
