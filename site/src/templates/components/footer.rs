use maud::{Markup, html};

use crate::{
    config::CONFIG,
    templates::components::{github_icon, google_scholar_icon, mail_icon, rss_icon},
};

pub fn footer() -> Markup {
    html! {
        footer class="footer" {
            div class="footer__content" {
                div class="footer__legal" {
                    p { "© 2026–present " (CONFIG.site.author) }
                    p {
                        a href="https://creativecommons.org/licenses/by-nc-sa/4.0/" rel="license" {
                            "CC BY-NC-SA 4.0"
                        }
                    }
                }

                nav class="footer__site-links" aria-label="Site information" {
                    a href="/uses/" { "Uses" }
                    a href="/colophon/" { "Colophon" }
                }

                nav class="footer__social-links" aria-label="Social links" {
                    a href="/rss.xml" aria-label="RSS" {
                        (rss_icon(None, None, None))
                    }
                    a href="mailto:{placeholder}" aria-label="Email" {
                        (mail_icon(None, None, None))
                    }
                    a href=(format!("https://github.com/{}", CONFIG.site.author_username)) aria-label="GitHub" target="_blank" rel="nofollow noopener noreferrer" {
                        (github_icon(None, None, None))
                    }
                    a href="https://scholar.google.com/citations?user=g6RMlQUAAAAJ" aria-label="Google Scholar" target="_blank" rel="nofollow noopener noreferrer" {
                        (google_scholar_icon(None, None, None))
                    }
                }
            }
        }
    }
}
