import { visit } from 'unist-util-visit'
import type { MDXTextExpression } from 'mdast-util-mdx-expression'
import type { Highlighter } from '@pinanek23/highlighter'
import type { Root, Parent, HTML, InlineCode, Content } from 'mdast'
import type { Plugin } from 'unified'

interface RemarkInlineCodeOptions {
  /** `@pinanek23/highlighter` instance */
  highlighter: Highlighter
}

const langRegex = /lang=\w+/

/**
 * A remark plugin that renders MDX inline code using using `@pinanek23/highlighter`
 * @param options User options
 */
const remarkInlineCode: Plugin<[RemarkInlineCodeOptions], Root> = ({ highlighter }) =>
  function transformer(tree) {
    visit(tree, 'inlineCode', (node: InlineCode, index: number | null, parent: Parent | null) => {
      // Make Typescript happy 😏
      if (index === null || parent === null) {
        return
      }

      let lang = 'text'

      if (parent.children.length > index + 1) {
        const nextNode = parent.children[index + 1] as Content | MDXTextExpression

        if (nextNode.type === 'mdxTextExpression' && langRegex.test(nextNode.value)) {
          lang = nextNode.value.replace('lang=', '')
          parent.children.splice(index + 1, 1)
        }
      }

      const highlightedCode = highlighter.codeToHtml({
        code: node.value,
        lang,
        renderOptions: {
          wrapperTag: 'none',
          codeTag: 'inlinecode'
        }
      })

      const newNode: HTML = {
        type: 'html',
        value: highlightedCode
      }

      parent.children.splice(index, 1, newNode)
    })
  }

export default remarkInlineCode
