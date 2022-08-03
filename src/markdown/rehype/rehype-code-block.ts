import { unified, type Plugin } from 'unified'
import { visit } from 'unist-util-visit'
import { toText } from 'hast-util-to-text'
import rehypeParse from 'rehype-parse'
import type { Highlighter } from '../highlighter'
import type { Element, Parent } from 'hast'

const codePropertyRegex = /\b([-\w]+)(?:=(?:"([^"]*)"|'([^']*)'|([^"'\s]+)))?/g

const hastParser = unified().use(rehypeParse, { fragment: true })

const rehypeCodeBlock: Plugin<
  [
    {
      highlighter: Highlighter
    }
  ]
> = ({ highlighter }) =>
  function transformer(tree) {
    visit(tree, 'element', (node: Element, index: number, parent: Parent) => {
      if (
        node.tagName !== 'pre' ||
        node.children.length !== 1 ||
        node.children[0].type !== 'element' ||
        node.children[0].tagName !== 'code'
      )
        return

      const codeNode = node.children[0]

      // Get lang
      let lang = ''
      if (codeNode.properties === undefined || codeNode.properties.className === undefined) {
        lang = 'text'
      } else {
        lang = (codeNode.properties.className as string[])[0].replace('language-', '')
      }

      // Get code and highlight it
      const code = toText(node).slice(0, -1)
      const highlightedCode = highlighter.codeToHtml(code, lang)

      // Set `lang` properties
      const newNode = hastParser.parse(highlightedCode).children[0] as Element
      if (newNode.properties === undefined) newNode.properties = {}
      newNode.properties.lang = lang

      // Set other properties
      if (codeNode.data !== undefined && codeNode.data.meta !== undefined) {
        codePropertyRegex.lastIndex = 0
        let match
        while ((match = codePropertyRegex.exec(codeNode.data.meta as string))) {
          newNode.properties[match[1]] = match[2] || match[3] || match[4] || ''
        }
      }

      parent.children.splice(index, 1, newNode)
    })
  }

export default rehypeCodeBlock
