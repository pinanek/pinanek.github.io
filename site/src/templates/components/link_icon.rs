// Icon source: Lucide (https://lucide.dev/).

use maud::{Markup, html};

pub fn link_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
    let size = size.unwrap_or("1em");

    html! {
        svg xmlns="http://www.w3.org/2000/svg"
            width=(size)
            height=(size)
            class=[class]
            color=[color]
            viewBox="0 0 24 24"
            aria-hidden="true"
        {
            path d="M0 0h24v24H0z" fill="none" {}

            g   fill="none"
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
            {
                path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" {}
                path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" {}
            }
        }
    }
}
