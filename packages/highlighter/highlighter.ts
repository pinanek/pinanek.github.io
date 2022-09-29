import * as Shiki from 'shiki'
import { defaultLanguages, type Language } from './languages'
import { colorReplacements, kenanipTheme } from './kenanip-theme'
import { escapeHtml } from './utils'

interface HighlighterOptions {
  /** Highlight theme */
  theme?: Shiki.Theme | 'kenanip'

  /** Highlight languages */
  langs?: Language[]
}

interface CodeToHtmlOptions {
  /** Input code string */
  code: string

  /** Input code language */
  lang: string

  /** Render customization */
  renderOptions: {
    /**
     * Wrapper tag of render codeblock.
     * Default: `pre`
     */
    wrapperTag?: string

    /**
     * Attributes of wrapper element of render codeblock.
     * Default: `undefined`
     */
    wrapperAttributes?: Record<string, unknown>

    /**
     * Wrapper tag of render codeblock.
     * Default: `code`
     */
    codeTag?: string
  }
}

class Highlighter {
  /** Internal Shiki instance is used to render */
  private shikiInstance!: Shiki.Highlighter

  constructor({ theme = 'kenanip', langs = defaultLanguages }: HighlighterOptions) {
    langs = langs.filter((lang) => lang !== 'text')

    const shihiTheme = theme === 'kenanip' ? kenanipTheme : theme

    Shiki.getHighlighter({ theme: shihiTheme, langs: langs as Shiki.Lang[] }).then(
      (highlighter) => (this.shikiInstance = highlighter)
    )
  }

  /**
   * Render a code string to HTML
   * @param option Render code options
   */
  public codeToHtml({
    code,
    lang,
    renderOptions: { wrapperTag = 'pre', wrapperAttributes, codeTag = 'code' }
  }: CodeToHtmlOptions) {
    const tokens = this.shikiInstance.codeToThemedTokens(code, lang, undefined, {
      includeExplanation: false
    })

    let highlightedCode = ''
    for (const lineTokens of tokens) {
      let line = ''

      for (const token of lineTokens) {
        const out = this.tokenToHtml(token, colorReplacements)
        line += out
      }

      highlightedCode += `<span class="line">${line}</span>\n`
    }

    // Remove last line newline
    highlightedCode = highlightedCode.replace(/\n$/, '')

    // Add user-input wrapper attributes

    let additionalAttributes = ''

    if (wrapperAttributes !== undefined && Object.entries(wrapperAttributes).length !== 0) {
      additionalAttributes =
        Object.entries(wrapperAttributes)
          .map(([attribute, value]) => {
            return `${attribute}="${value}"`
          })
          .join(' ') + ' '
    }

    const attributes =
      additionalAttributes + `style="background-color: var(--color-shiki-${colorReplacements['#000']})"`

    if (wrapperTag === 'none') {
      return `<${codeTag} ${attributes}>${highlightedCode}</${codeTag}>`
    }

    return `<${wrapperTag} ${attributes}><${codeTag}>${highlightedCode}</${codeTag}></${wrapperTag}>`
  }

  private tokenToHtml(token: Shiki.IThemedToken, colorReplacements: Record<string, string>) {
    let style = ''

    if (token.color === undefined) token.color = '#001'
    const variable = colorReplacements[token.color]
    style += `color: var(--color-shiki-${variable});`

    if (token.fontStyle !== undefined && token.fontStyle !== 0) style += `font-style: ${token.fontStyle};`

    const content = escapeHtml(token.content)

    const styleAttribute = style === '' ? '' : ` style="${style}"` // Add a space

    return `<span${styleAttribute}>${content}</span>`
  }
}

export { Highlighter }
