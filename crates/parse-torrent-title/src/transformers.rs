//! Transformers turn the matched text of a regex handler into the field value.
//!
//! A transformer receives the match (its first capture group when present) and the field's current value,
//! and returns `None` when the match should not count.

use std::collections::BTreeSet;
use std::sync::{Arc, LazyLock};

use chrono::NaiveDate;
use chrono::format::StrftimeItems;

use crate::{Error, Regex, Value};

pub type Transformer = Arc<dyn Fn(&str, Option<&Value>) -> Option<Value> + Send + Sync>;

/// The most numbers a range expands to, so a match like `s01-9999999999` does not allocate them all.
const MAX_RANGE: i64 = 10_000;

fn transformer(f: impl Fn(&str, Option<&Value>) -> Option<Value> + Send + Sync + 'static) -> Transformer {
    Arc::new(f)
}

/// The matched text as is.
#[must_use]
pub fn none() -> Transformer {
    transformer(|input, _| Some(Value::from(input)))
}

/// A fixed value; in a string value `$1` is replaced with the matched text.
#[must_use]
pub fn value(value: impl Into<Value>) -> Transformer {
    let value = value.into();
    transformer(move |input, _| match &value {
        Value::Str(template) => Some(Value::Str(template.replacen("$1", input, 1))),
        other => Some(other.clone()),
    })
}

/// The number written by the match, as an integer when it fits one.
///
/// Like js `parseInt`, a number followed by other text such as `12th` reads as its leading digits, but a whole
/// decimal number such as `12.5` or `1e3` keeps its value instead of being cut at the `.` or `e`.
#[must_use]
pub fn integer() -> Transformer {
    transformer(|input, _| {
        let input = input.trim();
        number(input).or_else(|| number(leading_integer(input)))
    })
}

fn number(digits: &str) -> Option<Value> {
    match digits.parse::<i64>() {
        Ok(number) => Some(Value::Int(number)),
        Err(_) => {
            let number = serde_json::Number::from_f64(digits.parse().ok()?)?;
            Some(Value::Json(serde_json::Value::Number(number)))
        }
    }
}

/// The optionally signed leading digits of `text`.
fn leading_integer(text: &str) -> &str {
    let sign = usize::from(text.starts_with(['+', '-']));
    let digits = text[sign..].bytes().take_while(u8::is_ascii_digit).count();
    &text[..sign + digits]
}

const CJK_DIGITS: [char; 9] = ['一', '二', '三', '四', '五', '六', '七', '八', '九'];

fn cjk_digit(text: &str) -> Option<i64> {
    let mut chars = text.chars();
    let digit = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    let index = CJK_DIGITS.iter().position(|&c| c == digit)?;
    i64::try_from(index + 1).ok()
}

