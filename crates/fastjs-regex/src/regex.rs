use std::borrow::Cow;
use std::fmt;
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use crate::Error;
use crate::translate::{Translation, translate};

/// A regular expression with ECMAScript semantics: a pattern and flags such as `"iu"` mean what they mean in js.
///
/// Lookbehind of any length, backreferences inside lookbehind, named groups and `\p{..}` classes are all supported,
/// as are the flags `d` (accepted, without effect), `g`, `i`, `m`, `s`, `u`, `v` and `y`. A `Regex` holds no
/// `lastIndex`: the methods take the start of the search as an argument instead, see [`Regex::find_at`].
///
/// The js methods map to:
///
/// | js                                     | here                                                |
/// |----------------------------------------|-----------------------------------------------------|
/// | `re.test(s)`                           | [`Regex::is_match`]                                 |
/// | `re.exec(s)`, `s.match(re)`            | [`Regex::find`], [`Regex::find_at`] for a lastIndex |
/// | `s.matchAll(re)`, `s.match(re)` with g | [`Regex::find_iter`]                                |
/// | `s.search(re)`                         | [`Regex::search`]                                   |
/// | `s.replace(re, ..)`                    | [`Regex::replace`], [`Regex::replace_with`]         |
/// | `s.replaceAll(re, ..)`                 | [`Regex::replace_all`], [`Regex::replace_all_with`] |
/// | `s.split(re, limit)`                   | [`Regex::split`], [`Regex::split_limit`]            |
///
/// Positions are byte positions into the text: [`utf16_index`] and [`byte_index`] convert them to and from the
/// utf-16 positions of js.
///
/// Matching runs on the linear time `regex` crate whenever the pattern translates to it exactly, and otherwise
/// on a backtracking engine with js semantics (`regress`), which starts searching where the translated pattern,
/// a superset, first matches. The `verify` feature checks each of these shortcuts against `regress`.
///
/// Known differences from V8:
/// - Without the `u` flag `ſ` and the kelvin sign (`\u{212a}`) match `s` and `k` under `i`, so `ſeason 1` reads
///   a season.
/// - Without the `u` flag js matches the two halves of a character outside the basic multilingual plane, such as
///   an emoji, separately, so `.` and negated classes see one character here where js sees two. For the same
///   reason an empty match moves on past the whole character, where js moves on by one half: no match and no
///   split piece starts or ends inside such a character.
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
    /// The names of the capture groups, empty for unnamed ones, `None` without named groups.
    names: Option<Arc<[Box<str>]>>,
    /// The `y` flag: a match only counts where the search starts.
    sticky: bool,
}

impl Compiled {
    fn new(source: &str, flags: &str) -> Result<Self, Error> {
        let invalid = |message: String| Error::InvalidRegex { pattern: source.to_owned(), message };
        validate_flags(flags).map_err(invalid)?;
        let engine_flags: String = flags.chars().filter(|flag| !matches!(flag, 'g' | 'd' | 'y')).collect();
        let js = regress::Regex::with_flags(&regress_source(source, flags), engine_flags.as_str())
            .map_err(|error| invalid(error.to_string()))?;
        let sticky = flags.contains('y');
        let translation = translate(source, flags).filter(|_| !sticky);
        let translated = translation
            .as_ref()
            .and_then(|translation| ::regex::RegexBuilder::new(&translation.pattern).size_limit(1 << 22).build().ok());
        Ok(Self { js, translation, translated, names: group_names(source), sticky })
    }

