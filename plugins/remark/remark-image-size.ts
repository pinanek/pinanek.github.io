import * as path from 'path'
import imageSize from 'image-size'
import { visit } from 'unist-util-visit'
import type { Plugin } from 'unified'
import type { MdxJsxAttribute, MdxJsxAttributeValueExpression, MdxJsxTextElement } from 'mdast-util-mdx-jsx'
import type { MDXJSEsm } from 'mdast-util-mdxjs-esm'
import type { ExpressionStatement, Identifier, ImportDeclaration } from 'estree'

const remarkImageSize: Plugin<[]> = () =>
  function transformer(tree, vFile) {
    const imageSrcs = new Map<string, string>()

    visit(tree, 'mdxjsEsm', (node: MDXJSEsm) => {
      if (node.value !== '_not_used_') return
      if (node.data === undefined || node.data.estree === undefined || node.data.estree.body === undefined) return

      const importBody = node.data.estree.body
      if (!Array.isArray(importBody) || importBody.length !== 1) return
      const importDeclaration = importBody[0] as ImportDeclaration

      if (Array.isArray(importDeclaration.specifiers) && importDeclaration.specifiers.length !== 1) return
      const name = importDeclaration.specifiers[0].local.name
      const src = importDeclaration.source.value as string
      imageSrcs.set(name, src)
    })

    if (imageSrcs.size === 0) return

    visit(tree, 'mdxJsxTextElement', (node: MdxJsxTextElement) => {
      if (node.name !== 'img') return

      let importName = ''

      ;(node.attributes as MdxJsxAttribute[]).forEach((attribute) => {
        if (attribute.name === 'src') {
          const importBody = (attribute.value as MdxJsxAttributeValueExpression).data?.estree?.body

          if (!Array.isArray(importBody) || importBody.length !== 1) return
          importName = ((importBody[0] as ExpressionStatement).expression as Identifier).name
        }
      })

      const src = imageSrcs.get(importName)
      if (src === undefined) return

      const imagePath = path.join(path.dirname(vFile.history[0]), src.replace(/\?preset=markdown$/g, ''))
      const { width, height } = imageSize(imagePath)

      if (width === undefined || height === undefined) return

      node.attributes.push({
        type: 'mdxJsxAttribute',
        name: 'width',
        value: width.toString()
      })

      node.attributes.push({
        type: 'mdxJsxAttribute',
        name: 'height',
        value: height.toString()
      })

      node.attributes.push({
        type: 'mdxJsxAttribute',
        name: 'style',
        value: `max-width: 100%; height: auto; aspect-ratio: ${width}/${height}`
      })
    })
  }

export default remarkImageSize
