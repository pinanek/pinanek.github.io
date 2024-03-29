import { defineConfig } from "astro/config";
import siteConfig from "./site.config";

import image from "@astrojs/image";
import sitemap from "@astrojs/sitemap";
import htmlMinifier from "./integrations/html-minifier";
import mdx from "@astrojs/mdx";

import remarkGfm from "remark-gfm";
import remarkUnwrapImages from "remark-unwrap-images";
import remarkMdxImages from "./lib/remark/remark-mdx-images";
import remarkCodeblock from "./lib/remark/remark-code-block";
import remarkInlineCode from "./lib/remark/remark-inline-code";

import postcssPresetEnv from "postcss-preset-env";

import { Highlighter } from "./lib/highlighter";

const markdownCodeblockHighlighter = new Highlighter();

export default defineConfig({
  outDir: "_dist",
  site: siteConfig.url,
  markdown: {
    syntaxHighlight: false,
    remarkPlugins: [
      remarkGfm,
      remarkUnwrapImages,
      remarkMdxImages,
      [remarkCodeblock, { highlighter: markdownCodeblockHighlighter }],
      [remarkInlineCode, { highlighter: markdownCodeblockHighlighter }],
    ],
  },
  integrations: [
    mdx(),
    image({
      cacheDir: "_cache/images",
      serviceEntryPoint: "@astrojs/image/sharp",
    }),
    sitemap(),
    htmlMinifier(),
  ],
  experimental: {
    contentCollections: true,
  },

  vite: {
    css: {
      postcss: {
        plugins: [
          postcssPresetEnv({
            autoprefixer: false,
            features: {
              "custom-properties": false,
              "custom-selectors": true,
              "custom-media-queries": {
                importFrom: ["./src/styles/media.css"],
              },
            },
          }),
        ],
      },
    },
  },
});
