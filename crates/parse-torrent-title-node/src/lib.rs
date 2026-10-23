//! Node.js bindings of `parse-torrent-title`, wrapped by `index.js` to offer the api of the js library.
//!
//! Values cross the boundary as json, which keeps the binding small: the default handlers and the built in
//! transformers run entirely in rust, and `index.js` wraps handlers and transformers written in js into functions
//! taking and returning json, called back with the arguments the js library gives them.

use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::thread::{self, ThreadId};

use napi::bindgen_prelude::FunctionRef;
use napi::{Env, Error, Result, Status, sys};
use napi_derive::napi;
use parse_torrent_title::transformers::{self, Transformer};
use parse_torrent_title::{
    Context, Handler, HandlerMatch, MatchInfo, Matched, Options, ParsedTitle, Parser, Position, Regex, Target, Value,
};
use serde_json::{Map, Value as Json, json};

/// A js function taking and returning json, see `index.js`.
///
/// Spelled out in the exported signatures, since the generated typescript definitions can not resolve aliases.
type JsonFn = FunctionRef<String, Option<String>>;

/// A js function with the env it was given in, to call it back on the js thread while parsing.
struct Callback {
    function: ManuallyDrop<JsonFn>,
    env: sys::napi_env,
    /// The js thread of `env`, the only one the function may be called and released on.
    thread: ThreadId,
}

// SAFETY: the function and its env are only used on `thread`, the js thread they belong to: `call` and `drop`
// check it, so a parser moved to another thread fails loudly instead of touching js from there.
unsafe impl Send for Callback {}
// SAFETY: as above.
unsafe impl Sync for Callback {}

impl Drop for Callback {
    fn drop(&mut self) {
        if thread::current().id() == self.thread {
            // SAFETY: the function is not used after this, and is released on its own js thread.
            unsafe { ManuallyDrop::drop(&mut self.function) };
        }
        // released anywhere else, the reference to the js function leaks rather than touching js off its thread
    }
}

thread_local! {
    /// The first error thrown by a js callback during a parse, rethrown once the parse is over.
    static CALLBACK_ERROR: RefCell<Option<Error>> = const { RefCell::new(None) };
}

impl Callback {
    fn new(env: &Env, function: JsonFn) -> Self {
        Self { function: ManuallyDrop::new(function), env: env.raw(), thread: thread::current().id() }
    }

    /// Calls the function, unless an earlier callback of the parse threw.
    fn call(&self, argument: String) -> Option<String> {
        assert_eq!(thread::current().id(), self.thread, "a js handler can only run on its own js thread");
        if CALLBACK_ERROR.with_borrow(Option::is_some) {
            return None;
        }
        let env = Env::from_raw(self.env);
        let result = self.function.borrow_back(&env).and_then(|function| function.call(argument));
        result.unwrap_or_else(|error| {
            CALLBACK_ERROR.set(Some(error));
            None
        })
    }
}

fn invalid(message: impl std::fmt::Display) -> Error {
    Error::new(Status::InvalidArg, message.to_string())
}

fn from_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(invalid)
}

fn transformer_from_spec(spec: &Json) -> Result<Transformer> {
    let kind = spec["kind"].as_str().ok_or_else(|| invalid("transformer should be a function"))?;
    let chain = || transformer_from_spec(&spec["chain"]);
    Ok(match kind {
        "none" => transformers::none(),
        "value" => transformers::value(Value::from_json(spec["value"].clone())),
        "integer" => transformers::integer(),
        "cjkInteger" => transformers::cjk_integer(),
        "boolean" => transformers::boolean(),
        "lowercase" => transformers::lowercase(),
        "uppercase" => transformers::uppercase(),
        "date" => transformers::date(spec["format"].as_str().unwrap_or_default()).map_err(invalid)?,
        "range" => transformers::range(),
        "list" => transformers::list(),
        "rangeUpTo" => transformers::range_up_to(),
        "yearRange" => transformers::year_range(),
        "array" => transformers::array(if spec["chain"].is_null() { transformers::none() } else { chain()? }),
        "uniqConcat" => transformers::uniq_concat(chain()?),
        other => return Err(invalid(format!("unknown transformer {other}"))),
    })
}

