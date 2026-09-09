use std::{env, fs, path::Path};

use anyhow::{Context, Result};
use rolldown::{
    Bundler, BundlerOptions, CodeSplittingMode, InputItem, OutputFormat, Platform, RawMinifyOptions,
};
use tokio::runtime::Builder;

use crate::config::CONFIG;

pub fn write_js() -> Result<()> {
    let cwd = env::current_dir().context("failed to get current directory")?;

    fs::create_dir_all(&CONFIG.dist_assets_dir).with_context(|| {
        format!(
            "failed to create assets directory: {}",
            CONFIG.dist_assets_dir.display()
        )
    })?;

    let runtime = Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to create Tokio runtime")?;

    runtime.block_on(bundle_javascript(&cwd))
}

async fn bundle_javascript(cwd: &Path) -> Result<()> {
    let main_input = CONFIG.assets_dir.join("scripts/main.ts");
    let photo_input = CONFIG.assets_dir.join("scripts/photos/main.ts");
    let post_input = CONFIG.assets_dir.join("scripts/posts/post.ts");

    let mut bundler = Bundler::new(BundlerOptions {
        input: Some(vec![
            InputItem {
                name: Some("main".into()),
                import: main_input.to_string_lossy().into_owned(),
            },
            InputItem {
                name: Some("photos/main".into()),
                import: photo_input.to_string_lossy().into_owned(),
            },
            InputItem {
                name: Some("posts/post".into()),
                import: post_input.to_string_lossy().into_owned(),
            },
        ]),
        cwd: Some(cwd.to_path_buf()),
        dir: Some(CONFIG.dist_assets_dir.to_string_lossy().into_owned()),
        entry_filenames: Some("[name].js".to_owned().into()),
        chunk_filenames: Some("[name].[hash].js".to_owned().into()),
        format: Some(OutputFormat::Esm),
        platform: Some(Platform::Browser),
        minify: Some(RawMinifyOptions::Bool(true)),
        code_splitting: Some(CodeSplittingMode::Bool(true)),
        ..Default::default()
    })
    .context("failed to initialize Rolldown")?;

    let write_result = bundler.write().await;
    let close_result = bundler.close().await;

    write_result.context("failed to bundle JavaScript")?;
    close_result.context("failed to close Rolldown")?;

    Ok(())
}
