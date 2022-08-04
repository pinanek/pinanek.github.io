function getSlugByFileName(filename: string) {
  return filename
    .replace(/\/index.mdx?$/, '')
    .split(/[/\\]/)
    .pop() as string
}

export { getSlugByFileName }