    fn to_match(&self, m: regress::Match) -> Match {
        Match { range: m.range(), groups: m.captures, names: self.names.clone() }
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

/// Accepts the flags of a js regex.
fn validate_flags(flags: &str) -> Result<(), String> {
    for (index, flag) in flags.char_indices() {
        if !"dgimsuvy".contains(flag) {
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

/// The names of the capture groups of `source`, in order, `None` when none is named.
fn group_names(source: &str) -> Option<Arc<[Box<str>]>> {
    if !source.contains("(?<") {
        return None;
    }
    let mut names: Vec<Box<str>> = Vec::new();
    let mut named = false;
    let mut in_class = false;
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                chars.next();
            }
            '[' => in_class = true,
            ']' => in_class = false,
            '(' if !in_class && chars.peek() != Some(&'?') => names.push("".into()),
            '(' if !in_class => {
                chars.next();
                if chars.next_if_eq(&'<').is_some() && chars.peek().is_some_and(|&c| c != '=' && c != '!') {
                    named = true;
                    names.push(chars.by_ref().take_while(|&c| c != '>').collect::<String>().into());
                }
            }
            _ => {}
        }
    }
    named.then(|| names.into())
}

/// A match of a [`Regex`], with byte ranges into the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    range: Range<usize>,
    /// Capture groups 1 and up.
    groups: Vec<Option<Range<usize>>>,
    /// The names of the capture groups, see [`Compiled::names`].
    names: Option<Arc<[Box<str>]>>,
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

    /// The range of the capture group named `name`, like `groups.name` of a js match.
    #[must_use]
    pub fn named_group(&self, name: &str) -> Option<Range<usize>> {
        let names = self.names.as_deref()?;
        names.iter().zip(&self.groups).filter(|(group, _)| group.as_ref() == name).find_map(|(_, range)| range.clone())
    }

    /// The number of capture groups, not counting the whole match.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

impl Regex {
    /// Compiles `source` with js flags such as `"iu"`.
    ///
    /// # Errors
    ///
    /// When the pattern or the flags are not valid in js.
    pub fn new(source: &str, flags: &str) -> Result<Self, Error> {
        let compiled = Compiled::new(source, flags)?;
        Ok(Self { source: source.into(), flags: flags.into(), compiled: Arc::new(OnceLock::from(compiled)) })
    }

    /// A pattern known to be valid, such as a literal of the program, compiled when first used: creating many
    /// costs nothing until they are used.
    ///
    /// # Panics
    ///
    /// On first use, when the pattern or the flags are not valid in js.
    #[must_use]
    pub fn lazy(source: &str, flags: &str) -> Self {
        Self { source: source.into(), flags: flags.into(), compiled: Arc::new(OnceLock::new()) }
    }

    fn compiled(&self) -> &Compiled {
        self.compiled.get_or_init(|| Compiled::new(&self.source, &self.flags).unwrap_or_else(|error| panic!("{error}")))
    }

    /// Whether the `verify` feature should check a shortcut against `regress`.
    fn check(&self) -> bool {
        cfg!(feature = "verify")
    }

    /// The translation into the `regex` crate syntax, exact or a superset, without compiling it: `None` when the
    /// pattern uses syntax the translation does not cover. Any match of the pattern contains a match of the
    /// translation, which makes it a prefilter, for example to extract literals every match contains.
    #[must_use]
    pub fn translation(&self) -> Option<Translation> {
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
    pub fn is_sticky(&self) -> bool {
        self.flags.contains('y')
    }

    /// The first match, like `exec` or `match` of a regex without the `g` flag in js.
    #[must_use]
    pub fn find(&self, text: &str) -> Option<Match> {
        self.find_at(text, 0)
    }

    /// The first match from the byte position `start`, like `exec` of a global or sticky regex whose `lastIndex` is
    /// `start` in js: a lookbehind sees the text before `start`, and a sticky regex only matches at `start`.
    ///
    /// # Panics
    ///
    /// When `start` is inside a character of `text`.
    #[must_use]
    pub fn find_at(&self, text: &str, start: usize) -> Option<Match> {
        if start > text.len() {
            return None;
        }
        let compiled = self.compiled();
        let expected = || compiled.js.find_from(text, start).next().filter(|m| !compiled.sticky || m.start() == start);
        if let Some(exact) = compiled.exact_for(text) {
            let found = compiled.find_exact(exact, text, start);
            if self.check() {
                assert_eq!(
                    found,
                    expected().map(|m| compiled.to_match(m)),
                    "the translation of {self} finds another match in {text:?}, see crate::translate"
                );
            }
            return found;
        }
        // a match of the translated superset starts at or before any js match, so the js search starts there
        let from = match compiled.translated_for(text) {
            Some(translated) if start == 0 => translated.find(text).map(|m| m.start()),
            Some(translated) => translated.is_match(text).then_some(start),
            None => Some(start),
        };
        let found = from
            .and_then(|from| compiled.js.find_from(text, from).next())
            .filter(|m| !compiled.sticky || m.start() == start)
            .map(|m| compiled.to_match(m));
        if self.check() {
            assert_eq!(
                found,
                expected().map(|m| compiled.to_match(m)),
                "the translation of {self} starts after the match in {text:?}, see crate::translate"
            );
        }
        found
    }

    /// Whether the translated pattern rules out any match in `text`.
    fn cannot_match(&self, text: &str) -> bool {
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

    /// Every match, like `matchAll` or `match` of a global regex in js, whatever the `g` flag.
    ///
    /// After an empty match the search goes on one character later. A sticky regex stops at the first match that
    /// does not start where the previous one ended.
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> impl Iterator<Item = Match> + use<'r, 't> {
        let compiled = self.compiled();
        let exact = compiled.exact_for(text).is_some();
        let mut backtracking = (!exact && !self.cannot_match(text)).then(|| compiled.js.find_iter(text));
        let mut next = Some(0);
        std::iter::from_fn(move || {
            if exact {
                let m = self.find_at(text, next?)?;
                next = if m.range().is_empty() { next_char(text, m.end()) } else { Some(m.end()) };
                return Some(m);
            }
            let m = backtracking.as_mut()?.next()?;
            if compiled.sticky {
                if next != Some(m.start()) {
                    return None;
                }
                next = if m.range().is_empty() { next_char(text, m.end()) } else { Some(m.end()) };
            }
            Some(compiled.to_match(m))
        })
    }

    /// Whether the regex matches, like `test` in js (with `lastIndex` 0).
    #[must_use]
    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    /// The utf-16 position of the first match, like `search` in js, which gives -1 for `None`.
    #[must_use]
    pub fn search(&self, text: &str) -> Option<usize> {
        self.find(text).map(|m| utf16_index(text, m.start()))
    }

    /// Replaces the first match, or every match of a global regex, like `replace` in js.
    ///
    /// In `replacement`, `$$` is a `$`, `$&` the match, `` $` `` and `$'` the text before and after it, `$1` to
    /// `$99` a capture group and `$<name>` a named one.
    pub fn replace<'t>(&self, text: &'t str, replacement: &str) -> Cow<'t, str> {
        self.substitute(text, self.is_global(), |m, out| expand(m, text, replacement, out))
    }

    /// Replaces every match whatever the `g` flag, like `replaceAll` in js; see [`Regex::replace`].
    pub fn replace_all<'t>(&self, text: &'t str, replacement: &str) -> Cow<'t, str> {
        self.substitute(text, true, |m, out| expand(m, text, replacement, out))
    }

