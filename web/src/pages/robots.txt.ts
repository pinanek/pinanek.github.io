import type { APIRoute } from 'astro'

const get: APIRoute = () => ({
  body: `User-agent: *\nAllow: /\n\nSitemap: ${import.meta.env.SITE}sitemap-index.xml\n`
})

export { get }
