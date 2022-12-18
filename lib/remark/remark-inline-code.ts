import { visit } from "unist-util-visit";
import type { Highlighter } from "~/lib/highlighter";
import type { Root, Parent, HTML, InlineCode } from "mdast";
import type { Plugin } from "unified";

interface RemarkInlineCodeOptions {
  highlighter: Highlighter;
}

const langRegex = /^({lang=(\w+)}).+$/;

/**
 * A remark plugin that renders MDX inline code using using `shiki` syntax highlighter.
 * @param options User options
 */
const remarkInlineCode: Plugin<[RemarkInlineCodeOptions], Root> = ({ highlighter }) =>
  function transformer(tree) {
    visit(tree, "inlineCode", (node: InlineCode, index: number | null, parent: Parent | null) => {
      // Make Typescript happy 😏
      if (index === null || parent === null) {
        return;
      }

      let lang = "text";

      if (parent.children.length > index + 1) {
        const nextNode = parent.children[index + 1];

        if (nextNode.type === "text" && langRegex.test(nextNode.value)) {
          const matches = nextNode.value.match(langRegex) as RegExpMatchArray;

          lang = (nextNode.value.match(langRegex) as RegExpMatchArray)[2];
          nextNode.value = nextNode.value.replace(matches[1], "");
        }

        // MDX
        // if (nextNode.type === "mdxTextExpression" && langRegex.test(nextNode.value)) {
        //   lang = nextNode.value.replace("lang=", "");
        //   parent.children.splice(index + 1, 1);
        // }
      }

      const highlightedCode = highlighter.codeToHtml({
        code: node.value,
        lang,
        renderOptions: {
          wrapperTag: "none",
          codeTag: "inline-code",
        },
      });

      const newNode: HTML = {
        type: "html",
        value: highlightedCode,
      };

      parent.children.splice(index, 1, newNode);
    });
  };

export default remarkInlineCode;
