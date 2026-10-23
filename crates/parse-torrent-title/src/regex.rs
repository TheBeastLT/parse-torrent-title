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
/// a superset, first matches. The `verify-regex` feature checks each of these shortcuts against `regress` for the
/// library's own patterns.
///
/// Known differences from V8:
/// - Without the `u` flag `ſ` matches `s` under `i`, so `ſeason 1` reads a season.
/// - Without the `u` flag js matches the two halves of a character outside the basic multilingual plane, such as
///   an emoji, separately, so `.` and negated classes see one character here where js sees two.
#[derive(Clone)]
pub struct Regex {
    source: Arc<str>,
    flags: Arc<str>,
    compiled: Arc<OnceLock<Compiled>>,
    /// Whether the `verify-regex` feature checks the shortcuts against `regress`, as done for the library's own
    /// patterns.
    checked: bool,
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
        let invalid = |message: String| Error::InvalidRegex { pattern: source.to_owned(), message };
        validate_flags(flags).map_err(invalid)?;
        let engine_flags: String = flags.chars().filter(|flag| !matches!(flag, 'g' | 'd')).collect();
        let js = regress::Regex::with_flags(&regress_source(source, flags), engine_flags.as_str())
            .map_err(|error| invalid(error.to_string()))?;
        let translation = translate(source, flags);
        let translated = translation
            .as_ref()
            .and_then(|translation| ::regex::RegexBuilder::new(&translation.pattern).size_limit(1 << 22).build().ok());
        Ok(Self { js, translation, translated })
    }

    /// The translation, exact or a superset, when it holds on `text`.
    fn translated_for(&self, text: &str) -> Option<&::regex::Regex> {
        let translation = self.translation.as_ref()?;
        self.translated.as_ref().filter(|_| translation.holds_for(text))
    }

    /// The exact translation, when it agrees with js on `text`.
    fn exact_for(&self, text: &str) -> Option<&::regex::Regex> {
        self.translated_for(text).filter(|_| self.translation.as_ref().is_some_and(|translation| translation.exact))
    }
}

/// The non word chars of js, without `ſ` and the kelvin sign.
const NON_WORD: &str = "\\0-/:-@\\[-^`{-\u{17e}\u{180}-\u{2129}\u{212b}-\u{10ffff}";

/// `source` for `regress`, which folds the `ſ` and the kelvin sign of a `\W` inside a class into `s` and `k` under
/// `i`, unlike js: `[\W\d]` would match `s`. Such a `\W` is spelled out without them, which only changes whether
/// `ſ` and the kelvin sign themselves match.
fn regress_source<'s>(source: &'s str, flags: &str) -> Cow<'s, str> {
    if !flags.contains('i') || flags.contains('v') || !source.contains("\\W") {
        return Cow::Borrowed(source);
    }
    let mut out = String::with_capacity(source.len() + NON_WORD.len());
    let mut in_class = false;
    let mut chars = source.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('W') if in_class => out.push_str(NON_WORD),
                escaped => out.extend(['\\'].into_iter().chain(escaped)),
            },
            '[' => {
                in_class = true;
                out.push(c);
            }
            ']' => {
                in_class = false;
                out.push(c);
            }
            c => out.push(c),
        }
    }
    Cow::Owned(out)
}

/// Accepts the flags of a js regex, except `y` whose sticky matching is not supported.
fn validate_flags(flags: &str) -> Result<(), String> {
    for (index, flag) in flags.char_indices() {
        if !"dgimsuv".contains(flag) {
            return Err(format!("unsupported flag {flag:?}"));
        }
        if flags[..index].contains(flag) {
            return Err(format!("duplicate flag {flag:?}"));
        }
    }
    if flags.contains('u') && flags.contains('v') {
        return Err("the u and v flags can not be combined".to_owned());
    }
    Ok(())
}

/// A match of a [`Regex`], with byte ranges into the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    range: Range<usize>,
    /// Capture groups 1 and up.
    groups: Vec<Option<Range<usize>>>,
}

impl Match {
    #[must_use]
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    #[must_use]
    pub fn start(&self) -> usize {
        self.range.start
    }

    #[must_use]
    pub fn end(&self) -> usize {
        self.range.end
    }

    /// The range of capture group `index`, group 0 being the whole match.
    #[must_use]
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
    ///
    /// # Errors
    ///
    /// When the pattern is not a valid js regex, or uses the unsupported `y` flag.
    pub fn new(source: &str, flags: &str) -> Result<Self, Error> {
        let compiled = Compiled::new(source, flags)?;
        Ok(Self {
            source: source.into(),
            flags: flags.into(),
            compiled: Arc::new(OnceLock::from(compiled)),
            checked: false,
        })
    }

