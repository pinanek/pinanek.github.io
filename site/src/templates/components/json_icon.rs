// Icon source: CoreUI Icons (https://icons.coreui.io/).

use maud::{Markup, html};

pub fn json_icon(class: Option<&str>, size: Option<&str>, color: Option<&str>) -> Markup {
    let size = size.unwrap_or("1em");

    html! {
        svg xmlns="http://www.w3.org/2000/svg"
            width=(size)
            height=(size)
            class=[class]
            color=[color]
            viewBox="0 0 32 32"
            aria-hidden="true"
        {
            path d="M0 0h32v32H0z" fill="none" {}
            path fill="currentColor" d="..." {}
        }
    }
}
