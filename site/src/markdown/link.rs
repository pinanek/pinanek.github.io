use anyhow::Result;
use comrak::{Node, nodes::NodeLink};
use maud::{Markup, PreEscaped, html};

use crate::{
    markdown::MarkdownContext, templates::components::external_link_icon, utils::is_external_url,
};

pub fn prepare_link(node: Node, link: &NodeLink, context: &mut MarkdownContext) -> Result<()> {
    let rendered = prose_link(&link.url, &link.title, &node.collect_text()).into_string();

    context.replace_node(node, rendered);

    Ok(())
}

pub fn prose_link(href: &str, title: &str, content: &str) -> Markup {
    let external = is_external_url(href);

    html! {
        a   class="prose-link"
            href=(href)
            title=(title)
            target=[external.then_some("_blank")]
            rel=[external.then_some("noopener noreferrer")]
        {
            (PreEscaped(content))

            @if external {
                (external_link_icon(Some("prose-link__external-icon"), Some("0.9em"), None))
            }
        }
    }
}
