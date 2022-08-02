import * as fs from 'fs'
import * as path from 'path'
import glob from 'fast-glob'
import { minify } from 'minify-xml'

const baseDir = process.cwd()

async function xmlMinifier() {
  const xmlGlobPaths = await glob('dist/**/*.xml', { cwd: baseDir })

  const minifiedAll = xmlGlobPaths.map(async (xmlGlobPath) => {
    const xmlFilePath = path.join(baseDir, xmlGlobPath)

    const xmlData = await fs.promises.readFile(path.join(baseDir, xmlGlobPath), { encoding: 'utf8' })

    const minifiedXmlData = minify(xmlData)

    await fs.promises.writeFile(xmlFilePath, minifiedXmlData, { encoding: 'utf8' })
  })

  await Promise.all(minifiedAll)
}

export default xmlMinifier
