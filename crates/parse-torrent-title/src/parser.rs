use std::fmt;
use std::sync::{Arc, LazyLock, OnceLock};

use indexmap::IndexMap;

use crate::literals::Literals;
use crate::regex::{Regex, byte_index, group, utf16_index, utf16_len};
use crate::transformers::{Transformer, none};
use crate::{Error, ParsedTitle, Value};

// chinese/japanese/russian chars https://stackoverflow.com/a/43419070
pub(crate) const ACCENTED_LATIN_CHARS: &str = "\u{00c0}-\u{00d6}\u{00d8}-\u{00f6}\u{00f8}-\u{024f}";
pub(crate) const CJK_CHARS: &str =
    "\u{3040}-\u{30ff}\u{3400}-\u{4dbf}\u{4e00}-\u{9fff}\u{f900}-\u{faff}\u{ff66}-\u{ff9f}";
const CYRILLIC_CHARS: &str = "\u{0400}-\u{04ff}";
const MARKING_SECTION: &str = r"(?:\[[^\]]*]|【[^】]*】|★[^★]*★)";

fn non_english_chars() -> String {
    [CJK_CHARS, CYRILLIC_CHARS].concat()
}

static RUSSIAN_CAST: LazyLock<Regex> =
    LazyLock::new(|| Regex::known(r"(?:\([^)]*[Ѐ-ӿ][^)]*\)|(?<=\/.*)\(.*\))(?=\s*(?:\[[^\]]*)?$)", ""));
static ALT_TITLES: LazyLock<Regex> = LazyLock::new(|| {
    let n = non_english_chars();
    Regex::known(&format!("[^/|(]*[{n}][^/|]*[/|]|[/|][^/|(]*[{n}][^/|]*"), "g")
});
static ACCENTED_LATIN: LazyLock<Regex> = LazyLock::new(|| Regex::known(&format!("[{ACCENTED_LATIN_CHARS}]"), ""));
static NOT_ONLY_NON_ENGLISH: LazyLock<Regex> = LazyLock::new(|| {
    let n = non_english_chars();
    Regex::known(&format!("(?<=[a-zA-Z][^{n}]+)[{n}].*[{n}]|[{n}].*[{n}](?=[^{n}]+[a-zA-Z])"), "g")
});
static NOT_ALLOWED_SYMBOLS_AT_START_AND_END: LazyLock<Regex> = LazyLock::new(|| {
    let n = non_english_chars();
    Regex::known(&format!(r"^[^\w{ACCENTED_LATIN_CHARS}{n}#[【★]+|[ \-:,/\\[|{{(#$&^]+$"), "g")
});
static REMAINING_NOT_ALLOWED_SYMBOLS_AT_START_AND_END: LazyLock<Regex> = LazyLock::new(|| {
    let n = non_english_chars();
    Regex::known(&format!(r"^[^\w{ACCENTED_LATIN_CHARS}{n}#]+|[[\]({{}} ]+$"), "g")
});
static BEFORE_TITLE: LazyLock<Regex> = LazyLock::new(|| {
    let n = non_english_chars();
    Regex::known(
        &format!(r"^(?:\[[^[\]]*]\s*)+(?=[^\s[(.…（【])|^\[[^[\]]+]|^(?:(?=[^/[]*[{n}])[^/[]*/)+(?=\s*[a-zA-Z])"),
        "",
    )
});
static CYRILLIC_E_IN_EPISODE_CODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::known(r"(?<=\b[Ss]\d{1,3})[Ее](?=\d)", "g"));
static TRAILING_QUOTED_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::known(r"^(?!movie[\s-]*[「『])(.+?)[\s-]*[「『][^「『」』]*[」』]$", "i"));
static YEAR_SUFFIX_AFTER_NON_ENGLISH: LazyLock<Regex> =
    LazyLock::new(|| Regex::known(&format!(r"(?<=[{CJK_CHARS}])['’]\d{{2}}$"), ""));
static CHANNEL_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"^ch[45][ .]+(?=\S)", "i"));
static LEADING_MARKING_SECTIONS: LazyLock<Regex> =
    LazyLock::new(|| Regex::known(&format!("^(?:{MARKING_SECTION}[ .]?)+(.+)"), ""));
static TRAILING_MARKING_SECTIONS: LazyLock<Regex> =
    LazyLock::new(|| Regex::known(&format!("(.+?)(?:[ .]?{MARKING_SECTION})+$"), ""));
