use std::{
    collections::BTreeSet,
    ops::{Range, RangeInclusive},
};

use anyhow::{Context, Result, bail};

use crate::highlight::{
    highlighter::{HighlightEvent, Highlighter},
    html::{HtmlRenderer, render_plain},
    rainbow::{RAINBOW_NAMES, RainbowHighlight},
    registry::{HighlightLanguageKind, HighlightRegistry},
};

pub fn highlight(
    languages: &HighlightRegistry,
    language_kind: Option<&HighlightLanguageKind>,
    content: &str,
    lines: Option<&[RangeInclusive<usize>]>,
) -> Result<String> {
    let Some(language_kind) = language_kind else {
        return Ok(render_plain(content, lines));
    };

    let mut highlighter = Highlighter::new();

    let events = highlighter
        .highlight(
            languages.configuration(*language_kind),
            content.as_bytes(),
            None,
            |name| languages.configuration_for(name),
        )
        .with_context(|| {
            format!(
                "failed to highlight content as {}",
                language_kind.display_name()
            )
        })?;

    // Injected layers are discovered while consuming events, so resolve their overlays first.
    let events = events
        .collect::<Result<Vec<_>>>()
        .context("failed to process Tree-sitter highlight event")?;
    let rainbows = rainbow_ranges(&highlighter.rainbow_highlights);
    let mut rainbow_index = 0;
    let mut renderer = HtmlRenderer::new(content.len(), lines);

    for event in events {
        match event {
            HighlightEvent::Source { start, end } => {
                let source = content
                    .get(start..end)
                    .with_context(|| format!("invalid highlighted source range: {start}..{end}"))?;

                let mut offset = start;
                while let Some((range, color)) = rainbows.get(rainbow_index) {
                    if range.end <= offset {
                        rainbow_index += 1;
                        continue;
                    }
                    if range.start >= end {
                        break;
                    }
                    let rainbow_start = range.start.max(offset);
                    let rainbow_end = range.end.min(end);
                    renderer.push_source(
                        source
                            .get(offset - start..rainbow_start - start)
                            .context("invalid source range before rainbow highlight")?,
                    );
                    renderer.start_highlight(RAINBOW_NAMES[*color]);
                    renderer.push_source(
                        source
                            .get(rainbow_start - start..rainbow_end - start)
                            .context("invalid rainbow highlight range")?,
                    );
                    renderer.end_highlight();
                    offset = rainbow_end;
                    if offset == end {
                        break;
                    }
                }
                renderer.push_source(
                    source
                        .get(offset - start..)
                        .context("invalid source range after rainbow highlight")?,
                );
            }

            HighlightEvent::HighlightStart(highlight) => {
                let name = languages
                    .highlight_names()
                    .get(highlight.0)
                    .with_context(|| format!("invalid highlight index: {}", highlight.0))?;
                renderer.start_highlight(name);
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

fn rainbow_ranges(highlights: &[RainbowHighlight]) -> Vec<(Range<usize>, usize)> {
    let mut boundaries = Vec::with_capacity(highlights.len() * 2);
    for (index, highlight) in highlights.iter().enumerate() {
        boundaries.push((highlight.range.start, true, index));
        boundaries.push((highlight.range.end, false, index));
    }
    boundaries.sort_unstable();

    let mut active = BTreeSet::<(usize, usize)>::new();
    let mut ranges = Vec::new();
    let mut offset = 0;
    for (position, starts, index) in boundaries {
        if offset < position {
            // The innermost injected language wins when overlay ranges overlap.
            if let Some(&(_, index)) = active.last() {
                ranges.push((offset..position, highlights[index].color));
            }
            offset = position;
        }
        let key = (highlights[index].layer, index);
        if starts {
            active.insert(key);
        } else {
            active.remove(&key);
        }
    }
    ranges
}
