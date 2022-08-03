import * as Shiki from 'shiki'
import { languages } from './languages'
import { theme, colorReplacements } from './theme'

// Create a custom highlighter
interface Highlighter {
  codeToHtml: (code: string, lang: string) => string
}

const highlighter: Highlighter = {
  codeToHtml: highlightCode
}

const [, ...supportedLanguages] = languages

const shikiHighlighter = await Shiki.getHighlighter({
  theme,
  langs: [...supportedLanguages]
})

// https://github.com/shikijs/shiki/blob/4b585d4f8eb54b804d95e2c31b29d32bee89208a/packages/shiki/src/renderer.ts#L46
const htmlEscapes: Record<string, string> = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
  "'": '&#39;'
}

function escapeHtml(html: string) {
  return html.replace(/[&<>"']/g, (chr) => htmlEscapes[chr])
}

function highlightCode(code: string, lang: string) {
  const tokens = shikiHighlighter.codeToThemedTokens(code, lang, undefined, {
    includeExplanation: false
  })

  let highlightedCode = ''
  for (const lineTokens of tokens) {
    let line = ''

    for (const token of lineTokens) {
      const out = tokenToHtml(token, colorReplacements)
      line += out
    }

    highlightedCode += `<span class="line">${line}</span>\n`
  }

  return `<pre style="background-color: var(--shiki-${colorReplacements['#000']})"><code>${highlightedCode}</code></pre>`
}

function tokenToHtml(token: Shiki.IThemedToken, colorReplacements: Record<string, string>) {
  let style = ''

  if (token.color === undefined) token.color = '#001'
  const variable = colorReplacements[token.color]
  style += `color: var(--shiki-${variable});`

  if (token.fontStyle !== undefined && token.fontStyle !== 0) style += `font-style: ${token.fontStyle};`

  const content = escapeHtml(token.content)

  return style === '' ? `<span>${content}</span>` : `<span style="${style}">${content}</span>`
}

export { highlighter, type Highlighter }
