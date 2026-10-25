//! Translation of ECMAScript patterns into patterns of the `regex` crate.
//!
//! The backtracking engine giving js semantics is slow, especially at finding out that a pattern does not match,
//! which is the common case when many patterns run over each text. So each pattern is also translated into
//! a pattern of the linear time `regex` crate:
//!
//! - When the pattern uses only what the `regex` crate supports, the translation is exact and replaces the
//!   backtracking engine entirely, for the texts given by [`Translation::holds_for`].
//! - Otherwise it matches a superset of the js pattern, a quick check that the pattern can match at all:
//!   lookarounds are dropped (an assertion only narrows the matches) and backreferences match anything.
//!
//! Either way the rest keeps its exact js meaning, such as the ascii only `\w`, `\d` and `\b`.

use regex_syntax::ast::{self, Ast, ClassSetItem};

/// Characters matched by js `\s`.
const JS_SPACES: &str =
    r"\t\n\x0B\x0C\r \x{a0}\x{1680}\x{2000}-\x{200a}\x{2028}\x{2029}\x{202f}\x{205f}\x{3000}\x{feff}";
const WORD: &str = "0-9A-Za-z_";

/// A js pattern translated into the syntax of the `regex` crate, see [`crate::Regex::translation`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Translation {
    /// Matches a superset of the js pattern, or exactly what it matches when `exact`.
    pub(crate) pattern: String,
    /// Whether the pattern finds the same matches and capture groups as the js pattern.
    pub(crate) exact: bool,
    /// Whether the `i` flag folds case by uppercasing, as js does without the `u` flag, unlike the simple case
    /// folding of the `regex` crate: the translation then only holds on the texts given by [`Translation::holds_for`].
    uppercase_folding: bool,
    /// Whether the pattern holds `ſ` or the kelvin sign, which the `regex` crate folds into `s` and `k` unlike js.
    folds_into_ascii: bool,
    /// Whether the pattern only holds ascii chars, so it folds the ascii letters only.
    ascii_pattern: bool,
}

/// The chars whose case folds differently in js without the `u` flag and in the `regex` crate, such as `ſ` which
/// js does not fold into `s` because it uppercases into ascii, or the ohm sign which js does not fold into `ω`.
///
/// Sorted, and checked against both foldings by a test.
const FOLDS_UNLIKE_JS: &[char] = &[
    '\u{c5}',
    '\u{df}',
    '\u{e5}',
    '\u{17f}',
    '\u{390}',
    '\u{398}',
    '\u{3a9}',
    '\u{3b0}',
    '\u{3b8}',
    '\u{3c9}',
    '\u{3d1}',
    '\u{3f4}',
    '\u{1e9e}',
    '\u{1f80}',
    '\u{1f81}',
    '\u{1f82}',
    '\u{1f83}',
    '\u{1f84}',
    '\u{1f85}',
    '\u{1f86}',
    '\u{1f87}',
    '\u{1f88}',
    '\u{1f89}',
    '\u{1f8a}',
    '\u{1f8b}',
    '\u{1f8c}',
    '\u{1f8d}',
    '\u{1f8e}',
    '\u{1f8f}',
    '\u{1f90}',
    '\u{1f91}',
    '\u{1f92}',
    '\u{1f93}',
    '\u{1f94}',
    '\u{1f95}',
    '\u{1f96}',
    '\u{1f97}',
    '\u{1f98}',
    '\u{1f99}',
    '\u{1f9a}',
    '\u{1f9b}',
    '\u{1f9c}',
    '\u{1f9d}',
    '\u{1f9e}',
    '\u{1f9f}',
    '\u{1fa0}',
    '\u{1fa1}',
    '\u{1fa2}',
    '\u{1fa3}',
    '\u{1fa4}',
    '\u{1fa5}',
    '\u{1fa6}',
    '\u{1fa7}',
    '\u{1fa8}',
    '\u{1fa9}',
    '\u{1faa}',
    '\u{1fab}',
    '\u{1fac}',
    '\u{1fad}',
    '\u{1fae}',
    '\u{1faf}',
    '\u{1fb3}',
    '\u{1fbc}',
    '\u{1fc3}',
    '\u{1fcc}',
    '\u{1fd3}',
    '\u{1fe3}',
    '\u{1ff3}',
    '\u{1ffc}',
    '\u{2126}',
    '\u{212a}',
    '\u{212b}',
    '\u{a7cf}',
    '\u{a7d3}',
    '\u{a7d5}',
    '\u{fb05}',
    '\u{fb06}',
    '\u{16ebb}',
    '\u{16ebc}',
    '\u{16ebd}',
    '\u{16ebe}',
    '\u{16ebf}',
    '\u{16ec0}',
    '\u{16ec1}',
    '\u{16ec2}',
    '\u{16ec3}',
    '\u{16ec4}',
    '\u{16ec5}',
    '\u{16ec6}',
    '\u{16ec7}',
    '\u{16ec8}',
    '\u{16ec9}',
    '\u{16eca}',
    '\u{16ecb}',
    '\u{16ecc}',
    '\u{16ecd}',
    '\u{16ece}',
    '\u{16ecf}',
    '\u{16ed0}',
    '\u{16ed1}',
    '\u{16ed2}',
    '\u{16ed3}',
];

