---
title: Colophon
description: How i build this site.
---

## The stack

This is a static site generated using [Rust](https://rust-lang.org/), without a framework.

The template engine is [maud](https://maud.lambda.xyz/) for rendering HTML and [comrak](https://github.com/kivikakk/comrak) for rendering markdown.

For bundling JavaScript and TypeScript, I use [Rolldown](https://rolldown.rs/), the Rust-based bundler that powers [Vite](https://vite.dev/), the popular build tool for the modern web.

About the CSS, [Lightning CSS](https://lightningcss.dev/) is used for fast CSS bundling and support many modern features.

To syntax highlighting, I come to [Tree-sitter](https://tree-sitter.github.io/tree-sitter/), the library used by editors such as [Neovim](https://neovim.io/), [Helix](https://helix-editor.com/) and [Zed](https://zed.dev/).

And for image optimization, the [image](https://github.com/image-rs/image) crate enables the ability to optimize and generate different image formats and qualities.

## Markdown

Inspired by [MDX](https://mdxjs.com/) in the JavaScript world, I wanted the Markdown parser to be able to replace elements such as images, blockquotes, and so on with custom components.

In Rust, there are two popular crates for handling Markdown: [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) and `comrak`. `pulldown-cmark` is a great library with fast parsing and rendering performance. It produces an iterator of events without allocating an entire syntax tree. However, I found it less intuitive for transforming whole elements, since you sometimes have to manually collect and process the events between their start and end, for example, when working with the contents of an entire paragraph.

`comrak` builds an AST from the markdown, which is great for my use case. Here is a snippet from the site’s source code showing how I handle those customizations:

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

So `comrak` handles most of the work, which makes my life much easier. I just replace the relevant nodes with custom `maud` components.

## Templating

Yeah, once again inspired by the JavaScript world, I wanted something like JSX :). And `maud` fits my needs perfectly.

In the snippet below, `main_layout` is a component that you can call just like a function. It uses the `html!` macro for HTML, and variables can be inserted using the `()` syntax. It’s pretty simple and straightforward. Personally, I find it less verbose, with less boilerplate, and easier to read than JSX.

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

But that convenience comes at a cost: Rust’s build time. Since the templates are Rust code, every change triggers a rebuild, so I have to wait for the build to finish before I can see the result.

It’s a bit annoying, but I can live with it :P.

## JavaScript and CSS

Rust-based JavaScript tooling has become increasingly popular over the past few years, and it’s great to see these libs mature enough to be used for building a website like this one.

For `rolldown`, the process is pretty simple: providing the inputs, telling the bunlder where is the outputs with some options and you are done.

```rs title="js.rs"
let mut bundler = Bundler::new(BundlerOptions {
    input: Some(vec![
        InputItem {
            name: Some("main".into()),
            import: main_input.to_string_lossy().into_owned(),
        },
        InputItem {
            name: Some("photos/main".into()),
            import: photo_input.to_string_lossy().into_owned(),
        },
        InputItem {
            name: Some("posts/post".into()),
            import: post_input.to_string_lossy().into_owned(),
        },
    ]),
    cwd: Some(cwd.to_path_buf()),
    dir: Some(CONFIG.dist_assets_dir.to_string_lossy().into_owned()),
    entry_filenames: Some("[name].js".to_owned().into()),
    chunk_filenames: Some("[name].[hash].js".to_owned().into()),
    format: Some(OutputFormat::Esm),
    platform: Some(Platform::Browser),
    minify: Some(RawMinifyOptions::Bool(true)),
    code_splitting: Some(CodeSplittingMode::Bool(true)),
    ..Default::default()
})
.context("failed to initialize Rolldown")?;

let write_result = bundler.write().await;
let close_result = bundler.close().await;

write_result.context("failed to bundle JavaScript")?;
close_result.context("failed to close Rolldown")?;
```

With `lightningcss`, beside a bunlder, it also provides an ability to use CSS draft features like [custom media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@custom-media).

For formatting, [oxlint](https://oxc.rs/docs/guide/usage/formatter.html) also comes in handy. It’s a Rust-based formatter that supports JavaScript, CSS, Markdown, and JSON, which is pretty nice and fast :).

## Syntax highlighting

As a `Zed` and `helix` user, I'm quite impressed with the quality of `tree-sitter` in syntax highlighting.

Since I was already used to the way `helix` highlights code, I decided to use the language queries from the `helix` [repo](https://github.com/helix-editor/helix/tree/master/runtime/queries) instead of the queries provided by the individual `tree-sitter` language crates. In future, I plan to add colored backets, which already provides rainbow queries.

You can see some above code blocks for better insights.

## Images

I want the images on my site to support:

- Modern image formats such as `AVIF` and `WebP` for smaller file sizes.
- LQIP (Low-Quality Image Placeholders) while images are loading.
- The ability to zoom in when an image is clicked.

To achieve this, I use the `image` crate to generate images in different formats and the `thumbhash` crate to generate LQIPs. On the client side, I use `medium-zoom` to provide the zoom-in functionality. For faster building time, I also cached the generated images.

![Wee Wee cat](./weewee.jpg "Click or inspect me wee wee!")

I also create a dedicated page for my personal photos, [check it out](/photos/)!.

## Wrapping Up

There are many small details that I haven’t covered here, so feel free to check out t[he source code of this site](https://github.com/pinanek/pinanek.github.io) if you’re interested.
