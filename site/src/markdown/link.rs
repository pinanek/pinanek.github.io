// Inspired by https://github.com/antfu/markdown-it-magic-link.

use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use comrak::{Node, nodes::NodeLink};
use maud::{Markup, html};
use url::Url;

use crate::{
    config::CONFIG, markdown::MarkdownContext, templates::components::external_link_icon,
    utils::is_external_url,
};

static LINK_IMAGE_OVERRIDES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("comrak.ee", "https://comrak.ee/favicon.png"),
        (
            "lightningcss.dev",
            "https://lightningcss.dev/favicon.b3fd0060.svg",
        ),
        ("oxc.rs", "https://oxc.rs/logo-without-border.svg"),
        ("rolldown.rs", "https://rolldown.rs/logo-without-border.svg"),
        (
            "rust-lang.org",
            "https://rust-lang.org/static/images/rust-logo-blk.svg",
        ),
        ("usehax.dev", "https://usehax.dev/favicon.svg"),
        ("vite.dev", "https://vite.dev/logo-without-border.svg"),
    ])
});

const GITHUB_SPECIAL_ROUTES: &[&str] = &[
    "about",
    "account",
    "apps",
    "codespaces",
    "collections",
    "dashboard",
    "discussions",
    "enterprise",
    "events",
    "explore",
    "features",
    "issues",
    "join",
    "login",
    "logout",
    "marketplace",
    "new",
    "notifications",
    "organizations",
    "orgs",
    "pricing",
    "pulls",
    "search",
    "security",
    "settings",
    "signup",
    "site",
    "sponsor",
    "sponsors",
    "topics",
    "trending",
    "users",
];

pub fn prepare_link(node: Node, link: &NodeLink, context: &mut MarkdownContext) -> Result<()> {
    let image = if context.link_icons {
        link_image(&link.url)
    } else {
        None
    };
    let rendered =
        prose_link(&link.url, &link.title, &node.collect_text(), image.as_ref()).into_string();

    context.replace_node(node, rendered);

    Ok(())
}

fn link_image(href: &str) -> Option<(String, bool)> {
    let url = Url::parse(href).ok()?;
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }

    let host = url.host_str()?;
    if Some(host) == Url::parse(CONFIG.site.url).ok()?.host_str() {
        return None;
    }

    if let Some(&image) = LINK_IMAGE_OVERRIDES.get(host) {
        return Some((image.to_owned(), false));
    }

    if host == "github.com" {
        let login = url.path().strip_prefix('/')?.trim_end_matches('/');

        if !login.is_empty()
            && login.len() <= 39
            && login
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            && !login.starts_with('-')
            && !login.ends_with('-')
            && !GITHUB_SPECIAL_ROUTES
                .iter()
                .any(|route| login.eq_ignore_ascii_case(route))
        {
            return Some((
                format!("https://avatars.githubusercontent.com/{login}?s=32"),
                true,
            ));
        }

        return Some((
            "https://avatars.githubusercontent.com/u/9919?v=4&s=32".to_owned(),
            false,
        ));
    }

    let mut image = Url::parse("https://www.google.com/s2/favicons").ok()?;
    image
        .query_pairs_mut()
        .append_pair("domain", host)
        .append_pair("sz", "32");
    Some((image.into(), false))
}

fn prose_link(href: &str, title: &str, content: &str, image: Option<&(String, bool)>) -> Markup {
    let external = is_external_url(href);

    html! {
        a   class="prose-link"
            href=(href)
            title=(title)
            target=[external.then_some("_blank")]
            rel=[external.then_some("noopener noreferrer")]
        {
            @if let Some((src, avatar)) = image {
                img
                    class={"prose-link__image" @if *avatar { " prose-link__image--avatar" }}
                    src=(src)
                    alt=""
                    width="16"
                    height="16"
                    loading="lazy"
                    decoding="async"
                    referrerpolicy="no-referrer";
            }

            span class="prose-link__label" { (content) }

            @if external {
                (external_link_icon(Some("prose-link__external-icon"), Some("0.9em"), None))
            }
        }
    }
}
