// Icon source: Lucide (https://lucide.dev/).

use maud::{Markup, html};

pub fn external_link_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
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
            path d="M7 7h10v10" {}
            path d="M7 17 17 7" {}
        }
    }
}
