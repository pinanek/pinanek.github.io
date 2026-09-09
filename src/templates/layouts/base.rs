use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::templates::{
    components::{footer, metadata, nav_bar},
    pages::PageMetadata,
};

const COLOR_MODE_SCRIPT: &str = r#"
(() => {
  const storageKey = "garden-color-mode";
  const root = document.documentElement;
  const storedMode = localStorage.getItem(storageKey);

  const mode =
    storedMode === "light" || storedMode === "dark"
      ? storedMode
      : window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light";

  root.classList.toggle("dark", mode === "dark");
  root.style.colorScheme = mode;
})();
"#;

pub fn base_layout(
    metadata_props: &PageMetadata<'_>,
    children: Markup,
    links: Markup,
    scripts: Markup,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" href="/favicon.ico";

                (metadata(metadata_props))

                link
                    rel="preload"
                    href="/fonts/hk-latin-n.woff2"
                    as="font"
                    type="font/woff2"
                    crossorigin;
                link
                    rel="preload"
                    href="/fonts/hk-vnese-n.woff2"
                    as="font"
                    type="font/woff2"
                    crossorigin;
                link rel="stylesheet" href="/assets/main.css";
                (links)
                script { (PreEscaped(COLOR_MODE_SCRIPT)) }
                script type="module" src="/assets/main.js" {}
                (scripts)
            }

            body {
                div id="root" {
                    (nav_bar())
                    div class="site-content" { (children) }
                    (footer())
                }
            }
        }
    }
}
