//! Parse torrent title to extract useful information.
//!
//! ```
//! let parsed = parse_torrent_title::parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU");
//!
//! assert_eq!(parsed.title(), "Game of Thrones");
//! assert_eq!(parsed.season(), Some(1));
//! assert_eq!(parsed.episode(), Some(1));
//! assert_eq!(parsed.resolution(), Some("720p"));
//! assert_eq!(parsed.group(), Some("CTU"));
//! ```
//!
//! The parser runs a list of handlers in order, each adding a field to the result.
//! [`create_parser`] returns a copy of the default parser to customize:
//!
//! ```
//! use parse_torrent_title::{Handler, Position, Regex, create_parser, transformers::lowercase};
//!
//! let mut parser = create_parser();
//! let network = Handler::regex("network", Regex::new(r"\b(NF|AMZN)\b", "")?).transform(lowercase());
//! parser.insert_handler(Position::before("site"), network)?;
//!
//! let parsed = parser.parse("Stranger.Things.S04E01.1080p.NF.WEB-DL");
//! assert_eq!(parsed.get("network").and_then(|value| value.as_str()), Some("nf"));
//! # Ok::<(), parse_torrent_title::Error>(())
//! ```

mod handlers;
mod literals;
mod parser;
mod regex;
pub mod transformers;
mod value;

use std::fmt;
use std::sync::LazyLock;

pub use handlers::add_defaults;
pub use parser::{
    Context, Handler, HandlerFn, HandlerInfo, HandlerKind, HandlerMatch, MatchInfo, Matched, Options, Parser, Position,
    RegexHandler, Target,
};
pub use regex::{Match, Regex, byte_index, utf16_index, utf16_len};
pub use value::{ParsedTitle, Value};

/// Runs the examples of the readme as doc tests.
#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
struct ReadmeExamples;

static DEFAULT_PARSER: LazyLock<Parser> = LazyLock::new(Parser::with_defaults);

/// Parses `title` with the default handlers.
#[must_use]
pub fn parse(title: &str) -> ParsedTitle {
    DEFAULT_PARSER.parse(title)
}

/// A copy of the default parser, to add or remove handlers without affecting [`parse`].
#[must_use]
pub fn create_parser() -> Parser {
    DEFAULT_PARSER.clone()
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The regex could not be compiled.
    Regex(fastjs_regex::Error),
    /// The format of [`transformers::date`] is not a valid `chrono` format.
    InvalidDateFormat { format: String },
    /// No handler matches the target of [`Parser::insert_handler`].
    UnknownTarget { name: String, index: Option<isize> },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Regex(error) => error.fmt(f),
            Error::InvalidDateFormat { format } => write!(f, "invalid date format {format:?}"),
            Error::UnknownTarget { name, index: None } => write!(f, "no handler found for {name:?}"),
            Error::UnknownTarget { name, index: Some(index) } => {
                write!(f, "no handler found for {name:?} at index {index}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<fastjs_regex::Error> for Error {
    fn from(error: fastjs_regex::Error) -> Self {
        Error::Regex(error)
    }
}
