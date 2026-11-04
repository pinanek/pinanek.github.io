import { cp, mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { bundle, Features } from "lightningcss";
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

for (const [stylesheet, source] of Object.entries(stylesheets)) {
  const { code } = bundle({
    filename: join(root, "assets/styles", source),
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