static UNCLOSED_BRACKET_AT_END: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"\s*[([{][^)\]}]*$", ""));
static UNOPENED_BRACKET: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"^([^([{]*)[)\]}]", ""));
static MOVIE_FLAG: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"[[(]movie[)\]]", "i"));
static DOT_SEPARATED: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"^[^\s.]*\.\S*$", ""));
static SUB_DUB_SUFFIX: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"\s+-+\s*(?:sub|dub)s?$", "i"));
static WHOLE_TEXT: LazyLock<Regex> = LazyLock::new(|| Regex::known(".*", ""));
static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"\s+", "g"));
static ALT_TITLE_SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"\s+[/|]\s+", ""));
static UNDERSCORES: LazyLock<Regex> = LazyLock::new(|| Regex::known("_+", "g"));
static UNCLOSED_LEADING_BRACKET: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"^\[([^[\]]+)\[(?=\s)", ""));
static BRACKET_SECTIONS: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"\[[^[\]]*]|[（【][^（）【】]*[）】]", "g"));
static LATIN_BRACKET_SECTIONS: LazyLock<Regex> = LazyLock::new(|| Regex::known(r"\[[^[\]]*[a-zA-Z][^[\]]*]", "g"));

/// Where a handler matched, keyed by its field name in [`Matched`].
///
/// Positions count utf-16 code units like the js library does, since handlers combine positions found in
/// different versions of the title; see [`utf16_index`](crate::utf16_index) and [`byte_index`](crate::byte_index).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MatchInfo {
    pub raw_match: Option<String>,
    /// Position, in utf-16 code units, in the title the handler was given.
    pub match_index: usize,
}

impl MatchInfo {
    #[must_use]
    pub fn new(match_index: usize, raw_match: Option<String>) -> Self {
        Self { raw_match, match_index }
    }
}

/// The first match of each field found so far.
pub type Matched = IndexMap<String, MatchInfo>;

/// What a handler gets to work with.
#[non_exhaustive]
pub struct Context<'a> {
    /// The name being parsed, without the matches removed by previous handlers.
    pub title: &'a str,
    pub result: &'a mut ParsedTitle,
    pub matched: &'a mut Matched,
}

/// A match reported by a handler.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct HandlerMatch {
    pub raw_match: Option<String>,
    /// Position, in utf-16 code units, in the title the handler was given; the title ends before the earliest match.
    pub match_index: usize,
    /// Removes `raw_match` from the title for the next handlers.
    pub remove: bool,
    /// Keeps the match from ending the title.
    pub skip_from_title: bool,
}

impl HandlerMatch {
    /// A match at `match_index`, in utf-16 code units, i.e. `{ matchIndex }` in js.
    #[must_use]
    pub fn at(match_index: usize) -> Self {
        Self { match_index, ..Self::default() }
    }

    /// The matched text, which [`HandlerMatch::remove`] removes from the title.
    #[must_use]
    pub fn raw_match(mut self, raw_match: impl Into<String>) -> Self {
        self.raw_match = Some(raw_match.into());
        self
    }

    #[must_use]
    pub fn remove(mut self, remove: bool) -> Self {
        self.remove = remove;
        self
    }

    #[must_use]
    pub fn skip_from_title(mut self, skip: bool) -> Self {
        self.skip_from_title = skip;
        self
    }
}

