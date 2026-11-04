mod colophon;
mod home;
mod photos;
mod post;
mod posts;
mod publications;
mod tag;
mod uses;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use maud::Markup;
use strum::{Display, EnumString, IntoStaticStr};

use crate::assets::image::ImageRequests;

pub use colophon::ColophonPage;
pub use home::HomePage;
pub use photos::PhotosPage;
pub use post::PostPage;
pub use posts::PostsPage;
pub use publications::PublicationsPage;
pub use tag::TagPage;
pub use uses::UsesPage;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum PageType {
    Website,
    Article,
}

#[derive(Debug, Clone)]
pub struct PageMetadata<'a> {
    pub title: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub canonical_url: Option<Cow<'a, str>>,
    pub image_url: Option<Cow<'a, str>>,
    pub image_alt: Option<Cow<'a, str>>,
    pub keywords: Vec<Cow<'a, str>>,
    pub page_type: PageType,
    pub published_time: Option<Cow<'a, str>>,
    pub modified_time: Option<Cow<'a, str>>,
}

impl<'a> PageMetadata<'a> {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            canonical_url: None,
            image_url: None,
            image_alt: None,
            keywords: Vec::new(),
            page_type: PageType::Website,
            published_time: None,
            modified_time: None,
        }
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn canonical_url(mut self, canonical_url: impl Into<Cow<'a, str>>) -> Self {
        self.canonical_url = Some(canonical_url.into());
        self
    }

    pub fn image(
        mut self,
        image_url: impl Into<Cow<'a, str>>,
        image_alt: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.image_url = Some(image_url.into());
        self.image_alt = Some(image_alt.into());
        self
    }

    pub fn keywords<I, S>(mut self, keywords: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    pub fn article(
        mut self,
        published_time: &DateTime<Utc>,
        modified_time: Option<&DateTime<Utc>>,
    ) -> Self {
        self.page_type = PageType::Article;
        self.published_time = Some(published_time.to_rfc3339().into());

        if let Some(modified_time) = modified_time {
            self.modified_time = Some(modified_time.to_rfc3339().into());
        }

        self
    }
}

pub trait Page: Sync {
    fn route(&self) -> &str;
    fn metadata(&self) -> &PageMetadata<'_>;
    fn images(&self) -> &ImageRequests;
    fn render(&self) -> Markup;
}

#[derive(Debug)]
pub struct RenderedPage<'page> {
    route: &'page str,
    metadata: &'page PageMetadata<'page>,
    images: &'page ImageRequests,
    html: String,
}

impl<'page> RenderedPage<'page> {
    pub fn route(&self) -> &str {
        self.route
    }

    pub fn metadata(&self) -> &PageMetadata<'_> {
        self.metadata
    }

    pub fn images(&self) -> &ImageRequests {
        self.images
    }

    pub fn html(&self) -> &str {
        &self.html
    }
}

pub fn render_pages<'rendered, 'page>(
    pages: &'rendered [Box<dyn Page + 'page>],
) -> Vec<RenderedPage<'rendered>>
where
    'page: 'rendered,
{
    pages
        .iter()
        .map(|page| RenderedPage {
            route: page.route(),
            metadata: page.metadata(),
            images: page.images(),
            html: page.render().into_string(),
        })
        .collect()
}
