use maud::{Markup, html};

use crate::templates::{layouts::base_layout, pages::PageMetadata};

pub fn main_layout(
    metadata: &PageMetadata<'_>,
    children: Markup,
    links: Markup,
    scripts: Markup,
) -> Markup {
    main_layout_with_header(metadata, main_header(metadata), children, links, scripts)
}

pub fn main_layout_with_header(
    metadata: &PageMetadata<'_>,
    header: Markup,
    children: Markup,
    links: Markup,
    scripts: Markup,
) -> Markup {
    base_layout(
        metadata,
        html! {
            main class="main-layout" {
                (header)
                (children)
            }
        },
        links,
        scripts,
    )
}

fn main_header(metadata: &PageMetadata<'_>) -> Markup {
    html! {
        @if let Some(title) = &metadata.title {
            header class="main-layout__header" {
                h1 { (title) }

                @if let Some(description) = &metadata.description {
                    p class="main-layout__description" { (description) }
                }
            }
        }
    }
}