pub type HandlerFn = Arc<dyn Fn(&mut Context<'_>) -> Option<HandlerMatch> + Send + Sync>;

/// Options of a regex handler, built from [`Options::new`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Options {
    /// Whether to skip the handler if its field was already found.
    pub skip_if_already_found: bool,
    /// Whether to exclude the match from the title.
    pub skip_from_title: bool,
    /// Whether to skip the handler if every other field was matched after it.
    pub skip_if_first: bool,
    /// Whether to skip the handler if no other field was matched before it.
    pub skip_if_nothing_before: bool,
    /// Skip the handler if it matches before any of these fields.
    pub skip_if_before: Vec<String>,
    /// Skip the handler if it matches after any of these fields.
    pub skip_if_after: Vec<String>,
    /// Whether to remove the match from the title for the next handlers.
    pub remove: bool,
    /// A value to set instead of the transformed match.
    pub value: Option<Value>,
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    /// The default options: only `skip_if_already_found` is set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            skip_if_already_found: true,
            skip_from_title: false,
            skip_if_first: false,
            skip_if_nothing_before: false,
            skip_if_before: Vec::new(),
            skip_if_after: Vec::new(),
            remove: false,
            value: None,
        }
    }

    #[must_use]
    pub fn skip_if_already_found(mut self, skip: bool) -> Self {
        self.skip_if_already_found = skip;
        self
    }

    #[must_use]
    pub fn skip_from_title(mut self, skip: bool) -> Self {
        self.skip_from_title = skip;
        self
    }

    #[must_use]
    pub fn skip_if_first(mut self, skip: bool) -> Self {
        self.skip_if_first = skip;
        self
    }

    #[must_use]
    pub fn skip_if_nothing_before(mut self, skip: bool) -> Self {
        self.skip_if_nothing_before = skip;
        self
    }

    #[must_use]
    pub fn skip_if_before(mut self, fields: &[&str]) -> Self {
        self.skip_if_before = fields.iter().map(|&field| field.to_owned()).collect();
        self
    }

    #[must_use]
    pub fn skip_if_after(mut self, fields: &[&str]) -> Self {
        self.skip_if_after = fields.iter().map(|&field| field.to_owned()).collect();
        self
    }

    #[must_use]
    pub fn remove(mut self, remove: bool) -> Self {
        self.remove = remove;
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<Value>) -> Self {
        self.value = Some(value.into());
        self
    }
}

#[derive(Clone)]
enum Kind {
    Regex { regex: Regex, transformer: Transformer, options: Options },
    Function(HandlerFn),
}

/// A named step of the parser: a [`RegexHandler`], or a function.
#[derive(Clone)]
pub struct Handler {
    name: String,
    kind: Kind,
}

/// A regex handler being built, see [`Handler::regex`].
#[derive(Clone)]
#[must_use]
pub struct RegexHandler {
    name: String,
    regex: Regex,
    transformer: Transformer,
    options: Options,
}

impl RegexHandler {
    /// Sets the transformer of the matched text, [`none`] by default.
    pub fn transform(mut self, transformer: Transformer) -> Self {
        self.transformer = transformer;
        self
    }

    pub fn options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }
}

impl From<RegexHandler> for Handler {
    fn from(handler: RegexHandler) -> Self {
        let RegexHandler { name, regex, transformer, options } = handler;
        Self { name, kind: Kind::Regex { regex, transformer, options } }
    }
}

impl Handler {
    /// A regex handler setting the field `name` to the matched text, or to its first capture group.
    pub fn regex(name: impl Into<String>, regex: Regex) -> RegexHandler {
        RegexHandler { name: name.into(), regex, transformer: none(), options: Options::new() }
    }

    #[must_use]
    pub fn function(
        name: impl Into<String>,
        handler: impl Fn(&mut Context<'_>) -> Option<HandlerMatch> + Send + Sync + 'static,
    ) -> Self {
        Self { name: name.into(), kind: Kind::Function(Arc::new(handler)) }
    }

    /// A function handler without a name, listed as `"unknown"`.
    #[must_use]
    pub fn anonymous(handler: impl Fn(&mut Context<'_>) -> Option<HandlerMatch> + Send + Sync + 'static) -> Self {
        Self::function("unknown", handler)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    fn pattern(&self) -> Option<&Regex> {
        match &self.kind {
            Kind::Regex { regex, .. } => Some(regex),
            Kind::Function(_) => None,
        }
    }

    fn run(&self, context: &mut Context<'_>) -> Option<HandlerMatch> {
        match &self.kind {
            Kind::Regex { regex, transformer, options } => run_regex(&self.name, regex, transformer, options, context),
            Kind::Function(handler) => handler(context),
        }
    }
}

impl fmt::Debug for Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Kind::Regex { regex, options, .. } => f
                .debug_struct("Handler")
                .field("name", &self.name)
                .field("regex", regex)
                .field("options", options)
                .finish(),
            Kind::Function(_) => f.debug_struct("Handler").field("name", &self.name).finish_non_exhaustive(),
        }
    }
}

