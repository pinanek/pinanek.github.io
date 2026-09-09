mod assets;
mod config;
mod highlight;
mod markdown;
mod og_image;
mod photos;
mod posts;
mod templates;
mod utils;

use anyhow::{Context, Ok, Result};
use std::fs;

use crate::assets::css::write_css;
use crate::assets::html::write_html;
use crate::assets::image::write_images;
use crate::assets::js::write_js;
use crate::assets::public::{copy_photos_dir, copy_public_dir};
use crate::assets::xml::{write_rss, write_sitemap};
use crate::config::CONFIG;
use crate::og_image::write_og_images;
use crate::photos::get_photos;
use crate::posts::{Post, get_posts, get_tags};
use crate::templates::pages::{
    ColophonPage, HomePage, Page, PhotosPage, PostPage, PostsPage, PublicationsPage, TagPage,
    UsesPage, render_pages,
};

fn main() -> Result<()> {
    let posts = get_posts()?;
    let photos = get_photos()?;

    let mut pages: Vec<Box<dyn Page>> = vec![
        Box::new(HomePage::new()?),
        Box::new(PostsPage::new(&posts)?),
        Box::new(PublicationsPage::new()?),
        Box::new(PhotosPage::new(photos)?),
        Box::new(UsesPage::new()?),
        Box::new(ColophonPage::new()?),
    ];

    for post in &posts {
        pages.push(Box::new(PostPage::new(post)?));
    }

    let post_rers: Vec<&Post> = posts.iter().collect();
    let tags = get_tags(&post_rers);
    for tag in tags {
        pages.push(Box::new(TagPage::new(tag)?));
    }

    let rendered_pages = render_pages(&pages);

    let images = rendered_pages
        .iter()
        .flat_map(|page| page.images().values())
        .collect::<Vec<_>>();

    clean_dist_dir()?;
    copy_public_dir()?;
    copy_photos_dir()?;
    write_html(&rendered_pages)?;
    write_js()?;
    write_css()?;
    write_og_images(&rendered_pages)?;
    write_rss(&posts)?;
    write_sitemap(&rendered_pages)?;
    write_images(&images)?;

    Ok(())
}

fn clean_dist_dir() -> Result<()> {
    let dist_dir = CONFIG.dist_dir;

    if let Err(error) = fs::remove_dir_all(dist_dir)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        return Err(error)
            .with_context(|| format!("failed to remove output directory: {}", dist_dir.display()));
    }

    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    Ok(())
}
