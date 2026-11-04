use std::sync::LazyLock;

use anyhow::{Context, Result};
use comrak::{Node, nodes::NodeMath};
use katex::{KatexContext, Settings, render_to_string};
use maud::{PreEscaped, html};

use crate::markdown::MarkdownContext;

static KATEX: LazyLock<KatexContext> = LazyLock::new(KatexContext::default);

pub fn prepare_math(node: Node, math: &NodeMath, context: &mut MarkdownContext) -> Result<()> {
    let settings = Settings::builder().display_mode(math.display_math).build();
    let rendered = render_to_string(&KATEX, &math.literal, &settings)
        .with_context(|| format!("failed to render LaTeX expression: {:?}", math.literal))?;

    let html = html! {
        span class="prose-math"
            .prose-math--display[math.display_math]
            .scrollable[math.display_math]
            tabindex=[math.display_math.then_some("0")]
            role=[math.display_math.then_some("region")]
            aria-label=[math.display_math.then_some("Equation")]
        {
            (PreEscaped(rendered))
        }
    };

    context.replace_node(node, html.into_string());

    Ok(())
}