    /// A pattern known to be valid, i.e. one of the library's own, compiled when first used.
    pub(crate) fn known(source: &str, flags: &str) -> Self {
        Self { source: source.into(), flags: flags.into(), compiled: Arc::new(OnceLock::new()), checked: true }
    }

    fn compiled(&self) -> &Compiled {
        self.compiled.get_or_init(|| Compiled::new(&self.source, &self.flags).unwrap_or_else(|error| panic!("{error}")))
    }

    /// Whether the `verify-regex` feature should check a shortcut against `regress`.
    fn check(&self) -> bool {
        cfg!(feature = "verify-regex") && self.checked
    }

    /// The translation into the `regex` crate syntax, without compiling it.
    pub(crate) fn translation(&self) -> Option<Translation> {
        translate(&self.source, &self.flags)
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn flags(&self) -> &str {
        &self.flags
    }

    #[must_use]
    pub fn is_global(&self) -> bool {
        self.flags.contains('g')
    }

    #[must_use]
    pub fn find(&self, text: &str) -> Option<Match> {
        let compiled = self.compiled();
        if let Some(exact) = compiled.exact_for(text) {
            let found = find_exact(exact, text);
            if self.check() {
                let expected = compiled.js.find(text).map(Match::from);
                assert_eq!(
                    found, expected,
                    "the translation of {self} finds another match in {text:?}, see crate::translate"
                );
            }
            return found;
        }
        // a match of the translated superset starts at or before any js match, so the js search starts there
        let start = match compiled.translated_for(text) {
            Some(translated) => translated.find(text).map(|m| m.start()),
            None => Some(0),
        };
        let found = start.and_then(|start| compiled.js.find_from(text, start).next()).map(Match::from);
        if self.check() {
            let expected = compiled.js.find(text).map(Match::from);
            assert_eq!(
                found, expected,
                "the translation of {self} starts after the match in {text:?}, see crate::translate"
            );
        }
        found
    }

    /// Whether the translated pattern rules out any match in `text`.
    pub(crate) fn cannot_match(&self, text: &str) -> bool {
        let compiled = self.compiled();
        let rejected = compiled.translated_for(text).is_some_and(|translated| !translated.is_match(text));
        if rejected && self.check() {
            assert!(
                compiled.js.find(text).is_none(),
                "the translation of {self} hides a match in {text:?}, see crate::translate"
            );
        }
        rejected
    }

    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> impl Iterator<Item = Match> + use<'r, 't> {
        let matches = (!self.cannot_match(text)).then(|| self.compiled().js.find_iter(text));
        matches.into_iter().flatten().map(Match::from)
    }

    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn found(source: &str, flags: &str, text: &str) -> Option<Range<usize>> {
        Regex::new(source, flags).unwrap().find(text).map(|m| m.range())
    }

    #[test]
    fn follows_the_flags_like_js() {
        assert_eq!(found("a.b", "s", "a\nb"), Some(0..3));
        assert_eq!(found("a.b", "", "a\nb"), None);
        assert_eq!(found("^b", "m", "a\nb"), Some(2..3));
        assert_eq!(found("a$", "m", "a\nb"), Some(0..1));
        assert_eq!(found("^b", "", "a\nb"), None);
        assert_eq!(found(r"\01", "", "\u{1}"), Some(0..1));
    }

    #[test]
    fn rejects_flags_js_rejects_or_that_are_not_supported() {
        for flags in ["x", "gg", "uv", "y"] {
            assert!(matches!(Regex::new("a", flags), Err(Error::InvalidRegex { .. })), "{flags}");
        }
        assert!(Regex::new("a", "dgimsu").is_ok());
    }

    #[test]
    fn folds_case_like_js() {
        // `regress` folds the `ſ` of a `\W` in a class into `s`
        assert_eq!(found(r"(?:[\W\d]|^)e(\d)(?<!x)", "i", "Movie se1e2"), Some(8..11));
        assert_eq!(found(r"[^\W\d]", "i", "1s"), Some(1..2));
        assert_eq!(found(r"a[\W]", "iu", "as a-"), Some(3..5));
        // the `regex` crate folds the ohm sign into `ω`, and does not fold what `\P{..}` matches
        assert_eq!(found(r"ω", "i", "\u{2126}"), None);
        assert_eq!(found(r"\P{Lowercase_Letter}", "iu", "a"), Some(0..1));
    }
}
