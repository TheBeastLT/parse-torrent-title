//! ECMAScript regex semantics with a linear-time fast path.
//!
//! ```
//! use fastjs_regex::Regex;
//!
//! let season = Regex::new(r"(?<=\bseason )(\d+)(?=\D|$)", "i")?;
//! let m = season.find("Show Season 2 [1080p]").unwrap();
//! assert_eq!(&"Show Season 2 [1080p]"[m.range()], "2");
//!
//! let words = Regex::new(r"\s*[,;]\s*", "g")?;
//! assert_eq!(words.replace("a , b;c", "|"), "a|b|c");
//! # Ok::<(), fastjs_regex::Error>(())
//! ```
//!
//! See [`Regex`] for how matching works and how it differs from V8.

mod regex;
mod translate;

use std::fmt;

pub use regex::{Match, Regex, byte_index, utf16_index, utf16_len};
pub use translate::Translation;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeExamples;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The regex could not be compiled.
    InvalidRegex { pattern: String, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidRegex { pattern, message } => write!(f, "invalid regex /{pattern}/: {message}"),
        }
    }
}

impl std::error::Error for Error {}
