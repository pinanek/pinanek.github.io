use maud::{Markup, html};

use crate::{posts::Post, templates::components::tag};

pub fn post_item(post: &Post) -> Markup {
    html! {
        li class="post-item" {
            article {
                h2 class="post-item__title" {
                    a href=(format!("/posts/{}/", post.slug)) {
                        (&post.metadata.title)
                    }
                }
                p class="post-item__description" { (&post.metadata.description) }
                div class="post-item__metadata" {
                    time datetime=(post.metadata.pub_date.to_rfc3339()) {
                        (post.metadata.pub_date.format("%B %-d, %Y"))
                    }

                    @if !post.metadata.tags.is_empty() {
                        div class="post-item__tags" aria-label="Post tags" {
                            @for item in &post.metadata.tags {
                                (tag(item))
                            }
                        }
                    }
                }
            }
        }
    }
}