impl Translation {
    /// The pattern in the syntax of the `regex` crate, matching a superset of the js pattern, or exactly what it
    /// matches when [`Translation::is_exact`], on the texts given by [`Translation::holds_for`].
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Whether the pattern finds the same matches and capture groups as the js pattern.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.exact
    }

    /// Whether the translation, exact or a superset, holds on `text`.
    ///
    /// Folding by uppercasing differs from the simple case folding only on `FOLDS_UNLIKE_JS`, about a hundred
    /// characters such as `ß` or the ohm sign, and on `s` and `k` for a pattern holding `ſ` or the kelvin sign.
    #[must_use]
    pub fn holds_for(&self, text: &str) -> bool {
        if !self.uppercase_folding {
            return true;
        }
        let ascii_holds = |byte: u8| !(self.folds_into_ascii && matches!(byte, b's' | b'S' | b'k' | b'K'));
        if text.is_ascii() {
            return !self.folds_into_ascii || text.bytes().all(ascii_holds);
        }
        text.chars().all(|c| match u8::try_from(c) {
            Ok(byte) if c.is_ascii() => ascii_holds(byte),
            _ => FOLDS_UNLIKE_JS.binary_search(&c).is_err(),
        })
    }

    /// Whether the pattern folds case like the ascii case insensitive search of `aho-corasick`, i.e. it is an
    /// ascii pattern folding by uppercasing, which only folds ascii letters into ascii letters.
    #[must_use]
    pub fn folds_ascii_letters_only(&self) -> bool {
        self.uppercase_folding && self.ascii_pattern
    }
}

/// Translates a js pattern, `None` when it uses syntax this translation does not know.
pub(crate) fn translate(source: &str, flags: &str) -> Option<Translation> {
    // the set notation of the `v` flag is not translated
    if flags.contains('v') {
        return None;
    }
    let (unicode, ignore_case) = (flags.contains('u'), flags.contains('i'));
    let chars: Vec<char> = source.chars().collect();
    let mut translator =
        Translator { unicode, ignore_case, dot_all: flags.contains('s'), multiline: flags.contains('m'), exact: true };
    let body = translator.body(&chars)?;
    let uppercase_folding = ignore_case && !unicode;
    Some(Translation {
        folds_into_ascii: uppercase_folding && folds_into_ascii(&body),
        pattern: if ignore_case { format!("(?i){body}") } else { body },
        exact: translator.exact,
        uppercase_folding,
        ascii_pattern: source.is_ascii() && !source.contains("\\u") && !source.contains("\\x"),
    })
}

