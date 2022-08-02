import * as Shiki from 'shiki'

import { languages } from './highlight-languages'

const [, ...supportedLanguages] = languages

const highlighter = await Shiki.getHighlighter({
  theme: 'dark-plus',
  langs: [...supportedLanguages]
})

export { highlighter }
