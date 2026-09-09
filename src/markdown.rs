mod alert;
mod code_block;
mod figure;
mod footnote;
mod footnote_ref;
mod heading;
mod link;

use std::{collections::HashMap, fmt::Write, path::Path};

use anyhow::{Context, Result};
use comrak::{
    Arena, Node, Options,
    html::{
        ChildRendering, Context as MarkdownHtmlContext, format_document_with_formatter,
        format_node_default,
    },
    nodes::{AstNode, NodeValue},
    options::Plugins,
    parse_document,
};
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    assets::image::ImageRequests,
    markdown::{
        alert::{ParsedAlert, prepare_alert, prose_alert},
        code_block::prepare_code_block,
        figure::prepare_figure,
        footnote::{ParsedFootnote, ParsedFootnoteDef, prepare_footnote_def, prose_footnotes},
        footnote_ref::pepare_footnote_ref,
        heading::{ParsedTocItem, prepare_heading, prose_toc},
        link::prepare_link,
    },
};

pub enum MarkdownNodeAction {
    Replace(String),
    Skip,
}

pub struct MarkdownContext<'a> {
    pub content_dir: &'a Path,
    pub alerts: Vec<ParsedAlert<'a>>,
    pub footnote_numbers: HashMap<String, usize>,
    pub footnote_definitions: Vec<ParsedFootnoteDef<'a>>,
    pub toc_items: Vec<ParsedTocItem>,
    images: ImageRequests,
    actions: HashMap<usize, MarkdownNodeAction>,
}

impl<'a> MarkdownContext<'a> {
    fn new(content_dir: &'a Path) -> Self {
        Self {
            content_dir,
            actions: HashMap::new(),
            images: ImageRequests::new(),
            alerts: Vec::new(),
            footnote_numbers: HashMap::new(),
            footnote_definitions: Vec::new(),
            toc_items: Vec::new(),
        }
    }

    pub fn replace_node(&mut self, node: Node, rendered: String) {
        self.insert_action(node, MarkdownNodeAction::Replace(rendered));
    }

    pub fn skip_node(&mut self, node: Node) {
        self.insert_action(node, MarkdownNodeAction::Skip);
    }

