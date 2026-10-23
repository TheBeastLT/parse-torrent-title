//! Assertions mirroring the chai assertions of the js test suite.

use std::time::{Duration, Instant};

use parse_torrent_title::parse;
pub use serde_json::json;
use serde_json::{Map, Value};

fn parsed(title: &str) -> Map<String, Value> {
    match serde_json::to_value(parse(title)) {
        Ok(Value::Object(fields)) => fields,
        other => panic!("parsed title should serialize to an object, got {other:?}"),
    }
}

/// `expect(parse(title)).to.deep.include(expected)`
#[track_caller]
pub fn assert_includes(title: &str, expected: Value) {
    let actual = parsed(title);
    for (field, value) in expected.as_object().expect("expected fields") {
        assert_eq!(actual.get(field), Some(value), "field `{field}` of {title:?}, parsed as {actual:?}");
    }
}

/// `expect(parse(title)).to.deep.equal(expected)`
#[track_caller]
pub fn assert_equals(title: &str, expected: Value) {
    assert_eq!(Value::Object(parsed(title)), expected, "{title:?}");
}

/// `expect(parse(title)).to.not.have.property(field)`
#[track_caller]
pub fn assert_missing(title: &str, fields: &[&str]) {
    let actual = parsed(title);
    for field in fields {
        assert!(!actual.contains_key(*field), "field `{field}` of {title:?} should be missing, parsed as {actual:?}");
    }
}

#[track_caller]
fn field(title: &str, field: &str) -> Value {
    parsed(title).remove(field).unwrap_or(Value::Null)
}

/// `expect(parse(title)[field]).to.equal(expected)`
#[track_caller]
pub fn assert_field(title: &str, name: &str, expected: Value) {
    assert_eq!(field(title, name), expected, "field `{name}` of {title:?}");
}

/// `expect(parse(title)[field]).to.not.equal(expected)`
#[track_caller]
pub fn assert_field_ne(title: &str, name: &str, unexpected: Value) {
    assert_ne!(field(title, name), unexpected, "field `{name}` of {title:?}");
}

/// `expect(parse(title)[field]).to.include(text)`
#[track_caller]
pub fn assert_field_contains(title: &str, name: &str, text: &str) {
    let value = field(title, name);
    assert!(value.as_str().is_some_and(|value| value.contains(text)), "field `{name}` of {title:?} is {value}");
}

/// `expect(parse(title)[field]).to.have.length(length)`
#[track_caller]
pub fn assert_field_len(title: &str, name: &str, length: usize) {
    let value = field(title, name);
    assert_eq!(value.as_array().map(Vec::len), Some(length), "field `{name}` of {title:?} is {value}");
}

/// `expect(parse(title)[field][index]).to.equal(expected)`
#[track_caller]
pub fn assert_field_at(title: &str, name: &str, index: usize, expected: Value) {
    let value = field(title, name);
    assert_eq!(value.get(index), Some(&expected), "field `{name}` of {title:?} is {value}");
}

/// `this.timeout(ms)` of a mocha test.
#[track_caller]
pub fn assert_parses_within(title: &str, millis: u64) {
    let start = Instant::now();
    parse(title);
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(millis), "parsing {title:?} took {elapsed:?}");
}