    /// Replaces the first match, or every match of a global regex, with what `replacement` returns for it, like a
    /// replacement function in js. The match holds the capture groups; `utf16_index(text, m.start())` is the
    /// offset js passes.
    pub fn replace_with<'t>(&self, text: &'t str, mut replacement: impl FnMut(&Match) -> String) -> Cow<'t, str> {
        self.substitute(text, self.is_global(), |m, out| out.push_str(&replacement(m)))
    }

    /// Replaces every match whatever the `g` flag, see [`Regex::replace_with`].
    pub fn replace_all_with<'t>(&self, text: &'t str, mut replacement: impl FnMut(&Match) -> String) -> Cow<'t, str> {
        self.substitute(text, true, |m, out| out.push_str(&replacement(m)))
    }

    fn substitute<'t>(&self, text: &'t str, all: bool, mut push: impl FnMut(&Match, &mut String)) -> Cow<'t, str> {
        let mut out = String::new();
        let mut last = 0;
        let mut splice = |m: Match| {
            out.push_str(&text[last..m.start()]);
            push(&m, &mut out);
            last = m.end();
        };
        if all {
            self.find_iter(text).for_each(&mut splice);
        } else if let Some(m) = self.find(text) {
            splice(m);
        }
        if last == 0 && out.is_empty() {
            return Cow::Borrowed(text);
        }
        out.push_str(&text[last..]);
        Cow::Owned(out)
    }

    /// Splits `text` around every match, like `split` in js.
    ///
    /// The capture groups of each match are spliced in, `None` for a group that did not participate (`undefined`
    /// in js). An empty match splits between two characters, never at the ends of the text, and the `g` and `y`
    /// flags have no effect.
    #[must_use]
    pub fn split<'t>(&self, text: &'t str) -> Vec<Option<&'t str>> {
        self.split_limit(text, usize::MAX)
    }

    /// The first `limit` items of [`Regex::split`], like `split` with a `limit` in js.
    #[must_use]
    pub fn split_limit<'t>(&self, text: &'t str, limit: usize) -> Vec<Option<&'t str>> {
        let mut parts = Vec::new();
        if limit == 0 {
            return parts;
        }
        if self.cannot_match(text) {
            parts.push(Some(text));
            return parts;
        }
        let compiled = self.compiled();
        let exact = compiled.exact_for(text);
        let find = |from: usize| match exact {
            Some(regex) => compiled.find_exact(regex, text, from),
            None => compiled.js.find_from(text, from).next().map(|m| compiled.to_match(m)),
        };
        if text.is_empty() {
            if find(0).is_none() {
                parts.push(Some(text));
            }
            return parts;
        }
        let (mut last, mut from) = (0, 0);
        while let Some(m) = find(from).filter(|m| m.start() < text.len()) {
            if m.end() == last {
                match next_char(text, m.start()) {
                    Some(next) => from = next,
                    None => break,
                }
                continue;
            }
            parts.push(Some(&text[last..m.start()]));
            let end = m.end();
            for index in 1..=m.group_count() {
                if parts.len() == limit {
                    return parts;
                }
                parts.push(m.group(index).map(|range| &text[range]));
            }
            if parts.len() == limit {
                return parts;
            }
            (last, from) = (end, end);
        }
        parts.push(Some(&text[last..]));
        parts
    }
}

