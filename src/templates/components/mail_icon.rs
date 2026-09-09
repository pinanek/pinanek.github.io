// Icon source: Lucide (https://lucide.dev/).

use maud::{Markup, html};

pub fn mail_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
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
            path d="m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7" {}
            rect x="2" y="4" width="20" height="16" rx="2" {}
        }
    }
}
