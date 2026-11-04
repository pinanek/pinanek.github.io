use std::ops::Range;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Query, QueryCursor};

pub const RAINBOW_NAMES: [&str; 6] = [
    "rainbow.1",
    "rainbow.2",
    "rainbow.3",
    "rainbow.4",
    "rainbow.5",
    "rainbow.6",
];

pub struct RainbowHighlight {
    pub range: Range<usize>,
    pub color: usize,
    pub layer: usize,
}

pub fn collect(
    query: &Query,
    root: Node,
    source: &[u8],
    cursor: &mut QueryCursor,
    layer: usize,
) -> Vec<RainbowHighlight> {
    let scope_capture = query.capture_index_for_name("rainbow.scope");
    let bracket_capture = query.capture_index_for_name("rainbow.bracket");
    let mut captures = cursor.captures(query, root, source);
    let mut scopes: Vec<(Node, bool)> = Vec::new();
    let mut highlights = Vec::new();

    while let Some((matched, index)) = captures.next() {
        let capture = matched.captures()[*index];
        let node = capture.node;
        while scopes
            .last()
            .is_some_and(|(scope, _)| scope.end_byte() <= node.start_byte())
        {
            scopes.pop();
        }

        if Some(capture.index) == scope_capture {
            let include_children = query
                .property_settings(matched.pattern_index)
                .iter()
                .any(|property| property.key.as_ref() == "rainbow.include-children");
            if let Some((scope, include)) = scopes.last_mut()
                && *scope == node
            {
                *include |= include_children;
            } else {
                scopes.push((node, include_children));
            }
        } else if Some(capture.index) == bracket_capture
            && let Some((scope, include_children)) = scopes.last()
            && (*include_children || node.parent() == Some(*scope))
            && node.start_byte() < node.end_byte()
        {
            highlights.push(RainbowHighlight {
                range: node.byte_range(),
                color: (scopes.len() - 1) % RAINBOW_NAMES.len(),
                layer,
            });
        }
    }

    highlights
}
