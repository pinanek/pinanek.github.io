// Slightly modified: https://github.com/remcohaszing/remark-mdx-images/blob/main/index.ts

import { visit } from "unist-util-visit";
import type { Plugin } from "unified";
import type { Root } from "mdast";
import type { MdxjsEsm, MdxJsxTextElement } from "mdast-util-mdx";

const urlPattern = /^(https?:)?\//;
const relativePathPattern = /\.\.?\//;

const remarkMdxImages: Plugin<[], Root> = () => {
  const imports: MdxjsEsm[] = [];
  const imported = new Map<string, string>();

  return function transformer(tree) {
    visit(tree, "image", (node, index, parent) => {
      const { alt = null, title } = node;
      let { url } = node;

      if (index === null || parent === null) return;

      if (urlPattern.test(node.url) || node.url.startsWith("/")) return;

      if (!relativePathPattern.test(url)) {
        url = `./${url}`;
      }

      let name = imported.get(url);
      if (!name) {
        name = `__${imported.size}_${url.replace(/\W/g, "_")}__`;

        imports.push({
          type: "mdxjsEsm",
          value: "_not_used_",
          data: {
            estree: {
              type: "Program",
              sourceType: "module",
              body: [
                {
                  type: "ImportDeclaration",
                  source: { type: "Literal", value: url, raw: JSON.stringify(url) },
                  specifiers: [
                    {
                      type: "ImportDefaultSpecifier",
                      local: { type: "Identifier", name },
                    },
                  ],
                },
              ],
            },
          },
        });

        imported.set(url, name);
      }

      const textElement: MdxJsxTextElement = {
        type: "mdxJsxTextElement",
        name: "img",
        children: [],
        attributes: [
          { type: "mdxJsxAttribute", name: "alt", value: alt },
          {
            type: "mdxJsxAttribute",
            name: "data",
            value: {
              type: "mdxJsxAttributeValueExpression",
              value: name,
              data: {
                estree: {
                  type: "Program",
                  sourceType: "module",
                  comments: [],
                  body: [{ type: "ExpressionStatement", expression: { type: "Identifier", name } }],
                },
              },
            },
          },
        ],
      };

      if (title) {
        textElement.attributes.push({ type: "mdxJsxAttribute", name: "caption", value: title });
      }
      parent.children.splice(index, 1, textElement);
    });

    tree.children.unshift(...imports);
  };
};

export default remarkMdxImages;
