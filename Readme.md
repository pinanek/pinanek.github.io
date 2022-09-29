# pinanek23' personal site

**Work-In-Process** btw 😉.

This is the monorepo source code for my site 😎. [Live site](http://pinanek23.pages.dev).

## Tech stack

- ⚙️ Generator: [Astro](http://astro.build)
- 🎨 Styling: [vanilla-extract](http://vanilla-extract.style)
- 🏝️ Island: [Solid JS](https://www.solidjs.com)
- 📝 Content: [MDX](http://mdxjs.com)
- 🚇 Deployment: [Cloudflare Pages](https://pages.cloudflare.com)

## Run locally

Make sure you have [Node.js](https://nodejs.org) and [pnpm](https://pnpm.io) installed.

```console
pnpm i
pnpm dev
```

## Directory structure

```bash
├── packages                # 📦 All packages which are used by `web`
│   ├── astro-html-minifier # 🚀 An Astro integration is used to minify build output HTMLs
│   ├── highlighter         # ✨ A custom MDX code block highlighter base on Shiki
│   ├── remark-code-block   # 💅 A Remark plugin is used to highlight MDX code block
│   ├── remark-inline-code  # 💅 A Remark plugin is used to highlight MDX inline code
│   └── rehype-info-bar     # 🪧 A Rehype plugin that can add additional variants for MDX blockquote
│
└── web                     # 📦 Main website code
    ├── content             # 📝 Containing `.mdx` posts
    ├── public              # 🖼️ Fonts, SEO images,...
    └── src
        ├── components      # 🧩 Site components
        ├── constants       # 🗿 Global constants
        ├── layouts         # 📏 Components that wrap page content
        ├── pages           # 📄 Components are used to create pages
        ├── styles          # 🎨 Global styles
        ├── types           # 🦺 Common typescript types
        └── utils           # 🛠️ Utils functions
```
