import * as fs from 'fs'
import * as path from 'path'
import * as htmlMinifierTerser from 'html-minifier-terser'
import glob from 'fast-glob'
import type { AstroIntegration } from 'astro'

/**
 * An astro integration to minify build output HTMLs 😘
 */
function astroHtmlMinifier(): AstroIntegration {
  return {
    name: '@pinanek23/astro-html-minifier',
    hooks: {
      'astro:build:done': async ({ dir }) => {
        const baseDir = dir.pathname

        // Glob all output HTMLs file
        const globPattern = `**/*.html`
        const htmlGlobPaths = await glob(globPattern, { cwd: baseDir })

        const minifiedAll = htmlGlobPaths.map(async (htmlGlobPath) => {
          const htmlFilePath = path.join(baseDir, htmlGlobPath)

          const htmlFile = await fs.promises.readFile(htmlFilePath, { encoding: 'utf8' })

          const minifiedHtmlFile = await htmlMinifierTerser.minify(htmlFile, {
            minifyCSS: true,
            minifyJS: true,
            sortClassName: true,
            sortAttributes: true,
            removeComments: true,
            collapseWhitespace: true
          })

          await fs.promises.writeFile(htmlFilePath, minifiedHtmlFile, { encoding: 'utf8' })
        })

        await Promise.all(minifiedAll)

        // eslint-disable-next-line no-console
        console.log(
          '%s@pinanek23/astro-html-minifier:%s %s\n',
          '\x1b[32m',
          '\x1b[0m',
          'All output html files are minified.'
        )
      }
    }
  }
}

export default astroHtmlMinifier
