use anyhow::Result;
use comrak::{Node, nodes::NodeFootnoteReference};
use maud::{Markup, html};

use crate::markdown::MarkdownContext;

pub fn pepare_footnote_ref(
    node: Node,
    footnote_ref: &NodeFootnoteReference,
    context: &mut MarkdownContext,
) -> Result<()> {
    let number = footnote_ref.ix as usize;
    let occurrence = footnote_ref.ref_num as usize;

    let rendered = prose_footnote_ref(number, occurrence).into_string();

    context.replace_node(node, rendered);
    context
        .footnote_numbers
        .entry(footnote_ref.name.to_owned())
        .or_insert(number);

    Ok(())
}

pub fn prose_footnote_ref(number: usize, occurrence: usize) -> Markup {
    html! {
        sup id=(format!("footnote-ref-{number}-{occurrence}"))
            class="prose-footnote-ref"
            style=(format!("anchor-name: --prose-footnote-{number}-{occurrence}"))
        {
            a   href=(format!("#footnote-{number}"))
                role="doc-noteref"
                aria-label=(format!("Footnote {number}"))
            { (number) }
        }
    }
}
