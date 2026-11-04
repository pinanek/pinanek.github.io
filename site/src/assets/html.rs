use std::{fs, path::Path};

use anyhow::{Context, Result};
use html_minifier::minify;
use rayon::prelude::*;

use crate::{config::CONFIG, templates::pages::RenderedPage};

pub fn write_html(pages: &[RenderedPage]) -> Result<()> {
    pages.par_iter().try_for_each(|page| -> Result<()> {
        let minified = minify(page.html())
            .with_context(|| format!("failed to minify HTML for route: {}", page.route()))?;

        let output_path = Path::new(page.route().trim_start_matches('/')).join("index.html");

        let output_path = CONFIG.dist_dir.join(output_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create output directory: {}", parent.display())
            })?;
        }

        fs::write(&output_path, minified)
            .with_context(|| format!("failed to write HTML: {}", output_path.display()))?;

        Ok(())
    })?;

    Ok(())
}
