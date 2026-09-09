use anyhow::Result;
use maud::{Markup, html};

use crate::{
    assets::image::ImageRequests,
    posts::Tag,
    templates::{
        components::post_item,
        layouts::main_layout_with_header,
        pages::{Page, PageMetadata},
    },
};

pub struct TagPage<'a> {
    tag: Tag<'a>,
    route: String,
    images: ImageRequests,
    metadata: PageMetadata<'a>,
}

impl<'a> TagPage<'a> {
    pub fn new(tag: Tag<'a>) -> Result<Self> {
        let route = format!("/tags/{}/", tag.name);

        let metadata = PageMetadata::new()
            .title(format!("Posts tagged {}", tag.name))
            .description(format!("Posts tagged {}.", tag.name))
            .keywords([tag.name.clone()])
            .canonical_url(route.clone());

        Ok(Self {
            tag,
            route,
            images: ImageRequests::new(),
            metadata,
        })
    }
}

impl Page for TagPage<'_> {
    fn route(&self) -> &str {
        &self.route
    }

    fn metadata(&self) -> &PageMetadata<'_> {
        &self.metadata
    }

    fn images(&self) -> &ImageRequests {
        &self.images
    }

    fn render(&self) -> Markup {
        main_layout_with_header(
            &self.metadata,
            html! {
                header class="main-layout__header" {
                    h1 { "Posts tagged " code { (&self.tag.name) } }
                    p class="main-layout__description" { "Posts tagged " (&self.tag.name) "." }
                }
            },
            html! {
                div class="tag-page" {
                    ul class="post-list" {
                        @for post in &self.tag.posts {
                            (post_item(post))
                        }
                    }
                }
            },
            html! {
                link rel="stylesheet" href="/assets/tags/main.css";
            },
            html! {},
        )
    }
}
