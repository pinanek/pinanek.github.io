use std::borrow::Cow;

use maud::{Markup, html};

use crate::{
    config::CONFIG,
    templates::pages::{PageMetadata, PageType},
    utils::get_absolute_url,
};

pub fn metadata(props: &PageMetadata<'_>) -> Markup {
    let title = match props.title.as_deref() {
        Some(title) => Cow::Owned(format!("{title} | {}", CONFIG.site.name)),
        None => Cow::Borrowed(CONFIG.site.name),
    };

    let description = props
        .description
        .as_deref()
        .unwrap_or(CONFIG.site.description);

    let canonical_url = props.canonical_url.as_deref().map(get_absolute_url);

    let image_url = props.image_url.as_deref().map(get_absolute_url);

    let image_alt = props.image_alt.as_deref();

    let keywords = (!props.keywords.is_empty()).then(|| props.keywords.join(", "));

    let twitter_card = if image_url.is_some() {
        "summary_large_image"
    } else {
        "summary"
    };

    let is_article = props.page_type == PageType::Article;

    html! {
        title { (&title) }

        meta name="description" content=(description);
        meta name="author" content=(CONFIG.site.author);
        meta name="robots" content="index, follow, max-image-preview:large";

        @if let Some(keywords) = keywords.as_deref() {
            meta name="keywords" content=(keywords);
        }

        @if let Some(canonical_url) = canonical_url.as_deref() {
            link rel="canonical" href=(canonical_url);
            meta property="og:url" content=(canonical_url);
        }


        link rel="alternate" type="application/rss+xml" title=(CONFIG.site.name) href="/rss.xml";

        meta property="og:title" content=(&title);
        meta property="og:type" content=(props.page_type.to_string());
        meta property="og:site_name" content=(CONFIG.site.name);
        meta property="og:description" content=(description);

        meta name="twitter:card" content=(twitter_card);
        meta name="twitter:title" content=(&title);
        meta name="twitter:description" content=(description);
        meta name="twitter:site" content=(CONFIG.site.author_username);
        meta name="twitter:creator" content=(CONFIG.site.author_username);

        @if let Some(image_url) = image_url.as_deref() {
            meta property="og:image" content=(image_url);
            meta name="twitter:image" content=(image_url);

            @if let Some(image_alt) = image_alt {
                meta property="og:image:alt" content=(image_alt);
                meta name="twitter:image:alt" content=(image_alt);
            }
        }

        @if is_article {
            meta property="article:author" content=(CONFIG.site.author);

            @if let Some(published_time) = props.published_time.as_deref() {
                meta property="article:published_time" content=(published_time);
            }

            @if let Some(modified_time) = props.modified_time.as_deref() {
                meta property="article:modified_time" content=(modified_time);
            }
        }
    }
}
