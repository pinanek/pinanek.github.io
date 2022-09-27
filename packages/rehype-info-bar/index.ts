import { visit } from 'unist-util-visit'
import { toText } from 'hast-util-to-text'
import type { Plugin } from 'unified'
import type { Root, Element } from 'hast'

const variantRegex = /^(Note|Success|Warning|Error)$/

/**
 * A rehype plugin that can add addition variants for MDX blockquote 😧,
 * using Github blockquote style
 */
const rehypeInfoBar: Plugin<[], Root> = () =>
  function transformer(tree) {
    visit(tree, 'element', (node: Element) => {
      if (node.tagName !== 'blockquote' || node.children.length < 2) {
        return
      }

      // node.children[0] is an text element with value = '\n'
      const firstParagraphNode = node.children[1]
      if (
        firstParagraphNode.type !== 'element' ||
        firstParagraphNode.tagName !== 'p' ||
        firstParagraphNode.children.length === 0
      ) {
        return
      }

      const variantNode = firstParagraphNode.children[0]
      if (variantNode.type !== 'element' || variantNode.tagName !== 'strong') {
        return
      }

      const variant = toText(variantNode)

      if (!variantRegex.test(variant)) {
        return
      }

      let title: string | undefined = undefined
      if (firstParagraphNode.children.length === 2 && firstParagraphNode.children[1].type === 'text') {
        title = toText(firstParagraphNode.children[1]).trim()
      }

      node.children.splice(0, 2)
      node.tagName = 'infobar'

      if (!node.properties) node.properties = {}
      node.properties['variant'] = variant.toLowerCase()

      if (title) node.properties['title'] = title
    })
  }

export default rehypeInfoBar