impl Compiled {
    fn find_exact(&self, regex: &::regex::Regex, text: &str, start: usize) -> Option<Match> {
        if regex.captures_len() == 1 {
            return regex.find_at(text, start).map(|m| Match { range: m.range(), groups: Vec::new(), names: None });
        }
        let captures = regex.captures_at(text, start)?;
        let groups = captures.iter().skip(1).map(|group| group.map(|group| group.range())).collect();
        Some(Match { range: captures.get_match().range(), groups, names: self.names.clone() })
    }
}

/// The byte position after the character at `index`, `None` at the end of `text`.
fn next_char(text: &str, index: usize) -> Option<usize> {
    text[index..].chars().next().map(|c| index + c.len_utf8())
}

/// Appends `replacement` for the match `m` of `text`, with its `$` patterns substituted like js does.
fn expand(m: &Match, text: &str, replacement: &str, out: &mut String) {
    let group = |index: usize| m.group(index).map_or("", |range| &text[range]);
    let in_groups = |index: usize| (1..=m.group_count()).contains(&index);
    let mut rest = replacement;
    while let Some(dollar) = rest.find('$') {
        out.push_str(&rest[..dollar]);
        rest = &rest[dollar..];
        let digit = |at: usize| rest.as_bytes().get(at).filter(|b| b.is_ascii_digit()).map(|b| usize::from(b - b'0'));
        let (value, length) = match rest.as_bytes().get(1) {
            Some(b'$') => ("$", 2),
            Some(b'&') => (&text[m.range()], 2),
            Some(b'`') => (&text[..m.start()], 2),
            Some(b'\'') => (&text[m.end()..], 2),
            Some(b'0'..=b'9') => {
                let first = digit(1).unwrap_or_default();
                match digit(2).map(|second| first * 10 + second) {
                    Some(index) if in_groups(index) => (group(index), 3),
                    _ if in_groups(first) => (group(first), 2),
                    _ => ("$", 1),
                }
            }
            Some(b'<') if m.names.is_some() => match rest[2..].find('>') {
                Some(end) => (m.named_group(&rest[2..2 + end]).map_or("", |range| &text[range]), end + 3),
                None => ("$", 1),
            },
            _ => ("$", 1),
        };
        out.push_str(value);
        rest = &rest[length..];
    }
    out.push_str(rest);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn found(source: &str, flags: &str, text: &str) -> Option<Range<usize>> {
        Regex::new(source, flags).unwrap().find(text).map(|m| m.range())
    }

    #[test]
    fn repeated_groups_that_prefer_empty_match_like_js() {
        assert_eq!(found("(?:|a)+", "", "aa"), Some(0..2));
        assert_eq!(found("(?:a??)+", "", "aa"), Some(0..2));
        assert_eq!(found("x(?:|a)+", "", "xaa"), Some(0..3));
        assert_eq!(found("(?:a|b)+", "", "abx"), Some(0..2));
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
    fn rejects_flags_js_rejects() {
        for flags in ["x", "gg", "uv", "yy"] {
            assert!(matches!(Regex::new("a", flags), Err(Error::InvalidRegex { .. })), "{flags}");
        }
        assert!(Regex::new("a", "dgimsuy").is_ok());
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
