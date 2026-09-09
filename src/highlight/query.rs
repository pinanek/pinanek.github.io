use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::config::CONFIG;

pub fn load_helix_query(language: &str, query_name: &str) -> Result<String> {
    load_query(language, query_name, false)
}

pub fn load_optional_helix_query(language: &str, query_name: &str) -> Result<String> {
    load_query(language, query_name, true)
}

fn load_query(language: &str, query_name: &str, optional: bool) -> Result<String> {
    let mut visited = HashSet::new();
    let mut output = String::new();

    load_query_recursive(language, query_name, optional, &mut visited, &mut output)?;

    Ok(output)
}

fn load_query_recursive(
    language: &str,
    query_name: &str,
    optional: bool,
    visited: &mut HashSet<PathBuf>,
    output: &mut String,
) -> Result<()> {
    let relative_path = PathBuf::from(language).join(format!("{query_name}.scm"));

    if !visited.insert(relative_path.clone()) {
        return Ok(());
    }

    let source = match read_query(&relative_path) {
        Ok(source) => source,

        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if optional {
                return Ok(());
            } else {
                let path = CONFIG.queries_dir.join(&relative_path);

                return Err(error)
                    .with_context(|| format!("failed to read query: {}", path.display()));
            }
        }

        Err(error) => {
            let path = CONFIG.queries_dir.join(&relative_path);

            return Err(error).with_context(|| format!("failed to read query: {}", path.display()));
        }
    };

    for inherited_language in parse_inherits(&source) {
        load_query_recursive(inherited_language, query_name, optional, visited, output)?;
    }

    output.push_str(&source);

    if !source.ends_with('\n') {
        output.push('\n');
    }

    *output = output.replace(
        "injection.include-unnamed-children",
        "injection.include-children",
    );

    Ok(())
}

fn read_query(relative_path: &Path) -> io::Result<String> {
    fs::read_to_string(CONFIG.queries_dir.join(relative_path))
}

fn parse_inherits(source: &str) -> impl Iterator<Item = &str> {
    source
        .lines()
        .take_while(|line| {
            let line = line.trim();

            line.is_empty() || line.starts_with(';')
        })
        .find_map(|line| line.trim().strip_prefix("; inherits:").map(str::trim))
        .into_iter()
        .flat_map(|languages| languages.split(','))
        .map(str::trim)
        .filter(|language| !language.is_empty())
}