/// Whether a translated pattern holds `ſ` or the kelvin sign, as a char or in a range of a class, `true` when it
/// can not be parsed.
fn folds_into_ascii(pattern: &str) -> bool {
    struct Visitor(bool);
    impl Visitor {
        fn check(&mut self, start: char, end: char) {
            self.0 |= ['\u{17f}', '\u{212a}'].iter().any(|c| (start..=end).contains(c));
        }
    }
    impl ast::Visitor for Visitor {
        type Output = bool;
        type Err = ();
        fn finish(self) -> Result<bool, ()> {
            Ok(self.0)
        }
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), ()> {
            if let Ast::Literal(literal) = ast {
                self.check(literal.c, literal.c);
            }
            Ok(())
        }
        fn visit_class_set_item_pre(&mut self, item: &ClassSetItem) -> Result<(), ()> {
            match item {
                ClassSetItem::Literal(literal) => self.check(literal.c, literal.c),
                ClassSetItem::Range(range) => self.check(range.start.c, range.end.c),
                _ => {}
            }
            Ok(())
        }
    }
    ast::parse::Parser::new().parse(pattern).map_or(true, |ast| ast::visit(&ast, Visitor(false)).unwrap_or(true))
}

struct Translator {
    unicode: bool,
    ignore_case: bool,
    /// The `s` flag: `.` also matches line terminators.
    dot_all: bool,
    /// The `m` flag: `^` and `$` also match at js line terminators, which are dropped as a superset.
    multiline: bool,
    /// Cleared as soon as something is approximated.
    exact: bool,
}

impl Translator {
    /// Translates a pattern, or the body of a lookaround.
    ///
    /// A positive lookbehind starting a top level alternative is kept as a group, since the text it matches comes
    /// right before the rest: `(?<=X)Y` can only match where `XY` does. The same holds for a positive lookahead
    /// ending a top level alternative. Anywhere else a lookaround could overlap the text around it, so it is dropped.
    fn body(&mut self, chars: &[char]) -> Option<String> {
        let mut out = String::with_capacity(chars.len() + 16);
        let mut depth = 0usize;
        let mut capturing_groups = Vec::new();
        let mut alternative_start = true;
        let mut i = 0;
        while i < chars.len() {
            let starts_alternative = alternative_start;
            alternative_start = false;
            match chars[i] {
                '\\' => {
                    let (translated, next) = self.escape(chars, i, false)?;
                    out.push_str(&translated);
                    i = next;
                }
                '[' => {
                    let (translated, next) = self.class(chars, i)?;
                    out.push_str(&translated);
                    i = next;
                }
                '(' => {
                    let rest: String = chars[i..chars.len().min(i + 4)].iter().collect();
                    let lookbehind = rest.starts_with("(?<=");
                    let lookahead = rest.starts_with("(?=");
                    if lookbehind || lookahead || rest.starts_with("(?!") || rest.starts_with("(?<!") {
                        self.exact = false;
                        let end = group_end(chars, i)?;
                        let after = skip_quantifier(chars, end);
                        let unquantified = after == end;
                        let next_is_lookaround = chars.get(end..end + 2) == Some(&['(', '?']);
                        let ends_alternative = matches!(chars.get(end), None | Some('|'));
                        let keep = depth == 0
                            && unquantified
                            && ((lookbehind && starts_alternative && !next_is_lookaround)
                                || (lookahead && ends_alternative));
                        if keep {
                            let body_start = if lookbehind { i + 4 } else { i + 3 };
                            out.push_str("(?:");
                            out.push_str(&self.body(&chars[body_start..end - 1])?);
                            out.push(')');
                        } else if starts_alternative {
                            // a dropped lookaround is zero width, so what follows still starts the alternative
                            alternative_start = true;
                        }
                        i = after;
                    } else {
                        if rest.starts_with("(?") && matches!(chars.get(i + 2), Some('i' | 'm' | 's' | '-')) {
                            return None;
                        }
                        // js clears the groups of a repeated group on each repetition, unlike the `regex` crate
                        let capturing = !rest.starts_with("(?") || rest.starts_with("(?<");
                        if capturing && depth > 0 {
                            self.exact = false;
                        }
                        capturing_groups.push((capturing, out.len()));
                        depth += 1;
                        if rest.starts_with("(?<") {
                            out.push_str("(?P<");
                            i += 3;
                        } else {
                            out.push('(');
                            i += 1;
                        }
                    }
                }
                ')' => {
                    let group = capturing_groups.pop();
                    if skip_quantifier(chars, i + 1) != i + 1
                        && group.is_some_and(|(capturing, start)| {
                            capturing || can_match_empty(&format!("{})", &out[start..]))
                        })
                    {
                        self.exact = false;
                    }
                    depth = depth.saturating_sub(1);
                    out.push(')');
                    i += 1;
                }
                '|' => {
                    alternative_start = depth == 0;
                    out.push('|');
                    i += 1;
                }
                '{' => match quantifier_end(chars, i) {
                    Some(end) => {
                        out.extend(&chars[i..end]);
                        i = end;
                    }
                    None => {
                        out.push_str(r"\{");
                        i += 1;
                    }
                },
                '}' => {
                    out.push_str(r"\}");
                    i += 1;
                }
                ']' => {
                    out.push_str(r"\]");
                    i += 1;
                }
                '.' => {
                    out.push_str(if self.dot_all { r"(?s:.)" } else { r"[^\n\r\x{2028}\x{2029}]" });
                    i += 1;
                }
                '^' | '$' if self.multiline => {
                    self.exact = false;
                    i += 1;
                }
                c => {
                    out.push(c);
                    i += 1;
                }
            }
        }
        Some(out)
    }

