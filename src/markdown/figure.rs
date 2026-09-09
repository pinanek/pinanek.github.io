use anyhow::Result;
use comrak::{Node, nodes::NodeValue};
use maud::{Markup, html};

use crate::{
    assets::image::{AssetImageFormat, ImageRequest, PictureAsset},
    markdown::MarkdownContext,
};

const MARKDOWN_IMAGE_FORMATS: &[AssetImageFormat] = &[
    AssetImageFormat::Avif,
    AssetImageFormat::Webp,
    AssetImageFormat::Png,
];

pub fn prepare_figure(node: Node, context: &mut MarkdownContext) -> Result<()> {
    let Some((url, title)) = node
        .first_child()
        .filter(|child| child.next_sibling().is_none())
        .and_then(|child| {
            let data = child.data();
            let NodeValue::Image(image) = &data.value else {
                return None;
            };

            Some((image.url.clone(), image.title.clone()))
        })
    else {
        return Ok(());
    };

    let path = context.content_dir.join(url);
    let request = ImageRequest::picture(&path, MARKDOWN_IMAGE_FORMATS)?;

    let ImageRequest::Picture(picture) = &request else {
        unreachable!("markdown images must be picture requests");
    };

    let rendered = prose_figure(picture, &node.collect_text(), &title).into_string();
    context.replace_node(node, rendered);
    context
        .images
        .insert(path.to_string_lossy().to_string(), request);

    Ok(())
}

fn prose_figure(picture: &PictureAsset, alt: &str, caption: &str) -> Markup {
    let original = picture.original();

    html! {
        figure class="prose-figure" {
            div class="prose-figure__wrapper"
                style=(format!("aspect-ratio: {} / {}", original.width, original.height))
            {
                div class="prose-figure__placeholder"
                    style=({
                        format!(
                            "background-image: url('data:image/png;base64,{}')",
                            picture.placeholder(),
                        )
                    }) {}

                picture class="prose-figure__picture" {
                    @for (format, srcset) in picture.srcsets() {
                        source type=(format.mime_type()) srcset=(srcset);
                    }

                    img src=(original.public_url.as_str())
                        width=(original.width)
                        height=(original.height)
                        alt=(alt)
                        loading="lazy"
                        decoding="async"
                        data-zoomable
                        data-zoom-src=(original.public_url.as_str())
                        class="prose-figure__image";
                }
            }

            @if !caption.is_empty() {
                figcaption class="prose-figure__caption" { (caption) }
            }
        }
    }
}
