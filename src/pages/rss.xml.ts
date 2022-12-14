import rss from "@astrojs/rss";
import siteConfig from "~/site.config";
import type { APIRoute } from "astro";
import type { MarkdownInstance } from "astro";
import type { PostFrontmatter } from "@/types/post";

const postsImportResult = import.meta.glob<MarkdownInstance<PostFrontmatter>>("/content/posts/**/index.mdx", {
  eager: true,
});

const posts = Object.values(postsImportResult);

const get: APIRoute = () =>
  rss({
    title: siteConfig.name,
    description: siteConfig.description,
    site: import.meta.env.SITE,
    items: posts.map(({ file, frontmatter: { seoTitle, description, dates } }) => {
      const slug = file
        .replace(/\/index.mdx$/, "")
        .split(/[/\\]/)
        .pop();

      return {
        link: `/posts/${slug}/`,
        description: description,
        title: seoTitle,
        pubDate: new Date(dates.published),
      };
    }),
    stylesheet: "/rss/styles.xsl",
  });

export { get };