    /// Translates the escape at `chars[i]`, returning it and the index after it.
    fn escape(&mut self, chars: &[char], i: usize, in_class: bool) -> Option<(String, usize)> {
        let (unicode, ignore_case) = (self.unicode, self.ignore_case);
        let c = *chars.get(i + 1)?;
        let hex = |from: usize, count: usize| -> Option<u32> {
            let digits: String = chars.get(from..from + count)?.iter().collect();
            u32::from_str_radix(&digits, 16).ok()
        };
        let literal = |code: u32| format!(r"\x{{{code:x}}}");
        let class_escape = |positive: &str, negated: bool| {
            // inside a class a negated class escape becomes a nested class, which the `regex` crate unions
            match (in_class, negated) {
                (true, false) => positive.to_owned(),
                (_, true) => format!("[^{positive}]"),
                (false, false) => format!("[{positive}]"),
            }
        };
        let translated = match c {
            'd' => class_escape("0-9", false),
            'D' => class_escape("0-9", true),
            'w' => class_escape(WORD, false),
            'W' => class_escape(WORD, true),
            's' => class_escape(JS_SPACES, false),
            'S' => class_escape(JS_SPACES, true),
            'b' if in_class => literal(8),
            // with both `i` and `u` js `\w` also holds `ſ` and the kelvin sign, so the boundary is dropped instead
            'b' | 'B' if unicode && ignore_case => {
                self.exact = false;
                String::new()
            }
            'b' => r"(?-u:\b)".to_owned(),
            'B' => r"(?-u:\B)".to_owned(),
            '1'..='9' if !in_class => {
                self.exact = false;
                let digits = chars[i + 1..].iter().take_while(|c| c.is_ascii_digit()).count();
                return Some(("(?s:.*)".to_owned(), i + 1 + digits));
            }
            'k' if !in_class && chars.get(i + 2) == Some(&'<') => {
                self.exact = false;
                let end = chars[i..].iter().position(|&c| c == '>')?;
                return Some(("(?s:.*)".to_owned(), i + end + 1));
            }
            // js folds the case of what `\P{..}` matches, while the `regex` crate negates the folded property
            'P' if unicode && ignore_case => return None,
            'p' | 'P' if unicode => {
                let end = chars[i..].iter().position(|&c| c == '}')?;
                return Some((chars[i..=i + end].iter().collect(), i + end + 1));
            }
            'u' if chars.get(i + 2) == Some(&'{') && unicode => {
                let end = chars[i..].iter().position(|&c| c == '}')?;
                let digits: String = chars[i + 3..i + end].iter().collect();
                return Some((literal(u32::from_str_radix(&digits, 16).ok()?), i + end + 1));
            }
            'u' => return Some((literal(hex(i + 2, 4)?), i + 6)),
            'x' => return Some((literal(hex(i + 2, 2)?), i + 4)),
            'n' => literal(10),
            'r' => literal(13),
            't' => literal(9),
            'f' => literal(12),
            'v' => literal(11),
            // a legacy octal escape such as `\01` is not translated
            '0' if chars.get(i + 2).is_some_and(char::is_ascii_digit) => return None,
            '0' => literal(0),
            'c' => return None,
            c if c.is_ascii_punctuation() => format!(r"\{c}"),
            // an identity escape of a letter (non unicode mode) or any other char is the char itself
            c => literal(c as u32),
        };
        Some((translated, i + 2))
    }

