use std::borrow::Cow;
use std::fmt;
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use crate::Error;
use crate::translate::{Translation, translate};

/// A regular expression with ECMAScript semantics, so the patterns read exactly like the js library's.
///
/// Lookbehind of any length, backreferences inside lookbehind and `\p{..}` classes are all supported.
/// The `g` flag makes [`Regex::replace`] replace every match, like `String.prototype.replace` does in js.
///
/// Matching runs on the linear time `regex` crate whenever the pattern translates to it exactly, and otherwise
/// on a backtracking engine with js semantics (`regress`), which starts searching where the translated pattern,
/// a superset, first matches. Debug builds check each of these shortcuts against `regress`.
///
/// One known difference of `regress` from V8: without the `u` flag it folds `ſ` with `s` and the kelvin sign
/// with `k` under `i`, so `/[\W]/i` matches `S`. The translated patterns follow V8 there, and since they decide
/// or bound every search, the library's own patterns match exactly like V8 (checked on 11.5 million matches).
/// A debug check failing on such a pattern is about this difference, not a translation bug.
#[derive(Clone)]
pub struct Regex {
    source: Arc<str>,
    flags: Arc<str>,
    compiled: Arc<OnceLock<Compiled>>,
}

struct Compiled {
    /// The js semantics, used when the translation is not exact.
    js: regress::Regex,
    translation: Option<Translation>,
    /// The translated pattern, exact or a superset, see [`crate::translate`].
    translated: Option<::regex::Regex>,
}

impl Compiled {
    fn new(source: &str, flags: &str) -> Result<Self, Error> {
        let engine_flags: String = flags.chars().filter(|&flag| flag != 'g').collect();
        let js = regress::Regex::with_flags(source, engine_flags.as_str())
            .map_err(|error| Error::InvalidRegex { source: source.to_owned(), message: error.to_string() })?;
        let translation = translate(source, flags);
        let translated = translation
            .as_ref()
            .and_then(|translation| ::regex::RegexBuilder::new(&translation.pattern).size_limit(1 << 22).build().ok());
        Ok(Self { js, translation, translated })
    }

    /// The exact translation, when it agrees with js on `text`.
    fn exact_for(&self, text: &str) -> Option<&::regex::Regex> {
        let translation = self.translation.as_ref()?;
        let usable = translation.exact && (!translation.ascii_text_only || text.is_ascii());
        self.translated.as_ref().filter(|_| usable)
    }
}

/// A match of a [`Regex`], with byte ranges into the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    range: Range<usize>,
    /// Capture groups 1 and up.
    groups: Vec<Option<Range<usize>>>,
}

impl Match {
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn start(&self) -> usize {
        self.range.start
    }

    pub fn end(&self) -> usize {
        self.range.end
    }

    /// The range of capture group `index`, group 0 being the whole match.
    pub fn group(&self, index: usize) -> Option<Range<usize>> {
        match index {
            0 => Some(self.range()),
            _ => self.groups.get(index - 1).cloned().flatten(),
        }
    }
}

impl From<regress::Match> for Match {
    fn from(m: regress::Match) -> Self {
        Self { range: m.range(), groups: m.captures }
    }
}

impl Regex {
    /// Compiles `source` with js flags such as `"iu"`.
    pub fn new(source: &str, flags: &str) -> Result<Self, Error> {
        let compiled = Compiled::new(source, flags)?;
        Ok(Self { source: source.into(), flags: flags.into(), compiled: Arc::new(OnceLock::from(compiled)) })
    }

    /// A pattern known to be valid, i.e. one of the library's own, compiled when first used.
    pub(crate) fn known(source: &str, flags: &str) -> Self {
        Self { source: source.into(), flags: flags.into(), compiled: Arc::new(OnceLock::new()) }
    }

    fn compiled(&self) -> &Compiled {
        self.compiled.get_or_init(|| Compiled::new(&self.source, &self.flags).unwrap_or_else(|error| panic!("{error}")))
    }