fn run_regex(
    name: &str,
    regex: &Regex,
    transformer: &Transformer,
    options: &Options,
    context: &mut Context<'_>,
) -> Option<HandlerMatch> {
    let current = context.result.get(name);
    if options.skip_if_already_found && current.is_some_and(Value::is_truthy) {
        return None;
    }

    let title = context.title;
    let m = regex.find(title)?;
    let raw_match = &title[m.range()];
    if raw_match.is_empty() {
        return None;
    }
    let clean_match = group(&m, title, 1).filter(|clean| !clean.is_empty()).unwrap_or(raw_match);
    let transformed = transformer(clean_match, current).filter(Value::is_truthy)?;

    let index = utf16_index(title, m.start());
    let matched = &*context.matched;
    let is_before_title = BEFORE_TITLE.find(title).is_some_and(|before| index < utf16_len(&title[before.range()]));
    let others = || matched.iter().filter(|(field, _)| field.as_str() != name);
    let is_skip_if_first =
        options.skip_if_first && others().next().is_some() && others().all(|(_, other)| index < other.match_index);
    let is_skip_if_nothing_before =
        options.skip_if_nothing_before && !others().any(|(_, other)| other.match_index <= index);
    let is_skip_if_before =
        options.skip_if_before.iter().any(|field| matched.get(field).is_some_and(|other| index < other.match_index));
    let is_skip_if_after =
        options.skip_if_after.iter().any(|field| matched.get(field).is_some_and(|other| index >= other.match_index));
    if is_skip_if_first || is_skip_if_nothing_before || is_skip_if_before || is_skip_if_after {
        return None;
    }

    context.matched.entry(name.to_owned()).or_insert_with(|| MatchInfo::new(index, Some(raw_match.to_owned())));
    let value = options.value.clone().filter(Value::is_truthy).unwrap_or(transformed);
    context.result.set(name, value);
    Some(HandlerMatch {
        raw_match: Some(raw_match.to_owned()),
        match_index: index,
        remove: options.remove,
        skip_from_title: is_before_title || options.skip_from_title,
    })
}

/// A handler as listed by [`Parser::handlers`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct HandlerInfo {
    pub name: String,
    /// Index of the handler within its field.
    pub index: usize,
    /// Index of the handler within all handlers of the parser.
    pub position: usize,
    pub kind: HandlerKind,
}

impl HandlerInfo {
    /// The regex of a regex handler.
    #[must_use]
    pub fn regex(&self) -> Option<&Regex> {
        match &self.kind {
            HandlerKind::Regex { regex, .. } => Some(regex),
            HandlerKind::Function => None,
        }
    }

    /// The options of a regex handler.
    #[must_use]
    pub fn options(&self) -> Option<&Options> {
        match &self.kind {
            HandlerKind::Regex { options, .. } => Some(options),
            HandlerKind::Function => None,
        }
    }
}

/// Whether a listed handler is a regex or a function.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum HandlerKind {
    Regex { regex: Regex, options: Options },
    Function,
}

/// A handler to insert relative to: a field's handlers, or one of them by index (negative counts from the end).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Target {
    pub name: String,
    pub index: Option<isize>,
}

impl Target {
    #[must_use]
    pub fn new(name: impl Into<String>, index: isize) -> Self {
        Self { name: name.into(), index: Some(index) }
    }
}

impl From<&str> for Target {
    fn from(name: &str) -> Self {
        Self { name: name.to_owned(), index: None }
    }
}

/// Where [`Parser::insert_handler`] puts a handler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    /// Before the target, or before the first handler of the target field.
    Before(Target),
    /// After the target, or after the last handler of the target field.
    After(Target),
}

impl Position {
    #[must_use]
    pub fn before(target: impl Into<Target>) -> Self {
        Self::Before(target.into())
    }

    #[must_use]
    pub fn after(target: impl Into<Target>) -> Self {
        Self::After(target.into())
    }
}

/// Runs its handlers in order over a title, each one adding a field to the result.
#[derive(Clone, Default)]
pub struct Parser {
    handlers: Vec<Handler>,
    /// The literals of the handlers, built on the first parse and shared by clones until the handlers change.
    literals: Arc<OnceLock<Literals>>,
}

impl fmt::Debug for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser").field("handlers", &self.handlers).finish_non_exhaustive()
    }
}

impl Parser {
    /// A parser without handlers; see [`Parser::with_defaults`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A parser with all the default handlers.
    #[must_use]
    pub fn with_defaults() -> Self {
        let mut parser = Self::new();
        crate::handlers::add_defaults(&mut parser, &[]);
        parser
    }

    /// Appends a handler.
    pub fn add_handler(&mut self, handler: impl Into<Handler>) {
        self.handlers.push(handler.into());
        self.literals = Arc::default();
    }

