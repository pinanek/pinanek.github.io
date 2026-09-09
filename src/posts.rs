use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::{assets::image::ImageRequests, config::CONFIG, markdown::render_markdown};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PostMetadata {
    pub title: String,
    pub description: String,

    #[serde(default)]
    pub tags: Vec<String>,

    pub pub_date: DateTime<Utc>,

    #[serde(default)]
    pub updated_date: Option<DateTime<Utc>>,
}

pub struct Post {
    pub slug: String,
    pub metadata: PostMetadata,
    pub rendered: String,
    pub images: ImageRequests,
    pub reading_minutes: usize,
}

pub fn get_posts() -> Result<Vec<Post>> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(&CONFIG.posts_dir).with_context(|| {
        format!(
            "failed to read posts directory: {}",
            CONFIG.posts_dir.display()
        )
    })? {
        let path = entry
            .context("failed to read post directory entry")?
            .path()
            .join("main.md");

        if path.is_file() {
            paths.push(path);
        }
    }

    paths.par_iter().map(parse_post).collect()
}

pub fn parse_post(path: &PathBuf) -> Result<Post> {
    let content_dir = path
        .parent()
        .with_context(|| format!("invalid content dir: {}", path.display()))?;
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read post: {}", path.display()))?;

    let slug = slug_from_path(path)?;

    let rendered = render_markdown::<PostMetadata>(&content, content_dir)?;

    let mut word_count = content.unicode_words().count();
    word_count = word_count.div_ceil(220).max(1);

    Ok(Post {
        slug,
        metadata: rendered.metadata,
        rendered: rendered.html,
        images: rendered.images,
        reading_minutes: word_count,
    })
}

fn slug_from_path(path: &Path) -> Result<String> {
    path.parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .with_context(|| format!("invalid post path: {}", path.display()))
}

pub struct Tag<'a> {
    pub name: String,
    pub posts: Vec<&'a Post>,
}

pub fn get_tags<'a>(posts: &[&'a Post]) -> Vec<Tag<'a>> {
    let mut tags = std::collections::HashMap::<&str, Vec<&'a Post>>::new();

    for &post in posts {
        for tag in &post.metadata.tags {
            tags.entry(tag).or_default().push(post);
        }
    }

    tags.into_iter()
        .map(|(name, posts)| Tag {
            name: name.to_owned(),
            posts,
        })
        .collect()
}
