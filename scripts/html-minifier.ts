import * as fs from 'fs'
import * as path from 'path'
import glob from 'fast-glob'
import { minify } from 'html-minifier-terser'

const baseDir = process.cwd()

async function htmlMinifier() {
  const htmlGlobPaths = await glob('dist/**/*.html', { cwd: baseDir })

  const minifiedAll = htmlGlobPaths.map(async (htmlGlobPath) => {
    const htmlFilePath = path.join(baseDir, htmlGlobPath)

    const htmlData = await fs.promises
      .readFile(path.join(baseDir, htmlGlobPath), { encoding: 'utf8' })
      // Remove unused attributes
      .then((outData) => outData.replace(/data-head-attrs="[\w\d\s]*"/g, ''))

    const minifiedHtmlData = await minify(htmlData, {
      minifyCSS: true,
      minifyJS: true,
      removeComments: true,
      collapseWhitespace: true
    })

    await fs.promises.writeFile(htmlFilePath, minifiedHtmlData, {
      encoding: 'utf8'
    })
  })

  await Promise.all(minifiedAll)
}

export default htmlMinifier
