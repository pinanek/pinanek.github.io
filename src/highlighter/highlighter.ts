import * as shiki from "shiki";
import { languages } from "./languages";
import { colors, kenanipTheme } from "./kenanip-theme";
import { escapeHtml } from "./utils";

interface CodeToHtmlOptions {
  /** Input code string */
  code: string;

  /** Input code language */
  lang: string;

  /** Render customization */
  renderOptions: {
    /**
     * Wrapper tag of render codeblock.
     * Default: `pre`
     */
    wrapperTag?: string;

    /**
     * Attributes of wrapper element of render codeblock.
     * Default: `undefined`
     */
    wrapperAttributes?: Record<string, unknown>;

    /**
     * Wrapper tag of render codeblock.
     * Default: `code`
     */
    codeTag?: string;
  };
}

class Highlighter {
  /** Internal Shiki instance is used to render */
  private shikiInstance!: shiki.Highlighter;

  constructor() {
    const langs = languages.filter((lang) => lang !== "text") as shiki.Lang[];

    shiki.getHighlighter({ theme: kenanipTheme, langs }).then((highlighter) => (this.shikiInstance = highlighter));
  }

  /**
   * Render a code string to HTML
   * @param option Render code options
   */
  public codeToHtml({
    code,
    lang,
    renderOptions: { wrapperTag = "pre", wrapperAttributes, codeTag = "code" },
  }: CodeToHtmlOptions) {
    const tokens = this.shikiInstance.codeToThemedTokens(code, lang, undefined, {
      includeExplanation: false,
    });

    let highlightedCode = "";
    for (const lineTokens of tokens) {
      let line = "";

      for (const token of lineTokens) {
        const out = this.tokenToHtml(token);
        line += out;
      }

      highlightedCode += `<span class="line">${line}</span>\n`;
    }

    // Remove last line newline
    highlightedCode = highlightedCode.replace(/\n$/, "");

    // Add user-input wrapper attributes

    let additionalAttributes = "";

    if (wrapperAttributes !== undefined && Object.entries(wrapperAttributes).length !== 0) {
      additionalAttributes =
        Object.entries(wrapperAttributes)
          .map(([attribute, value]) => {
            return `${attribute}="${value}"`;
          })
          .join(" ") + " ";
    }

    const attributes = additionalAttributes + `style="background-color: ${colors["#000"]};"`;

    if (wrapperTag === "none") {
      return `<${codeTag} ${attributes}>${highlightedCode}</${codeTag}>`;
    }

    return `<${wrapperTag} ${attributes}><${codeTag}>${highlightedCode}</${codeTag}></${wrapperTag}>`;
  }

  private tokenToHtml(token: shiki.IThemedToken) {
    let style = "";

    if (token.color === undefined) token.color = "#001";
    style += `color: ${colors[token.color]};`;

    if (token.fontStyle !== undefined && token.fontStyle !== 0) style += `font-style: ${token.fontStyle};`;

    const content = escapeHtml(token.content);

    const styleAttribute = style === "" ? "" : ` style="${style}"`; // Add a space

    return `<span${styleAttribute}>${content}</span>`;
  }
}

export { Highlighter };
