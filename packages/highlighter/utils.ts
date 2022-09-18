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

export { escapeHtml }
