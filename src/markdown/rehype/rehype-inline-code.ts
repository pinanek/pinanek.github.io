import { unified, type Plugin } from 'unified'
import { visit } from 'unist-util-visit'
import { toText } from 'hast-util-to-text'
import rehypeParse from 'rehype-parse'
import type { Highlighter } from '../highlighter'
import type { Element, Parent } from 'hast'

const langRegex = /^{lang\s*=\s*(\w+)}/

const hastParser = unified().use(rehypeParse, { fragment: true })

const rehypeInlineCode: Plugin<
  [
    {
      highlighter: Highlighter
    }
  ]
> = ({ highlighter }) =>
  function transformer(tree) {
    visit(tree, 'element', (node: Element, index: number, parent: Parent) => {
      if (node.tagName !== 'code' || parent.type !== 'element' || (parent as Element).tagName === 'pre') return

      let lang = ''

      if (parent.children.length <= index + 1) {
        lang = 'text'
      } else {
        const nextNode = parent.children[index + 1]

        // Get highlighting lang
        if (nextNode.type !== 'text') return

        if (langRegex.test(nextNode.value)) {
          const [, foundLang] = nextNode.value.match(langRegex) as RegExpMatchArray
          lang = foundLang
        } else {
          lang = 'text'
        }

        // Remove `{lang=???}`
        nextNode.value = nextNode.value.replace(langRegex, '')
      }

      // Highlight code
      const highlightedCode = highlighter.codeToHtml(toText(node), lang)
      const newNodeWrapper = hastParser.parse(highlightedCode).children[0] as Element

      // Replace current node with new inlineCode node
      const newNode = newNodeWrapper.children[0] as Element
      newNode.tagName = 'inlineCode'
      newNode.properties = newNodeWrapper.properties
      parent.children.splice(index, 1, newNode)
    })
  }

export default rehypeInlineCode
