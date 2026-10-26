import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { bundle, Features } from "lightningcss";
import { rolldown } from "rolldown";

const root = fileURLToPath(new URL("../", import.meta.url));
const outputDir = join(root, "dist/assets");

const javascript = await rolldown({
  cwd: root,
  input: {
    main: "assets/scripts/main.ts",
    "photos/main": "assets/scripts/photos/main.ts",
    "posts/post": "assets/scripts/posts/post.ts",
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

const stylesheets = [
  "main.css",
  "home/main.css",
  "posts/post.css",
  "publications/main.css",
  "photos/main.css",
  "posts/posts.css",
  "uses/main.css",
  "tags/main.css",
  "colophon/main.css",
];

for (const stylesheet of stylesheets) {
  const { code } = bundle({
    filename: join(root, "assets/styles", stylesheet),
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
