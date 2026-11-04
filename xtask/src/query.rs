use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::{Component, Path},
    process::Command,
};

use anyhow::{Context, Result, bail, ensure};
use site_internal::{QuerySource, Repository};
use tree_sitter::{Node, Parser};

const QUERY_KINDS: &[&str] = &["highlights", "injections", "locals", "rainbows"];

pub fn update(root: &Path) -> Result<()> {
    let path = root.join("repositories.json");
    let mut repositories: Vec<Repository> = read_json(&path)?;
    let mut names = HashSet::new();

    for repository in &mut repositories {
        validate_name(&repository.name)?;
        ensure!(
            names.insert(repository.name.clone()),
            "duplicate repository: {}",
            repository.name
        );
        ensure!(
            !repository.url.is_empty() && !repository.url.starts_with('-'),
            "invalid repository URL"
        );
        let output = git(root, &["ls-remote", &repository.url, "HEAD"])?;
        let revision = output
            .lines()
            .find_map(|line| line.strip_suffix("\tHEAD"))
            .with_context(|| format!("no remote HEAD found for {}", repository.name))?;
        ensure!(
            revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "invalid remote revision for {}: {revision}",
            repository.name
        );
        println!("{}: {} -> {revision}", repository.name, repository.rev);
        repository.rev = revision.to_owned();
    }

    // Do not change any pins unless every remote was resolved successfully.
    let source = serde_json::to_string_pretty(&repositories)? + "\n";
    fs::write(&path, source).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn pull(root: &Path) -> Result<()> {
    let repositories: Vec<Repository> = read_json(&root.join("repositories.json"))?;
    let queries: BTreeMap<String, QuerySource> = read_json(&root.join("queries.json"))?;
    let mut names = HashSet::new();

    for repository in &repositories {
        validate_name(&repository.name)?;
        ensure!(
            names.insert(&repository.name),
            "duplicate repository: {}",
            repository.name
        );
        ensure!(
            repository.rev.len() == 40 && repository.rev.bytes().all(|b| b.is_ascii_hexdigit()),
            "repository {} must be pinned to a full commit SHA",
            repository.name
        );
        ensure!(!repository.url.starts_with('-'), "invalid repository URL");
    }

    for (name, query) in &queries {
        validate_name(name)?;
        ensure!(
            names.contains(&query.repo),
            "unknown repository {} for {name}",
            query.repo
        );
        ensure!(
            !query.path.is_empty()
                && Path::new(&query.path)
                    .components()
                    .all(|part| matches!(part, Component::Normal(_))),
            "invalid query path for {name}: {}",
            query.path
        );
        ensure!(
            query.grammar_crate.is_some() == query.language.is_some(),
            "{name} must specify both crate and language, or neither"
        );
    }

    let temporary =
        tempfile::tempdir().context("failed to create temporary repository directory")?;
    let mut sources = HashMap::new();
    let mut licenses = BTreeMap::new();
    for repository in &repositories {
        let repository_dir = temporary.path().join(&repository.name);
        fs::create_dir_all(&repository_dir)?;
        git(&repository_dir, &["init", "--bare", "--quiet"])?;
        println!("Fetching {} at {}", repository.name, repository.rev);
        git(
            &repository_dir,
            &[
                "fetch",
                "--quiet",
                "--depth=1",
                &repository.url,
                &repository.rev,
            ],
        )?;
        let commit = format!("{}^{{commit}}", repository.rev);
        git(&repository_dir, &["rev-parse", "--verify", &commit])?;
        let entries = git(
            &repository_dir,
            &["ls-tree", "--name-only", &repository.rev],
        )?;
        for filename in entries
            .lines()
            .filter(|name| name.starts_with("LICENSE") || name.starts_with("COPYING"))
        {
            let source = git(
                &repository_dir,
                &["show", &format!("{}:{filename}", repository.rev)],
            )?;
            licenses.insert((repository.name.clone(), filename.to_owned()), source);
        }

        for (name, query) in queries
            .iter()
            .filter(|(_, query)| query.repo == repository.name)
        {
            let directory = git(
                &repository_dir,
                &[
                    "ls-tree",
                    "-d",
                    "--name-only",
                    &repository.rev,
                    "--",
                    &query.path,
                ],
            )?;
            ensure!(
                !directory.trim().is_empty(),
                "missing query directory {} in {}",
                query.path,
                repository.name
            );

            for kind in QUERY_KINDS {
                let path = format!("{}/{kind}.scm", query.path);
                let entry = git(
                    &repository_dir,
                    &["ls-tree", "--name-only", &repository.rev, "--", &path],
                )?;
                if !entry.trim().is_empty() {
                    let object = format!("{}:{path}", repository.rev);
                    let source = git(&repository_dir, &["show", &object])?;
                    sources.insert((name.clone(), (*kind).to_owned()), source);
                }
            }
        }
    }

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_tsquery::LANGUAGE.into())?;
    let mut generated = BTreeMap::new();

    // Resolve and validate everything before replacing any output files.
    for (name, _) in queries.iter().filter(|(_, query)| query.language.is_some()) {
        for kind in QUERY_KINDS {
            let mut expanded = String::new();
            let mut visited = HashSet::new();
            expand(
                name,
                kind,
                &queries,
                &sources,
                &mut visited,
                &mut Vec::new(),
                &mut parser,
                &mut expanded,
            )?;
            if !expanded.trim().is_empty() {
                let minified = minify(&mut parser, &expanded)
                    .with_context(|| format!("failed to minify {name}/{kind}.scm"))?;
                let mut output = String::from("; Generated by cargo xtask query pull.\n");
                let mut inherited = visited.into_iter().collect::<Vec<_>>();
                inherited.sort();
                for source_name in inherited {
                    if sources.contains_key(&(source_name.clone(), (*kind).to_owned())) {
                        let query = &queries[&source_name];
                        let repository = repositories
                            .iter()
                            .find(|repo| repo.name == query.repo)
                            .expect("validated repository reference");
                        output.push_str(&format!(
                            "; Source: {} @ {} {}/{kind}.scm\n",
                            repository.url, repository.rev, query.path
                        ));
                    }
                }
                output.push_str("; Licenses: ../licenses/ (see ../README.md).\n");
                output.push_str(&minified);
                generated.insert((name.clone(), (*kind).to_owned()), output);
            }
        }
    }

    for ((repository, filename), source) in licenses {
        let directory = root.join("queries/licenses").join(repository);
        fs::create_dir_all(&directory)?;
        fs::write(directory.join(filename), source)?;
    }

    for (name, _) in queries.iter().filter(|(_, query)| query.language.is_some()) {
        let directory = root.join("queries").join(name);
        fs::create_dir_all(&directory)?;
        for kind in QUERY_KINDS {
            let path = directory.join(format!("{kind}.scm"));
            if let Some(source) = generated.get(&(name.clone(), (*kind).to_owned())) {
                fs::write(&path, source)
                    .with_context(|| format!("failed to write {}", path.display()))?;
            } else if path.exists() {
                fs::remove_file(&path)?;
            }
        }
    }

    println!("Wrote {} queries to queries/", generated.len());
    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&source).with_context(|| format!("invalid JSON in {}", path.display()))
}

