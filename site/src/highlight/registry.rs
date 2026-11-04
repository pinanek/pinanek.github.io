use anyhow::{Context, Result};
use rayon::prelude::*;
use tree_sitter_language::LanguageFn;

use crate::highlight::highlighter::HighlightConfiguration;

struct LanguageSpec {
    name: &'static str,
    display_name: &'static str,
    language_fn: LanguageFn,
    highlights: &'static str,
    injections: &'static str,
    locals: &'static str,
    rainbows: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/highlight_languages.rs"));

pub struct HighlightRegistry {
    configurations: Box<[HighlightConfiguration]>,
}

impl HighlightRegistry {
    pub fn new() -> Result<Self> {
        let configurations = HighlightLanguageKind::ALL
            .par_iter()
            .map(|language| {
                let mut configuration = HighlightConfiguration::new(
                    language.grammar().into(),
                    language.name(),
                    language.highlights(),
                    language.injections(),
                    language.locals(),
                    language.rainbows(),
                )
                .with_context(|| {
                    format!(
                        "failed to create highlight configuration for {}",
                        language.name()
                    )
                })?;

                configuration.configure(HIGHLIGHT_NAMES);
                Ok(configuration)
            })
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice();

        Ok(Self { configurations })
    }

    pub fn configuration(&self, language: HighlightLanguageKind) -> &HighlightConfiguration {
        &self.configurations[language as usize]
    }

    pub fn configuration_for(&self, name: &str) -> Option<&HighlightConfiguration> {
        let language = name.trim().parse().ok()?;
        Some(self.configuration(language))
    }

    pub fn highlight_names(&self) -> &'static [&'static str] {
        HIGHLIGHT_NAMES
    }
}
