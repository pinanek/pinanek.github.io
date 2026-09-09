use anyhow::Result;
use maud::{Markup, html};

use crate::{
    assets::image::ImageRequests,
    photos::Photo,
    templates::{
        layouts::base_layout,
        pages::{Page, PageMetadata},
    },
};

#[derive(Debug)]
pub struct PhotosPage {
    route: String,
    photos: Vec<Photo>,
    images: ImageRequests,
    metadata: PageMetadata<'static>,
}

impl PhotosPage {
    pub fn new(photos: Vec<Photo>) -> Result<Self> {
        let route = String::from("/photos/");

        Ok(Self {
            route: route.clone(),
            photos,
            images: ImageRequests::new(),
            metadata: PageMetadata::new()
                .title("Photos")
                .description("A collection of photographs.")
                .canonical_url(route),
        })
    }
}

impl Page for PhotosPage {
    fn route(&self) -> &str {
        &self.route
    }

    fn metadata(&self) -> &PageMetadata<'_> {
        &self.metadata
    }

    fn images(&self) -> &ImageRequests {
        &self.images
    }

    fn render(&self) -> Markup {
        base_layout(
            &self.metadata,
            html! {
                main class="photos-page" {
                    h1 class="photos-page__title" { "Photos" }
                    div class="photos-page__grid" {
                        @for photo in &self.photos {
                            (photo_item(photo))
                        }
                    }
                }
            },
            html! {
                link rel="stylesheet" href="/assets/photos/main.css";
            },
            html! {
                script type="module" src="/assets/photos/main.js" {}
            },
        )
    }
}

fn photo_item(photo: &Photo) -> Markup {
    html! {
        figure class="photos-page__item"
            style=(format!(
                "background-image: url('data:image/png;base64,{}')",
                photo.placeholder,
            ))
        {
            img class="photos-page__image" src=(&photo.public_url)
                width=(photo.width)
                height=(photo.height)
                alt=(&photo.alt)
                loading="lazy"
                decoding="async"
                data-photo-zoomable
                data-zoom-src=(&photo.public_url);
        }
    }
}