    /// Translates the class starting at `chars[i]`, returning it and the index after it.
    fn class(&mut self, chars: &[char], i: usize) -> Option<(String, usize)> {
        let mut out = String::from("[");
        let mut j = i + 1;
        if chars.get(j) == Some(&'^') {
            out.push('^');
            j += 1;
        }
        if chars.get(j) == Some(&']') {
            return None; // `[]` and `[^]` have no direct equivalent
        }
        // whether the last item was a single char, which a following `-` makes the start of a range
        let mut last_was_char = false;
        while *chars.get(j)? != ']' {
            let c = chars[j];
            if c == '\\' {
                let next = *chars.get(j + 1)?;
                let (translated, after) = self.escape(chars, j, true)?;
                out.push_str(&translated);
                last_was_char = !matches!(next, 'd' | 'D' | 'w' | 'W' | 's' | 'S' | 'p' | 'P');
                j = after;
            } else if c == '-'
                && last_was_char
                && chars.get(j + 1).is_some_and(|&next| next != ']')
                && !is_class_escape(chars, j + 1)
            {
                out.push('-');
                last_was_char = false;
                j += 1;
            } else {
                if c.is_ascii_punctuation() {
                    out.push('\\');
                }
                out.push(c);
                last_was_char = true;
                j += 1;
            }
        }
        out.push(']');
        Some((out, j + 1))
    }
}

fn is_class_escape(chars: &[char], i: usize) -> bool {
    chars.get(i) == Some(&'\\') && matches!(chars.get(i + 1), Some('d' | 'D' | 'w' | 'W' | 's' | 'S' | 'p' | 'P'))
}

/// The index after the group starting at `chars[i]`.
fn group_end(chars: &[char], i: usize) -> Option<usize> {
    let mut depth = 0;
    let mut in_class = false;
    let mut j = i;
    while j < chars.len() {
        match chars[j] {
            '\\' => j += 1,
            '[' => in_class = true,
            ']' => in_class = false,
            '(' if !in_class => depth += 1,
            ')' if !in_class => {
                depth -= 1;
                if depth == 0 {
                    return Some(j + 1);
                }
            }
            _ => {}
        }
        j += 1;
    }
    None
}

