use maud::{Markup, html};

const WIDTH: usize = 1200;
const HEIGHT: usize = 630;
const PADDING: usize = 80;

pub fn og(
    title: &str,
    description: &str,
    route: &str,
    author: &str,
    site_url: &str,
    background_data_url: &str,
) -> Markup {
    let title_lines = wrap_text(title, 21, 3);
    let description_lines = wrap_text(description, 52, 2);
    let route_label = if route == "/" { "Home" } else { route };

    let title_y = 250;
    let title_last_y = title_y + title_lines.len().saturating_sub(1) * 76;
    let description_y = title_last_y + 50;
    let description_last_y = description_y + description_lines.len().saturating_sub(1) * 36;
    let swatches_y = description_last_y + 40;

    html! {
        svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
            width=(WIDTH) height=(HEIGHT) viewBox=(format!("0 0 {WIDTH} {HEIGHT}"))
        {
            image href=(background_data_url) xlink:href=(background_data_url)
                x="0" y="0" width=(WIDTH) height=(HEIGHT)
                preserveAspectRatio="xMidYMid slice" {}

            g fill="#606064" opacity="0.16" {
                @for row in 0..4 {
                    @for column in 0..4 {
                        circle cx=(710 + column * 25) cy=(78 + row * 25) r="4" {}
                    }
                }
            }

            g transform=(format!("translate({PADDING} 0)")) {
                text x="0" y="150" fill="#303033" font-size="24" font-weight="600"
                    letter-spacing="7" { "GARDEN" }
                line x1="178" y1="126" x2="178" y2="156" stroke="#89898e" stroke-width="1.5" {}
                text x="208" y="150" fill="#737378" font-size="24" font-weight="400" {
                    (route_label)
                }

                text x="0" y=(title_y) fill="#303033" font-size="76" font-weight="700"
                    letter-spacing="-4"
                {
                    @for (index, line) in title_lines.iter().enumerate() {
                        tspan x="0" dy=(if index == 0 { "0" } else { "76" }) { (line) }
                    }
                }

                @if !description_lines.is_empty() {
                    text x="0" y=(description_y) fill="#49494d" font-size="30" font-weight="400" {
                        @for (index, line) in description_lines.iter().enumerate() {
                            tspan x="0" dy=(if index == 0 { "0" } else { "36" }) { (line) }
                        }
                    }
                }

                g transform=(format!("translate(0 {swatches_y})")) {
                    rect width="130" height="20" rx="10" fill="#647fc5" {}
                    rect x="144" width="112" height="20" rx="10" fill="#398460" {}
                    rect x="270" width="96" height="20" rx="10" fill="#bd6b32" {}
                    rect x="380" width="66" height="20" rx="10" fill="#c66a5e" {}
                }

                text x="0" y="586" fill="#737378" font-size="21" font-weight="400" {
                    (author) "  •  " (site_url)
                }
            }
        }
    }
}

fn wrap_text(text: &str, max_characters: usize, max_lines: usize) -> Vec<String> {
    let words = text.split_whitespace().collect::<Vec<_>>();
    let mut lines = Vec::new();
    let mut line = String::new();

    for word in &words {
        let separator = usize::from(!line.is_empty());

        if line.len() + separator + word.len() > max_characters && !line.is_empty() {
            lines.push(line);
            line = String::new();

            if lines.len() == max_lines {
                break;
            }
        }

        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }

    if lines.len() < max_lines && !line.is_empty() {
        lines.push(line);
    }

    let has_more_words = lines
        .iter()
        .map(|line| line.split_whitespace().count())
        .sum::<usize>()
        < words.len();
    if has_more_words && let Some(last_line) = lines.last_mut() {
        last_line.push('…');
    }

    lines
}
