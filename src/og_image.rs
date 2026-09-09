use std::{fs, path::Path};

use anyhow::{Context, Result};
use base64::prelude::*;
use rayon::prelude::*;
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{Options, Tree},
};

use crate::{
    config::CONFIG,
    templates::{components::og, pages::RenderedPage},
};

pub fn write_og_images(pages: &[RenderedPage]) -> Result<()> {
    fs::create_dir_all(&CONFIG.dist_og_dir).with_context(|| {
        format!(
            "failed to create OG image directory: {}",
            CONFIG.dist_og_dir.display(),
        )
    })?;

    let font_path = CONFIG.assets_dir.join("_fonts/hk-n.ttf");
    let font_data = fs::read(&font_path)
        .with_context(|| format!("failed to read OG image font: {}", font_path.display()))?;

    let background_path = CONFIG.assets_dir.join("_images/og_bg.png");
    let background_data = fs::read(&background_path).with_context(|| {
        format!(
            "failed to read OG image background: {}",
            background_path.display()
        )
    })?;
    let background_data_url = format!(
        "data:image/png;base64,{}",
        BASE64_STANDARD.encode(background_data)
    );

    pages
        .par_iter()
        .map_init(
            || create_svg_options(&font_data),
            |options, page| write_page_og_image(page, options, &background_data_url),
        )
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}

fn create_svg_options(font_data: &[u8]) -> Options<'static> {
    let mut options = Options::default();
    options.fontdb_mut().load_font_data(font_data.to_vec());
    options.font_family = "Hanken Grotesk".into();
    options
}

fn write_page_og_image(
    page: &RenderedPage,
    options: &Options<'_>,
    background_data_url: &str,
) -> Result<()> {
    let title = page.metadata().title.as_deref().unwrap_or(CONFIG.site.name);
    let description = page
        .metadata()
        .description
        .as_deref()
        .unwrap_or(CONFIG.site.description);

    let svg = og(
        title,
        description,
        page.route(),
        CONFIG.site.url,
        background_data_url,
    )
    .into_string();

    let path = CONFIG.dist_og_dir.join(og_image_name(page.route()));

    render_svg_to_png(&svg, options, &path)
        .with_context(|| format!("failed to render OG image for route: {}", page.route()))
}

fn render_svg_to_png(svg: &str, options: &Options<'_>, path: &Path) -> Result<()> {
    let tree = Tree::from_str(svg, options).context("failed to parse OG SVG")?;

    let size = tree.size().to_int_size();

    let mut pixmap =
        Pixmap::new(size.width(), size.height()).context("failed to allocate OG image pixmap")?;

    resvg::render(&tree, Transform::identity(), &mut pixmap.as_mut());

    pixmap
        .save_png(path)
        .with_context(|| format!("failed to save OG PNG: {}", path.display()))
}

pub fn og_image_public_path(route: &str) -> String {
    format!("/og/{}", og_image_name(route))
}

fn og_image_name(route: &str) -> String {
    format!("{}.png", normalize_page_route(route))
}

fn normalize_page_route(route: &str) -> String {
    if route == "/" {
        "garden".to_owned()
    } else {
        route.trim_matches('/').replace('/', "-")
    }
}