    /// Inserts a handler relative to another handler.
    ///
    /// # Errors
    ///
    /// When no handler matches the target of `position`.
    pub fn insert_handler(&mut self, position: Position, handler: impl Into<Handler>) -> Result<(), Error> {
        let (target, before) = match &position {
            Position::Before(target) => (target, true),
            Position::After(target) => (target, false),
        };
        let positions: Vec<usize> =
            self.handlers.iter().enumerate().filter(|(_, h)| h.name == target.name).map(|(i, _)| i).collect();
        let index = target.index.unwrap_or(if before { 0 } else { -1 });
        let resolved = if index < 0 { positions.len().checked_sub(index.unsigned_abs()) } else { Some(index as usize) };
        let found = resolved.and_then(|index| positions.get(index)).copied();
        let Some(found) = found else {
            return Err(Error::UnknownTarget { name: target.name.clone(), index: target.index });
        };
        self.handlers.insert(if before { found } else { found + 1 }, handler.into());
        self.literals = Arc::default();
        Ok(())
    }

    /// Removes the handlers of the given fields, returning how many were removed.
    pub fn remove_handlers(&mut self, names: &[&str]) -> usize {
        self.remove_handlers_where(|handler| names.contains(&handler.name.as_str()))
    }

    /// Removes the handlers matching `predicate`, returning how many were removed.
    pub fn remove_handlers_where(&mut self, mut predicate: impl FnMut(&HandlerInfo) -> bool) -> usize {
        let infos = self.handlers();
        let count = self.handlers.len();
        let mut infos = infos.iter();
        self.handlers.retain(|_| !infos.next().is_some_and(&mut predicate));
        self.literals = Arc::default();
        count - self.handlers.len()
    }

    /// Lists the handlers in the order they run.
    #[must_use]
    pub fn handlers(&self) -> Vec<HandlerInfo> {
        let mut counts: IndexMap<&str, usize> = IndexMap::new();
        self.handlers
            .iter()
            .enumerate()
            .map(|(position, handler)| {
                let count = counts.entry(&handler.name).or_default();
                *count += 1;
                let kind = match &handler.kind {
                    Kind::Regex { regex, options, .. } => {
                        HandlerKind::Regex { regex: regex.clone(), options: options.clone() }
                    }
                    Kind::Function(_) => HandlerKind::Function,
                };
                HandlerInfo { name: handler.name.clone(), index: *count - 1, position, kind }
            })
            .collect()
    }

    #[must_use]
    pub fn parse(&self, title: &str) -> ParsedTitle {
        let title = UNDERSCORES.replace(title, " ");
        let title = UNCLOSED_LEADING_BRACKET.replace(&title, "[$1]");
        let mut title = CYRILLIC_E_IN_EPISODE_CODE
            .replace_with(&title, |letter| if letter == "\u{0415}" { "E".into() } else { "e".into() })
            .into_owned();
        let mut result = ParsedTitle::default();
        let mut matched = Matched::new();
        // js slice semantics: a negative end counts from the end of the title
        let mut end_of_title = utf16_len(&title) as isize;
        let mut tag_matches: Vec<String> = Vec::new();
        let literals = self
            .literals
            .get_or_init(|| Literals::new(&self.handlers.iter().map(Handler::pattern).collect::<Vec<_>>()));
        let mut candidates = literals.candidates(&title);

        for (position, handler) in self.handlers.iter().enumerate() {
            if !candidates[position] {
                // skipping is only an optimization, so debug builds (the test suite) check it never skips a match
                debug_assert!(
                    handler.pattern().is_none_or(|regex| regex.find(&title).is_none()),
                    "the literals of {handler:?} skip a match in {title:?}, see crate::literals"
                );
                continue;
            }
            let mut context = Context { title: &title, result: &mut result, matched: &mut matched };
            let Some(found) = handler.run(&mut context) else {
                continue;
            };
            let raw_match = found.raw_match.as_deref().unwrap_or_default();
            let index = found.match_index as isize;
            if found.skip_from_title {
                tag_matches.push(raw_match.trim().to_owned());
            }
            if found.remove {
                let start = byte_index(&title, found.match_index);
                let end = byte_index(&title, found.match_index + utf16_len(raw_match));
                title.replace_range(start..end.max(start), "");
                literals.add_around(&title, start, &mut candidates);
            }
            if !found.skip_from_title && found.match_index != 0 && index < end_of_title {
                end_of_title = index;
            }
            if found.remove && found.skip_from_title && index < end_of_title {
                // adjust title index in case part of it should be removed and skipped
                end_of_title -= utf16_len(raw_match) as isize;
            }
        }

        let end = if end_of_title < 0 {
            utf16_len(&title).saturating_sub(end_of_title.unsigned_abs())
        } else {
            end_of_title as usize
        };
        let title_part = &title[..byte_index(&title, end)];
        let cleaned_title = clean_title(title_part);
        let is_tag_title = tag_matches.iter().any(|tag| tag.to_lowercase() == cleaned_title.to_lowercase());
        let title = if !cleaned_title.is_empty() && !is_tag_title {
            cleaned_title
        } else {
            Some(clean_title(&longest_bracket_text(title_part, &tag_matches)))
                .filter(|t| !t.is_empty())
                .unwrap_or(cleaned_title)
        };
        result.set("title", title);
        result
    }
}

