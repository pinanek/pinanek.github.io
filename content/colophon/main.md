+++
title = "Colophon"
description = "How I build this site."
+++

## The stack

This is a static site generated using [Rust](https://rust-lang.org/), without a framework. The generator reads Markdown and data files, renders the pages, and writes everything to `dist/`.

I use [maud](https://maud.lambda.xyz/) for HTML templates and [comrak](https://github.com/kivikakk/comrak) for Markdown. Browser assets are built separately through Node.js, with [Rolldown](https://rolldown.rs/) for JavaScript and TypeScript and [Lightning CSS](https://lightningcss.dev/) for stylesheets.

Syntax highlighting uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/), the parser used by editors such as [Neovim](https://neovim.io/), [Helix](https://helix-editor.com/) and [Zed](https://zed.dev/). For image processing, I use the [image](https://github.com/image-rs/image) crate.

The Rust workspace has a `site` crate for the generator and an `xtask` crate for maintaining syntax queries. They share the manifest types in `site_internal`. GitHub Actions builds the site and deploys it to GitHub Pages.

## Markdown

Inspired by [MDX](https://mdxjs.com/) in the JavaScript world, I wanted to replace Markdown elements with custom components: headings with anchor links, images with responsive sources and placeholders, and code blocks with highlighting and copy buttons.

In Rust, two popular choices for Markdown are [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) and `comrak`. `pulldown-cmark` produces an iterator of events without allocating an entire syntax tree. It’s fast, but I found it less intuitive for transforming whole elements. For something like a blockquote, I’d have to collect and process the events between its start and end.

`comrak` builds an abstract syntax tree (AST), which fits my use case. I walk the document and prepare replacements for the nodes I want to customize:

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

## Templating

Yeah, once again inspired by the JavaScript world, I wanted something like JSX :). And `maud` fits my needs perfectly.

In the snippet below, `main_layout` is a function that wraps the page content. The `html!` macro produces HTML, and the `()` syntax inserts values or other components. Personally, I find it less verbose and easier to read than JSX.

```rs title="uses.rs"
main_layout(
    &self.metadata,
    html! {
        article class="prose" {
            (PreEscaped(&self.rendered))
        }
    },
    html! {
        link rel="stylesheet" href="/assets/uses/main.css";
    },
    html! {},
)
```

But that convenience comes at a cost: Rust’s build time. Changing a template means recompiling the generator before I can see the result. Markdown and stylesheet edits don’t require recompiling Rust, though I still need to rerun the generator or asset build.

It’s a bit annoying, but I can live with it :P.

## JavaScript and CSS

I use Rolldown, the Rust-based bundler behind [Vite](https://vite.dev/), through its JavaScript API. Both the JavaScript and CSS builds run in `scripts/build-assets.js`, so the bundlers aren’t part of the site’s Cargo dependency graph.

The JavaScript setup is pretty simple: provide the entry points, choose the output names, and enable minification and code splitting. Here’s that part of the script:

```javascript title="build-assets.js"
const javascript = await rolldown({
  cwd: root,
  input: {
    main: "assets/scripts/main.ts",
    "photos/main": "assets/scripts/photos/main.ts",
    "posts/post": "assets/scripts/posts/post.ts",
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

For formatting, I use [Oxfmt](https://oxc.rs/docs/guide/usage/formatter.html). It handles JavaScript, CSS, Markdown, and JSON, which is pretty handy :).

## Syntax highlighting

I like how Helix highlights code, so I use its [queries](https://github.com/helix-editor/helix/tree/master/runtime/queries) for most languages.

I pin the queries to specific versions so the highlighting stays consistent between builds. A small build tool downloads them and combines any shared rules, with a separate step for updating to newer versions.

At compile time, `site/build.rs` generates the language enum and getters, collects the highlight names, and embeds the queries with `include_str!`. The generator compiles the Tree-sitter queries in parallel at startup, then shares the configurations across Markdown renderers. There’s no query-file reading while the site is being generated.

The highlighter is adapted from Tree-sitter’s Rust implementation, with a custom HTML renderer. It handles injected languages, such as JavaScript inside HTML. Rainbow brackets use the query’s scope information to cycle through six colors, so matching delimiters share a color without treating brackets inside strings or comments as nesting.

You can see it in the code blocks above.

## Images

I want images on the site to have responsive versions, low-quality image placeholders while loading, and click-to-zoom support.

The `image` crate generates WebP and PNG variants at different widths for Markdown images. I cache the generated variants using hashes of the source contents, dimensions, and format, so unchanged images don’t need to be encoded again.

For placeholders, `thumbhash` turns a small thumbnail into a compact representation that I decode into a tiny PNG. Gallery photos use scaled JPEG decoding through `jpeg-decoder`, so generating a placeholder doesn’t need a full-resolution RGB buffer. The image orientation is applied before creating the placeholder.

On the client side, `medium-zoom` handles clicking an image to zoom in:

![Wee Wee cat](./weewee.jpg "Click or inspect me wee wee!")

There’s also a dedicated page for my personal photos, [check it out](/photos/)!

For link previews, `resvg` renders the Open Graph images from SVG templates. The background is rendered once and reused for every page, with the title and description drawn on top.

## Wrapping Up

There are many small details I haven’t covered here, so feel free to check out [the source code of this site](https://github.com/pinanek/pinanek.github.io) if you’re interested.