/// A transformer written in js, called with `[input, current value]` as json and returning the value as json.
fn custom_transformer(callback: Callback) -> Transformer {
    Arc::new(move |input: &str, current: Option<&Value>| {
        let returned = callback.call(json!([input, current]).to_string())?;
        serde_json::from_str(&returned).ok()
    })
}

fn options_from_json(options: &Json) -> Options {
    let flag = |key: &str, default: bool| options.get(key).and_then(Json::as_bool).unwrap_or(default);
    let strings = |key: &str| -> Vec<&str> {
        options.get(key).and_then(Json::as_array).into_iter().flatten().filter_map(Json::as_str).collect()
    };
    let parsed = Options::new()
        .skip_if_already_found(flag("skipIfAlreadyFound", true))
        .skip_from_title(flag("skipFromTitle", false))
        .skip_if_first(flag("skipIfFirst", false))
        .skip_if_nothing_before(flag("skipIfNothingBefore", false))
        .skip_if_before(&strings("skipIfBefore"))
        .skip_if_after(&strings("skipIfAfter"))
        .remove(flag("remove", false));
    match options.get("value").filter(|value| !value.is_null()) {
        Some(value) => parsed.value(Value::from_json(value.clone())),
        None => parsed,
    }
}

fn options_to_json(options: &Options) -> Json {
    let mut object = json!({
        "skipIfAlreadyFound": options.skip_if_already_found,
        "skipFromTitle": options.skip_from_title,
        "skipIfFirst": options.skip_if_first,
        "skipIfNothingBefore": options.skip_if_nothing_before,
        "skipIfBefore": options.skip_if_before,
        "skipIfAfter": options.skip_if_after,
        "remove": options.remove,
    });
    if let Some(value) = &options.value {
        object["value"] = json!(value);
    }
    object
}

fn target_from_json(target: &Json) -> Target {
    let name = target.as_str().or_else(|| target["name"].as_str()).unwrap_or_default();
    match target["index"].as_i64().and_then(|index| isize::try_from(index).ok()) {
        Some(index) => Target::new(name, index),
        None => Target::from(name),
    }
}

/// A position from js, where a negative or fractional one counts as 0.
fn index_from_json(value: &Json) -> usize {
    value.as_u64().and_then(|index| usize::try_from(index).ok()).unwrap_or_default()
}

/// A handler written in js, called with `{ title, result, matched }` as json and returning
/// `{ result, matched, returned }` as json, `returned` being what the handler returned.
fn function_handler(name: String, callback: Callback) -> Handler {
    Handler::function(name, move |context: &mut Context<'_>| {
        let matched: Map<String, Json> = context
            .matched
            .iter()
            .map(|(field, info)| {
                let mut entry = json!({ "matchIndex": info.match_index });
                if let Some(raw_match) = &info.raw_match {
                    entry["rawMatch"] = json!(raw_match);
                }
                (field.clone(), entry)
            })
            .collect();
        let argument = json!({ "title": context.title, "result": &*context.result, "matched": matched });
        let returned: Json = serde_json::from_str(&callback.call(argument.to_string())?).ok()?;

        // fields a handler deleted are gone, and fields of types the rust result can not hold are left out
        let fields = returned["result"].as_object().into_iter().flatten();
        *context.result = fields
            .filter_map(|(field, value)| Some((field.clone(), serde_json::from_value(value.clone()).ok()?)))
            .collect::<ParsedTitle>();
        *context.matched = returned["matched"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(field, entry)| {
                let raw_match = entry["rawMatch"].as_str().map(str::to_owned);
                (field.clone(), MatchInfo::new(index_from_json(&entry["matchIndex"]), raw_match))
            })
            .collect::<Matched>();

        let returned = &returned["returned"];
        returned.is_object().then(|| {
            let found = HandlerMatch::at(index_from_json(&returned["matchIndex"]))
                .remove(returned["remove"].as_bool().unwrap_or_default())
                .skip_from_title(returned["skipFromTitle"].as_bool().unwrap_or_default());
            match returned["rawMatch"].as_str() {
                Some(raw_match) => found.raw_match(raw_match),
                None => found,
            }
        })
    })
}

/// The rust parser, wrapped by the `Parser` class of `index.js`.
#[napi]
pub struct NativeParser {
    inner: Parser,
}