    fn insert_action(&mut self, node: Node, action: MarkdownNodeAction) {
        let id = std::ptr::from_ref(node).addr();
        self.actions.insert(id, action);
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct MarkdownOptions {
    pub toc: bool,
    pub sidenote: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            toc: true,
            sidenote: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MarkdownFrontmatter<T> {
    #[serde(flatten)]
    pub metadata: T,

    #[serde(default)]
    pub options: MarkdownOptions,
}

pub struct RenderedMarkdown<T> {
    pub metadata: T,
    pub html: String,
    pub images: ImageRequests,
}

pub fn render_markdown<T: DeserializeOwned>(
    content: &str,
    content_dir: &Path,
) -> Result<RenderedMarkdown<T>> {
    let arena = Arena::new();

    let mut engine_options = Options::default();
    engine_options.extension.strikethrough = true;
    engine_options.extension.tagfilter = true;
    engine_options.extension.table = true;
    engine_options.extension.autolink = true;
    engine_options.extension.tasklist = true;
    engine_options.extension.footnotes = true;
    engine_options.extension.alerts = true;
    engine_options.render.r#unsafe = true;

    let (frontmatter, content) = parse_frontmatter::<T>(content)?;
    let markdown_options = frontmatter.options;
    let plugins = Plugins::default();

    let document = parse_document(&arena, &content, &engine_options);
    let mut html = String::new();

    let (prepared, images) = prepare_document(
        content_dir,
        &arena,
        document,
        &engine_options,
        &plugins,
        markdown_options,
    )?;

    format_document_with_formatter(
        document,
        &engine_options,
        &mut html,
        &plugins,
        formatter,
        &prepared,
    )
    .context("failed to render Markdown as HTML")?;

    let rendered = RenderedMarkdown {
        metadata: frontmatter.metadata,
        html,
        images,
    };

    Ok(rendered)
}

pub fn parse_frontmatter<T: DeserializeOwned>(
    content: &str,
) -> Result<(MarkdownFrontmatter<T>, String)> {
    let matter = Matter::<YAML>::new();
    let parsed_data = matter
        .parse::<MarkdownFrontmatter<T>>(content)
        .context("failed to parse metadata of post")?;

    let frontmatter = parsed_data.data.context("content has no front matter")?;

    Ok((frontmatter, parsed_data.content))
}

fn prepare_document<'a>(
    content_dir: &'a Path,
    arena: &'a Arena<'a>,
    document: Node<'a>,
    options: &'a Options<'a>,
    plugins: &'a Plugins,
    markdown_options: MarkdownOptions,
) -> Result<(HashMap<usize, MarkdownNodeAction>, ImageRequests)> {
    let mut context = MarkdownContext::new(content_dir);

    for node in document.descendants() {
        let data = node.data();

        match &data.value {
            NodeValue::Heading(heading) => prepare_heading(node, heading, &mut context)?,
            NodeValue::Link(link) => prepare_link(node, link, &mut context)?,
            NodeValue::CodeBlock(code_block) => prepare_code_block(node, code_block, &mut context)?,
            NodeValue::Paragraph => prepare_figure(node, &mut context)?,
            NodeValue::Alert(alert) => prepare_alert(node, alert, &mut context)?,
            NodeValue::FootnoteReference(footnote_ref) => {
                pepare_footnote_ref(node, footnote_ref, &mut context)?
            }
            NodeValue::FootnoteDefinition(footnote_def) => {
                prepare_footnote_def(node, footnote_def, &mut context)?
            }
            _ => (),
        };
    }

    // Alerts
    if !context.alerts.is_empty() {
        for parsed_alert in std::mem::take(&mut context.alerts).into_iter().rev() {
            let html = render_children(parsed_alert.node, options, plugins, &context.actions)?;
            let rendered = prose_alert(parsed_alert.kind, &html).into_string();

            context.replace_node(parsed_alert.node, rendered);
        }
    }

    // Append footnotes
    if !context.footnote_definitions.is_empty() {
        let footnote_definitions = std::mem::take(&mut context.footnote_definitions);
        let mut footnotes = Vec::with_capacity(footnote_definitions.len());

        for definition in footnote_definitions {
            let html = render_children(definition.node, options, plugins, &context.actions)?;

            let number = *context
                .footnote_numbers
                .get(&definition.name)
                .with_context(|| {
                    format!("missing reference for footnote [^{}]", definition.name,)
                })?;

            footnotes.push(ParsedFootnote {
                number,
                reference_count: definition.reference_count,
                content: html.trim().to_owned(),
            });
        }

        let rendered = prose_footnotes(&footnotes).into_string();
        let footnotes_node = arena.alloc(AstNode::from(NodeValue::Raw(rendered)));

        document.append(footnotes_node);
    }

    // Prepend rendered ToC
    if markdown_options.toc && !context.toc_items.is_empty() {
        let rendered = prose_toc(&context.toc_items).into_string();
        let toc_node = arena.alloc(AstNode::from(NodeValue::Raw(rendered)));

        document.prepend(toc_node);
    }

    Ok((context.actions, context.images))
}

fn render_children(
    node: Node,
    options: &Options,
    plugins: &Plugins,
    actions: &HashMap<usize, MarkdownNodeAction>,
) -> Result<String> {
    let mut html = String::new();

    for child in node.children() {
        format_document_with_formatter(child, options, &mut html, plugins, formatter, actions)
            .context("failed to render Markdown node children")?;
    }

    Ok(html.trim().to_owned())
}

fn formatter(
    engine_context: &mut MarkdownHtmlContext<'_, '_, &HashMap<usize, MarkdownNodeAction>>,
    node: Node<'_>,
    entering: bool,
) -> std::result::Result<ChildRendering, std::fmt::Error> {
    let key = std::ptr::from_ref(node).addr();

    let prepared = engine_context.user;

    if let Some(MarkdownNodeAction::Replace(rendered)) = prepared.get(&key)
        && entering
    {
        engine_context.write_str(rendered)?;
    }

    if prepared.contains_key(&key) {
        return Ok(ChildRendering::Skip);
    }

    format_node_default(engine_context, node, entering)
}
