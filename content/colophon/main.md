+++
title = "Colophon"
description = "How I build this site."
+++

I write this site in Markdown and turn it into pages with a Rust program I wrote. JavaScript and CSS have their own build. If you’re curious about how those pieces become the page you’re reading, here’s how I put it together.

## The stack

There’s no framework here. My [Rust](https://rust-lang.org/) generator reads Markdown and data files, renders the pages, and writes a static site to `dist/`.

I use [maud](https://github.com/lambda-fairy/maud) for HTML templates and [comrak](https://comrak.ee/) for Markdown. On the Node.js side, [Rolldown](https://rolldown.rs/) builds the browser scripts and [Lightning CSS](https://lightningcss.dev/) handles the stylesheets.

Code highlighting comes from [Tree-sitter](https://tree-sitter.github.io/tree-sitter/), the parser you might know from editors like [Neovim](https://neovim.io/) and [Helix](https://helix-editor.com/). The [image](https://github.com/image-rs/image) crate handles image processing.

The Rust code is split into a `site` crate for the generator and an `xtask` crate for maintaining syntax queries. Both use the manifest types in `site_internal`. GitHub Actions takes care of building the site and deploying it to GitHub Pages.

Each post gets a directory under `content/posts/`, with a `main.md` file and its images together. At the top of the file, TOML front matter holds the title, description, dates, and tags. Pages like this one and Uses are Markdown too. Publications come from a TOML data file.

At build time, the generator turns that content into pages and tag archives, then writes minified HTML, an RSS feed, and a sitemap. It also generates the images and copies the public files into the output directory. GitHub Pages serves those files directly. The Rust program has already done its job by then, so it doesn’t need to run on the server.

## Markdown

I liked the idea behind [MDX](https://github.com/mdx-js/mdx) in the JavaScript world: write in Markdown, but use custom components where you need them. I wanted headings with anchor links, images with responsive sources and placeholders, and code blocks with highlighting and copy buttons.

Two popular Rust options are [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) and `comrak`. `pulldown-cmark` gives you an iterator of events without allocating a whole syntax tree. It’s fast, but I found it awkward when I wanted to replace an entire element. To handle a blockquote, for example, I’d have to collect and process the events between its start and end.

`comrak` builds an abstract syntax tree (AST), so I can work with whole elements. I walk the document and prepare replacements for the nodes I want to customize:

```rs title="markdown.rs"
for node in document.descendants() {
    let data = node.data();

    match &data.value {
        NodeValue::Heading(heading) => prepare_heading(node, heading, &mut context)?,
        NodeValue::Link(link) => prepare_link(node, link, &mut context)?,
        NodeValue::CodeBlock(code_block) => prepare_code_block(node, code_block, &mut context)?,
        NodeValue::Paragraph => prepare_figure(node, &mut context)?,
        NodeValue::Alert(alert) => prepare_alert(node, alert, &mut context)?,
        NodeValue::FootnoteReference(footnote_ref) => {
            pepare_footnote_ref(node, footnote_ref, &mut context)?
        }
        NodeValue::FootnoteDefinition(footnote_def) => {
            prepare_footnote_def(node, footnote_def, &mut context)?
        }
        _ => (),
    };
}
```

So `comrak` handles most of the work, which makes my life much easier. I just replace the relevant nodes with custom `maud` components and let it render the rest.

Nested elements need a little care. An alert can contain links or code blocks that also need replacing, so I render those children before wrapping them in the alert component. I collect footnotes and add them at the end, and use the headings to build a table of contents at the beginning.

When I want to show a filename above a code block, I put it in the fence’s `title` attribute. The `lines` attribute lets me highlight particular lines without adding markup to the code itself. Math expressions go through `katex-rs` during the build, and only pages with math load the extra stylesheet.

## Templating

Yeah, once again inspired by the JavaScript world, I wanted something like JSX :). And `maud` fits my needs perfectly.

Here, `main_layout` wraps the page content. The `html!` macro produces HTML, and `()` inserts a value or another component. Personally, I find it less verbose and easier to read than JSX.

```rs title="uses.rs"
main_layout(
    &self.metadata,
    html! {
        article class="prose" {
            (PreEscaped(&self.rendered))
        }
    },
    html! {
        link rel="stylesheet" href="/assets/prose.css";
    },
    html! {},
)
```

Layouts are ordinary Rust functions too. The base layout adds the document head, navigation, and footer. Each page passes in its content, metadata, and any extra stylesheets or scripts it needs. I don’t have a separate template language to configure, and reusing a component is just a function call.

Changing a template means recompiling the Rust generator before I can see the result. Markdown and stylesheet changes don’t need that step, though I still have to rerun the generator or asset build.

It’s a bit annoying, but I can live with it :P.

## JavaScript and CSS

I call Rolldown, the Rust-based bundler behind [Vite](https://vite.dev/), through its JavaScript API. The JavaScript and CSS builds both run in `scripts/build-assets.js`, keeping the bundlers out of the site’s Cargo dependency graph.

The JavaScript setup is pretty simple: provide the entry points, choose the output names, and enable minification and code splitting. Here’s that part of the script:

```javascript title="build-assets.js"
const javascript = await rolldown({
  cwd: root,
  input: {
    main: "assets/scripts/main.js",
    "photos/main": "assets/scripts/photos/main.js",
    "posts/post": "assets/scripts/prose/main.js",
  },
  platform: "browser",
});

try {
  await javascript.write({
    dir: outputDir,
    entryFileNames: "[name].js",
    chunkFileNames: "[name].[hash].js",
    format: "esm",
    minify: true,
    codeSplitting: true,
  });
} finally {
  await javascript.close();
}
```

Lightning CSS bundles and minifies the stylesheets. It also lets me use features such as [custom media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@custom-media), alongside nesting and media range syntax.

Most of the browser code is for the things you can click or scroll through: the color-mode button, code copying, image zoom, and the table of contents. The shared entry point loads the theme button and link-icon handling. Prose pages and the photo gallery each have their own entry point. The animation library loads through dynamic imports when the theme or copy-button animations need it.

A small inline script in the document head applies the color mode before the body renders. It uses your saved preference, or your system setting if you haven’t picked one. After that, the button keeps your choice in sync across tabs and follows system changes when there’s no saved preference.

For formatting, I use [Oxfmt](https://oxc.rs/docs/guide/usage/formatter.html). It handles JavaScript, CSS, Markdown, and JSON, which is pretty handy :).

## Syntax highlighting

I like how Helix highlights code, so I use its [queries](https://github.com/helix-editor/helix/tree/master/runtime/queries) for most languages.

I pin those queries to specific versions so the highlighting doesn’t change between builds. A small build tool downloads them and combines the shared rules. Updating to a newer version is a separate step.

When I compile the site, `site/build.rs` generates the language enum and getters, collects the highlight names, and embeds the queries with `include_str!`. At startup, the generator compiles the Tree-sitter queries in parallel and shares the configurations across Markdown renderers. The queries are already embedded, so it doesn’t need to read query files while generating pages.

I adapted the highlighter from Tree-sitter’s Rust implementation and added a custom HTML renderer. It can handle one language inside another, like JavaScript inside HTML. For rainbow brackets, it uses the query’s scope information to cycle through six colors. Matching delimiters get the same color, while brackets in strings and comments don’t count as nesting.

The code blocks above use this setup. Their highlighting is already in the HTML, so your browser doesn’t need to parse the snippets or run a highlighting library. If the renderer doesn’t recognize a fence’s language, it escapes the code and shows it as plain text.

## Images

For images, I wanted three things: responsive sizes, low-quality placeholders while they load, and click-to-zoom support.

The `image` crate makes WebP and PNG versions of Markdown images at different widths. I cache them using hashes of the source contents, dimensions, and format. If an image hasn’t changed, I don’t need to encode it again.

I put those versions in a `picture` element with source sets and let the browser pick one for the available space. The full-size original stays available for zooming. The markup includes the width and height, with a placeholder behind the image while it loads.

To make a placeholder, `thumbhash` turns a small thumbnail into a compact representation, which I decode into a tiny PNG. For gallery photos, I use scaled JPEG decoding through `jpeg-decoder`. That lets me make the placeholder without a full-resolution RGB buffer. I apply the image orientation before creating it.

In the browser, `medium-zoom` handles the zoom. You can try it on this image:

![Wee Wee cat](./weewee.jpg "Click or inspect me wee wee!")

My personal photos have [their own page](/photos/) too! That gallery keeps the JPEGs without generating the responsive versions used for Markdown images. It reads dates from the filenames to show newer photos first, loads images lazily, and uses a small script to arrange them in a masonry grid.

For link previews, I use `resvg` to render Open Graph images from SVG templates. I render the background once and reuse it for every page, then draw each page’s title and description on top.

## Fonts and color

You’re reading Hanken Grotesk, with JetBrains Mono for code. Both fonts are self-hosted as WOFF2 files. To avoid loading characters a page doesn’t need, I split Hanken Grotesk into Latin, extended Latin, and Vietnamese subsets. Only the regular Latin subset is preloaded in the document head.

The site has light and dark modes, including the code blocks. I use CSS variables for text, backgrounds, borders, and accents, so each theme can change those colors without changing the page markup. Code highlighting has a separate set of variables for comments, strings, keywords, and the other syntax colors.

## Building and publishing

Locally, `just build` runs the Rust generator in release mode, followed by the Node.js asset script. That order matters because the generator clears `dist/` first. The asset build then adds the JavaScript, CSS, and math fonts. For a local preview, `just run` builds everything and serves it on port 3000.

When I push to `main`, GitHub Actions runs the same build, caches the Rust dependencies and generated images, and uploads `dist/` for deployment to GitHub Pages. I keep the image cache outside `dist/`, so clearing the output doesn’t mean encoding all those unchanged images again.

## Wrapping up

If you want to look at the details I haven’t covered here, you can browse [the source code of this site](https://github.com/pinanek/pinanek.github.io).