fn validate_name(name: &str) -> Result<()> {
    ensure!(
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
            && name != "."
            && name != "..",
        "invalid manifest name: {name}"
    );
    Ok(())
}

fn git(directory: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .context("failed to run system git")?;
    ensure!(
        output.status.success(),
        "git {} failed in {}: {}",
        args.join(" "),
        directory.display(),
        String::from_utf8_lossy(&output.stderr).trim()
    );
    String::from_utf8(output.stdout).context("git returned non-UTF-8 content")
}

#[expect(clippy::too_many_arguments)]
fn expand(
    name: &str,
    kind: &str,
    queries: &BTreeMap<String, QuerySource>,
    sources: &HashMap<(String, String), String>,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>,
    parser: &mut Parser,
    output: &mut String,
) -> Result<()> {
    ensure!(
        queries.contains_key(name),
        "unknown inherited query source: {name}"
    );
    if stack.iter().any(|entry| entry == name) {
        bail!(
            "query inheritance cycle for {kind}: {} -> {name}",
            stack.join(" -> ")
        );
    }
    if !visited.insert(name.to_owned()) {
        return Ok(());
    }
    let Some(source) = sources.get(&(name.to_owned(), kind.to_owned())) else {
        return Ok(());
    };
    stack.push(name.to_owned());
    let tree = parser
        .parse(source, None)
        .context("query parsing cancelled")?;
    ensure!(
        !tree.root_node().has_error(),
        "invalid query syntax in {name}/{kind}.scm"
    );
    let mut comments = Vec::new();
    collect_comments(tree.root_node(), &mut comments);
    let mut offset = 0;

    for comment in comments {
        let text = &source[comment.byte_range()];
        if let Some(inherits) = text
            .trim_start_matches(';')
            .trim()
            .strip_prefix("inherits:")
        {
            output.push_str(&source[offset..comment.start_byte()]);
            output.push('\n');
            for inherited in inherits
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
            {
                expand(
                    inherited, kind, queries, sources, visited, stack, parser, output,
                )?;
            }
            offset = comment.end_byte();
        }
    }
    output.push_str(&source[offset..]);
    output.push('\n');
    stack.pop();
    Ok(())
}

fn collect_comments<'tree>(node: Node<'tree>, comments: &mut Vec<Node<'tree>>) {
    if node.kind() == "comment" {
        comments.push(node);
    } else if node.kind() != "string" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_comments(child, comments);
        }
    }
}

fn minify(parser: &mut Parser, source: &str) -> Result<String> {
    let tree = parser
        .parse(source, None)
        .context("query parsing cancelled")?;
    ensure!(
        !tree.root_node().has_error(),
        "invalid expanded query syntax"
    );
    let mut tokens = Vec::new();
    collect_tokens(tree.root_node(), &mut tokens);
    let mut output = String::new();
    let mut comments = Vec::new();
    collect_comments(tree.root_node(), &mut comments);
    for comment in comments {
        let text = &source[comment.byte_range()];
        let lower = text.to_ascii_lowercase();
        if lower.contains("copyright") || lower.contains("license") || lower.contains("spdx") {
            output.push_str(text);
            output.push('\n');
        }
    }
    let mut end = 0;

    for token in tokens {
        if !output.is_empty() && token.start_byte() > end {
            output.push(' ');
        }
        output.push_str(&source[token.byte_range()]);
        end = token.end_byte();
    }
    output.push('\n');
    let minified = parser
        .parse(&output, None)
        .context("query parsing cancelled")?;
    ensure!(
        !minified.root_node().has_error()
            && tree.root_node().to_sexp().replace(" (comment)", "")
                == minified.root_node().to_sexp().replace(" (comment)", ""),
        "minification changed the query structure"
    );
    Ok(output)
}

fn collect_tokens<'tree>(node: Node<'tree>, tokens: &mut Vec<Node<'tree>>) {
    if node.kind() == "comment" {
        return;
    }
    if node.kind() == "string" || node.child_count() == 0 {
        tokens.push(node);
    } else {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_tokens(child, tokens);
        }
    }
}
