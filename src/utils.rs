use std::borrow::Cow;

use crate::config::CONFIG;

pub fn is_external_url(href: &str) -> bool {
    href.starts_with("http://") || href.starts_with("https://")
}

pub fn get_absolute_url<'a>(url: &'a str) -> Cow<'a, str> {
    if url.starts_with("https://") || url.starts_with("http://") {
        Cow::Borrowed(url)
    } else {
        Cow::Owned(format!(
            "{}/{}",
            CONFIG.site.url.trim_end_matches('/'),
            url.trim_start_matches('/'),
        ))
    }
}

pub fn uppercase_first(value: &str) -> String {
    let mut chars = value.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
