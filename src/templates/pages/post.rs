use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_tz::Etc::GMTPlus12;
use maud::{Markup, PreEscaped, html};

use crate::{
    assets::image::ImageRequests,
    og_image::og_image_public_path,
    posts::Post,
    templates::{
        components::tag,
        layouts::base_layout,
        pages::{Page, PageMetadata},
    },
};

pub struct PostPage<'a> {
    post: &'a Post,
    route: String,
    metadata: PageMetadata<'a>,
}

impl<'page> PostPage<'page> {
    pub fn new(post: &'page Post) -> Result<Self> {
        let route = format!("/posts/{}/", post.slug);

        let post_metadata = post.metadata.to_owned();

        let metadata = PageMetadata::new()
            .title(post_metadata.title)
            .description(post_metadata.description.clone())
            .keywords(post_metadata.tags)
            .article(&post_metadata.pub_date, post_metadata.updated_date.as_ref())
            .image(og_image_public_path(&route), post_metadata.description)
            .canonical_url(route.clone());

        Ok(Self {
            post,
            route,
            metadata,
        })
    }
}

impl Page for PostPage<'_> {
    fn route(&self) -> &str {
        &self.route
    }

    fn metadata(&self) -> &PageMetadata<'_> {
        &self.metadata
    }

    fn images(&self) -> &ImageRequests {
        &self.post.images
    }

    fn render(&self) -> Markup {
        let post_meta = &self.post.metadata;

        let published_date = format_aoe(&post_meta.pub_date);
        let updated_date = post_meta.updated_date.map(|date| format_aoe(&date));

        base_layout(
            &self.metadata,
            html! {
                div class="post" {
                    header class="post-header" {
                        h1 class="post-header__title" { (&post_meta.title) }
                        p class="post-header__description" { (&post_meta.description) }

                        div class="post-header__metadata" {
                            span class="post-header__date" {
                                "Published: "
                                time datetime=(&published_date) { (&published_date) }
                            }

                            @if let Some(updated_date) = &updated_date {
                                span { "-" }

                                span class="post-header__date" {
                                    "Updated: "
                                    time datetime=(updated_date) { (updated_date) }
                                }
                            }
                            span { "-" }
                            span class="post-header__reading-time" {
                                (self.post.reading_minutes)
                                " min read"
                            }
                        }

                        @if !post_meta.tags.is_empty() {
                            div class="post-header__tags-wrapper" {
                                span class="post-header__tags-title" { "Tags:" }

                                ul class="post-header__tags" aria-label="Post tags" {
                                    @for item in &post_meta.tags { (tag(item)) }
                                }
                            }
                        }
                    }
                    article class="post-article prose" { (PreEscaped(&self.post.rendered)) }
                }
            },
            html! {
                link rel="stylesheet" href="/assets/posts/post.css";
            },
            html! {
                script type="module" src="/assets/posts/post.js" {}
            },
        )
    }
}

pub fn format_aoe(date: &DateTime<Utc>) -> String {
    date.with_timezone(&GMTPlus12)
        .format("%B %d, %Y, %H:%M AoE")
        .to_string()
}
