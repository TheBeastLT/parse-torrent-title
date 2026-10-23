//! Transformers turn the matched text of a regex handler into the field value.
//!
//! A transformer receives the match (its first capture group when present) and the field's current value,
//! and returns `None` when the match should not count.

use std::collections::BTreeSet;
use std::sync::{Arc, LazyLock};

use crate::Value;
use crate::regex::{Regex, js_trim, parse_int};

pub type Transformer = Arc<dyn Fn(&str, Option<&Value>) -> Option<Value> + Send + Sync>;

fn transformer(f: impl Fn(&str, Option<&Value>) -> Option<Value> + Send + Sync + 'static) -> Transformer {
    Arc::new(f)
}

/// The matched text as is.
pub fn none() -> Transformer {
    transformer(|input, _| Some(Value::from(input)))
}

/// A fixed value; in a string value `$1` is replaced with the matched text.
pub fn value(value: impl Into<Value>) -> Transformer {
    let value = value.into();
    transformer(move |input, _| match &value {
        Value::Str(template) => Some(Value::Str(template.replacen("$1", input, 1))),
        other => Some(other.clone()),
    })
}

pub fn integer() -> Transformer {
    transformer(|input, _| parse_int(input).map(Value::Int))
}

const CJK_DIGITS: [char; 9] = ['一', '二', '三', '四', '五', '六', '七', '八', '九'];

fn cjk_digit(text: &str) -> Option<i64> {
    let mut chars = text.chars();
    let digit = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    CJK_DIGITS.iter().position(|&c| c == digit).map(|index| index as i64 + 1)
}

/// Arabic, full width or chinese numerals, i.e. `4`, `４`, `四`, `十二`.
pub fn cjk_integer() -> Transformer {
    transformer(|input, _| {
        let digits: String = input
            .chars()
            .map(|c| match c {
                '０'..='９' => char::from_u32(c as u32 - 0xFEE0).unwrap_or(c),
                _ => c,
            })
            .collect();
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            return parse_int(&digits).map(Value::Int);
        }
        let number = match digits.split_once('十') {
            Some((tens, units)) => {
                let units = units.split('十').next().unwrap_or_default();
                cjk_digit(tens).unwrap_or(1) * 10 + cjk_digit(units).unwrap_or(0)
            }
            None => cjk_digit(&digits).unwrap_or(0),
        };
        Some(Value::Int(number))
    })
}

pub fn boolean() -> Transformer {
    transformer(|_, _| Some(Value::Bool(true)))
}

pub fn lowercase() -> Transformer {
    transformer(|input, _| Some(Value::from(input.to_lowercase())))
}

pub fn uppercase() -> Transformer {
    transformer(|input, _| Some(Value::from(input.to_uppercase())))
}

/// The ascii digit runs of `input`, i.e. `input.replace(/\D+/g, " ").trim().split(" ")`.
fn numbers(input: &str) -> Vec<i64> {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse().ok())
        .collect()
}

static SEPARATED_PAIR: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"^\d+[ .]*[,&+][ .]*\d+$", ""));

/// A list of consecutive numbers, `1-3` being expanded to `[1, 2, 3]`; `None` when not consecutive.
pub fn range() -> Transformer {
    transformer(|input, _| {
        let mut array = numbers(input);
        if array.is_empty() {
            return None;
        }
        if array.len() == 2 && array[0] < array[1] && !SEPARATED_PAIR.is_match(js_trim(input)) {
            array = (array[0]..=array[1]).collect();
        }
        if !array.windows(2).all(|pair| pair[0] + 1 == pair[1]) {
            return None; // array is not in sequence and ascending order
        }
        Some(Value::Ints(array))
    })
}

/// Every number of the match, deduplicated and sorted.
pub fn list() -> Transformer {
    transformer(|input, _| {
        let numbers: BTreeSet<i64> = numbers(input).into_iter().collect();
        Some(Value::Ints(numbers.into_iter().collect()))
    })
}

/// `1..=n` for the matched number `n`.
pub fn range_up_to() -> Transformer {
    transformer(|input, _| match parse_int(input) {
        Some(end) if end > 0 => Some(Value::Ints((1..=end).collect())),
        _ => None,
    })
}