/// The index after a `{n}`, `{n,}` or `{n,m}` quantifier starting at `chars[i]`.
fn quantifier_end(chars: &[char], i: usize) -> Option<usize> {
    let close = i + chars[i..].iter().position(|&c| c == '}')?;
    let inner: String = chars[i + 1..close].iter().collect();
    let (min, max) = inner.split_once(',').unwrap_or((&inner, "0"));
    let valid = !min.is_empty() && min.bytes().all(|b| b.is_ascii_digit()) && max.bytes().all(|b| b.is_ascii_digit());
    valid.then_some(close + 1)
}

/// The index after the quantifier (if any) at `i`, such as the one of a dropped lookaround ending there.
fn skip_quantifier(chars: &[char], i: usize) -> usize {
    let end = match chars.get(i) {
        Some('*' | '+' | '?') => i + 1,
        Some('{') => quantifier_end(chars, i).unwrap_or(i),
        _ => return i,
    };
    if chars.get(end) == Some(&'?') { end + 1 } else { end }
}

fn can_match_empty(group_body: &str) -> bool {
    regex_syntax::parse(group_body).map_or(true, |hir| hir.properties().minimum_len() == Some(0))
}

#[cfg(test)]
mod tests {
    use regex_syntax::hir::{ClassUnicode, ClassUnicodeRange};

    use super::*;

    fn pattern(source: &str, flags: &str) -> String {
        translate(source, flags).unwrap().pattern
    }

    fn exact(source: &str, flags: &str) -> bool {
        translate(source, flags).unwrap().exact
    }

    #[test]
    fn keeps_the_js_meaning_of_classes() {
        assert_eq!(pattern(r"\d{1,2}\W", "i"), r"(?i)[0-9]{1,2}[^0-9A-Za-z_]");
        assert_eq!(pattern(r"[[(]\w[^\]\d-]", ""), r"[\[\(][0-9A-Za-z_][^\]0-9\-]");
        assert_eq!(pattern(r"[a-z]{3,}x{", ""), r"[a-z]{3,}x\{");
        assert_eq!(pattern(r"a.b", ""), r"a[^\n\r\x{2028}\x{2029}]b");
    }

    #[test]
    fn approximates_lookarounds_and_backreferences() {
        assert_eq!(pattern(r"(?<=\p{Ll}.*)\bMALAY\b", "u"), r"(?:\p{Ll}[^\n\r\x{2028}\x{2029}]*)(?-u:\b)MALAY(?-u:\b)");
        assert_eq!(pattern(r"(?<!\d)x(?=y)", ""), "x(?:y)");
        assert_eq!(pattern(r"(?<!\d)(?<=a)x(?=y)z", ""), "(?:a)xz");
        assert_eq!(pattern(r"(?<=a)(?<=b)x|y(?<=c)z|(?:(?<=d)w)", ""), "(?:b)x|yz|(?:w)");
        assert_eq!(pattern(r"(?<=a)?x(?=y)+", ""), "x");
        assert_eq!(pattern(r"(\d)[. ]\1", ""), r"([0-9])[\. ](?s:.*)");
        assert!(!exact(r"(?<=a)x", ""));
        assert!(!exact(r"(\d)\1", ""));
        assert!(!exact(r"\bx", "iu"));
    }

    #[test]
    fn is_exact_only_when_nothing_is_approximated() {
        assert!(exact(r"\b(?:x26[45]|hevc)\b", "i"));
        assert!(exact(r"(?:^|\D)(\d{3,4})[pi]", "i"));
        assert!(exact(r"\bx\b", "u"));
        // a group inside a repeated group keeps its last value in the `regex` crate, unlike in js
        assert!(!exact(r"(?:(a)|b)+", ""));
        assert!(!exact(r"(a*)*b", ""));
        assert!(!exact(r"(a|)+b", ""));
        assert!(!exact(r"(?:a|)+b", ""));
        assert!(!exact(r"(?:|a)+", ""));
        assert!(exact(r"(?:a|b)+", ""));
        // the few chars that fold unlike js without the `u` flag are left to `Translation::holds_for`
        assert!(exact(r"[a-zé]", "i"));
        assert!(exact(r"[a-zé]", ""));
    }

