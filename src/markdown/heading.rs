use anyhow::Result;
use comrak::{Node, nodes::NodeHeading};
use maud::{Markup, PreEscaped, html};
use slug::slugify;

use crate::{markdown::MarkdownContext, templates::components::link_icon};

pub struct ParsedTocItem {
    pub level: u8,
    pub id: String,
    pub title: String,
}

pub fn prepare_heading(
    node: Node,
    heading: &NodeHeading,
    context: &mut MarkdownContext,
) -> Result<()> {
    let text = node.collect_text();

    let sourcepos = node.data().sourcepos;
    let input = format!("{}:{}:{text}", sourcepos.start.line, sourcepos.start.column,);
    let hash = blake3::hash(input.as_bytes());
    let short = &hash.to_hex()[..4];
    let id = format!("{}-{short}", slugify(&text));

    context.toc_items.push(ParsedTocItem {
        level: heading.level,
        id: id.clone(),
        title: text.clone(),
    });

    let rendered = prose_heading(heading.level, &id, &text).into_string();
    context.replace_node(node, rendered);

    Ok(())
}

pub fn prose_heading(level: u8, id: &str, content: &str) -> Markup {
    macro_rules! heading {
        ($tag:ident) => {
            html! {
                $tag id=(id) class="prose-heading" {
                    a
                        class="prose-heading__anchor"
                        href={"#" (id)}
                        aria-label="Link to heading"
                    {
                        (link_icon(Some("prose-heading__anchor-icon"), None, None))
                    }

                    (PreEscaped(content))
                }
            }
        };
    }

    match level {
        1 => heading!(h1),
        2 => heading!(h2),
        3 => heading!(h3),
        4 => heading!(h4),
        5 => heading!(h5),
        6 => heading!(h6),
        _ => Markup::default(),
    }
}

pub fn prose_toc(items: &[ParsedTocItem]) -> Markup {
    html! {
        nav class="prose-toc" aria-label="Table of contents" {
            div class="prose-toc__inner" {
                div class="prose-toc__title" { "Table of Contents" }

                div class="prose-toc__body" {
                    svg class="prose-toc__path" aria-hidden="true" {
                        path class="prose-toc__path-active" {}
                    }

                    ol class="prose-toc__list" {
                        @for item in items {
                            li class="prose-toc__item" data-level=(item.level) {
                                a href=(format!("#{}", item.id)) { (&item.title) }
                            }
                        }
                    }
                }
            }
        }
    }
}