/// A year, or a `"start-end"` year range; a two digit end inherits the century of the start.
pub fn year_range() -> Transformer {
    transformer(|input, _| {
        let parts: Vec<&str> = input.split(|c: char| !c.is_ascii_digit()).collect();
        let parts = collapse_separators(&parts);
        let start = parts.first().filter(|part| !part.is_empty()).and_then(|part| parse_int(part));
        let end = if parts.len() > 1 { parts.last().and_then(|part| parse_int(part)) } else { None };

        let Some(mut end) = end.filter(|&end| end != 0) else {
            return start.map(Value::Int);
        };
        let start = start?;
        if end < 100 {
            end = end + start - start % 100;
        }
        if end <= start {
            return None; // not a year range, try another year handler
        }
        Some(Value::Str(format!("{start}-{end}")))
    })
}

/// `split(/\D+/)` keeps one part per run of separators, while `str::split` yields one per separator.
fn collapse_separators<'a>(parts: &[&'a str]) -> Vec<&'a str> {
    let mut collapsed = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        let is_edge = index == 0 || index == parts.len() - 1;
        if !part.is_empty() || is_edge {
            collapsed.push(*part);
        }
    }
    collapsed
}

/// The value of `chain` wrapped in a list; `array(none())` wraps the matched text.
pub fn array(chain: Transformer) -> Transformer {
    transformer(move |input, current| match chain(input, current)? {
        Value::Int(number) => Some(Value::Ints(vec![number])),
        Value::Str(text) => Some(Value::Strs(vec![text])),
        other => Some(other),
    })
}

