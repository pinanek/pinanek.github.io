use std::{cmp::Reverse, fs, path::Path};

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use rayon::prelude::*;

use crate::{assets::image::thumbhash_placeholder, config::CONFIG};

#[derive(Debug)]
pub struct Photo {
    pub public_url: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
    taken_at: Option<NaiveDateTime>,
}

pub fn get_photos() -> Result<Vec<Photo>> {
    let paths = fs::read_dir(CONFIG.photos_dir)
        .with_context(|| {
            format!(
                "failed to read photos directory: {}",
                CONFIG.photos_dir.display()
            )
        })?
        .map(|entry| {
            entry
                .context("failed to read photo directory entry")
                .map(|entry| entry.path())
        })
        .collect::<Result<Vec<_>>>()?;

    let mut photos = paths
        .into_par_iter()
        .filter(|path| is_photo(path))
        .map(|path| photo_from_path(&path))
        .collect::<Result<Vec<_>>>()?;

    photos.sort_by_key(|photo| Reverse(photo.taken_at));

    Ok(photos)
}

fn photo_from_path(path: &Path) -> Result<Photo> {
    let filename = path
        .file_name()
        .and_then(|filename| filename.to_str())
        .with_context(|| format!("invalid photo filename: {}", path.display()))?;

    let (width, height) = image::image_dimensions(path)
        .with_context(|| format!("failed to read dimensions: {}", path.display()))?;
    let placeholder = thumbhash_placeholder(path)
        .with_context(|| format!("failed to generate placeholder: {}", path.display()))?;

    Ok(Photo {
        public_url: format!("/photos/{filename}"),
        alt: photo_alt(path),
        width,
        height,
        placeholder,
        taken_at: photo_timestamp(path),
    })
}

fn is_photo(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                matches!(extension.to_ascii_lowercase().as_str(), "jpg" | "jpeg")
            })
}

fn photo_timestamp(path: &Path) -> Option<NaiveDateTime> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| NaiveDateTime::parse_from_str(stem, "%m-%d-%Y_%H%M%S").ok())
}

fn photo_alt(path: &Path) -> String {
    match photo_timestamp(path) {
        Some(timestamp) => format!("Photo taken {}", timestamp.format("%B %-d, %Y at %H:%M")),
        None => path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map_or_else(|| String::from("Photo"), String::from),
    }
}
