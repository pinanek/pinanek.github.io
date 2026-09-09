use std::{path::Path, sync::LazyLock};

#[derive(Debug)]
pub struct Site {
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub author_username: &'static str,
    pub url: &'static str,
}

#[derive(Debug)]
pub struct Config {
    pub dist_dir: &'static Path,
    pub dist_assets_dir: &'static Path,
    pub dist_og_dir: &'static Path,

    pub queries_dir: &'static Path,
    pub data_dir: &'static Path,
    pub assets_dir: &'static Path,
    pub cache_images_dir: &'static Path,
    pub content_dir: &'static Path,
    pub photos_dir: &'static Path,
    pub posts_dir: &'static Path,

    pub site: Site,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    dist_dir: Path::new("dist"),
    dist_assets_dir: Path::new("dist/assets"),

    queries_dir: Path::new("queries"),
    data_dir: Path::new("data"),
    assets_dir: Path::new("assets"),
    cache_images_dir: Path::new(".cache/images"),
    dist_og_dir: Path::new("dist/og"),
    content_dir: Path::new("content"),
    photos_dir: Path::new("assets/photos"),
    posts_dir: Path::new("content/posts"),

    site: Site {
        name: "pinanek's garden",
        description: "Personal website and digital garden.",
        author: "Ngo Duc Hoang Son",
        author_username: "pinanek",
        url: "https://pinanek.github.io",
    },
});