/// Prefers the alternative titles without accented chars.
fn remove_accented_alt_titles(title: &str) -> String {
    let titles = ALT_TITLE_SEPARATOR.split(title);
    let plain_titles: Vec<&str> = titles.iter().copied().filter(|alias| !ACCENTED_LATIN.is_match(alias)).collect();
    if !plain_titles.is_empty() && plain_titles.len() < titles.len() {
        plain_titles.join(" / ")
    } else {
        title.to_owned()
    }
}

fn clean_title(raw_title: &str) -> String {
    let mut title = raw_title.to_owned();
    if !title.contains(' ') && title.contains('.') {
        title = title.replace('.', " ");
    }

    let title = title.replace('_', " ");
    let title = CHANNEL_PREFIX.replace(&title, "");
    let title = MOVIE_FLAG.replace(&title, ""); // clear movie indication flag
    let title = NOT_ALLOWED_SYMBOLS_AT_START_AND_END.replace(&title, "");
    let title = RUSSIAN_CAST.replace(&title, ""); // clear russian cast information
    let title = LEADING_MARKING_SECTIONS.replace(&title, "$1"); // remove release group markings sections from the start
    let title = TRAILING_MARKING_SECTIONS.replace(&title, "$1"); // remove unneeded markings section at the end if present
    let title = TRAILING_QUOTED_SECTION.replace(&title, "$1");
    let title = YEAR_SUFFIX_AFTER_NON_ENGLISH.replace(&title, "");
    let title = ALT_TITLES.replace(&title, ""); // remove alt language titles
    let title = NOT_ONLY_NON_ENGLISH.replace(&title, ""); // remove non english chars if they are not the only ones left
    let title = REMAINING_NOT_ALLOWED_SYMBOLS_AT_START_AND_END.replace(&title, "");
    let title = DOT_SEPARATED.replace_with(&title, |title| title.replace('.', " ")); // replace dots with spaces if title is dot separated
    let title = UNCLOSED_BRACKET_AT_END.replace(&title, ""); // remove unclosed bracket section at the end
    let title = UNOPENED_BRACKET.replace(&title, "$1"); // remove closing bracket without an opening one
    let title = SUB_DUB_SUFFIX.replace(&title, "");
    let title = WHOLE_TEXT.replace_with(&title, remove_accented_alt_titles); // prefer alt titles without accented chars
    let title = WHITESPACE.replace(&title, " ");
    title.trim().to_owned()
}

/// A name made only of bracketed sections ("[Show Name][2022][S3E06]") keeps its longest one as the title.
fn longest_bracket_text(text: &str, tags: &[String]) -> String {
    let mut remainder = BRACKET_SECTIONS.replace(text, "").into_owned();
    for tag in tags {
        remainder = remainder.replacen(tag.as_str(), "", 1);
    }
    if remainder.chars().any(|c| !(c.is_whitespace() || c == '+' || c == '-')) {
        return String::new();
    }
    let mut sections: Vec<&str> =
        LATIN_BRACKET_SECTIONS.find_iter(text).map(|m| &text[m.start() + 1..m.end() - 1]).collect();
    // js compares string lengths in utf-16 code units
    sections.sort_by_key(|section| std::cmp::Reverse(section.encode_utf16().count()));
    sections.first().map(|&section| section.to_owned()).unwrap_or_default()
}
