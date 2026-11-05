import { cp, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

import fontaine from "fontaine/postcss";
import { bundleAsync, Features } from "lightningcss";
import postcss from "postcss";
import { rolldown } from "rolldown";

const root = fileURLToPath(new URL("../", import.meta.url));
const outputDir = join(root, "dist/assets");

// Match the KaTeX assets to the version tracked by katex-rs.
const katexDir = join(root, "node_modules/katex");
await cp(join(katexDir, "dist/fonts"), join(outputDir, "fonts"), { recursive: true });
await cp(join(katexDir, "LICENSE"), join(outputDir, "fonts/katex-LICENSE"));

const javascript = await rolldown({
  cwd: root,
  input: {
    main: "assets/scripts/main.js",
    "photos/main": "assets/scripts/photos/main.js",
    "posts/post": "assets/scripts/prose/main.js",
  },
  platform: "browser",
});

try {
  await javascript.write({
    dir: outputDir,
    entryFileNames: "[name].js",
    chunkFileNames: "[name].[hash].js",
    format: "esm",
    minify: true,
    codeSplitting: true,
  });
} finally {
  await javascript.close();
}

const stylesheets = {
  "main.css": "main.css",
  "prose.css": "prose.css",
  "math.css": "prose/math.css",
  "post-list.css": "post-list.css",
  "home/main.css": "pages/home.css",
  "posts/post.css": "pages/post.css",
  "publications/main.css": "pages/publications.css",
  "photos/main.css": "pages/photos.css",
};

const fontFallbacks = postcss([
  fontaine({
    fallbacks: {},
    resolvePath: (id) => new URL(`../public${id}`, import.meta.url),
  }),
]);

for (const [stylesheet, source] of Object.entries(stylesheets)) {
  const { code } = await bundleAsync({
    filename: join(root, "assets/styles", source),
    resolver: {
      async read(filename) {
        const css = await readFile(filename, "utf8");

        if (normalize(filename) !== join(root, "assets/styles/base/fonts.css")) {
          return css;
        }

        // Fontaine needs quoted font names before Lightning CSS normalizes them.
        const result = await fontFallbacks.process(css, { from: filename, map: false });

        return result.css;
      },
    },
    minify: true,
    drafts: { customMedia: true },
    include:
      Features.FontFamilySystemUi |
      Features.Nesting |
      Features.CustomMediaQueries |
      Features.MediaRangeSyntax,
  });
  const outputPath = join(outputDir, stylesheet);

  await mkdir(dirname(outputPath), { recursive: true });
  await writeFile(outputPath, code);
}