    /// The single char js `toUpperCase` of `c`, which js folds case by without the `u` flag.
    fn js_canonical(c: char) -> char {
        let mut upper = c.to_uppercase();
        match (upper.next(), upper.next()) {
            (Some(upper), None) if c.is_ascii() || !upper.is_ascii() => upper,
            _ => c,
        }
    }

    #[test]
    fn lists_the_chars_that_fold_unlike_js() {
        let folds_unlike_js = |c: char| {
            let mut class = ClassUnicode::new([ClassUnicodeRange::new(c, c)]);
            class.case_fold_simple();
            let folded: Vec<char> = class.iter().flat_map(|range| range.start()..=range.end()).collect();
            let canonical = js_canonical(c);
            !folded.contains(&canonical) || folded.iter().any(|&other| js_canonical(other) != canonical)
        };
        let expected: Vec<char> =
            (0..=0x10_ffff).filter_map(char::from_u32).filter(|c| !c.is_ascii() && folds_unlike_js(*c)).collect();
        assert!(FOLDS_UNLIKE_JS.is_sorted());
        assert_eq!(FOLDS_UNLIKE_JS, expected, "the unicode tables changed, update FOLDS_UNLIKE_JS");
    }

    #[test]
    fn holds_where_both_foldings_agree() {
        let holds = |source: &str, flags: &str, text: &str| translate(source, flags).unwrap().holds_for(text);
        assert!(holds(r"\W", "i", "Seasons ¼ Жж"));
        assert!(!holds(r"\W", "i", "ſeason"));
        assert!(!holds(r"x", "i", "\u{2126}"));
        assert!(holds(r"x", "iu", "\u{2126}"));
        assert!(holds(r"[\u0100-\u024f]", "i", "movie"));
        assert!(!holds(r"[\u0100-\u024f]", "i", "season"));
        assert!(holds(r"[\u0100-\u024f]", "", "\u{2126}"));
    }

    #[test]
    fn leaves_modifiers_to_js() {
        assert!(translate(r"(?i:a)b", "").is_none());
        assert!(translate(r"(?-i:a)b", "i").is_none());
        assert!(translate(r"(?:a)b", "i").is_some());
    }

    #[test]
    fn leaves_folded_negated_properties_to_js() {
        assert!(translate(r"\P{Ll}", "iu").is_none());
        assert!(translate(r"\P{Ll}", "u").is_some());
        assert!(translate(r"\P", "i").is_some());
    }

    #[test]
    fn follows_the_flags() {
        assert_eq!(pattern(r"a.b", "s"), r"a(?s:.)b");
        assert!(exact(r"a.b", "s"));
        assert_eq!(pattern(r"^a$", "m"), "a");
        assert!(!exact(r"^a$", "m"));
        assert!(translate(r"[\w--\d]", "v").is_none());
        assert!(translate(r"\01", "").is_none());
    }

    #[test]
    fn matches_a_superset() {
        let check = |source: &str, flags: &str, text: &str| {
            let js = regress::Regex::with_flags(source, flags).unwrap();
            let translated = ::regex::Regex::new(&translate(source, flags).unwrap().pattern).unwrap();
            if js.find(text).is_some() {
                assert!(translated.is_match(text), "/{source}/{flags} on {text:?}");
            }
        };
        check(r"(?<=\p{Ll}.*)\b(?:MALAY)\b", "u", "Movie MALAY");
        check(r"[^\w\-. []+[^\-. [)\]\d]", "i", "group]x");
        check(r"\bS(\d{1,2})[ .]?E\1", "i", "S01E01");
        check(r"\s+[/|]\s+", "", "a \u{3000}/ b");
        check(r"[^[\]]*]", "", "abc]");
        check(r"^b", "m", "a\nb");
        check(r"a$", "m", "a\nb");
        check(r"a.b", "s", "a\nb");
    }
}
