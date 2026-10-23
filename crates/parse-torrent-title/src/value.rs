use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A single parsed field value.
///
/// Values (de)serialize untagged, so a [`ParsedTitle`] serializes to the same JSON as the js library's result.
/// The default handlers only produce the typed variants; [`Value::Json`] holds anything else a custom handler sets.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Value {
    Bool(bool),
    Int(i64),
    Str(String),
    Ints(Vec<i64>),
    Strs(Vec<String>),
    Json(serde_json::Value),
}

impl Value {
    /// Whether the value counts as a match: `0`, `""`, `false` and `null` do not, as in the js library.
    #[must_use]
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            Value::Int(value) => *value != 0,
            Value::Str(value) => !value.is_empty(),
            Value::Ints(_) | Value::Strs(_) => true,
            Value::Json(value) => match value {
                serde_json::Value::Null => false,
                serde_json::Value::Bool(value) => *value,
                serde_json::Value::Number(number) => number.as_f64().is_some_and(|number| number != 0.0),
                serde_json::Value::String(value) => !value.is_empty(),
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => true,
            },
        }
    }

    /// The value as json.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    /// The value of a json value, typed when it fits one of the typed variants.
    #[must_use]
    pub fn from_json(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap_or_else(|_| unreachable!("every json value is a Value::Json"))
    }

    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(value) => Some(value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_ints(&self) -> Option<&[i64]> {
        match self {
            Value::Ints(values) => Some(values),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_strs(&self) -> Option<&[String]> {
        match self {
            Value::Strs(values) => Some(values),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(value.to_owned())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Str(value)
    }
}

impl From<Vec<i64>> for Value {
    fn from(values: Vec<i64>) -> Self {
        Value::Ints(values)
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        Value::from_json(value)
    }
}

impl From<Vec<String>> for Value {
    fn from(values: Vec<String>) -> Self {
        Value::Strs(values)
    }
}

macro_rules! accessors {
    ($($(#[$doc:meta])* $method:ident: $key:literal => $as:ident -> $ty:ty;)*) => {
        $(
            $(#[$doc])*
            pub fn $method(&self) -> Option<$ty> {
                self.get($key).and_then(Value::$as)
            }
        )*
    };
}

/// The fields parsed from a title, in the order the handlers found them.
///
/// Field names match the js library (`bitDepth`, `audioChannels`, ...), so custom handlers can add any field.
/// Typed accessors cover the fields of the default handlers.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParsedTitle(IndexMap<String, Value>);

impl ParsedTitle {
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.0.get(field)
    }

    pub fn contains(&self, field: &str) -> bool {
        self.0.contains_key(field)
    }

    /// Sets a field, keeping its position when it was already set.
    pub fn set(&mut self, field: impl Into<String>, value: impl Into<Value>) {
        self.0.insert(field.into(), value.into());
    }

    pub fn remove(&mut self, field: &str) -> Option<Value> {
        self.0.shift_remove(field)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(field, value)| (field.as_str(), value))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The cleaned up title, always set after parsing.
    pub fn title(&self) -> &str {
        self.get("title").and_then(Value::as_str).unwrap_or_default()
    }

    /// Either a single year or a year range such as `"2010-2012"`.
    pub fn year(&self) -> Option<&Value> {
        self.get("year")
    }

    accessors! {
        date: "date" => as_str -> &str;
        resolution: "resolution" => as_str -> &str;
        region: "region" => as_str -> &str;
        container: "container" => as_str -> &str;
        extension: "extension" => as_str -> &str;
        source: "source" => as_str -> &str;
        codec: "codec" => as_str -> &str;
        bit_depth: "bitDepth" => as_str -> &str;
        three_d: "threeD" => as_str -> &str;
        audio: "audio" => as_str -> &str;
        audio_channels: "audioChannels" => as_str -> &str;
        group: "group" => as_str -> &str;
        site: "site" => as_str -> &str;
        episode_code: "episodeCode" => as_str -> &str;
        hdr: "hdr" => as_strs -> &[String];
        languages: "languages" => as_strs -> &[String];
        volumes: "volumes" => as_ints -> &[i64];
        seasons: "seasons" => as_ints -> &[i64];
        episodes: "episodes" => as_ints -> &[i64];
        season: "season" => as_int -> i64;
        episode: "episode" => as_int -> i64;
        extended: "extended" => as_bool -> bool;
        unrated: "unrated" => as_bool -> bool;
        uncut: "uncut" => as_bool -> bool;
        proper: "proper" => as_bool -> bool;
        repack: "repack" => as_bool -> bool;
        convert: "convert" => as_bool -> bool;
        hardcoded: "hardcoded" => as_bool -> bool;
        retail: "retail" => as_bool -> bool;
        remastered: "remastered" => as_bool -> bool;
        complete: "complete" => as_bool -> bool;
        extras: "extras" => as_bool -> bool;
        half_episode: "halfEpisode" => as_bool -> bool;
        dubbed: "dubbed" => as_bool -> bool;
    }
}

impl FromIterator<(String, Value)> for ParsedTitle {
    fn from_iter<I: IntoIterator<Item = (String, Value)>>(fields: I) -> Self {
        Self(fields.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a ParsedTitle {
    type Item = (&'a String, &'a Value);
    type IntoIter = indexmap::map::Iter<'a, String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
