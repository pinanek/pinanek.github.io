import rss from "@astrojs/rss";
import siteConfig from "~/site.config";
import type { APIRoute } from "astro";
import { getCollection } from "astro:content";

const get: APIRoute = async () => {
  const posts = await getCollection("posts");

  return rss({
    title: siteConfig.name,
    description: siteConfig.description,
    site: import.meta.env.SITE,
    items: posts.map(({ slug, data: { seoTitle, description, dates } }) => {
      return {
        link: `/posts/${slug}/`,
        description: description,
        title: seoTitle,
        pubDate: new Date(dates.published),
      };
    }),
    stylesheet: "/rss/styles.xsl",
  });
};

export { get };
