use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::config::CONFIG;

pub fn copy_public_dir() -> Result<()> {
    let source = Path::new("public");

    if !source.exists() {
        return Ok(());
    }

    copy_dir_contents(source, CONFIG.dist_dir)
}

pub fn copy_photos_dir() -> Result<()> {
    if !CONFIG.photos_dir.exists() {
        return Ok(());
    }

    copy_dir_contents(CONFIG.photos_dir, &CONFIG.dist_dir.join("photos"))
}

fn copy_dir_contents(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory: {}", destination.display()))?;

    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read directory: {}", source.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read entry in directory: {}", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if entry
            .file_type()
            .with_context(|| format!("failed to determine file type: {}", source_path.display()))?
            .is_dir()
        {
            copy_dir_contents(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }

    Ok(())
}
