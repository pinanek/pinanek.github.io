// Icon source: Lucide (https://lucide.dev/).

use maud::{Markup, html};

pub fn rss_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
    let size = size.unwrap_or("1em");

    html! {
        svg xmlns="http://www.w3.org/2000/svg"
            width=(size)
            height=(size)
            class=[class]
            color=[color]
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        {
            path d="M4 11a9 9 0 0 1 9 9" {}
            path d="M4 4a16 16 0 0 1 16 16" {}
            circle cx="5" cy="19" r="1" {}
        }
    }
}
