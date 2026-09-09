use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow};
use lightningcss::{
    bundler::{Bundler, FileProvider},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions},
    targets::{Features, Targets},
};

use crate::config::CONFIG;

pub fn write_css() -> Result<()> {
    fs::create_dir_all(&CONFIG.dist_assets_dir).with_context(|| {
        format!(
            "failed to create assets directory: {}",
            CONFIG.dist_assets_dir.display()
        )
    })?;

    write_css_file(&CONFIG.assets_dir.join("styles/main.css"), "main.css")?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/home/main.css"),
        "home/main.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/posts/post.css"),
        "posts/post.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/publications/main.css"),
        "publications/main.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/photos/main.css"),
        "photos/main.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/posts/posts.css"),
        "posts/posts.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/uses/main.css"),
        "uses/main.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/tags/main.css"),
        "tags/main.css",
    )?;
    write_css_file(
        &CONFIG.assets_dir.join("styles/colophon/main.css"),
        "colophon/main.css",
    )?;

    Ok(())
}

fn write_css_file(input: &Path, output: &str) -> Result<()> {
    let provider = FileProvider::new();

    let parser_options = ParserOptions {
        flags: ParserFlags::NESTING | ParserFlags::CUSTOM_MEDIA,
        ..Default::default()
    };
    let mut bundler = Bundler::new(&provider, None, parser_options);

    let mut stylesheet = bundler
        .bundle(input)
        .map_err(|error| anyhow!("failed to bundle {}: {error}", input.display()))?;

    let targets = Targets {
        include: Features::FontFamilySystemUi
            | Features::Nesting
            | Features::CustomMediaQueries
            | Features::MediaRangeSyntax,
        ..Default::default()
    };
    let minify_options = MinifyOptions {
        targets,
        ..Default::default()
    };
    stylesheet
        .minify(minify_options)
        .map_err(|error| anyhow!("failed to optimize {}: {error}", input.display()))?;

    let result = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            targets,
            ..Default::default()
        })
        .map_err(|error| anyhow!("failed to serialize {}: {error}", input.display()))?;

    let output_path = CONFIG.dist_assets_dir.join(output);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create CSS output directory: {}",
                parent.display()
            )
        })?;
    }

    fs::write(&output_path, result.code)
        .with_context(|| format!("failed to write CSS bundle: {}", output_path.display()))?;

    Ok(())
}
