use std::fs;

use anyhow::Result;
use maud::{PreEscaped, html};
use serde::{Deserialize, Serialize};

use crate::{
    assets::image::ImageRequests,
    config::CONFIG,
    markdown::render_markdown,
    templates::{
        layouts::main_layout,
        pages::{Page, PageMetadata},
    },
};

#[derive(Serialize, Deserialize)]
pub struct UsesMarkdownMetadata {
    title: String,
    description: String,
}

pub struct UsesPage<'a> {
    route: String,
    images: ImageRequests,
    metadata: PageMetadata<'a>,
    rendered: String,
}

impl<'a> UsesPage<'a> {
    pub fn new() -> Result<Self> {
        let route = String::from("/uses/");

        let content_dir = CONFIG.content_dir.join("uses");
        let content_path = content_dir.join("main.md");
        let content = fs::read_to_string(&content_path)?;

        let rendered_markdown = render_markdown::<UsesMarkdownMetadata>(&content, &content_dir)?;
        let markdown_metadata = rendered_markdown.metadata;
        let images = ImageRequests::from(rendered_markdown.images);

        Ok(Self {
            images,
            route: route.clone(),
            rendered: rendered_markdown.html,
            metadata: PageMetadata::new()
                .title(markdown_metadata.title)
                .description(markdown_metadata.description)
                .canonical_url(route),
        })
    }
}

impl<'a> Page for UsesPage<'a> {
    fn route(&self) -> &str {
        &self.route
    }

    fn metadata(&self) -> &PageMetadata<'_> {
        &self.metadata
    }

    fn images(&self) -> &ImageRequests {
        &self.images
    }

    fn render(&self) -> maud::Markup {
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
    }
}
