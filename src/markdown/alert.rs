use anyhow::Result;
use comrak::{
    Node,
    nodes::{AlertType as ComrakAlertKind, NodeAlert},
};
use maud::{Markup, PreEscaped, html};
use strum::{Display, EnumString, IntoStaticStr};

use crate::{markdown::MarkdownContext, utils::uppercase_first};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum AlertKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl From<ComrakAlertKind> for AlertKind {
    fn from(value: ComrakAlertKind) -> Self {
        match value {
            ComrakAlertKind::Note => Self::Note,
            ComrakAlertKind::Tip => Self::Tip,
            ComrakAlertKind::Important => Self::Important,
            ComrakAlertKind::Warning => Self::Warning,
            ComrakAlertKind::Caution => Self::Caution,
        }
    }
}

#[derive(Debug)]
pub struct ParsedAlert<'a> {
    pub node: Node<'a>,
    pub kind: AlertKind,
}

pub fn prepare_alert<'a>(
    node: Node<'a>,
    alert: &NodeAlert,
    context: &mut MarkdownContext<'a>,
) -> Result<()> {
    context.alerts.push(ParsedAlert {
        node,
        kind: alert.alert_type.into(),
    });

    Ok(())
}

pub fn prose_alert(kind: AlertKind, content: &str) -> Markup {
    html! {
        aside
            class=(format!("prose-alert prose-alert--{}", kind))
            data-alert-kind=(kind)
        {
            div class="prose-alert__header" {
                span class="prose-alert__title" { (uppercase_first(&kind.to_string())) }
            }
            div class="prose-alert__content" { (PreEscaped(content)) }
        }
    }
}