#[napi]
impl NativeParser {
    #[napi(constructor, catch_unwind)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { inner: Parser::new() }
    }

    /// A parser with the default handlers, sharing their compiled patterns with every other one.
    #[napi(factory, catch_unwind)]
    pub fn with_defaults() -> Self {
        Self { inner: parse_torrent_title::create_parser() }
    }

    #[napi(catch_unwind)]
    pub fn add_defaults(&mut self, exclude: Vec<String>) {
        let exclude: Vec<&str> = exclude.iter().map(String::as_str).collect();
        parse_torrent_title::add_defaults(&mut self.inner, &exclude);
    }

    /// Adds a regex handler whose transformer is either a json spec of a built in one, or a js function.
    #[napi(catch_unwind)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_regex_handler(
        &mut self,
        env: Env,
        name: String,
        source: String,
        flags: String,
        transformer_spec: Option<String>,
        #[napi(ts_arg_type = "((json: string) => string | null) | undefined")] transformer_function: Option<JsonFn>,
        options: String,
        position: String,
    ) -> Result<()> {
        let regex = Regex::new(&source, &flags).map_err(invalid)?;
        let transformer = match (transformer_function, transformer_spec) {
            (Some(function), _) => custom_transformer(Callback::new(&env, function)),
            (None, Some(spec)) => transformer_from_spec(&from_json(&spec)?)?,
            (None, None) => transformers::none(),
        };
        let options = options_from_json(&from_json(&options)?);
        self.insert(Handler::regex(name, regex).transform(transformer).options(options).into(), &position)
    }

    #[napi(catch_unwind)]
    pub fn add_function_handler(
        &mut self,
        env: Env,
        name: String,
        handler: FunctionRef<String, Option<String>>,
        position: String,
    ) -> Result<()> {
        self.insert(function_handler(name, Callback::new(&env, handler)), &position)
    }

    /// Removes the handlers at the given positions, returning how many were removed.
    #[napi(catch_unwind)]
    pub fn remove_at(&mut self, positions: Vec<u32>) -> u32 {
        let removed = self.inner.remove_handlers_where(|handler| {
            u32::try_from(handler.position).is_ok_and(|position| positions.contains(&position))
        });
        u32::try_from(removed).unwrap_or(u32::MAX)
    }

    /// `[{ name, index, position, source?, flags?, options? }]` as json.
    #[napi(catch_unwind)]
    pub fn get_handlers(&self) -> String {
        let handlers: Vec<Json> = self
            .inner
            .handlers()
            .iter()
            .map(|handler| {
                let mut object = json!({ "name": handler.name, "index": handler.index, "position": handler.position });
                if let (Some(regex), Some(options)) = (handler.regex(), handler.options()) {
                    object["source"] = json!(regex.source());
                    object["flags"] = json!(regex.flags());
                    object["options"] = options_to_json(options);
                }
                object
            })
            .collect();
        Json::Array(handlers).to_string()
    }

    /// The parsed title as json.
    #[napi(catch_unwind)]
    pub fn parse(&self, title: String) -> Result<String> {
        let parsed = self.inner.parse(&title);
        if let Some(error) = CALLBACK_ERROR.take() {
            return Err(error);
        }
        serde_json::to_string(&parsed).map_err(|error| Error::from_reason(error.to_string()))
    }

    #[napi(js_name = "clone", catch_unwind)]
    pub fn clone_parser(&self) -> Self {
        Self { inner: self.inner.clone() }
    }

    fn insert(&mut self, handler: Handler, position: &str) -> Result<()> {
        let position: Json = from_json(position)?;
        let (before, after) = (&position["before"], &position["after"]);
        let position = match (before.is_null(), after.is_null()) {
            (false, false) => {
                return Err(invalid("Handler can not be positioned both before and after another handler"));
            }
            (false, true) => Position::Before(target_from_json(before)),
            (true, false) => Position::After(target_from_json(after)),
            (true, true) => {
                self.inner.add_handler(handler);
                return Ok(());
            }
        };
        self.inner.insert_handler(position, handler).map_err(invalid)
    }
}

/// Runs a built in transformer given by its json spec, with the current value as json.
#[napi(catch_unwind)]
pub fn apply_transformer(spec: String, input: String, current: Option<String>) -> Result<Option<String>> {
    let transformer = transformer_from_spec(&from_json(&spec)?)?;
    let current: Option<Value> = current.and_then(|current| serde_json::from_str(&current).ok());
    Ok(transformer(&input, current.as_ref()).map(|value| json!(value).to_string()))
}
