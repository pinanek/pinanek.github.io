use maud::{Markup, html};

use crate::utils::uppercase_first;

pub fn tag(tag: &str) -> Markup {
    html! {
        a class="tag" href=(format!("/tags/{tag}")) { (uppercase_first(tag)) }
    }
}
