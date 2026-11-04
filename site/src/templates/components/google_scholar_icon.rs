// Icon source: Simple Icons (https://simpleicons.org/).

use maud::{Markup, html};

pub fn google_scholar_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
    let size = size.unwrap_or("1em");

    html! {
        svg xmlns="http://www.w3.org/2000/svg"
            width=(size)
            height=(size)
            class=[class]
            color=[color]
            viewBox="0 0 24 24"
            role="img"
            aria-hidden="true"
        {
            title { "Google Scholar" }
            path
                d="M5.242 13.769L0 9.5 12 0l12 9.5-5.242 4.269C17.548 11.249 14.978 9.5 12 9.5c-2.977 0-5.548 1.748-6.758 4.269zM12 10a7 7 0 1 0 0 14 7 7 0 0 0 0-14z"
                fill="currentColor" {}
        }
    }
}
