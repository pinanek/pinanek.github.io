// Icon source: Lucide (https://lucide.dev/).

use maud::{Markup, html};

const SUN_PATH: &str = "M15.994 12.216C15.878 14.366 14.081 16.038 11.928 15.999C9.776 15.961 \
     8.04 14.225 8.001 12.073C7.962 9.92 9.633 8.123 11.783 8.006C11.969 \
     7.996 12.156 7.999 12.341 8.015C14.281 8.181 15.819 9.719 15.985 \
     11.658C16.001 11.844 16.004 12.03 15.994 12.216Z";

const MOON_PATH: &str = "M20.985 12.486C20.724 17.323 16.68 21.086 11.837 20.999C6.993 \
     20.913 3.087 17.007 3 12.163C2.912 7.32 6.675 3.276 11.512 \
     3.014C11.917 2.992 12.129 3.474 11.914 3.817C8.541 9.214 14.785 \
     15.458 20.182 12.085C20.526 11.87 21.007 12.081 20.985 12.486Z";

const SUN_RAYS: &[&str] = &[
    "M12 4h.01",
    "M20 12h.01",
    "M12 20h.01",
    "M4 12h.01",
    "M17.657 6.343h.01",
    "M17.657 17.657h.01",
    "M6.343 17.657h.01",
    "M6.343 6.343h.01",
];

pub fn color_mode_button() -> Markup {
    html! {
        button
            class="nav-bar__icon-button"
            type="button"
            aria-label="Switch to dark mode"
            aria-pressed="false"
            title="Switch to dark mode"
            data-color-mode-button
        {
            svg xmlns="http://www.w3.org/2000/svg"
                width="1.5rem"
                height="1.5rem"
                viewBox="0 0 24 24"
                fill="currentColor"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            {
                defs {
                    path data-theme-sun-target d=(SUN_PATH) {}
                    path data-theme-moon-target d=(MOON_PATH) {}
                }
                path data-theme-icon d=(SUN_PATH) {}

                g data-theme-rays style="transform-box: fill-box; transform-origin: center;" {
                    @for ray in SUN_RAYS {
                        path d=(*ray) {}
                    }
                }
            }
        }
    }
}
