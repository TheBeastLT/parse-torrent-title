# parse-torrent-title

This library extracts information from a torrent or file name: title, year, season and episode numbers,
resolution, source, codec, audio, HDR, languages, release group and more.

It is written in Rust, and is also published to npm as `@thebeastlt/parse-torrent-title`, see
[Usage from Node.js](#usage-from-nodejs).

## Installation

```bash
cargo add parse-torrent-title
```

## Usage

```rust
let information = parse_torrent_title::parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU");

assert_eq!(information.title(), "Game of Thrones");
assert_eq!(information.seasons(), Some(&[1][..]));
assert_eq!(information.season(), Some(1));
assert_eq!(information.episodes(), Some(&[1][..]));
assert_eq!(information.episode(), Some(1));
assert_eq!(information.resolution(), Some("720p"));
assert_eq!(information.source(), Some("HDTV"));
assert_eq!(information.codec(), Some("x264"));
assert_eq!(information.group(), Some("CTU"));
```

The result serializes with [serde](https://serde.rs) into the same object as the js library returns.
A few more examples:

```rust
use parse_torrent_title::parse;
use serde_json::{json, to_value};

assert_eq!(
    to_value(parse("The.Mandalorian.S02E05.2160p.DSNP.WEB-DL.DDP5.1.Atmos.DV.HDR10.HEVC-FLUX.mkv"))?,
    json!({
        "title": "The Mandalorian", "seasons": [2], "season": 2, "episodes": [5], "episode": 5,
        "resolution": "4k", "source": "WEB-DL", "hdr": ["DV", "HDR"], "bitDepth": "10bit", "codec": "hevc",
        "audio": "atmos", "group": "FLUX", "container": "mkv", "extension": "mkv"
    })
);

assert_eq!(
    to_value(parse("[SubsPlease] Jujutsu Kaisen - 12.5 (1080p) [8A3F2C1D].mkv"))?,
    json!({
        "title": "Jujutsu Kaisen", "episodes": [12], "episode": 12, "halfEpisode": true, "resolution": "1080p",
        "episodeCode": "8A3F2C1D", "group": "SubsPlease", "container": "mkv", "extension": "mkv"
    })
);

assert_eq!(
    to_value(parse("Interstellar (2014) [1080p] [Dual Audio] [Hindi + English] [x264]"))?,
    json!({
        "title": "Interstellar", "year": 2014, "resolution": "1080p", "codec": "x264",
        "languages": ["dual audio", "english", "hindi"], "dubbed": true
    })
);

assert_eq!(
    to_value(parse("The Daily Show 2024.03.12 720p WEB h264-BAE"))?,
    json!({
        "title": "The Daily Show", "date": "2024-03-12", "resolution": "720p", "source": "WEB", "codec": "h264",
        "group": "BAE"
    })
);
# Ok::<(), serde_json::Error>(())
```

### Parsed fields

Only fields that were found are present in the result, except for `title`, which is always set.
Each field has an accessor on `ParsedTitle`, named like the field in snake case, and `get("field")` returns any
field as a `Value`, including the ones added by [custom handlers](#advanced-usage).

| Field           | Type         | Examples                                                                                        |
|-----------------|--------------|-------------------------------------------------------------------------------------------------|
| `title`         | `&str`       | `"Game of Thrones"`                                                                             |
| `year`          | `Value`      | `Int(2014)`, or `Str("2010-2015")` for a year range                                             |
| `date`          | `&str`       | `"2024-03-12"` (always formatted as `YYYY-MM-DD`)                                               |
| `seasons`       | `&[i64]`     | `[1]`, `[1, 2, 3, 4, 5]`                                                                        |
| `season`        | `i64`        | set only when `seasons` holds a single season                                                   |
| `episodes`      | `&[i64]`     | `[5]`, `[1, 2, 3]`                                                                              |
| `episode`       | `i64`        | set only when `episodes` holds a single episode                                                 |
| `halfEpisode`   | `bool`       | `true` for `12.5`-style recap/special episodes (`episode` is then `12`)                         |
| `episodeCode`   | `&str`       | `"8A3F2C1D"` (anime CRC32 hash)                                                                 |
| `volumes`       | `&[i64]`     | `[1, 2, 3]`                                                                                     |
| `complete`      | `bool`       | complete series, collections, box sets, trilogies, batches                                      |
| `extras`        | `bool`       | packs that also bundle OVAs, specials, movies, OSTs and similar                                 |
| `resolution`    | `&str`       | `"4k"`, `"1080p"`, `"720p"`, `"576p"`, `"480p"`                                                 |
| `source`        | `&str`       | `"BluRay REMUX"`, `"BluRay"`, `"BDRip"`, `"BRRip"`, `"UHDRip"`, `"WEB-DL"`, `"WEBRip"`, `"WEB"`, `"HDTV"`, `"HDRip"`, `"DVDRip"`, `"DVD"`, `"TVRip"`, `"SATRip"`, `"PPVRip"`, `"SCR"`, `"R5"`, `"TeleCine"`, `"TeleSync"`, `"CAM"` |
| `codec`         | `&str`       | `"x264"`, `"x265"`, `"h264"`, `"h265"`, `"hevc"`, `"avc"`, `"av1"`, `"xvid"`, `"divx"`, `"mpeg2"` |
| `bitDepth`      | `&str`       | `"8bit"`, `"10bit"`, `"12bit"`                                                                  |
| `hdr`           | `&[String]`  | `["DV"]`, `["HDR10+"]`, `["DV", "HDR"]`                                                         |
| `threeD`        | `&str`       | `"3D"`, `"3D SBS"`, `"3D HSBS"`, `"3D OU"`, `"3D HOU"`                                          |
| `audio`         | `&str`       | `"aac"`, `"ac3"`, `"eac3"`, `"dd5.1"`, `"dts"`, `"dts-hd"`, `"truehd"`, `"atmos"`, `"7.1 Atmos"`, `"flac"`, `"mp3"`, `"2.0"` |
| `audioChannels` | `&str`       | `"2.0"`, `"5.1"`, `"7.1"`                                                                       |
| `languages`     | `&[String]`  | `["english", "hindi"]`, `["multi audio"]`, `["dual audio"]`, `["multi subs"]`, `["latino"]`     |
| `dubbed`        | `bool`       | `true` when dubbed, or when `languages` contains `multi audio` or `dual audio`                  |
| `group`         | `&str`       | `"FLUX"`, `"SubsPlease"`                                                                        |
| `site`          | `&str`       | `"www.Torrenting.com"` from a `www.Site.com - ` prefix                                          |
| `container`     | `&str`       | `"mkv"`, `"avi"`, `"mp4"`, `"wmv"`, `"mpg"`, `"mpeg"`                                           |
| `extension`     | `&str`       | `"mkv"`, `"mp4"`, `"srt"`, `"nfo"` (video, subtitle and some other file extensions)             |
| `region`        | `&str`       | `"R5"`                                                                                          |
| `extended`      | `bool`       |                                                                                                 |
| `unrated`       | `bool`       | also set for uncensored releases                                                                |
| `uncut`         | `bool`       | only when uncut ends the title before its year, resolution or season                            |
| `remastered`    | `bool`       |                                                                                                 |
| `proper`        | `bool`       |                                                                                                 |
| `repack`        | `bool`       | also set for rerips                                                                             |
| `retail`        | `bool`       |                                                                                                 |
| `convert`       | `bool`       |                                                                                                 |
| `hardcoded`     | `bool`       | hardcoded subtitles                                                                             |

## Advanced usage

The parser is a list of handlers that run in order. Each handler belongs to a category, the result field it fills,
such as `resolution` or `languages`. A category usually has several handlers, tried one after another.
The title is the part of the name before the earliest match, cleaned of dots, underscores, brackets and
alternative-language titles.

To customize the parser, create your own copy of the default one:

```rust
use parse_torrent_title::{Handler, Regex, create_parser, parse, transformers::integer};

let mut parser = create_parser(); // a copy of the default parser
parser.add_handler(Handler::regex("part", Regex::new(r"Part[. ]([0-9])", "i")?).transform(integer()));

let name = "Silent.Witness.S18E03.Falling.Angels.Part.1.720p.HDTV.x264-FTP";
assert_eq!(parser.parse(name).get("part").and_then(|part| part.as_int()), Some(1));
assert_eq!(parse(name).get("part"), None);
# Ok::<(), parse_torrent_title::Error>(())
```

The default parser used by `parse` can not be changed, so customizing a copy never affects other users of `parse`.

### Regular expression handlers

`Handler::regex(name, regex)` creates a handler from a `Regex`, which takes a pattern and flags with the syntax
and the meaning of a javascript regular expression, such as `Regex::new(r"\b(4k)\b", "i")`. All flags but the
sticky `y` are supported.
If the regular expression has a capturing group, only the first group is passed to the transformer.
Otherwise the whole match is used.

#### Transformers

A transformer converts the matched string into the stored value, set with `.transform(..)`. If it returns `None`,
or a value of `0`, `""`, `false` or `null`, the match is ignored. The built-in transformers are in `transformers`:

| Transformer              | Result                                                                                          |
|--------------------------|-------------------------------------------------------------------------------------------------|
| `none()` (default)       | the matched string as is                                                                        |
| `value(v)`               | a fixed value `v`. A `$1` in a string value is replaced by the match, e.g. `value("$1p")`       |
| `integer()`              | the leading integer of the match                                                                |
| `boolean()`              | `true`                                                                                          |
| `lowercase()`            | the match in lower case                                                                         |
| `uppercase()`            | the match in upper case                                                                         |
| `date(format)?`          | a `YYYY-MM-DD` string, parsed with a [chrono format](https://docs.rs/chrono/latest/chrono/format/strftime/) such as `"%d %b %Y"`. Separators, ordinals (`1st`) and month names (`Sept`) are read loosely, and an invalid date does not match |
| `range()`                | consecutive numbers: `"1-3"` gives `[1, 2, 3]`, `"S01 S02"` gives `[1, 2]`. `None` for numbers that are not in ascending sequence |
| `range_up_to()`          | `[1..n]` from a single number                                                                   |
| `list()`                 | a sorted list of the unique numbers in the match                                                |
| `year_range()`           | a year, or a `"2010-2015"` string for a year range                                              |
| `array(chain)`           | the value from `chain` wrapped in a list, e.g. `array(integer())`                               |
| `uniq_concat(chain)`     | the value from `chain` appended to the existing list without duplicates, e.g. `uniq_concat(value("english"))` |

A transformer is any `Arc<dyn Fn(&str, Option<&Value>) -> Option<Value>>`, the second argument being the current
value of the field.

#### Options

Options are set with `.options(..)`, built from `Options::new()`:

| Option                          | Default | Description                                                                                       |
|---------------------------------|---------|---------------------------------------------------------------------------------------------------|
| `skip_if_already_found(bool)`   | `true`  | Skip this handler if the field was already set by an earlier handler.                             |
| `remove(bool)`                  | `false` | Remove the matched text from the name, so later handlers do not see it.                          |
| `skip_from_title(bool)`         | `false` | Do not let this match end the title. Use it for markers that can appear before or inside the title. |
| `skip_if_first(bool)`           | `false` | Skip the match if it comes before every other match found so far, i.e. it is probably part of the title. |
| `skip_if_before(&[fields])`     | `[]`    | Skip the match if it comes before a match of any of the listed fields.                            |
| `skip_if_after(&[fields])`      | `[]`    | Skip the match if it comes after a match of any of the listed fields.                             |
| `value(v)`                      |         | Store this value instead of the transformer result.                                              |

```rust
use parse_torrent_title::{Handler, Options, Regex, create_parser};

let mut parser = create_parser();
let network = Regex::new(r"\b(AMZN|NF|DSNP)\b", "")?;
parser.add_handler(Handler::regex("network", network).options(Options::new().remove(true)));

let information = parser.parse("Stranger.Things.S04E01.1080p.NF.WEB-DL.x264");
assert_eq!(information.get("network").and_then(|network| network.as_str()), Some("NF"));
# Ok::<(), parse_torrent_title::Error>(())
```

### Function handlers

A handler can also be a function, created with `Handler::function(name, f)`. It receives a `Context` with:

* `title`: the name being parsed, with the matches of earlier `remove` handlers taken out
* `result`: the result, which the handler can change
* `matched`: the matches found so far, as `MatchInfo { raw_match, match_index }` by field

```rust
use parse_torrent_title::{Handler, Regex, create_parser};

let mut parser = create_parser();
let harry_potter = Regex::new(r"harry.potter", "i")?;
parser.add_handler(Handler::function("isHarryPotterRelated", move |context| {
    if harry_potter.is_match(context.title) {
        context.result.set("isHarryPotterRelated", true);
    }
    None
}));

let information = parser.parse("[REQ] Harry Potter And The Prisoner Of Azkaban 2004 1080p BluRay DTS x264-hV");
assert_eq!(information.get("isHarryPotterRelated").and_then(|value| value.as_bool()), Some(true));
# Ok::<(), parse_torrent_title::Error>(())
```

If the matched text should end the title, return `Some(HandlerMatch::at(match_index))` from the handler.
Return `Some(HandlerMatch::at(match_index).raw_match(text).remove(true))` to also remove the match from the name.
Like in javascript, positions count utf-16 code units: `utf16_index` and `byte_index` convert them from and to
byte positions of the title.

### Customizing handlers

#### Positioning handlers

`add_handler` adds a handler at the end, so it runs after all default handlers.
Use `insert_handler` to run it earlier:

```rust
use parse_torrent_title::{Handler, Options, Position, Regex, Target, create_parser, transformers::value};

let mut parser = create_parser();

// before the first resolution handler
let eight_k = Handler::regex("resolution", Regex::new(r"\b(8k|4320p)\b", "i")?).transform(value("8k"));
parser.insert_handler(Position::before("resolution"), eight_k.options(Options::new().remove(true)))?;

// after the last resolution handler
let network = Handler::regex("network", Regex::new(r"\b(AMZN|NF|DSNP)\b", "")?);
parser.insert_handler(Position::after("resolution"), network.options(Options::new().remove(true)))?;

// after the handler with index 3 of the seasons category (0-based, negative counts from the end)
parser.insert_handler(Position::after(Target::new("seasons", 3)), Handler::function("myStep", |_| None))?;

assert_eq!(parser.parse("Movie.2024.8K.WEB").resolution(), Some("8k"));
# Ok::<(), parse_torrent_title::Error>(())
```

The position is resolved when the handler is added. `insert_handler` returns an error if the target does not exist.

#### Removing handlers

`remove_handlers` takes category names, `remove_handlers_where` a filter, and both return how many handlers they
removed:

```rust
let mut parser = parse_torrent_title::create_parser();

parser.remove_handlers(&["site"]);
parser.remove_handlers(&["languages", "dubbed"]);
parser.remove_handlers_where(|handler| handler.name == "resolution" && handler.index > 5);
```

Some categories are derived from others: `season` from `seasons`, `episode` from `episodes`,
and `dubbed` partly from `languages`. Removing a category does not remove the categories derived from it,
they just find nothing.

To leave categories out when adding the default handlers to a parser, use `add_defaults`:

```rust
use parse_torrent_title::{Parser, add_defaults};

let mut parser = Parser::new(); // has no handlers
add_defaults(&mut parser, &["site", "languages"]);
```

#### Listing handlers

`handlers` returns a description of each handler, which helps to find the index to target:

```rust
let parser = parse_torrent_title::create_parser();

let resolution = parser.handlers().into_iter().filter(|handler| handler.name == "resolution").collect::<Vec<_>>();
assert_eq!(resolution[1].index, 1);
assert_eq!(resolution[1].regex().map(|regex| regex.source()), Some(r"\b[([]?4k[)\]]?\b"));
```

`index` is the position within the category and `position` the position within all handlers.
`regex` and `options` are only set for regular expression handlers.

#### Copying parsers

`Parser` implements `Clone`, which copies a parser so it can be changed without affecting the original.
`create_parser()` is the same as cloning the default parser.

## Usage from Node.js

The [`@thebeastlt/parse-torrent-title`](crates/parse-torrent-title-node) npm package runs this crate as a native addon,
with the javascript api of parse-torrent-title:

```javascript
const ptt = require("@thebeastlt/parse-torrent-title");

ptt.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU");
// { title: "Game of Thrones", seasons: [1], season: 1, episodes: [1], episode: 1, resolution: "720p", ... }
```

See [its readme](crates/parse-torrent-title-node/README.md) for the javascript usage.

## Development

```bash
cargo test                                      # the test suite in crates/parse-torrent-title/tests
cargo run --release --example parse < names.txt # parses each line, printed as json
cargo run --release --example bench < names.txt # measures parsing speed

cd crates/parse-torrent-title-node
npm ci && npm run build && npm test             # builds the native addon and runs the javascript test suite
```

To release, publish a GitHub release with a `v<version>` tag such as `v2.1.0`. The release sets the version, so the
versions in the repository stay `0.0.0`, and publishes the crate and the npm packages.
