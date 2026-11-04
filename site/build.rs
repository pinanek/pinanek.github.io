use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
};

use anyhow::{Context, Result, ensure};
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use site_internal::QuerySource;
use tree_sitter::{Node, Parser};

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("site must be inside the workspace")?;
    let manifest_path = root.join("queries.json");
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let queries: BTreeMap<String, QuerySource> = serde_json::from_str(
        &fs::read_to_string(&manifest_path).context("failed to read queries.json")?,
    )
    .context("invalid queries.json")?;

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_tsquery::LANGUAGE.into())?;
    let mut variants = Vec::new();
    let mut variant_identifiers = Vec::new();
    let mut specs = Vec::new();
    let mut names = BTreeSet::new();
    let mut variant_names = BTreeSet::new();
    let mut aliases = BTreeMap::new();

    for (name, source) in &queries {
        ensure!(
            source.grammar_crate.is_some() == source.language.is_some(),
            "{name} must specify both crate and language, or neither"
        );
        let (Some(grammar_crate), Some(language)) = (&source.grammar_crate, &source.language)
        else {
            ensure!(
                source.variant.is_none()
                    && source.display_name.is_none()
                    && source.aliases.is_empty(),
                "shared fragment {name} cannot specify variant, display_name, or aliases"
            );
            continue;
        };
        ensure!(
            !name.is_empty()
                && name != "."
                && name != ".."
                && name
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.')),
            "invalid query name: {name}"
        );
        let variant_name = source
            .variant
            .as_deref()
            .with_context(|| format!("missing variant for {name}"))?;
        ensure!(
            variant_names.insert(variant_name),
            "duplicate language variant: {variant_name}"
        );
        let variant = identifier(variant_name)?;
        let grammar_crate = identifier(grammar_crate)?;
        let language = identifier(language)?;
        let display_name = source
            .display_name
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .with_context(|| format!("missing display_name for {name}"))?;
        let mut language_aliases = BTreeSet::new();
        for alias in std::iter::once(name.as_str())
            .chain(source.aliases.iter().map(String::as_str))
            .chain(std::iter::once(display_name))
        {
            ensure!(
                !alias.is_empty() && alias.trim() == alias,
                "invalid alias for {name}: {alias:?}"
            );
            let normalized = alias.to_ascii_lowercase();
            if language_aliases.insert(normalized.clone())
                && let Some(previous) = aliases.insert(normalized, name)
            {
                anyhow::bail!("alias {alias:?} is shared by {previous} and {name}");
            }
        }
        variants.push(quote! {
            #[strum(to_string = #display_name, #(serialize = #language_aliases),*)]
            #variant
        });
        variant_identifiers.push(variant);

        let directory = root.join("queries").join(name);
        println!("cargo:rerun-if-changed={}", directory.display());
        let mut embedded = Vec::new();
        for kind in ["highlights", "injections", "locals", "rainbows"] {
            let path = directory.join(format!("{kind}.scm"));
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(error)
                    if error.kind() == std::io::ErrorKind::NotFound && kind != "highlights" =>
                {
                    embedded.push(quote! { "" });
                    continue;
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!(
                            "failed to read {}; run cargo xtask query pull first",
                            path.display()
                        )
                    });
                }
            };
            println!("cargo:rerun-if-changed={}", path.display());
            let tree = parser
                .parse(&content, None)
                .context("query parsing cancelled")?;
            ensure!(
                !tree.root_node().has_error(),
                "invalid query syntax in {}",
                path.display()
            );
            if kind == "highlights" {
                collect_highlight_names(tree.root_node(), &content, &mut names)?;
            }
            let path = path.to_str().context("query path is not UTF-8")?;
            embedded.push(quote! { include_str!(#path) });
        }
        let [highlights, injections, locals, rainbows] = embedded.as_slice() else {
            unreachable!("all four query types are embedded");
        };
        specs.push(quote! {
            LanguageSpec {
                name: #name,
                display_name: #display_name,
                language_fn: #grammar_crate::#language,
                highlights: #highlights,
                injections: #injections,
                locals: #locals,
                rainbows: #rainbows,
            }
        });
    }
    ensure!(!variants.is_empty(), "queries.json defines no languages");

    let generated = quote! {
        #[derive(Debug, strum::Display, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::IntoStaticStr)]
        #[repr(usize)]
        #[strum(ascii_case_insensitive)]
        pub enum HighlightLanguageKind {
            #(#variants),*
        }

        impl HighlightLanguageKind {
            pub const ALL: &'static [Self] = &[#(Self::#variant_identifiers),*];

            pub const fn name(self) -> &'static str {
                LANGUAGE_SPECS[self as usize].name
            }

            pub const fn display_name(self) -> &'static str {
                LANGUAGE_SPECS[self as usize].display_name
            }

            pub const fn grammar(self) -> LanguageFn {
                LANGUAGE_SPECS[self as usize].language_fn
            }

            pub const fn highlights(self) -> &'static str {
                LANGUAGE_SPECS[self as usize].highlights
            }

            pub const fn injections(self) -> &'static str {
                LANGUAGE_SPECS[self as usize].injections
            }

            pub const fn locals(self) -> &'static str {
                LANGUAGE_SPECS[self as usize].locals
            }

            pub const fn rainbows(self) -> &'static str {
                LANGUAGE_SPECS[self as usize].rainbows
            }
        }

        const LANGUAGE_SPECS: &[LanguageSpec] = &[#(#specs),*];
        const HIGHLIGHT_NAMES: &[&str] = &[#(#names),*];
    };
    let output = env::var_os("OUT_DIR").context("OUT_DIR is not set")?;
    fs::write(
        Path::new(&output).join("highlight_languages.rs"),
        generated.to_string(),
    )?;
    Ok(())
}

fn identifier(value: &str) -> Result<Ident> {
    let tokens: TokenStream = value
        .parse()
        .map_err(|error| anyhow::anyhow!("invalid Rust identifier {value:?}: {error}"))?;
    let mut tokens = tokens.into_iter();
    let Some(TokenTree::Ident(identifier)) = tokens.next() else {
        anyhow::bail!("invalid Rust identifier: {value:?}");
    };
    ensure!(
        tokens.next().is_none(),
        "invalid Rust identifier: {value:?}"
    );
    Ok(identifier)
}

fn collect_highlight_names(node: Node, source: &str, names: &mut BTreeSet<String>) -> Result<()> {
    if node.kind() == "capture" {
        let name = node
            .child_by_field_name("name")
            .context("capture has no name")?;
        let name = name.utf8_text(source.as_bytes())?;
        if !name.starts_with('_') {
            names.insert(name.to_owned());
        }
    } else {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_highlight_names(child, source, names)?;
        }
    }
    Ok(())
}
