import { visit } from 'unist-util-visit'
import type { Plugin } from 'unified'
import type { Element, Parent } from 'hast'

const rehypeInlineCode: Plugin<[]> = () =>
  function transformer(tree) {
    visit(tree, 'element', (node: Element, _, parent: Parent) => {
      if (node.tagName !== 'code' || parent.type !== 'element' || (parent as Element).tagName !== 'p') return
      node.tagName = 'inlineCode'
    })
  }

export default rehypeInlineCode
