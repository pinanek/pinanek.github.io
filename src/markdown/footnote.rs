use anyhow::Result;
use comrak::{Node, nodes::NodeFootnoteDefinition};
use maud::{Markup, PreEscaped, html};

use crate::markdown::MarkdownContext;

pub struct ParsedFootnoteDef<'a> {
    pub node: Node<'a>,
    pub name: String,
    pub reference_count: usize,
}

pub struct ParsedFootnote {
    pub number: usize,
    pub reference_count: usize,
    pub content: String,
}

pub fn prepare_footnote_def<'a>(
    node: Node<'a>,
    footnote: &NodeFootnoteDefinition,
    context: &mut MarkdownContext<'a>,
) -> Result<()> {
    context.skip_node(node);
    context.footnote_definitions.push(ParsedFootnoteDef {
        node,
        name: footnote.name.to_owned(),
        reference_count: footnote.total_references as usize,
    });

    Ok(())
}

pub fn prose_footnote(footnote: &ParsedFootnote) -> Markup {
    let number = footnote.number;

    let reference_anchor = format!("--prose-footnote-{number}-1");
    let note_anchor = format!("--prose-footnote-note-{number}");

    let style = if number == 1 {
        format!(
            "position-anchor: {reference_anchor}; \
             anchor-name: {note_anchor}; \
             inset-block-start: calc(anchor({reference_anchor} top) - 0.1rem)"
        )
    } else {
        let previous_note_anchor = format!("--prose-footnote-note-{}", number - 1);

        format!(
            "position-anchor: {reference_anchor}; \
             anchor-name: {note_anchor}; \
             inset-block-start: max(\
               calc(anchor({reference_anchor} top)), \
               calc(anchor({previous_note_anchor} bottom))\
             )"
        )
    };

    html! {
        li id=(format!("footnote-{number}")) class="prose-footnote" role="doc-endnote" style=(style) {
            a   class="prose-footnote__number"
                href=(format!("#footnote-ref-{number}-1"))
                aria-label=(format!("Return to footnote {number} reference"))
            { (number) }

            div class="prose-footnote__content" {
                (PreEscaped(&footnote.content))

                @if footnote.reference_count > 1 {
                    span class="prose-footnote__backrefs" {
                        @for occurrence in 2..=footnote.reference_count {
                            a   href=(format!("#footnote-ref-{number}-{occurrence}"))
                                aria-label=({
                                    format!(
                                        "Return to reference {occurrence} of footnote {number}",
                                    )
                                })
                            { "↩" (occurrence) }
                        }
                    }
                }
            }
        }
    }
}

pub fn prose_footnotes(footnotes: &[ParsedFootnote]) -> Markup {
    html! {
        h2 class="prose-footnotes__title" { "Footnotes" }

        section class="prose-footnotes" aria-label="Footnotes" role="doc-endnotes" {
            ol class="prose-footnotes__list" {
                @for footnote in footnotes { (prose_footnote(footnote)) }
            }
        }
    }
}
