import { visit } from 'unist-util-visit'

function rehypeInlineCode() {
  return function transformer(tree) {
    visit(tree, 'element', (node, _, parent) => {
      if (node.tagName !== 'code' || parent.tagName !== 'p') return
      node.tagName = 'inlineCode'
    })
  }
}

export default rehypeInlineCode
