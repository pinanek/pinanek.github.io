import { visit } from "unist-util-visit";
import type { Highlighter } from "~/lib/highlighter";
import type { Root, Code, Parent, HTML } from "mdast";
import type { Plugin } from "unified";

interface RemarkCodeBlockOptions {
  highlighter: Highlighter;
}

const codePropertyRegex = /\b([-\w]+)(?:=(?:"([^"]*)"|'([^']*)'|([^"'\s]+)))?/g;

/**
 * A remark plugin that renders MDX code block using custom `shiki` syntax highlighter.
 * @param options User options
 */
const remarkCodeBlock: Plugin<[RemarkCodeBlockOptions], Root> = ({ highlighter }) =>
  function transformer(tree) {
    visit(tree, "code", (node: Code, index: number | null, parent: Parent | null) => {
      // Make Typescript happy 😏
      if (index === null || parent === null) {
        return;
      }

      // Fallback to `text` if `lang` doesn't exist
      if (!node.lang) {
        node.lang = "text";
      }

      // Get code meta data
      const meta: Record<string, unknown> = {
        language: node.lang,
      };

      // Get meta data (file name, ...)
      if (node.meta) {
        let match;
        codePropertyRegex.lastIndex = 0;

        while ((match = codePropertyRegex.exec(node.meta))) {
          meta[match[1]] = match[2] || match[3] || match[4] || "";
        }
      }

      const highlightedCode = highlighter.codeToHtml({
        code: node.value,
        lang: node.lang,
        renderOptions: {
          wrapperTag: "pre",
          wrapperAttributes: meta,
        },
      });

      const newNode: HTML = {
        type: "html",
        value: highlightedCode,
      };

      parent.children.splice(index, 1, newNode);
    });
  };

export default remarkCodeBlock;
