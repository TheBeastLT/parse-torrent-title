//! Translation of ECMAScript patterns into patterns of the `regex` crate.
//!
//! The backtracking engine giving js semantics is slow, especially at finding out that a pattern does not match,
//! which is the common case: most handlers do not match a given title. So each pattern is also translated into
//! a pattern of the linear time `regex` crate:
//!
//! - When the pattern uses only what the `regex` crate supports, the translation is exact and replaces the
//!   backtracking engine entirely, for the texts given by [`Translation::ascii_text_only`].
//! - Otherwise it matches a superset of the js pattern, a quick check that the pattern can match at all:
//!   lookarounds are dropped (an assertion only narrows the matches) and backreferences match anything.
//!
//! Either way the rest keeps its exact js meaning, such as the ascii only `\w`, `\d` and `\b`.

/// Characters matched by js `\s`.
const JS_SPACES: &str =
    r"\t\n\x0B\x0C\r \x{a0}\x{1680}\x{2000}-\x{200a}\x{2028}\x{2029}\x{202f}\x{205f}\x{3000}\x{feff}";
const WORD: &str = "0-9A-Za-z_";

/// A js pattern translated into the syntax of the `regex` crate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Translation {
    /// Matches a superset of the js pattern, or exactly what it matches when `exact`.
    pub pattern: String,
    /// Whether the pattern finds the same matches and capture groups as the js pattern.
    pub exact: bool,
    /// Whether the exact pattern only agrees with the js pattern on ascii text.
    ///
    /// Without the `u` flag js folds case by uppercasing, unlike the case folding of the `regex` crate, and the
    /// two only provably agree for an ascii pattern on ascii text.
    pub ascii_text_only: bool,
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
    let ascii_text_only = ignore_case && !unicode;
    let ascii_pattern = source.is_ascii() && !source.contains("\\u") && !source.contains("\\x");
    Some(Translation {
        pattern: if ignore_case { format!("(?i){body}") } else { body },
        exact: translator.exact && (!ascii_text_only || ascii_pattern),
        ascii_text_only,
    })
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
                        // js clears the groups of a repeated group on each repetition, unlike the `regex` crate
                        let capturing = !rest.starts_with("(?") || rest.starts_with("(?<");
                        if capturing && depth > 0 {
                            self.exact = false;
                        }
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

/// The index after the quantifier (if any) of a dropped lookaround ending at `i`.
fn skip_quantifier(chars: &[char], i: usize) -> usize {
    let end = match chars.get(i) {
        Some('*' | '+' | '?') => i + 1,
        Some('{') => quantifier_end(chars, i).unwrap_or(i),
        _ => return i,
    };
    if chars.get(end) == Some(&'?') { end + 1 } else { end }
}

#[cfg(test)]
mod tests {
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
        // an accented letter does not fold like js without the `u` flag
        assert!(!exact(r"[a-zé]", "i"));
        assert!(exact(r"[a-zé]", ""));
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