/// Arabic, full width or chinese numerals, i.e. `4`, `４`, `四`, `十二`.
#[must_use]
pub fn cjk_integer() -> Transformer {
    transformer(|input, _| {
        let digits: String = input
            .chars()
            .map(|c| match c {
                '０'..='９' => char::from_u32(u32::from(c) - 0xFEE0).unwrap_or(c),
                _ => c,
            })
            .collect();
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            return number(&digits);
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

#[must_use]
pub fn boolean() -> Transformer {
    transformer(|_, _| Some(Value::Bool(true)))
}

#[must_use]
pub fn lowercase() -> Transformer {
    transformer(|input, _| Some(Value::from(input.to_lowercase())))
}

#[must_use]
pub fn uppercase() -> Transformer {
    transformer(|input, _| Some(Value::from(input.to_uppercase())))
}

/// The ascii digit runs of `input`, `None` when one does not fit an integer.
fn numbers(input: &str) -> Option<Vec<i64>> {
    input.split(|c: char| !c.is_ascii_digit()).filter(|part| !part.is_empty()).map(|part| part.parse().ok()).collect()
}

static SEPARATED_PAIR: LazyLock<Regex> = LazyLock::new(|| Regex::lazy(r"^\d+[ .]*[,&+][ .]*\d+$", ""));

/// A list of consecutive numbers, `1-3` being expanded to `[1, 2, 3]`; `None` when not consecutive.
#[must_use]
pub fn range() -> Transformer {
    transformer(|input, _| {
        let mut array = numbers(input)?;
        if let [start, end] = array[..]
            && start < end
            && !SEPARATED_PAIR.is_match(input.trim())
        {
            if end - start > MAX_RANGE {
                return None;
            }
            array = (start..=end).collect();
        }
        let consecutive = array.windows(2).all(|pair| pair[0].checked_add(1) == Some(pair[1]));
        // an empty list or numbers out of sequence are not a range
        (consecutive && !array.is_empty()).then_some(Value::Ints(array))
    })
}

/// Every number of the match, deduplicated and sorted.
#[must_use]
pub fn list() -> Transformer {
    transformer(|input, _| {
        let numbers: BTreeSet<i64> = numbers(input)?.into_iter().collect();
        Some(Value::Ints(numbers.into_iter().collect()))
    })
}

/// `1..=n` for the matched number `n`.
#[must_use]
pub fn range_up_to() -> Transformer {
    transformer(|input, _| match input.trim().parse::<i64>() {
        Ok(end @ 1..=MAX_RANGE) => Some(Value::Ints((1..=end).collect())),
        _ => None,
    })
}

/// A year, or a `"start-end"` year range; a two digit end inherits the century of the start.
#[must_use]
pub fn year_range() -> Transformer {
    transformer(|input, _| {
        let mut parts = input.split(|c: char| !c.is_ascii_digit()).filter(|part| !part.is_empty());
        let start: i64 = parts.next()?.parse().ok()?;
        let Some(mut end) = parts.next_back().and_then(|end| end.parse::<i64>().ok()).filter(|&end| end != 0) else {
            return Some(Value::Int(start));
        };
        if end < 100 {
            end = end.checked_add(start - start % 100)?;
        }
        // an end before the start is not a year range, so another year handler gets a chance
        (end > start).then(|| Value::Str(format!("{start}-{end}")))
    })
}

/// The value of `chain` wrapped in a list; `array(none())` wraps the matched text.
#[must_use]
pub fn array(chain: Transformer) -> Transformer {
    transformer(move |input, _| {
        let value = chain(input, None)?;
        Some(Value::from_json(serde_json::Value::Array(vec![value.to_json()])))
    })
}

/// Appends the value of `chain` to the field's list unless an equal value is already in it; a list value is
/// appended item by item, like js `concat`.
#[must_use]
pub fn uniq_concat(chain: Transformer) -> Transformer {
    transformer(move |input, current| {
        let value = chain(input, None)?.to_json();
        let mut values = match current.map(Value::to_json) {
            Some(serde_json::Value::Array(values)) => values,
            _ => Vec::new(),
        };
        // a list is appended item by item like js `concat`, without checking whether its items are already there
        let found = !value.is_array() && values.contains(&value);
        if !found {
            match value {
                serde_json::Value::Array(items) => values.extend(items),
                item => values.push(item),
            }
        }
        Some(Value::from_json(serde_json::Value::Array(values)))
    })
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

/// A `YYYY-MM-DD` date read with a [`chrono` format](chrono::format::strftime) such as `"%d %b %Y"`.
///
/// The match is first reduced to its words separated by single spaces, ordinal suffixes are dropped (`1st`) and
/// month names are shortened to what `%b` reads (`Sept`, `March`), so `"[1st.March.2019]"` reads with `"%d %b %Y"`.
/// The separators of the format are read the same way, so `"%Y-%m-%d"` is the same format as `"%Y %m %d"`.
/// A match which is not a valid date does not count.
///
/// # Errors
///
/// When `format` is not a valid `chrono` format.
pub fn date(format: &str) -> Result<Transformer, Error> {
    if StrftimeItems::new(format).parse().is_err() {
        return Err(Error::InvalidDateFormat { format: format.to_owned() });
    }
    let format = date_words_format(format);
    Ok(transformer(move |input, _| {
        let words: Vec<&str> = input.split(|c: char| !c.is_alphanumeric()).filter(|word| !word.is_empty()).collect();
        let mut normalized = Vec::with_capacity(words.len());
        for (index, word) in words.iter().enumerate() {
            let follows_number = index > 0 && words[index - 1].bytes().all(|b| b.is_ascii_digit());
            if follows_number && is_ordinal_suffix(word) {
                continue;
            }
            normalized.push(normalize_date_word(word));
        }
        let date = NaiveDate::parse_from_str(&normalized.join(" "), &format).ok()?;
        Some(Value::Str(date.format("%Y-%m-%d").to_string()))
    }))
}

/// `format` with its separators as single spaces, like the words of the matched text.
fn date_words_format(format: &str) -> String {
    let mut words = vec![String::new()];
    let mut chars = format.chars();
    while let Some(c) = chars.next() {
        let word = words.last_mut().expect("there is always a word");
        if c == '%' {
            // a specifier, with its padding flag if it has one, is part of the word
            word.push(c);
            word.extend(chars.next());
            if word.ends_with(['-', '_', '0']) {
                word.extend(chars.next());
            }
        } else if c.is_alphanumeric() {
            word.push(c);
        } else if !word.is_empty() {
            words.push(String::new());
        }
    }
    words.retain(|word| !word.is_empty());
    words.join(" ")
}

fn is_ordinal_suffix(word: &str) -> bool {
    ["st", "nd", "rd", "th"].iter().any(|suffix| word.eq_ignore_ascii_case(suffix))
}

/// `1st` as `1`, and a month name or its abbreviation (`sept`) as the three letters `%b` reads.
fn normalize_date_word(word: &str) -> String {
    let digits = word.bytes().take_while(u8::is_ascii_digit).count();
    if digits > 0 && is_ordinal_suffix(&word[digits..]) {
        return word[..digits].to_owned();
    }
    let lower = word.to_ascii_lowercase();
    match MONTHS.iter().find(|month| lower.len() >= 3 && month.starts_with(&lower)) {
        Some(month) => month[..3].to_owned(),
        None => word.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply(transformer: Transformer, input: &str) -> Option<Value> {
        transformer(input, None)
    }

    fn date(format: &str) -> Transformer {
        super::date(format).unwrap()
    }

    #[test]
    fn reads_dates() {
        assert_eq!(apply(date("%Y %m %d"), "[2019.03.12]"), Some("2019-03-12".into()));
        assert_eq!(apply(date("%Y%m%d"), "20190312"), Some("2019-03-12".into()));
        assert_eq!(apply(date("%d %b %Y"), "1st March 2019"), Some("2019-03-01".into()));
        assert_eq!(apply(date("%d %b %Y"), "6 th August 2020"), Some("2020-08-06".into()));
        assert_eq!(apply(date("%d %b %y"), "12 Sept 19"), Some("2019-09-12".into()));
        assert_eq!(apply(date("%m %d %y"), "04.25.75"), Some("1975-04-25".into()));
        assert_eq!(apply(date("%Y %m %d"), "2019 02 31"), None);
        assert_eq!(apply(date("%Y-%m-%d"), "2019.03.12"), Some("2019-03-12".into()));
        assert_eq!(apply(date("%-d/%m/%Y"), "5/03/2019"), Some("2019-03-05".into()));
        assert!(super::date("%Q").is_err());
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
    fn does_not_expand_huge_ranges() {
        assert_eq!(apply(range(), "s01-9999999999999"), None);
        assert_eq!(apply(range(), "1-99999999999999999999"), None);
        assert_eq!(
            apply(range(), "9223372036854775806 9223372036854775807"),
            Some(vec![i64::MAX - 1, i64::MAX].into())
        );
        assert_eq!(apply(range_up_to(), "9999999999"), None);
    }

    #[test]
    fn composes_like_js() {
        let current = Value::Ints(vec![1]);
        assert_eq!(apply(array(boolean()), "x"), Some(Value::Json(serde_json::json!([true]))));
        assert_eq!(apply(array(range()), "1-3"), Some(Value::Json(serde_json::json!([[1, 2, 3]]))));
        assert_eq!(uniq_concat(range())("2-3", Some(&current)), Some(vec![1, 2, 3].into()));
        assert_eq!(uniq_concat(integer())("1", Some(&current)), Some(vec![1].into()));
        let strings = Value::Strs(vec!["a".into()]);
        assert_eq!(uniq_concat(integer())("5", Some(&strings)), Some(Value::Json(serde_json::json!(["a", 5]))));
    }

    #[test]
    fn reads_integers() {
        assert_eq!(apply(integer(), "05"), Some(Value::Int(5)));
        assert_eq!(apply(integer(), "99999999999999999999"), Some(Value::Json(serde_json::json!(1e20))));
        assert_eq!(apply(integer(), "x"), None);
        assert_eq!(apply(integer(), "12th"), Some(Value::Int(12)));
        assert_eq!(apply(integer(), "-3rd"), Some(Value::Int(-3)));
        assert_eq!(apply(integer(), "12.5"), Some(Value::Json(serde_json::json!(12.5))));
        assert_eq!(apply(integer(), "inf"), None);
    }

    #[test]
    fn reads_cjk_integers() {
        assert_eq!(apply(cjk_integer(), "４"), Some(Value::Int(4)));
        assert_eq!(apply(cjk_integer(), "十二"), Some(Value::Int(12)));
        assert_eq!(apply(cjk_integer(), "二十"), Some(Value::Int(20)));
        assert_eq!(apply(cjk_integer(), "三"), Some(Value::Int(3)));
    }
}
