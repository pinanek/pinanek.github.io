import { z, defineCollection } from "astro:content";

const posts = defineCollection({
  slug(entry) {
    return entry.id.replace(/\/index.mdx$/, "");
  },

  schema: {
    title: z.string(),
    seoTitle: z.string(),
    description: z.string(),
    categories: z.array(z.string()),
    image: z.object({
      src: z.string(),
      alt: z.string(),
      caption: z.string(),
      width: z.optional(z.number()).default(0),
      height: z.optional(z.number()).default(0),
    }),
    dates: z.object({
      published: z.string(),
      updated: z.optional(z.string()),
    }),
    tableOfContent: z.optional(z.boolean()),
  },
});

const collections = {
  posts,
};

export { collections };
