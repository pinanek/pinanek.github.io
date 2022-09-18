/** Supported highlight languages */
type Language =
  | 'text'
  | 'py'
  | 'cpp'
  | 'csharp'
  | 'php'
  | 'js'
  | 'ts'
  | 'tsx'
  | 'bash'
  | 'diff'
  | 'json'
  | 'jsonc'
  | 'yaml'
  | 'css'
  | 'scss'
  | 'md'

const defaultLanguages: Language[] = [
  'text',
  'py',
  'cpp',
  'csharp',
  'php',
  'js',
  'ts',
  'tsx',
  'bash',
  'diff',
  'json',
  'jsonc',
  'yaml',
  'css',
  'scss',
  'md'
]

/** Names of supported highlight languages */
const languageNames: Record<Language, string> = {
  py: 'Python',
  cpp: 'C++',
  csharp: 'C#',
  php: 'PHP',
  js: 'Javascript',
  ts: 'Typescript',
  tsx: 'TSX',
  bash: 'Bash',
  diff: 'Diff',
  json: 'JSON',
  jsonc: 'JSONC',
  yaml: 'Yaml',
  text: 'Text',
  css: 'CSS',
  scss: 'SCSS',
  md: 'Markdown'
}

function getHighlightLangName(lang: string): string {
  return lang in languageNames ? languageNames[lang as Language] : 'text'
}

export { defaultLanguages, languageNames, getHighlightLangName, type Language }
