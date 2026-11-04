// Icon source: Lucide (https://lucide.dev/).

use maud::{Markup, html};

pub fn console_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
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
                path d="m7 11l2-2l-2-2m4 6h4" {}
                rect width="18" height="18" x="3" y="3" rx="2" ry="2" {}
            }
        }
    }
}
