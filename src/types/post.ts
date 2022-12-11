import type { MDXInstance } from "astro";

interface RawPostFrontmatter {
  title: string;
  seoTitle: string;
  description: string;
  categories: string[];
  image: {
    src: string;
    alt: string;
    caption?: string;
  };
  publishedDate: string;
  lastUpdated?: string;
}

interface PostFrontmatter extends RawPostFrontmatter {
  image: {
    src: string;
    alt: string;
    width: number;
    height: number;
    caption?: string;
  };
}

type PostData = MDXInstance<PostFrontmatter>;

export type { PostData, PostFrontmatter, RawPostFrontmatter };
