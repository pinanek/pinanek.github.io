// Current @astro/mdx integration isn't implemented all feature of legacy astro markdown
// This is used to improve glob result of mdx files 😎
function astroMdxImprovement() {
  /** @type {import('astro').AstroIntegration} */
  const integration = {
    name: 'astro-mdx-improvement',
    hooks: {
      'astro:config:setup': ({ updateConfig }) => {
        updateConfig({
          vite: {
            plugins: [
              {
                name: 'astro-mdx-improvement',
                transform(code, id) {
                  if (!id.endsWith('.mdx')) return

                  // Add file property
                  code += `\nexport const file = "${id}";`

                  return code
                }
              }
            ]
          }
        })
      }
    }
  }

  return integration
}

export default astroMdxImprovement