/// Appends the value of `chain` to the field's list, unless it is already in it.
pub fn uniq_concat(chain: Transformer) -> Transformer {
    transformer(move |input, current| {
        let value = chain(input, current)?;
        match (current, value) {
            (Some(Value::Strs(values)), Value::Str(value)) => {
                let mut values = values.clone();
                if !values.contains(&value) {
                    values.push(value);
                }
                Some(Value::Strs(values))
            }
            (Some(Value::Ints(values)), Value::Int(value)) => {
                let mut values = values.clone();
                if !values.contains(&value) {
                    values.push(value);
                }
                Some(Value::Ints(values))
            }
            (_, Value::Str(value)) => Some(Value::Strs(vec![value])),
            (_, Value::Int(value)) => Some(Value::Ints(vec![value])),
            (_, value) => Some(value),
        }
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DateToken {
    FullYear,
    ShortYear,
    Month,
    MonthName,
    Day,
}

const MONTHS: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

/// A `YYYY-MM-DD` date read with a moment.js style format such as `"DD MMM YYYY"` or `"YYYYMMDD"`.
///
/// Like moment's forgiving parsing each token takes the next run of text it can read, and an impossible
/// date such as February 31st reads as `"Invalid date"`.
pub fn date(format: &str) -> Transformer {
    let tokens = date_tokens(format);
    transformer(move |input, _| {
        let sanitized: String = input
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        Some(Value::Str(read_date(&tokens, &sanitized).unwrap_or_else(|| "Invalid date".to_owned())))
    })
}

fn date_tokens(format: &str) -> Vec<DateToken> {
    let mut tokens = Vec::new();
    let mut rest = format;
    while !rest.is_empty() {
        let (token, length) = [
            ("YYYY", DateToken::FullYear),
            ("YY", DateToken::ShortYear),
            ("MMM", DateToken::MonthName),
            ("MM", DateToken::Month),
            ("DD", DateToken::Day),
        ]
        .into_iter()
        .find(|(pattern, _)| rest.starts_with(pattern))
        .map(|(pattern, token)| (Some(token), pattern.len()))
        .unwrap_or((None, rest.chars().next().map_or(1, char::len_utf8)));
        tokens.extend(token);
        rest = &rest[length..];
    }
    tokens
}

/// Reads up to `max` digits from the first digit run of `text`, returning the number and the rest.
fn take_number(text: &str, max: usize) -> Option<(i64, &str)> {
    let start = text.find(|c: char| c.is_ascii_digit())?;
    let digits = text[start..].bytes().take(max).take_while(u8::is_ascii_digit).count();
    let end = start + digits;
    Some((text[start..end].parse().ok()?, &text[end..]))
}

/// Reads the first month name (full or abbreviated) of `text`, returning the month and the rest.
fn take_month_name(text: &str) -> Option<(i64, &str)> {
    let lower = text.to_ascii_lowercase();
    (0..lower.len()).filter(|&start| lower.is_char_boundary(start)).find_map(|start| {
        let candidate = &lower[start..];
        MONTHS.iter().enumerate().find_map(|(index, month)| {
            let length = if candidate.starts_with(month) {
                month.len()
            } else if candidate.starts_with(&month[..3]) {
                3
            } else {
                0
            };
            (length > 0).then(|| (index as i64 + 1, &text[start + length..]))
        })
    })
}

fn read_date(tokens: &[DateToken], input: &str) -> Option<String> {
    let (mut year, mut month, mut day) = (None, None, None);
    let mut rest = input;
    for token in tokens {
        let (number, remaining) = match token {
            DateToken::FullYear => take_number(rest, 4)?,
            DateToken::ShortYear => {
                let (short, remaining) = take_number(rest, 2)?;
                (short + if short > 68 { 1900 } else { 2000 }, remaining)
            }
            DateToken::Month | DateToken::Day => take_number(rest, 2)?,
            DateToken::MonthName => take_month_name(rest)?,
        };
        match token {
            DateToken::FullYear | DateToken::ShortYear => year = Some(number),
            DateToken::Month | DateToken::MonthName => month = Some(number),
            DateToken::Day => day = Some(number),
        }
        rest = remaining;
    }

    let (year, month, day) = (year?, month.unwrap_or(1), day.unwrap_or(1));
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1..=12 => 31,
        _ => return None,
    };
    (1..=days).contains(&day).then(|| format!("{year:04}-{month:02}-{day:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply(transformer: Transformer, input: &str) -> Option<Value> {
        transformer(input, None)
    }

    #[test]
    fn reads_dates_like_moment() {
        assert_eq!(apply(date("YYYY MM DD"), "[2019.03.12]"), Some("2019-03-12".into()));
        assert_eq!(apply(date("YYYYMMDD"), "20190312"), Some("2019-03-12".into()));
        assert_eq!(apply(date("DD MMM YYYY"), "1st March 2019"), Some("2019-03-01".into()));
        assert_eq!(apply(date("DD MMM YY"), "12 Sept 19"), Some("2019-09-12".into()));
        assert_eq!(apply(date("MM DD YY"), "04.25.75"), Some("1975-04-25".into()));
        assert_eq!(apply(date("YYYY MM DD"), "2019 02 31"), Some("Invalid date".into()));
    }

    #[test]
    fn reads_year_ranges() {
        assert_eq!(apply(year_range(), "2010-2012"), Some("2010-2012".into()));
        assert_eq!(apply(year_range(), "2010 - 12"), Some("2010-2012".into()));
        assert_eq!(apply(year_range(), "2010"), Some(Value::Int(2010)));
        assert_eq!(apply(year_range(), "2012-2010"), None);
    }

    #[test]
    fn reads_ranges() {
        assert_eq!(apply(range(), "1-3"), Some(vec![1, 2, 3].into()));
        assert_eq!(apply(range(), "1, 2"), Some(vec![1, 2].into()));
        assert_eq!(apply(range(), "1 3"), Some(vec![1, 2, 3].into()));
        assert_eq!(apply(range(), "1, 3"), None);
        assert_eq!(apply(list(), "3e1e3"), Some(vec![1, 3].into()));
    }

    #[test]
    fn reads_cjk_integers() {
        assert_eq!(apply(cjk_integer(), "４"), Some(Value::Int(4)));
        assert_eq!(apply(cjk_integer(), "十二"), Some(Value::Int(12)));
        assert_eq!(apply(cjk_integer(), "二十"), Some(Value::Int(20)));
        assert_eq!(apply(cjk_integer(), "三"), Some(Value::Int(3)));
    }
}
