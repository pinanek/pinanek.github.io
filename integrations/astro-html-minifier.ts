import * as fs from 'fs'
import * as path from 'path'
import * as htmlMinifierTerser from 'html-minifier-terser'
import * as glob from 'fast-glob'
import type { AstroIntegration } from 'astro'

const baseDir = process.cwd()

function astroHtmlMinifier(): AstroIntegration {
  return {
    name: 'astro-html-minifier',
    hooks: {
      'astro:build:done': async function buildDone() {
        const htmlGlobPaths = glob.sync('dist/**/*.html', { cwd: baseDir })

        const minifiedAll = htmlGlobPaths.map(async (htmlGlobPath) => {
          const htmlFilePath = path.join(baseDir, htmlGlobPath)

          const htmlFile = await fs.promises.readFile(path.join(baseDir, htmlGlobPath), { encoding: 'utf8' })

          const minifiedHtmlFile = await htmlMinifierTerser.minify(htmlFile, {
            minifyCSS: true,
            minifyJS: true,
            removeComments: true,
            collapseWhitespace: true
          })

          await fs.promises.writeFile(htmlFilePath, minifiedHtmlFile, { encoding: 'utf8' })
        })

        await Promise.all(minifiedAll)

        // eslint-disable-next-line no-console
        console.log('%sastro-html-minifier:%s %s\n', '\x1b[32m', '\x1b[0m', 'All output html files are minified.')
      }
    }
  }
}

export default astroHtmlMinifier
