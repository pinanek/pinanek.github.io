use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::{Context, Result};
use maud::{Markup, html};

use crate::{config::CONFIG, posts::Post, templates::pages::RenderedPage};

pub fn write_sitemap(pages: &[RenderedPage]) -> Result<()> {
    let output_path = CONFIG.dist_dir.join("sitemap.xml");

    let file = File::create(&output_path)
        .with_context(|| format!("failed to create {}", output_path.display()))?;

    let mut writer = BufWriter::new(file);

    let markup = sitemap(pages);

    writer
        .write_all(br#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .with_context(|| format!("failed to write sitemap header: {}", output_path.display()))?;
    writer
        .write_all(b"\n")
        .with_context(|| format!("failed to write sitemap header: {}", output_path.display()))?;
    writer
        .write_all(markup.into_string().as_bytes())
        .with_context(|| format!("failed to write sitemap: {}", output_path.display()))?;
    writer
        .flush()
        .with_context(|| format!("failed to flush sitemap: {}", output_path.display()))?;

    Ok(())
}

fn sitemap(pages: &[RenderedPage]) -> Markup {
    html! {
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for page in pages {
                @let route = page.route();

                @if !matches!(route, "/404" | "/404/") {
                    @let url = if route == "/" {
                        CONFIG.site.url.to_owned()
                    } else {
                        format!("{}/{}", CONFIG.site.url, route.trim_start_matches('/'))
                    };

                    url {
                        loc { (url) }
                    }
                }
            }
        }
    }
}

pub fn write_rss(posts: &[Post]) -> Result<()> {
    let output_path = CONFIG.dist_dir.join("rss.xml");

    let file = File::create(&output_path)
        .with_context(|| format!("failed to create {}", output_path.display()))?;

    let mut writer = BufWriter::new(file);

    let markup = rss(posts);

    writer
        .write_all(br#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .with_context(|| format!("failed to write RSS header: {}", output_path.display()))?;
    writer
        .write_all(b"\n")
        .with_context(|| format!("failed to write RSS header: {}", output_path.display()))?;
    writer
        .write_all(markup.into_string().as_bytes())
        .with_context(|| format!("failed to write RSS feed: {}", output_path.display()))?;
    writer
        .flush()
        .with_context(|| format!("failed to flush RSS feed: {}", output_path.display()))?;

    Ok(())
}

fn rss(posts: &[Post]) -> Markup {
    html! {
        rss version="2.0" {
            channel {
                title { (CONFIG.site.name) }
                link { (CONFIG.site.url) }
                description { (CONFIG.site.description) }

                @for post in posts {
                    @let url = format!(
                        "{}/posts/{}/",
                        CONFIG.site.url,
                        post.slug.trim_matches('/'),
                    );

                    item {
                        title { (&post.metadata.title) }
                        link { (&url) }
                        guid isPermaLink="true" { (&url) }
                        description { (&post.metadata.description) }
                    }
                }
            }
        }
    }
}
