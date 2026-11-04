use std::{collections::HashMap, fmt::Write};

use comrak::{
    Node,
    html::{ChildRendering, Context as MarkdownHtmlContext, format_node_default},
};

use crate::markdown::MarkdownNodeAction;

pub fn format_table(
    context: &mut MarkdownHtmlContext<'_, '_, &HashMap<usize, MarkdownNodeAction>>,
    node: Node<'_>,
    entering: bool,
) -> std::result::Result<ChildRendering, std::fmt::Error> {
    if entering {
        context.write_str(
            "<div class=\"prose-table scrollable\" tabindex=\"0\" role=\"region\" aria-label=\"Table\">",
        )?;
    }

    let rendering = format_node_default(context, node, entering)?;

    if !entering {
        context.write_str("</div>\n")?;
    }

    Ok(rendering)
}
