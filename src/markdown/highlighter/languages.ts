type Language = typeof languages[number]

const languages = [
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
] as const

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

export { languages, languageNames, type Language }