    /// The translation into the `regex` crate syntax, without compiling it.
    pub(crate) fn translation(&self) -> Option<Translation> {
        translate(&self.source, &self.flags)
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn flags(&self) -> &str {
        &self.flags
    }

    pub fn is_global(&self) -> bool {
        self.flags.contains('g')
    }

    pub fn find(&self, text: &str) -> Option<Match> {
        let compiled = self.compiled();
        if let Some(exact) = compiled.exact_for(text) {
            let found = find_exact(exact, text);
            debug_assert_eq!(
                found,
                compiled.js.find(text).map(Match::from),
                "the translation of {self} finds another match in {text:?}, see crate::translate"
            );
            return found;
        }
        // a match of the translated superset starts at or before any js match, so the js search starts there
        let start = match &compiled.translated {
            Some(translated) => translated.find(text).map(|m| m.start()),
            None => Some(0),
        };
        let found = start.and_then(|start| compiled.js.find_from(text, start).next()).map(Match::from);
        debug_assert_eq!(
            found,
            compiled.js.find(text).map(Match::from),
            "the translation of {self} starts after the match in {text:?}, see crate::translate"
        );
        found
    }

    /// Whether the translated pattern rules out any match in `text`.
    pub(crate) fn cannot_match(&self, text: &str) -> bool {
        let compiled = self.compiled();
        let rejected = compiled.translated.as_ref().is_some_and(|translated| !translated.is_match(text));
        // the translation is only an optimization, so debug builds (the test suite) check it never hides a match
        debug_assert!(
            !rejected || compiled.js.find(text).is_none(),
            "the translation of {self} hides a match in {text:?}, see crate::translate"
        );
        rejected
    }

    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> impl Iterator<Item = Match> + use<'r, 't> {
        let skip = self.cannot_match(text);
        self.compiled().js.find_iter(text).take_while(move |_| !skip).map(Match::from)
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    /// Replaces the first match, or every match of a global regex; `$1` refers to a capture group.
    pub fn replace<'t>(&self, text: &'t str, replacement: &str) -> Cow<'t, str> {
        let js = &self.compiled().js;
        if self.cannot_match(text) {
            Cow::Borrowed(text)
        } else if self.is_global() {
            Cow::Owned(js.replace_all(text, replacement))
        } else {
            Cow::Owned(js.replace(text, replacement))
        }
    }

    /// Replaces the first match, or every match of a global regex, with `replacement` of the matched text.
    pub fn replace_with<'t>(&self, text: &'t str, replacement: impl Fn(&str) -> String) -> Cow<'t, str> {
        let js = &self.compiled().js;
        if self.cannot_match(text) {
            Cow::Borrowed(text)
        } else if self.is_global() {
            Cow::Owned(js.replace_all_with(text, |m: &regress::Match| replacement(&text[m.range()])))
        } else {
            Cow::Owned(js.replace_with(text, |m: &regress::Match| replacement(&text[m.range()])))
        }
    }

    /// Splits `text` around every match, like `String.prototype.split` with a regex without groups.
    pub fn split<'t>(&self, text: &'t str) -> Vec<&'t str> {
        let mut parts = Vec::new();
        let mut last = 0;
        for m in self.find_iter(text) {
            if m.range().is_empty() {
                continue;
            }
            parts.push(&text[last..m.start()]);
            last = m.end();
        }
        parts.push(&text[last..]);
        parts
    }
}

fn find_exact(regex: &::regex::Regex, text: &str) -> Option<Match> {
    if regex.captures_len() == 1 {
        return regex.find(text).map(|m| Match { range: m.range(), groups: Vec::new() });
    }
    let captures = regex.captures(text)?;
    let groups = captures.iter().skip(1).map(|group| group.map(|group| group.range())).collect();
    Some(Match { range: captures.get_match().range(), groups })
}

impl fmt::Debug for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}/{}", self.source, self.flags)
    }
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}/{}", self.source, self.flags)
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.flags == other.flags
    }
}

/// The text of capture group `index`, if it participated in the match.
pub(crate) fn group<'t>(m: &Match, text: &'t str, index: usize) -> Option<&'t str> {
    m.group(index).map(|range| &text[range])
}

/// Whether `c` is matched by js `\s`.
pub(crate) fn is_js_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}'
    )
}

/// `String.prototype.trim`.
pub(crate) fn js_trim(text: &str) -> &str {
    text.trim_matches(is_js_space)
}

/// `parseInt(text, 10)`: leading digits after optional whitespace and sign, `None` for `NaN`.
pub(crate) fn parse_int(text: &str) -> Option<i64> {
    let text = text.trim_start_matches(is_js_space);
    let (sign, rest) = match text.as_bytes().first() {
        Some(b'-') => (-1, &text[1..]),
        Some(b'+') => (1, &text[1..]),
        _ => (1, text),
    };
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    rest[..digits].parse::<i64>().ok().map(|number| sign * number)
}

/// The length of `text` in utf-16 code units, the unit of positions in js and in this library.
pub fn utf16_len(text: &str) -> usize {
    if text.is_ascii() { text.len() } else { text.encode_utf16().count() }
}

/// The position, in utf-16 code units, of the byte position `byte` of `text`.
pub fn utf16_index(text: &str, byte: usize) -> usize {
    let mut byte = byte.min(text.len());
    while !text.is_char_boundary(byte) {
        byte -= 1;
    }
    utf16_len(&text[..byte])
}

/// The byte position of the position `index`, in utf-16 code units, of `text`.
///
/// Positions past the end are clamped to it, and a position inside a surrogate pair moves before the pair.
pub fn byte_index(text: &str, index: usize) -> usize {
    if text.is_ascii() {
        return index.min(text.len());
    }
    let mut units = 0;
    for (byte, c) in text.char_indices() {
        units += c.len_utf16();
        if units > index {
            return byte;
        }
    }
    text.len()
}

/// `text.slice(start)` of js, for a position that may be stale.
pub(crate) fn slice_from(text: &str, start: usize) -> &str {
    &text[byte_index(text, start)..]
}
