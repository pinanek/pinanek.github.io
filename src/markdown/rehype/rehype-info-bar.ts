import { visit } from 'unist-util-visit'
import { toText } from 'hast-util-to-text'
import type { Plugin } from 'unified'
import type { Element } from 'hast'

const variantRegex = /^(Note|Success|Warning|Error)$/

const rehypeInfoBar: Plugin<[]> = () => {
  return function transformer(tree) {
    visit(tree, 'element', (node: Element) => {
      if (
        node.tagName !== 'blockquote' ||
        node.children.length <= 1 ||
        node.children[1].type !== 'element' ||
        node.children[1].children.length === 0
      )
        return

      const firstNodeOfParagraphNode = node.children[1].children[0]

      if (firstNodeOfParagraphNode.type !== 'element' || firstNodeOfParagraphNode.tagName !== 'strong') return

      const variant = toText(firstNodeOfParagraphNode)
      if (!variantRegex.test(variant)) return

      let title = undefined
      if (node.children[1].children.length > 1) {
        node.children[1].children.splice(0, 1)
        title = toText(node.children[1]).trim()
      }

      if (variantRegex.test(variant)) {
        node.children.splice(0, 2)
        node.tagName = 'infoBar'
        node.properties = {
          title,
          variant: variant.toLowerCase()
        }
      }
    })
  }
}

export default rehypeInfoBar
