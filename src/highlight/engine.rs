use std::ops::RangeInclusive;

use anyhow::{Context, Result, bail};
use tree_sitter_highlight::{HighlightEvent, Highlighter};

use crate::highlight::{
    html::{HtmlRenderer, render_plain},
    registry::HighlightLanguageKind,
};

pub fn highlight(
    language_kind: Option<&HighlightLanguageKind>,
    content: &str,
    lines: Option<&[RangeInclusive<usize>]>,
) -> Result<String> {
    let Some(language_kind) = language_kind else {
        return Ok(render_plain(content, lines));
    };

    let language = language_kind.language();

    let mut highlighter = Highlighter::new();

    let events = highlighter
        .highlight(
            language.configuration(),
            content.as_bytes(),
            None,
            HighlightLanguageKind::configuration_for,
        )
        .with_context(|| {
            format!(
                "failed to highlight content as {}",
                language_kind.display_name()
            )
        })?;

    let mut renderer = HtmlRenderer::new(content.len(), language.highlight_names(), lines);

    for event in events {
        match event.context("failed to process Tree-sitter highlight event")? {
            HighlightEvent::Source { start, end } => {
                let source = content
                    .get(start..end)
                    .with_context(|| format!("invalid highlighted source range: {start}..{end}"))?;

                renderer.push_source(source);
            }

            HighlightEvent::HighlightStart(highlight) => {
                if !renderer.start_highlight(highlight.0) {
                    bail!("invalid highlight index: {}", highlight.0);
                }
            }

            HighlightEvent::HighlightEnd => {
                if !renderer.end_highlight() {
                    bail!("unbalanced Tree-sitter highlight events");
                }
            }
        }
    }

    if renderer.has_active_highlights() {
        bail!("unbalanced Tree-sitter highlight events");
    }

    Ok(renderer.finish())
}
