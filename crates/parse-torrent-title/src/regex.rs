pub use fastjs_regex::{Match, Regex, byte_index, utf16_index, utf16_len};

/// The text of capture group `index`, if it participated in the match.
pub(crate) fn group<'t>(m: &Match, text: &'t str, index: usize) -> Option<&'t str> {
    m.group(index).map(|range| &text[range])
}

/// `text.slice(start)` of js, for a position that may be stale.
pub(crate) fn slice_from(text: &str, start: usize) -> &str {
    &text[byte_index(text, start)..]
}
