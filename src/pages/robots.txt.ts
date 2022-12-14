import type { APIRoute } from "astro";

const get: APIRoute = () => ({
  body: `User-agent: *\nAllow: /\n\nSitemap: ${new URL("sitemap-index.xml", import.meta.env.SITE).href}\n`,
});

export { get };
