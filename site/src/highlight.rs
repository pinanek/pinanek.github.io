mod engine;
#[allow(dead_code)]
mod highlighter;
mod html;
mod rainbow;
mod registry;

pub use engine::highlight;
pub use registry::{HighlightLanguageKind, HighlightRegistry};
