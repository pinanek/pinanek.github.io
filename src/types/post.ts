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
  dates: {
    published: string;
    updated?: string;
  };
  tableOfContent?: boolean;
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
