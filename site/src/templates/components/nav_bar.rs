use maud::{Markup, html};

use crate::{
    config::CONFIG,
    templates::components::{color_mode_button, document_icon, images_icon, scroll_text_icon},
};

pub fn nav_bar() -> Markup {
    html! {
        nav class="nav-bar" {
            span class="nav-bar__brand" {
                a href="/" { (format!("@{}", CONFIG.site.author_username)) }
            }

            div class="nav-bar__controls" {
                div class="nav-bar__links" {
                    a class="nav-bar__link nav-bar__icon-button" href="/posts/" aria-label="Posts" {
                        (document_icon(Some("nav-bar__link-icon"), Some("1.15rem"), None))
                        span class="nav-bar__link-label" { "Posts" }
                    }
                    a class="nav-bar__link nav-bar__icon-button" href="/publications/" aria-label="Publications" {
                        (scroll_text_icon(Some("nav-bar__link-icon"), Some("1.15rem"), None))
                        span class="nav-bar__link-label" { "Publications" }
                    }

                    a class="nav-bar__link nav-bar__icon-button" href="/photos/" aria-label="Photos" {
                        (images_icon(Some("nav-bar__link-icon"), Some("1.15rem"), None))
                        span class="nav-bar__link-label" { "Photos" }
                    }
                }

                div class="nav-bar__color-mode" {
                    (color_mode_button())
                }
            }
        }
    }
}
