# parse-torrent-title

This package extracts information from a torrent or file name: title, year, season and episode numbers,
resolution, source, codec, audio, HDR, languages, release group and more.

It is a fork of [clement-escolano/parse-torrent-title](https://github.com/clement-escolano/parse-torrent-title),
extended with many more handlers for anime, foreign language and pack releases.

## Installation

Install it from this repository:
```bash
npm install github:TheBeastLT/parse-torrent-title
```
Node 12 or higher is required.

## Usage

```javascript
const ptt = require("parse-torrent-title");
const information = ptt.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU");

console.log(information.title);      // Game of Thrones
console.log(information.seasons);    // [1]
console.log(information.season);     // 1
console.log(information.episodes);   // [1]
console.log(information.episode);    // 1
console.log(information.resolution); // 720p
console.log(information.source);     // HDTV
console.log(information.codec);      // x264
console.log(information.group);      // CTU
```

A few more examples:

```javascript
ptt.parse("The.Mandalorian.S02E05.2160p.DSNP.WEB-DL.DDP5.1.Atmos.DV.HDR10.HEVC-FLUX.mkv");
// {
//   title: "The Mandalorian", seasons: [2], season: 2, episodes: [5], episode: 5,
//   resolution: "4k", source: "WEB-DL", hdr: ["DV", "HDR"], bitDepth: "10bit", codec: "hevc",
//   audio: "atmos", group: "FLUX", container: "mkv", extension: "mkv"
// }

ptt.parse("[SubsPlease] Jujutsu Kaisen - 12.5 (1080p) [8A3F2C1D].mkv");
// {
//   title: "Jujutsu Kaisen", episodes: [12], episode: 12, halfEpisode: true, resolution: "1080p",
//   episodeCode: "8A3F2C1D", group: "SubsPlease", container: "mkv", extension: "mkv"
// }

ptt.parse("Interstellar (2014) [1080p] [Dual Audio] [Hindi + English] [x264]");
// {
//   title: "Interstellar", year: 2014, resolution: "1080p", codec: "x264",
//   languages: ["dual audio", "english", "hindi"], dubbed: true
// }

ptt.parse("The Daily Show 2024.03.12 720p WEB h264-BAE");
// { title: "The Daily Show", date: "2024-03-12", resolution: "720p", source: "WEB", codec: "h264", group: "BAE" }
```

### Parsed fields

Only fields that were found are present in the result, except for `title`, which is always set.

| Field           | Type       | Examples                                                                                        |
|-----------------|------------|-------------------------------------------------------------------------------------------------|
| `title`         | `string`   | `"Game of Thrones"`                                                                             |
| `year`          | `number` or `string` | `2014`, or `"2010-2015"` for a year range                                             |
| `date`          | `string`   | `"2024-03-12"` (always formatted as `YYYY-MM-DD`)                                               |
| `seasons`       | `number[]` | `[1]`, `[1, 2, 3, 4, 5]`                                                                        |
| `season`        | `number`   | set only when `seasons` holds a single season                                                   |
| `episodes`      | `number[]` | `[5]`, `[1, 2, 3]`                                                                              |
| `episode`       | `number`   | set only when `episodes` holds a single episode                                                 |
| `halfEpisode`   | `boolean`  | `true` for `12.5`-style recap/special episodes (`episode` is then `12`)                         |
| `episodeCode`   | `string`   | `"8A3F2C1D"` (anime CRC32 hash)                                                                 |
| `volumes`       | `number[]` | `[1, 2, 3]`                                                                                     |
| `complete`      | `boolean`  | complete series, collections, box sets, trilogies, batches                                      |
| `extras`        | `boolean`  | packs that also bundle OVAs, specials, movies, OSTs and similar                                 |
| `resolution`    | `string`   | `"4k"`, `"1080p"`, `"720p"`, `"576p"`, `"480p"`                                                 |
| `source`        | `string`   | `"BluRay REMUX"`, `"BluRay"`, `"BDRip"`, `"BRRip"`, `"UHDRip"`, `"WEB-DL"`, `"WEBRip"`, `"WEB"`, `"HDTV"`, `"HDRip"`, `"DVDRip"`, `"DVD"`, `"TVRip"`, `"SATRip"`, `"PPVRip"`, `"SCR"`, `"R5"`, `"TeleCine"`, `"TeleSync"`, `"CAM"` |
| `codec`         | `string`   | `"x264"`, `"x265"`, `"h264"`, `"h265"`, `"hevc"`, `"avc"`, `"av1"`, `"xvid"`, `"divx"`, `"mpeg2"` |
| `bitDepth`      | `string`   | `"8bit"`, `"10bit"`, `"12bit"`                                                                  |
| `hdr`           | `string[]` | `["DV"]`, `["HDR10+"]`, `["DV", "HDR"]`                                                         |
| `threeD`        | `string`   | `"3D"`, `"3D SBS"`, `"3D HSBS"`, `"3D OU"`, `"3D HOU"`                                          |
| `audio`         | `string`   | `"aac"`, `"ac3"`, `"eac3"`, `"dd5.1"`, `"dts"`, `"dts-hd"`, `"truehd"`, `"atmos"`, `"7.1 Atmos"`, `"flac"`, `"mp3"`, `"2.0"` |
| `audioChannels` | `string`   | `"2.0"`, `"5.1"`, `"7.1"`                                                                       |
| `languages`     | `string[]` | `["english", "hindi"]`, `["multi audio"]`, `["dual audio"]`, `["multi subs"]`, `["latino"]`     |
| `dubbed`        | `boolean`  | `true` when dubbed, or when `languages` contains `multi audio` or `dual audio`                  |
| `group`         | `string`   | `"FLUX"`, `"SubsPlease"`                                                                        |
| `site`          | `string`   | `"www.Torrenting.com"` from a `www.Site.com - ` prefix                                          |
| `container`     | `string`   | `"mkv"`, `"avi"`, `"mp4"`, `"wmv"`, `"mpg"`, `"mpeg"`                                           |
| `extension`     | `string`   | `"mkv"`, `"mp4"`, `"srt"`, `"nfo"` (video, subtitle and some other file extensions)             |
| `region`        | `string`   | `"R5"`                                                                                          |
| `extended`      | `boolean`  |                                                                                                 |
| `unrated`       | `boolean`  | also set for uncensored releases                                                                |
| `remastered`    | `boolean`  |                                                                                                 |
| `proper`        | `boolean`  |                                                                                                 |
| `repack`        | `boolean`  | also set for rerips                                                                             |
| `retail`        | `boolean`  |                                                                                                 |
| `convert`       | `boolean`  |                                                                                                 |
| `hardcoded`     | `boolean`  | hardcoded subtitles                                                                             |

## Advanced usage

The parser is a list of handlers that run in order. Each handler can fill fields in the result.
The title is the part of the name before the earliest match, cleaned of dots, underscores, brackets and
alternative-language titles.

### Regular expression handlers

To populate an extra field, add a handler with a regular expression:

```javascript
const ptt = require("parse-torrent-title");
const { integer } = ptt.Transformers;

ptt.addHandler("part", /Part[. ]([0-9])/i, integer);
const information = ptt.parse("Silent.Witness.S18E03.Falling.Angels.Part.1.720p.HDTV.x264-FTP");

console.log(information.part); // 1
```

The signature is `addHandler(name, regExp, [transformer], [options])`.
If the regular expression has a [capturing group](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions/Groups_and_backreferences),
only the first group is passed to the transformer. Otherwise the whole match is used.

#### Transformers

A transformer converts the matched string into the stored value. If it returns a falsy value, the match is ignored.
Built-in transformers are exported as `Transformers`:

| Transformer           | Result                                                                                          |
|-----------------------|-------------------------------------------------------------------------------------------------|
| `none` (default)      | the matched string as is                                                                        |
| `value(v)`            | a fixed value `v`. A `$1` in a string value is replaced by the match, e.g. `value("$1p")`       |
| `integer`             | `parseInt` of the match                                                                         |
| `boolean`             | `true`                                                                                          |
| `lowercase`           | the match in lower case                                                                         |
| `uppercase`           | the match in upper case                                                                         |
| `date(format)`        | a `YYYY-MM-DD` string, parsed with a [moment](https://momentjs.com/docs/#/parsing/string-format/) format such as `"DD MM YYYY"` |
| `range`               | an array of consecutive numbers: `"1-3"` gives `[1, 2, 3]`, `"S01 S02"` gives `[1, 2]`. Returns `null` for numbers that are not in ascending sequence |
| `rangeUpTo`           | `[1..n]` from a single number                                                                   |
| `list`                | a sorted array of the unique numbers in the match                                               |
| `yearRange`           | a year, or a `"2010-2015"` string for a year range                                              |
| `array(chain)`        | the value from `chain` wrapped in an array, e.g. `array(integer)`                               |
| `uniqConcat(chain)`   | the value from `chain` appended to the existing array without duplicates, e.g. `uniqConcat(value("english"))` |

#### Options

| Option               | Default | Description                                                                                       |
|----------------------|---------|---------------------------------------------------------------------------------------------------|
| `skipIfAlreadyFound` | `true`  | Skip this handler if the field was already set by an earlier handler.                             |
| `remove`             | `false` | Remove the matched text from the name, so later handlers do not see it.                          |
| `skipFromTitle`      | `false` | Do not let this match end the title. Use it for markers that can appear before or inside the title. |
| `skipIfFirst`        | `false` | Skip the match if it comes before every other match found so far, i.e. it is probably part of the title. |
| `skipIfBefore`       | `[]`    | Skip the match if it comes before a match of any of the listed fields.                            |
| `skipIfAfter`        | `[]`    | Skip the match if it comes after a match of any of the listed fields.                             |
| `value`              |         | Store this value instead of the transformer result.                                              |

The options object can be passed as the third argument when no transformer is needed:

```javascript
ptt.addHandler("network", /\b(AMZN|NF|DSNP)\b/, { remove: true });
```

### Function handlers

A handler can also be a function. It receives a context object with:

* `title`: the name being parsed, with the matches of earlier `remove` handlers taken out
* `result`: the result object, which the handler can change
* `matched`: the matches found so far, as `{ [field]: { rawMatch, matchIndex } }`

```javascript
const ptt = require("parse-torrent-title");

ptt.addHandler(({ title, result }) => {
    if (/harry.potter/i.test(title)) {
        result.isHarryPotterRelated = true;
    }
});

const information = ptt.parse("[REQ] Harry Potter And The Prisoner Of Azkaban 2004 1080p BluRay DTS x264-hV");
console.log(information.isHarryPotterRelated); // true
```

A named function handler is added with `addHandler(name, fn)`.
If the matched text should end the title, return `{ matchIndex }` from the handler.
Return `{ rawMatch, matchIndex, remove: true }` to also remove the match from the name.

### Multiple parsers

`addHandler` and `parse` on the module work on a shared default parser.
To keep your handlers separate, create a new parser:

```javascript
const { Parser, addDefaults } = require("parse-torrent-title");

const parser = new Parser(); // has no handlers
addDefaults(parser);         // adds the default handlers
parser.addHandler("part", /Part[. ]([0-9])/i, require("parse-torrent-title").Transformers.integer);
```

Handlers run in the order they are added, so handlers added after `addDefaults` run after the default ones.

### Usage with TypeScript

If you add new fields with `addHandler` in TypeScript, the `DefaultParserResult` type does not include them,
and you get a TS2339 error.

To fix this, create an interface that extends `DefaultParserResult` with the extra fields and use it as the
`Parser` type parameter:

```ts
import { Parser, DefaultParserResult, addDefaults } from "parse-torrent-title";

interface ParserResult extends DefaultParserResult {
    part?: number;
}

const parser = new Parser<ParserResult>();
addDefaults(parser);
parser.addHandler("part", /(?:Part|CD)[. ]?([0-9])/i, input => parseInt(input, 10));

const result = parser.parse("Watergate.2018.Part1.DOC.SUBFRENCH.1080p.HDTV.H264-ELEARNiNG");
console.log(result.year); // 2018
console.log(result.part); // 1
```

## Development

```bash
npm install
npm test      # mocha test suite in test/
npm run lint  # eslint
```
