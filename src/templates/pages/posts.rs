use std::cmp::Reverse;

use anyhow::Result;
use maud::{Markup, html};

use crate::{
    assets::image::ImageRequests,
    posts::Post,
    templates::{
        components::post_item,
        layouts::main_layout,
        pages::{Page, PageMetadata},
    },
};

pub struct PostsPage<'a> {
    route: String,
    posts: Vec<&'a Post>,
    images: ImageRequests,
    metadata: PageMetadata<'a>,
}

impl<'a> PostsPage<'a> {
    pub fn new(posts: &'a [Post]) -> Result<Self> {
        let route = String::from("/posts/");
        let mut posts = posts.iter().collect::<Vec<_>>();
        posts.sort_by_key(|post| Reverse(post.metadata.pub_date));

        Ok(Self {
            route: route.clone(),
            posts,
            images: ImageRequests::new(),
            metadata: PageMetadata::new()
                .title("Posts")
                .description("All posts.")
                .canonical_url(route),
        })
    }
}

impl Page for PostsPage<'_> {
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
        let links = html! {
            link rel="stylesheet" href="/assets/posts/posts.css";
        };

        main_layout(
            &self.metadata,
            html! {
                div class="posts-page" {
                    ul class="post-list" {
                        @for post in &self.posts {
                            (post_item(post))
                        }
                    }
                }
            },
            links,
            html! {},
        )
    }
}
