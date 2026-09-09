use std::ops::RangeInclusive;

use anyhow::{Context, Result, bail};
use comrak::{Node, nodes::NodeCodeBlock};
use maud::{Markup, PreEscaped, html};

use crate::{
    highlight::{HighlightLanguageKind, highlight},
    markdown::MarkdownContext,
    templates::components::{
        check_icon, copy_icon, css_icon, docker_icon, html_icon, javascript_icon, json_icon,
        jsx_icon, markdown_icon, python_icon, rust_icon, typescript_icon,
    },
};

#[derive(Debug)]
struct ParsedCodeBlockInfo {
    language_name: Option<String>,
    title: Option<String>,
    lines: Option<Vec<RangeInclusive<usize>>>,
}

pub fn prepare_code_block(
    node: Node,
    code_block: &NodeCodeBlock,
    context: &mut MarkdownContext,
) -> Result<()> {
    let parsed = parse_code_block_info(&code_block.info)
        .with_context(|| format!("failed to parse code block info: {:?}", code_block.info))?;
    let language_kind = parsed
        .language_name
        .as_deref()
        .and_then(|name| name.parse::<HighlightLanguageKind>().ok());

    let highlighted = highlight(
        language_kind.as_ref(),
        &code_block.literal,
        parsed.lines.as_deref(),
    )
    .with_context(|| format!("failed to highlight code block: {:?}", code_block.info))?;

    let rendered = prose_code_block(
        parsed.title.as_deref(),
        language_kind.as_ref(),
        &highlighted,
    )
    .into_string();

    context.replace_node(node, rendered);

    Ok(())
}

fn parse_code_block_info(info: &str) -> Result<ParsedCodeBlockInfo> {
    let language_name = info
        .split_whitespace()
        .next()
        .filter(|value| !value.contains('='))
        .map(str::to_owned);

    let lines = parse_attribute(info, "lines")
        .map(|value| parse_line_ranges(&value))
        .transpose()
        .with_context(|| format!("invalid lines attribute in code block info: {info:?}"))?;

    Ok(ParsedCodeBlockInfo {
        language_name,
        title: parse_attribute(info, "title"),
        lines,
    })
}

fn parse_attribute(input: &str, name: &str) -> Option<String> {
    let prefix = format!("{name}=\"");
    let start = input.find(&prefix)? + prefix.len();
    let end = input[start..].find('"')? + start;

    Some(input[start..end].to_owned())
}

fn parse_line_ranges(input: &str) -> Result<Vec<RangeInclusive<usize>>> {
    input
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (start, end) = match part.split_once('-') {
                Some((start, end)) => (
                    start.trim().parse::<usize>().with_context(|| {
                        format!("invalid range start {start:?} in lines attribute {input:?}")
                    })?,
                    end.trim().parse::<usize>().with_context(|| {
                        format!("invalid range end {end:?} in lines attribute {input:?}")
                    })?,
                ),
                None => {
                    let line = part.parse::<usize>().with_context(|| {
                        format!("invalid line number {part:?} in lines attribute {input:?}")
                    })?;
                    (line, line)
                }
            };

            if start == 0 || start > end {
                bail!("invalid line range {part:?} in lines attribute {input:?}");
            }

            Ok(start..=end)
        })
        .collect()
}

pub fn prose_code_block(
    title: Option<&str>,
    language_kind: Option<&HighlightLanguageKind>,
    content: &str,
) -> Markup {
    html! {
        div class="prose-code-block" {
            div class="prose-code-block__header" {
                span class="prose-code-block__title" {
                    @if let Some(title) = title { (title) }
                }

                span class="prose-code-block__language" {
                    @if let Some(language_kind) = language_kind {
                        span class="prose-code-block__language-title" { (language_kind) }
                        (language_icon(language_kind))
                    } @else {
                        span class="prose-code-block__language-title" { "Plain" }
                    }
                }
            }
            button
                class="prose-code-block__copy"
                type="button"
                aria-label="Copy code"
                title="Copy code"
                data-copy-code
            { (copy_icon(Some("copy-icon"), None, None)) (check_icon(Some("check-icon"), None, None)) }

            pre class="prose-code-block__pre scrollable" {
                code class="scrollable-content" { (PreEscaped(content)) }
            }
        }
    }
}

fn language_icon(language: &HighlightLanguageKind) -> Markup {
    const CLASS: &str = "prose-code-block__language-icon";

    match language {
        HighlightLanguageKind::JavaScript => {
            javascript_icon(Some(CLASS), None, Some("var(--c-cpn-yellow)"))
        }
        HighlightLanguageKind::TypeScript => {
            typescript_icon(Some(CLASS), None, Some("var(--c-cpn-blue)"))
        }
        HighlightLanguageKind::Tsx => jsx_icon(Some(CLASS), None, Some("var(--c-cpn-sky)")),
        HighlightLanguageKind::Html => html_icon(Some(CLASS), None, Some("var(--c-cpn-peach)")),
        HighlightLanguageKind::Css => css_icon(Some(CLASS), None, Some("var(--c-cpn-blue)")),
        HighlightLanguageKind::Python => {
            python_icon(Some(CLASS), None, Some("var(--c-cpn-yellow)"))
        }
        HighlightLanguageKind::Rust => rust_icon(Some(CLASS), None, Some("var(--c-cpn-maroon)")),
        HighlightLanguageKind::Markdown => {
            markdown_icon(Some(CLASS), None, Some("var(--c-cpn-subtext0)"))
        }
        HighlightLanguageKind::Json => json_icon(Some(CLASS), None, Some("var(--c-cpn-yellow)")),
        HighlightLanguageKind::Dockerfile => {
            docker_icon(Some(CLASS), None, Some("var(--c-cpn-blue)"))
        }
    }
}
