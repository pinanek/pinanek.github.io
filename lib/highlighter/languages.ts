type Language = keyof typeof languageNames;

/** Names of supported highlight languages */
const languageNames = {
  py: "Python",
  cpp: "C++",
  rs: "Rust",
  csharp: "C#",
  php: "PHP",
  js: "Javascript",
  ts: "Typescript",
  tsx: "TSX",
  bash: "Bash",
  diff: "Diff",
  json: "JSON",
  jsonc: "JSONC",
  yaml: "Yaml",
  text: "Text",
  css: "CSS",
  scss: "SCSS",
  html: "HTML",
  md: "Markdown",
};

const languages = Object.keys(languageNames) as Language[];

function getHighlightLangName(lang: string): string {
  return lang in languageNames ? languageNames[lang as Language] : "text";
}

export { languages, languageNames, getHighlightLangName, type Language };
