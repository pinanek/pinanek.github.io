use maud::{Markup, html};

pub fn tag(tag: &str) -> Markup {
    html! {
        a class="tag" href=(format!("/tags/{tag}")) { (tag) }
    }
}
