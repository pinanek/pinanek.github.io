import * as fs from "fs";
import * as path from "path";
import * as minifyHtml from "html-minifier-terser";
import * as os from "os";
import glob from "fast-glob";
import type { AstroIntegration } from "astro";

/** An Astro integration that minifies built output HTML files 🤏 */
function astroHtmlMinifier(): AstroIntegration {
  return {
    name: "@pinanek23/astro-html-minifier",
    hooks: {
      "astro:build:done": async ({ dir }) => {
        // Build output directory
        const outDir = os.platform() === "win32" ? dir.pathname.slice(1) : dir.pathname;

        // Glob all output HTML files
        const relativeHtmlFilePaths = await glob("**/*.html", { cwd: outDir });

        const minifyAll = relativeHtmlFilePaths.map(async (htmlGlobPath) => {
          const filePath = path.join(outDir, htmlGlobPath);

          const fileData = await fs.promises.readFile(filePath, { encoding: "utf8" });

          const minifiedHtmlFile = await minifyHtml.minify(fileData, {
            minifyCSS: true,
            minifyJS: true,
            sortClassName: true,
            sortAttributes: true,
            removeComments: true,
            collapseWhitespace: true,
          });

          await fs.promises.writeFile(filePath, minifiedHtmlFile, { encoding: "utf8" });
        });

        await Promise.all(minifyAll);

        // eslint-disable-next-line no-console
        console.log(
          "%s@pinanek23/astro-html-minifier:%s %s\n",
          "\x1b[32m",
          "\x1b[0m",
          "All output html files are minified.",
        );
      },
    },
  };
}

export default astroHtmlMinifier;
