use std::ops::RangeInclusive;

pub struct HtmlRenderer<'a> {
    output: String,
    active_highlights: Vec<usize>,
    highlight_names: &'a [String],
    highlighted_lines: Option<&'a [RangeInclusive<usize>]>,
    line_number: usize,
}

impl<'a> HtmlRenderer<'a> {
    pub fn new(
        source_length: usize,
        highlight_names: &'a [String],
        highlighted_lines: Option<&'a [RangeInclusive<usize>]>,
    ) -> Self {
        let mut renderer = Self {
            output: String::with_capacity(source_length.saturating_mul(2)),
            active_highlights: Vec::with_capacity(8),
            highlight_names,
            highlighted_lines,
            line_number: 1,
        };

        renderer.open_line();

        renderer
    }

    pub fn push_source(&mut self, source: &str) {
        let mut remaining = source;

        while let Some(newline_index) = remaining.find('\n') {
            self.output
                .push_str(&html_escape::encode_text(&remaining[..newline_index]));

            self.close_active_highlights();
            self.output.push_str("</span>\n");

            self.line_number += 1;
            self.open_line();

            self.reopen_active_highlights();

            remaining = &remaining[newline_index + 1..];
        }

        self.output.push_str(&html_escape::encode_text(remaining));
    }

    pub fn start_highlight(&mut self, index: usize) -> bool {
        let Some(name) = self.highlight_names.get(index) else {
            return false;
        };

        push_highlight_start(name, &mut self.output);
        self.active_highlights.push(index);

        true
    }

    pub fn end_highlight(&mut self) -> bool {
        if self.active_highlights.pop().is_none() {
            return false;
        }

        self.output.push_str("</span>");

        true
    }

    pub fn has_active_highlights(&self) -> bool {
        !self.active_highlights.is_empty()
    }

    pub fn finish(mut self) -> String {
        self.output.push_str("</span>");
        self.output
    }

    fn open_line(&mut self) {
        if is_highlighted_line(self.line_number, self.highlighted_lines) {
            self.output
                .push_str("<span class=\"ts-line ts-line-highlighted\">");
        } else {
            self.output.push_str("<span class=\"ts-line\">");
        }
    }

    fn close_active_highlights(&mut self) {
        for _ in 0..self.active_highlights.len() {
            self.output.push_str("</span>");
        }
    }

    fn reopen_active_highlights(&mut self) {
        for &index in &self.active_highlights {
            let name = &self.highlight_names[index];

            push_highlight_start(name, &mut self.output);
        }
    }
}

fn push_highlight_start(name: &str, output: &mut String) {
    output.push_str("<span class=\"ts-");

    let mut first = true;

    for end in name
        .match_indices('.')
        .map(|(index, _)| index)
        .chain(std::iter::once(name.len()))
    {
        if !first {
            output.push_str(" ts-");
        }

        push_css_name(&name[..end], output);
        first = false;
    }

    output.push_str("\">");
}

fn push_css_name(name: &str, output: &mut String) {
    for byte in name.bytes() {
        match byte {
            b'.' | b'_' => output.push('-'),

            byte if byte.is_ascii_alphanumeric() || byte == b'-' => {
                output.push(char::from(byte));
            }

            _ => output.push('-'),
        }
    }
}

pub fn render_plain(content: &str, highlighted_lines: Option<&[RangeInclusive<usize>]>) -> String {
    let mut output = String::with_capacity(content.len());
    let mut lines = content.split('\n').enumerate();

    if let Some((index, first)) = lines.next() {
        push_line_start(index + 1, highlighted_lines, &mut output);

        output.push_str(&html_escape::encode_text(first));
    }

    for (index, line) in lines {
        output.push_str("</span>\n");

        push_line_start(index + 1, highlighted_lines, &mut output);

        output.push_str(&html_escape::encode_text(line));
    }

    output.push_str("</span>");

    output
}

fn push_line_start(
    line_number: usize,
    highlighted_lines: Option<&[RangeInclusive<usize>]>,
    output: &mut String,
) {
    if is_highlighted_line(line_number, highlighted_lines) {
        output.push_str("<span class=\"ts-line ts-line-highlighted\">");
    } else {
        output.push_str("<span class=\"ts-line\">");
    }
}

fn is_highlighted_line(
    line_number: usize,
    highlighted_lines: Option<&[RangeInclusive<usize>]>,
) -> bool {
    highlighted_lines.is_some_and(|ranges| ranges.iter().any(|range| range.contains(&line_number)))
}
