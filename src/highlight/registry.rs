use std::{collections::BTreeSet, sync::LazyLock};

use anyhow::{Context, Result};
use strum::{Display, EnumString, IntoStaticStr};
use tree_sitter::{Language, Query};
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_language::LanguageFn;

use crate::highlight::query::{load_helix_query, load_optional_helix_query};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr)]
#[repr(usize)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum HighlightLanguageKind {
    #[strum(to_string = "CSS")]
    Css,

    #[strum(
        to_string = "Dockerfile",
        serialize = "docker",
        serialize = "containerfile"
    )]
    Dockerfile,

    #[strum(to_string = "HTML")]
    Html,

    #[strum(to_string = "JavaScript", serialize = "javascript", serialize = "js")]
    JavaScript,

    #[strum(to_string = "JSON", serialize = "json", serialize = "jsonc")]
    Json,

    #[strum(to_string = "Markdown", serialize = "markdown", serialize = "md")]
    Markdown,

    #[strum(to_string = "Python", serialize = "python", serialize = "py")]
    Python,

    #[strum(to_string = "Rust", serialize = "rust", serialize = "rs")]
    Rust,

    #[strum(to_string = "TSX")]
    Tsx,

    #[strum(to_string = "TypeScript", serialize = "typescript", serialize = "ts")]
    TypeScript,
}

impl HighlightLanguageKind {
    pub fn language(self) -> &'static HighlightLanguage {
        HIGHLIGHT_REGISTRY.language(self)
    }

    pub fn display_name(self) -> &'static str {
        LANGUAGE_SPECS[self as usize].display_name
    }

    pub fn configuration_for<'config>(
        language_name: &str,
    ) -> Option<&'config HighlightConfiguration> {
        let language_kind = language_name.trim().parse::<Self>().ok()?;

        let configuration: &'static HighlightConfiguration =
            language_kind.language().configuration();

        Some(configuration)
    }
}

enum QuerySource {
    Helix,

    Builtin {
        highlights: &'static str,
        injections: &'static str,
        locals: &'static str,
    },
}

struct LanguageSpec {
    name: &'static str,
    display_name: &'static str,
    language_fn: LanguageFn,
    queries: QuerySource,
}

const LANGUAGE_SPECS: &[LanguageSpec] = &[
    LanguageSpec {
        name: "css",
        display_name: "CSS",
        language_fn: tree_sitter_css::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "dockerfile",
        display_name: "Docker",
        language_fn: tree_sitter_containerfile::LANGUAGE,
        queries: QuerySource::Builtin {
            highlights: tree_sitter_containerfile::HIGHLIGHTS_QUERY,
            injections: tree_sitter_containerfile::INJECTIONS_QUERY,
            locals: "",
        },
    },
    LanguageSpec {
        name: "html",
        display_name: "HTML",
        language_fn: tree_sitter_html::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "javascript",
        display_name: "JavaScript",
        language_fn: tree_sitter_javascript::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "json",
        display_name: "JSON",
        language_fn: tree_sitter_json::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "markdown",
        display_name: "Markdown",
        language_fn: tree_sitter_md::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "python",
        display_name: "Python",
        language_fn: tree_sitter_python::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "rust",
        display_name: "Rust",
        language_fn: tree_sitter_rust::LANGUAGE,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "tsx",
        display_name: "TSX",
        language_fn: tree_sitter_typescript::LANGUAGE_TSX,
        queries: QuerySource::Helix,
    },
    LanguageSpec {
        name: "typescript",
        display_name: "TypeScript",
        language_fn: tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        queries: QuerySource::Helix,
    },
];

struct LoadedLanguage {
    name: &'static str,
    language: Language,
    highlights_query: String,
    injections_query: String,
    locals_query: String,
}

impl LoadedLanguage {
    fn load(spec: &LanguageSpec) -> Result<Self> {
        let language: Language = spec.language_fn.into();

        let (highlights_query, injections_query, locals_query) = match spec.queries {
            QuerySource::Helix => {
                let highlights_query =
                    load_helix_query(spec.name, "highlights").with_context(|| {
                        format!("failed to load highlights query for {}", spec.name)
                    })?;

                let injections_query = load_optional_helix_query(spec.name, "injections")
                    .with_context(|| {
                        format!("failed to load injections query for {}", spec.name)
                    })?;

                let locals_query = load_optional_helix_query(spec.name, "locals")
                    .with_context(|| format!("failed to load locals query for {}", spec.name))?;

                (highlights_query, injections_query, locals_query)
            }

            QuerySource::Builtin {
                highlights,
                injections,
                locals,
            } => (
                highlights.to_owned(),
                injections.to_owned(),
                locals.to_owned(),
            ),
        };

        Ok(Self {
            name: spec.name,
            language,
            highlights_query,
            injections_query,
            locals_query,
        })
    }
}

pub struct HighlightLanguage {
    configuration: HighlightConfiguration,
}

impl HighlightLanguage {
    pub fn configuration(&self) -> &HighlightConfiguration {
        &self.configuration
    }

    pub fn highlight_names(&self) -> &'static [String] {
        HIGHLIGHT_REGISTRY.highlight_names()
    }
}

struct HighlightRegistry {
    languages: Box<[HighlightLanguage]>,
    highlight_names: Box<[String]>,
}

impl HighlightRegistry {
    fn new() -> Result<Self> {
        let loaded_languages = LANGUAGE_SPECS
            .iter()
            .map(LoadedLanguage::load)
            .collect::<Result<Vec<_>>>()?;

        let highlight_names = Self::collect_highlight_names(&loaded_languages)?;

        let languages = {
            let highlight_name_refs = highlight_names
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();

            loaded_languages
                .into_iter()
                .map(|loaded| {
                    let mut configuration = HighlightConfiguration::new(
                        loaded.language,
                        loaded.name,
                        &loaded.highlights_query,
                        &loaded.injections_query,
                        &loaded.locals_query,
                    )
                    .with_context(|| {
                        format!(
                            "failed to create highlight \
                                 configuration for {}",
                            loaded.name
                        )
                    })?;

                    configuration.configure(&highlight_name_refs);

                    Ok(HighlightLanguage { configuration })
                })
                .collect::<Result<Vec<_>>>()?
                .into_boxed_slice()
        };

        assert_eq!(
            languages.len(),
            LANGUAGE_SPECS.len(),
            "language registry is incomplete"
        );

        Ok(Self {
            languages,
            highlight_names,
        })
    }

    fn collect_highlight_names(languages: &[LoadedLanguage]) -> Result<Box<[String]>> {
        let mut names = BTreeSet::new();

        for language in languages {
            let query =
                Query::new(&language.language, &language.highlights_query).with_context(|| {
                    format!("failed to parse highlight query for {}", language.name)
                })?;

            names.extend(
                query
                    .capture_names()
                    .iter()
                    .filter(|name| !name.starts_with('_'))
                    .map(|name| (*name).to_owned()),
            );
        }

        Ok(names.into_iter().collect::<Vec<_>>().into_boxed_slice())
    }

    fn language(&self, language_kind: HighlightLanguageKind) -> &HighlightLanguage {
        &self.languages[language_kind as usize]
    }

    fn highlight_names(&self) -> &[String] {
        &self.highlight_names
    }
}

static HIGHLIGHT_REGISTRY: LazyLock<HighlightRegistry> = LazyLock::new(|| {
    HighlightRegistry::new().unwrap_or_else(|error| {
        panic!(
            "failed to initialize highlight registry: \
                 {error:#}"
        )
    })
});
