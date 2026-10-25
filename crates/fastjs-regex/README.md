# fastjs-regex

ECMAScript regex semantics with a linear-time fast path.

`fastjs-regex` runs JavaScript regular expressions from Rust with the answers V8 gives: lookbehind of any length,
backreferences, named groups, `\p{..}` classes, the ASCII-only `\w`, `\d` and `\b`, the `d`, `g`, `i`, `m`, `s`,
`u`, `v` and `y` flags, and the methods of `String.prototype` (`test`, `exec`, `match`, `matchAll`, `search`,
`replace`, `replaceAll`, `split`) with their own rules for empty matches, `$` replacement patterns and captures in
`split`. It was extracted from [parse-torrent-title](https://github.com/TheBeastLT/parse-torrent-title), a port
of a JavaScript library whose ~450 patterns had to keep matching exactly like they do in Node.

```rust
use fastjs_regex::{Regex, utf16_index};

let episode = Regex::new(r"(?<=\bS\d{1,2})E(?<episode>\d{1,3})(?!\d)", "i")?;
let title = "Show.S01E02.1080p";
let m = episode.find(title).unwrap();
assert_eq!(&title[m.named_group("episode").unwrap()], "02");
assert_eq!(episode.search(title), Some(utf16_index(title, m.start())));

let words = Regex::new(r"(\s*)[,;]\s*", "g")?;
assert_eq!(words.replace("a , b;c", "[$&|$1]"), "a[ , | ]b[;|]c");
assert_eq!(words.split_limit("a , b;c", 3), [Some("a"), Some(" "), Some("b")]);

let sticky = Regex::new(r"\d", "y")?;
assert!(sticky.find_at("a1", 1).is_some() && sticky.find_at("a1", 0).is_none());
# Ok::<(), fastjs_regex::Error>(())
```

## How it works

Every pattern is compiled twice:

- by [`regress`](https://crates.io/crates/regress), a backtracking engine with ECMAScript semantics, which can
  answer everything, and
- after a translation into the syntax of the linear-time [`regex`](https://crates.io/crates/regex) crate, keeping
  the JavaScript meaning of each construct (`\w` stays ASCII, `.` excludes the JavaScript line terminators, ...).

When the translation is exact (no lookaround, backreference, quantified capture group, multiline anchor, ...), it
replaces the backtracking engine for `find`, `find_at` and `is_match`. Otherwise it matches a superset of the pattern: lookarounds are dropped
or kept as plain groups, backreferences match anything. That superset is a prefilter: when it does not match, there
is no match, which is the common case when many patterns run over each text, and when it does, the backtracking
search starts where the superset first matched. Patterns the translation does not cover (`v` set operations,
modifiers such as `(?i:..)`, a case-folded `\P{..}`, legacy octal escapes, sticky patterns, ...) run on `regress`
alone.

`i` without `u` folds case by uppercasing in JavaScript, unlike the simple case folding of the `regex` crate: the
few characters where the two differ make the translation stand aside for the texts holding them.

The `verify` feature checks every shortcut against `regress` and panics when one changes a match. It is meant for
test suites; `tests/js_parity.rs` runs every method against answers recorded from Node by
`tests/fixtures/gen.mjs` (`node crates/fastjs-regex/tests/fixtures/gen.mjs` regenerates them).

## JavaScript methods

| JavaScript                                 | `fastjs_regex::Regex`                                         |
|--------------------------------------------|---------------------------------------------------------------|
| `re.test(s)`                               | `is_match(s)`                                                 |
| `re.exec(s)`, `s.match(re)`                | `find(s)`, a `Match` with byte ranges, groups and named groups |
| `re.exec(s)` with `re.lastIndex = i`       | `find_at(s, byte_index(s, i))`                                |
| `s.matchAll(re)`, `s.match(re)` with `g`   | `find_iter(s)`                                                |
| `s.search(re)`                             | `search(s)`, `None` for -1                                    |
| `s.replace(re, "$1")`                      | `replace(s, "$1")`, every match with the `g` flag             |
| `s.replace(re, (match, ...groups) => ..)`  | `replace_with(s, \|m\| ..)`                                   |
| `s.replaceAll(re, ..)`                     | `replace_all(s, ..)`, `replace_all_with(s, \|m\| ..)`         |
| `s.split(re)`, `s.split(re, limit)`        | `split(s)`, `split_limit(s, limit)`                           |

A `Regex` holds no `lastIndex`: `find_at` takes it as an argument, so a regex can be shared between threads.
Positions are byte positions into the `&str`; `utf16_index` and `byte_index` convert them from and to the UTF-16
positions of JavaScript. `split` gives `None` for a capture group that did not participate (`undefined`).

## Known differences from V8

- Without the `u` flag, `ſ` and the kelvin sign (U+212A) match `s` and `k` under `i`.
- Without the `u` flag, JavaScript sees a character outside the basic multilingual plane (an emoji) as two halves:
  `.` and negated classes match one half, and an empty match moves on by one half. Texts here are `&str`, so such a
  character is one character: no match, split piece or position falls inside it.

## Not supported

- Lone surrogates, in texts or as the result of matching half a character (see above): texts are UTF-8 `&str`.
  Supporting them means matching UTF-16 input and returning UTF-16 slices, which `regress` can do but which is a
  different API.
- Mutable `lastIndex` state, `RegExp.prototype.compile`, the legacy `RegExp.$1` statics and subclassing through
  `Symbol.species`: `find_at` is the stateless equivalent of `exec` with a `lastIndex`.
- The `indices` array of the `d` flag: the flag is accepted, and every `Match` already has the ranges of the match
  and its groups.

## Compared to

- [`regress`](https://crates.io/crates/regress): the engine this crate falls back to, with the same ECMAScript
  semantics. It backtracks on every search, and its `replace` follows its own `$0` and `${name}` syntax rather than
  JavaScript's `$&` and `$<name>`; there is no `split`.
- [`regex`](https://crates.io/crates/regex): linear time, but no lookaround or backreferences, and `\w`, `\d`, `\b`
  and case folding are Unicode-aware, unlike JavaScript.
- [`fancy-regex`](https://crates.io/crates/fancy-regex): adds lookaround and backreferences on top of `regex`, with
  Perl/Python syntax and semantics (no variable-length lookbehind, other escapes and flags).
- [`ecma_regex`](https://crates.io/crates/ecma_regex): a binding to the QuickJS regex engine, so a C dependency;
  stateless without JavaScript flag semantics, and last released in 2023.

## Benchmark

Mean time to find the first match in each of 1,285 torrent titles (the titles of parse-torrent-title's tests), best
of 5 runs of 300 rounds, release build with LTO, Apple M4 Pro, rustc 1.99.1:

| Pattern                                  | Path                       | Matches | fastjs-regex | regress | regex           |
|------------------------------------------|----------------------------|--------:|-------------:|--------:|-----------------|
| `/\b(?:x26[45]\|hevc)\b/i`               | translated, exact          |     345 |        35 ns |  322 ns | 21 ns           |
| `/(?<![a-z\d])(\d{3,4})p(?![a-z\d])/i`   | superset prefilter         |     634 |       125 ns |  209 ns | not expressible |
| `/(?<=\[)(?:hevc\|x265)(?=\])/i`         | superset prefilter, rare   |       8 |        45 ns |   55 ns | not expressible |
| `/[\p{L}--[a-z]]{2}\d/v`                 | `regress` only             |     227 |       263 ns |  255 ns | not expressible |

The `regex` column runs the hand-written equivalent `(?i)(?-u:\b)(?:x26[45]|hevc)(?-u:\b)`; the difference to it
is the cost of the checks around the translated pattern.
