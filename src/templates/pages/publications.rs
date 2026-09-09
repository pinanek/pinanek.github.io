use std::fs;

use anyhow::{Context, Result};
use maud::{Markup, html};
use serde::Deserialize;

use crate::{
    assets::image::ImageRequests,
    config::CONFIG,
    templates::{
        components::external_link_icon,
        layouts::main_layout,
        pages::{Page, PageMetadata},
    },
    utils::uppercase_first,
};

#[derive(Deserialize)]
struct Publications {
    publication: Vec<Publication>,
}

#[derive(Deserialize)]
struct Publication {
    title: String,
    authors: Vec<String>,
    year: u16,
    #[serde(rename = "type")]
    kind: String,
    venue: String,
    link: String,
}

struct PublicationYear {
    year: u16,
    publications: Vec<Publication>,
}

pub struct PublicationsPage {
    route: String,
    years: Vec<PublicationYear>,
    images: ImageRequests,
    metadata: PageMetadata<'static>,
}

impl PublicationsPage {
    pub fn new() -> Result<Self> {
        let route = String::from("/publications/");
        let path = CONFIG.data_dir.join("publications.toml");
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read publications: {}", path.display()))?;
        let mut publications = toml::from_str::<Publications>(&content)
            .with_context(|| format!("failed to parse publications: {}", path.display()))?
            .publication;

        publications.sort_by_key(|publication| std::cmp::Reverse(publication.year));
        for publication in &mut publications {
            publication.kind = uppercase_first(&publication.kind);
        }

        let mut years = Vec::<PublicationYear>::new();
        for publication in publications {
            if let Some(year) = years.last_mut()
                && year.year == publication.year
            {
                year.publications.push(publication);
            } else {
                years.push(PublicationYear {
                    year: publication.year,
                    publications: vec![publication],
                });
            }
        }

        Ok(Self {
            route: route.clone(),
            years,
            images: ImageRequests::new(),
            metadata: PageMetadata::new()
                .title("Publications")
                .description("My research publications, papers, and academic work.")
                .canonical_url(route),
        })
    }
}

impl Page for PublicationsPage {
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
        let links = html! {
            link rel="stylesheet" href="/assets/publications/main.css";
        };

        main_layout(
            &self.metadata,
            html! {
                div class="pub-page" {
                    div class="pub-page__list" {
                        @for year in &self.years {
                            section class="pub-page__year" {
                                h2 class="pub-page__year-title" { (year.year) }
                                ul class="pub-page__year-list" {
                                    @for publication in &year.publications {
                                        li class="pub-page__item" {
                                            h3 class="pub-page__item-title" {
                                                a
                                                    href=(&publication.link)
                                                    target="_blank"
                                                    rel="nofollow noopener noreferrer"
                                                {
                                                    (&publication.title)
                                                    (external_link_icon(Some("pub-page__external-icon"), Some("0.9em"), None))
                                                }
                                            }
                                            p class="pub-page__item-authors" { (&publication.authors.join(", ")) }
                                            p class="pub-page__item-metadata" {
                                                (&publication.venue)
                                                " · "
                                                (&publication.kind)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            links,
            html! {},
        )
    }
}
