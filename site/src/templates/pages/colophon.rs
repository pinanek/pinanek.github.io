use std::fs;

use anyhow::Result;
use maud::{PreEscaped, html};
use serde::{Deserialize, Serialize};

use crate::{
    assets::image::ImageRequests,
    config::CONFIG,
    highlight::HighlightRegistry,
    markdown::render_markdown,
    templates::{
        layouts::main_layout,
        pages::{Page, PageMetadata},
    },
};

#[derive(Serialize, Deserialize)]
pub struct ColophonMarkdownMetadata {
    title: String,
    description: String,
}

pub struct ColophonPage<'a> {
    images: ImageRequests,
    route: &'static str,
    metadata: PageMetadata<'a>,
    rendered: String,
}

impl<'a> ColophonPage<'a> {
    pub fn new(languages: &HighlightRegistry) -> Result<Self> {
        let route = "/colophon/";

        let content_dir = CONFIG.content_dir.join("colophon");
        let content_path = content_dir.join("main.md");
        let content = fs::read_to_string(&content_path)?;

        let rendered_markdown =
            render_markdown::<ColophonMarkdownMetadata>(&content, &content_dir, languages)?;
        let markdown_metadata = rendered_markdown.metadata;

        Ok(Self {
            images: rendered_markdown.images,
            route,
            rendered: rendered_markdown.html,
            metadata: PageMetadata::new()
                .title(markdown_metadata.title)
                .description(markdown_metadata.description)
                .canonical_url(route)
                .math(rendered_markdown.has_math),
        })
    }
}

impl<'a> Page for ColophonPage<'a> {
    fn route(&self) -> &str {
        self.route
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
                link rel="stylesheet" href="/assets/prose.css";
            },
            html! {
                script type="module" src="/assets/posts/post.js" {}
            },
        )
    }
}
