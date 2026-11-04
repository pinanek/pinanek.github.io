use std::{collections::HashMap, fmt::Write};

use comrak::{
    Node,
    html::{ChildRendering, Context as MarkdownHtmlContext},
    nodes::{NodeTaskItem, NodeValue},
};
use maud::html;

use crate::markdown::MarkdownNodeAction;

pub fn format_task_item(
    context: &mut MarkdownHtmlContext<'_, '_, &HashMap<usize, MarkdownNodeAction>>,
    node: Node<'_>,
    entering: bool,
    task: &NodeTaskItem,
) -> std::result::Result<ChildRendering, std::fmt::Error> {
    let in_list = node
        .parent()
        .is_some_and(|parent| matches!(&parent.data().value, NodeValue::List(_)));

    if entering {
        if in_list {
            context.write_str("<li>")?;
        }

        let text = node
            .first_child()
            .map(|child| child.collect_text())
            .unwrap_or_default();
        let label = if text.trim().is_empty() {
            "Task"
        } else {
            &text
        };
        let checkbox = html! {
            input type="checkbox" checked[task.symbol.is_some()] disabled aria-label=(label);
        };

        context.write_str(&checkbox.into_string())?;
        context.write_char(' ')?;
    } else if in_list {
        context.write_str("</li>\n")?;
    }

    Ok(ChildRendering::HTML)
}
