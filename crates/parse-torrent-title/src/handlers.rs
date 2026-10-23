//! The default handlers, run in the order they are added.

use crate::Value;
use crate::parser::{Context, Handler, HandlerMatch, MatchInfo, Options, Parser};
use crate::regex::{Regex, byte_index, group, parse_int, slice_from, utf16_index, utf16_len};
use crate::transformers::{
    Transformer, array, boolean, cjk_integer, date, integer, list, lowercase, none, range, range_up_to, uniq_concat,
    uppercase, value, year_range,
};

const VIDEO_EXTENSIONS: &str = "3g2|3gp|avi|flv|mkv|mk3d|mov|mp2|mp4|m4v|mpe|mpeg|mpg|mpv|webm|wmv|ogm|divx|ts|m2ts";
const SUBTITLE_EXTENSIONS: &str = "sub|smi|srt|ssa|ass|vtt";
const PACK_EXTRA: &str = r"(?:\d+\s*)?(?:OVAs?|OAVs?|OADs?|ONAs?|Specials?|SPs?|Movies?|Films?|Extras?|Bonus(?:es)?|Novels?|Mangas?|OSTs?|Soundtracks?|Artbooks?|OP|ED|NC ?(?:OP|ED)s?)\b";
const PLUS_EXTRA: &str = r"(?:\d+\s*)?(?:Extras?|Bonus(?:es)?)\b";
const EPISODE_CODE: &str = r"(?:\bS\d{1,2}E\d{1,3}|\b\d{1,2}x\d{1,3}\b)";
const HALF_EPISODE_END: &str = concat!(
    r"\.5(?:v\d)?(?=[ ._\-)\][]|$)(?![ .-]\d)(?![ ._]?(?:gb|mb|ch|kbps|fps|bit|x\d|th\b|km\b|dimensional))",
    r"(?![._-](?:minut[eio]s?|seconds?|hours?|days?|weeks?|months?|years?|anni|o'?clock|[ap]m|to|million|ways|signs|stages|",
    r"steps|times|reasons|things|babies|quarts|acres|stars?|miles|miglia|lit(?:ri|ers|res)|b[oö]l[uü]m|jigen)\b)",
);

fn marked_half_episode() -> String {
    [
        r"(?<!",
        EPISODE_CODE,
        r".*)(?:\bs\d{1,2}e|\be|\bep(?:isode)?[ .]*|[ .]-[ .]?|^(?=\d{1,3}\.5(?:v\d)? - \D))0*(\d{1,3})",
        HALF_EPISODE_END,
        r"(?!.*",
        EPISODE_CODE,
        ")",
    ]
    .concat()
}

fn bare_half_episode() -> String {
    [
        r"(?<!", EPISODE_CODE, r".*|\d - .*|(?:imdb|part|seasons?|sezon|сезон|ios|android|version)[ ._]?|\b(?:19|20)\d{2})(?:[ .]|\[)(?=\d{2})0*(\d{1,3})",
        HALF_EPISODE_END, r"(?!.*(?:", EPISODE_CODE, r"| - \d))",
    ]
    .concat()
}

fn opts() -> Options {
    Options::default()
}

/// Adds the library's own handlers, whose patterns are known to be valid.
struct Defaults<'a>(&'a mut Parser);

impl Defaults<'_> {
    fn regex(&mut self, name: &str, source: &str, flags: &str, transformer: Transformer, options: Options) {
        self.0.add_handler(Handler::regex(name, Regex::known(source, flags)).transform(transformer).options(options));
    }

    /// A regex which only matches when not surrounded by letters of any alphabet.
    fn standalone(&mut self, name: &str, source: &str, flags: &str, transformer: Transformer, options: Options) {
        self.regex(
            name,
            &[r"(?<!\p{L})(?:", source, r")(?!\p{L})"].concat(),
            &[flags, "u"].concat(),
            transformer,
            options,
        );
    }

    fn function(
        &mut self,
        name: &str,
        handler: impl Fn(&mut Context<'_>) -> Option<HandlerMatch> + Send + Sync + 'static,
    ) {
        self.0.add_handler(Handler::function(name, handler));
    }

    fn language_name(&mut self, name: &str, language: &str, options: Options) {
        let caps: String = name
            .chars()
            .map(|letter| match letter {
                'i' => "[iI]".to_owned(),
                'a'..='z' => letter.to_ascii_uppercase().to_string(),
                _ => letter.to_string(),
            })
            .collect();
        let skip_if_first = options.skip_if_first;
        let mut options = options.skip_if_already_found(false);
        options.skip_if_first = false;
        let language = || uniq_concat(value(language));
        self.regex("languages", &[r"(?<=\p{Ll}.*)\b(?:", &caps, r")\b"].concat(), "u", language(), options.clone());
        self.regex(
            "languages",
            &[r"\b(?:", name, r")\b(?![ .]+(?!(?:sub(?:title)?s?|language|dub\w*)\b)\p{L}+(?![\p{L}\p{N}]|$))"]
                .concat(),
            "iu",
            language(),
            Options { skip_if_first, ..options.clone() },
        );
        self.regex(
            "languages",
            &[r"\b(?:", name, r")\b"].concat(),
            "iu",
            language(),
            Options { skip_if_first: true, skip_if_before: vec!["year".to_owned()], ..options },
        );
    }
}

/// Adds the default handlers to `parser`, leaving out the handlers of the `exclude` fields.
pub fn add_defaults(parser: &mut Parser, exclude: &[&str]) {
    let mut p = Defaults(parser);

    // Site
    p.regex(
        "site",
        r"^(www?[., ][\w-]+[. ][\w-]+(?:[. ][\w-]+)?)\s+-\s*",
        "i",
        none(),
        opts().skip_from_title().remove(),
    );

    // Uncut
    p.regex(
        "uncut",
        r"(?<=\w[ ._-]+)[[(]?\buncut\b[\])]?(?=[ ._-]*(?:[[(]?(?:19\d|20[012])\d\b|\d{3,4}p\b|S\d|$))",
        "i",
        boolean(),
        opts(),
    );

    // Episode code
    p.regex(
        "episodeCode",
        r"[[(]([a-z0-9]{8}|[A-Z0-9]{8})[\])](?=\.[a-zA-Z0-9]{1,5}$|$)",
        "",
        uppercase(),
        opts().remove(),
    );
    p.regex("episodeCode", r"\[(?=[A-Z]+\d|\d+[A-Z])([A-Z0-9]{8})]", "", uppercase(), opts().remove());

    // Resolution
    p.regex("resolution", r"(?<=\b(?:4k|2160p)\b.*)\b(480|720|1080)p\b", "i", value("$1p"), opts().remove());
    p.regex("resolution", r"\b[([]?4k[)\]]?\b", "i", value("4k"), opts().remove());
    p.regex("resolution", r"21600?[pi]", "i", value("4k"), opts().skip_if_already_found(false).remove());
    p.regex("resolution", r"[([]?3840x\d{4}[)\]]?", "i", value("4k"), opts().remove());
    p.regex("resolution", r"[([]?1920x\d{3,4}[)\]]?", "i", value("1080p"), opts().remove());
    p.regex("resolution", r"[([]?1280x\d{3}[)\]]?", "i", value("720p"), opts().remove());
    p.regex(
        "resolution",
        r"[([]?(?<!\d)(?!(?:19[5-9]|20[012])\dx\d{3}(?!\d))\d{3,4}x(\d{3,4})[)\]]?",
        "i",
        value("$1p"),
        opts().remove(),
    );
    p.regex("resolution", r"(480|720|1080)0[pi]", "i", value("$1p"), opts().remove());
    p.regex("resolution", r"(?:BD|HD|M)(720|1080|2160)", "", value("$1p"), opts().remove());
    p.regex("resolution", r"(480|576|720|1080|2160)[piр]", "i", value("$1p"), opts().remove());
    p.regex("resolution", r"(?:^|\D)(\d{3,4})[pi]", "i", value("$1p"), opts().remove());
    p.regex("resolution", r"[([](480|576|720|1080|2160)[)\]]", "", value("$1p"), opts().remove());

    // Year
    p.regex("date", r"(?<=\W|^)([([]?(?:19[6-9]|20[012])[0-9]([. \-/\\])(?:0[1-9]|1[012])\2(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)", "", date("YYYY MM DD"), opts().remove());
    p.regex("date", r"(?<=\W|^)([([]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)", "", date("DD MM YYYY"), opts().remove());
    p.regex(
        "date",
        r"(?<=\W)([([]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)",
        "",
        date("MM DD YYYY"),
        opts().remove(),
    );
    p.regex("date", r"(?<=\W)([([]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)", "", date("MM DD YY"), opts().remove());
    p.regex("date", r"(?<=\W)([([]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)", "", date("DD MM YY"), opts().remove());
    p.regex("date", r"(?<=\W|^)([([]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:19[7-9]|20[012])[0-9][)\]]?)(?=\W|$)", "i", date("DD MMM YYYY"), opts().remove());
    p.regex("date", r"(?<=\W|^)([([]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:0[1-9]|[0126789][0-9])[)\]]?)(?=\W|$)", "i", date("DD MMM YY"), opts().remove());
    p.regex(
        "date",
        r"(?<=\W|^)([([]?20[012][0-9](?:0[1-9]|1[012])(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)",
        "",
        date("YYYYMMDD"),
        opts().remove(),
    );
    p.regex(
        "date",
        r"^([([]?(?:19[6-9]|20[012])[0-9] (?:0[1-9]|1[012])-(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)",
        "",
        date("YYYY MM DD"),
        opts().remove(),
    );

    // Year
    p.regex(
        "year",
        r"[([*]?[ .]?(?<!\bE)((?:19\d|20[012])\d[ .]?-[ .]?(?:19\d|20[012])\d)(?:\s?[*)\]])?",
        "",
        year_range(),
        opts().remove(),
    );
    p.regex("year", r"[([*][ .]?((?:19\d|20[012])\d[ .]?-[ .]?\d{2})(?:\s?[*)\]])?", "", year_range(), opts().remove());
    p.regex(
        "year",
        r"[([][ .]?((?:19\d|20[012])\d(?:(?:[ .]?[,—–/][ .]?|[ .])(?:19\d|20[012])\d)+)(?!\d)",
        "",
        year_range(),
        opts().remove(),
    );
    p.regex(
        "year",
        r"[([]((?:19\d|20[012])\d(?:[)\]][ .]*&(?:(?!\bS\d)[^()[\]])*[([](?:19\d|20[012])\d)+)[)\]]",
        "i",
        year_range(),
        opts().remove(),
    );
    p.regex(
        "year",
        r"(?<=[ .](?:19\d|20[012])\d(?:[ .]+\/[^/()[\]]+)*[ .]+)[([]((?:19\d|20[012])\d)[)\]]",
        "",
        integer(),
        opts().remove(),
    );
    p.regex("year", r"(?<!\b(?:complete|collection|duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|anthology|saga|movies)\b.*|\band\b.*|&.*|\b\d[ .]?-[ .]?\d\b.*)(?<!(?:19\d|20[012])\d[ .](?:19\d|20[012])\d)(?<=[ .](?:19\d|20[012])\d)[ .]((?:19\d|20[012])\d)(?<!\2[ .](\d{4}))(?=[ .]|$)(?![ .](?:19\d|20[012])\d\b|[ .]?x[ .]?\d)", "i", integer(), opts().remove());
    p.regex("year", r"(?<!\d)(?!1920x)((?:19\d|20[012])\d)(?=x\d{1,3}(?!\d))", "i", integer(), opts());
    p.regex(
        "year",
        r"[([*]?(?!^)(?<!\d|Cap[. ]?|\bep[. ]?|\bE|\bS(?=\d{4}E\d))((?:19\d|20[012])\d)(?!\d|kbps)[*)\]]?",
        "i",
        integer(),
        opts().remove(),
    );
    p.regex(
        "year",
        r"^[([]?((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?(?![ ._]+(?:S\d{1,2}[ .]?E\d|E\d{1,3}\b))",
        "i",
        integer(),
        opts().remove(),
    );

    // Extended
    p.regex("extended", r"EXTENDED", "", boolean(), opts());
    p.regex("extended", r"- Extended", "i", boolean(), opts());

    // Convert
    p.regex("convert", r"CONVERT", "", boolean(), opts());

    // Hardcoded
    p.regex("hardcoded", r"HC|HARDCODED", "", boolean(), opts());

    // Proper
    p.regex("proper", r"(?:REAL.)?PROPER", "", boolean(), opts());

    // Repack
    p.regex("repack", r"REPACK|RERIP", "", boolean(), opts());

    // Retail
    p.regex("retail", r"\bRetail\b", "i", boolean(), opts());

    // Remastered
    p.regex("remastered", r"\bRemaster(?:ed)?\b", "i", boolean(), opts());
    p.regex("remastered", r"\b[[(]?REKONSTRUKCJA[\])]?\b", "i", boolean(), opts());

    // Unrated
    p.regex("unrated", r"\bunrated|uncensored\b", "i", boolean(), opts());

    // Region
    p.regex("region", r"R\d\b(?![ ._]+(?:-[ ._]*)?(?:[Ee][Pp]\.?[ ._]*)?\d)", "", none(), opts().skip_if_first());

    // Video depth
    p.regex("bitDepth", r"(?:8|10|12)[- ]?bit", "i", lowercase(), opts().remove());
    p.regex("bitDepth", r"\bhevc\s?10\b", "i", value("10bit"), opts());
    p.regex("bitDepth", r"\bhdr10\b", "i", value("10bit"), opts());
    p.regex("bitDepth", r"\bhi10\b", "i", value("10bit"), opts().remove().skip_from_title());
    p.function("bitDepth", |c| {
        if let Some(bit_depth) = c.result.bit_depth().filter(|b| !b.is_empty()) {
            let bit_depth = bit_depth.replacen([' ', '-'], "", 1);
            c.result.set("bitDepth", bit_depth);
        }
        None
    });

    // Source
    p.regex(
        "source",
        r"\b(?:H[DQ][ .-]*)?CAM(?:H[DQ])?(?:[ .-]*Rip)?\b(?![ .-]*(?:S\d|E\d|\((?:19|20)\d{2}\)))",
        "i",
        value("CAM"),
        opts().remove(),
    );
    p.regex("source", r"\b(?:H[DQ][ .-]*)?S[ .-]+print", "i", value("CAM"), opts().remove());
    p.regex("source", r"\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b", "i", value("TeleSync"), opts().remove());
    p.regex("source", r"\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b", "", value("TeleCine"), opts().remove());
    p.regex("source", r"\bBlu[ .-]*Ray\b(?=.*remux)", "i", value("BluRay REMUX"), opts().remove());
    p.regex("source", r"(?:BD|BR|UHD)[- ]?remux", "i", value("BluRay REMUX"), opts().remove());
    p.regex("source", r"(?<=remux.*)\bBlu[ .-]*Ray\b", "i", value("BluRay REMUX"), opts().remove());
    p.regex("source", r"\bBlu[ .-]*Ray\b(?![ .-]*Rip)", "i", value("BluRay"), opts().remove());
    p.regex("source", r"\bUHD[ .-]*Rip\b", "i", value("UHDRip"), opts().remove());
    p.regex("source", r"\bP(?:re)?-HD(?:Rip)?\b", "i", value("SCR"), opts().remove());
    p.regex("source", r"\bHD[ .-]*Rip\b", "i", value("HDRip"), opts().remove());
    p.regex("source", r"\bMicro[ .-]*HD\b", "i", value("HDRip"), opts().remove());
    p.regex("source", r"\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b", "i", value("BRRip"), opts().remove());
    p.regex("source", r"\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|[[(]BD[\]) .,-]", "i", value("BDRip"), opts().remove());
    p.regex("source", r"\b(?:HD[ .-]*)?DVD[ .-]*Rip\b", "i", value("DVDRip"), opts().remove());
    p.regex("source", r"\bVHS[ .-]*Rip\b", "i", value("DVDRip"), opts().remove());
    p.regex("source", r"\b(?:DVD?|BD|BR|HD)?[ .-]*Scr(?:eener)?\b", "i", value("SCR"), opts().remove());
    p.regex("source", r"\bP(?:re)?DVD(?:Rip)?\b", "i", value("SCR"), opts().remove());
    p.regex("source", r"\bDVD(?:R\d?)?\b", "i", value("DVD"), opts().remove());
    p.regex("source", r"\bVHS\b", "i", value("DVD"), opts().remove().skip_if_first());
    p.regex("source", r"\bPPVRip\b", "i", value("PPVRip"), opts().remove());
    p.regex("source", r"\bHD[ .-]*TV(?:Rip)?\b", "i", value("HDTV"), opts().remove());
    p.regex("source", r"\bDVB[ .-]*(?:Rip)?\b", "i", value("HDTV"), opts().remove());
    p.regex("source", r"\bSAT[ .-]*Rips?\b", "i", value("SATRip"), opts().remove());
    p.regex("source", r"\bTVRips?\b", "i", value("TVRip"), opts().remove());
    p.regex("source", r"\bR5\b", "i", value("R5"), opts().remove());
    p.regex("source", r"\bWEB[ .-]*DL(?:Rip)?\b", "i", value("WEB-DL"), opts().remove());
    p.regex("source", r"\bWEB[ .-]*Rip\b", "i", value("WEBRip"), opts().remove());
    p.regex("source", r"\b(?:DL|WEB|BD|BR)MUX\b", "i", none(), opts());
    p.regex("source", r"\b(DivX|XviD)\b", "", none(), opts());

    // HDR
    p.regex(
        "hdr",
        r"\bDV\b|dolby.?vision|\bDoVi\b",
        "i",
        uniq_concat(value("DV")),
        opts().remove().skip_if_already_found(false),
    );
    p.regex(
        "hdr",
        r"HDR10(?:\+|plus)",
        "i",
        uniq_concat(value("HDR10+")),
        opts().remove().skip_if_already_found(false),
    );
    p.regex("hdr", r"\bHDR(?:10)?\b", "i", uniq_concat(value("HDR")), opts().remove().skip_if_already_found(false));

    // 3D
    p.regex("threeD", r"\b(3D)\b.*\b(Half-?SBS|H[-\\/]?SBS)\b", "i", value("3D HSBS"), opts());
    p.regex("threeD", r"\bHalf.Side.?By.?Side\b", "i", value("3D HSBS"), opts());
    p.regex("threeD", r"\b(3D)\b.*\b(Full-?SBS|SBS)\b", "i", value("3D SBS"), opts());
    p.regex("threeD", r"\bSide.?By.?Side\b", "i", value("3D SBS"), opts());
    p.regex("threeD", r"\b(3D)\b.*\b(Half-?OU|H[-\\/]?OU)\b", "i", value("3D HOU"), opts());
    p.regex("threeD", r"\bHalf.?Over.?Under\b", "i", value("3D HOU"), opts());
    p.regex("threeD", r"\b(3D)\b.*\b(OU)\b", "i", value("3D OU"), opts());
    p.regex("threeD", r"\bOver.?Under\b", "i", value("3D OU"), opts());
    p.regex("threeD", r"\b((?:BD)?3D)\b", "i", value("3D"), opts().skip_if_first());

    // Codec
    p.regex("codec", r"\b[xh][-. ]?26[45]", "i", lowercase(), opts().remove());
    p.regex("codec", r"\bhevc(?:\s?10)?\b", "i", value("hevc"), opts().remove().skip_if_already_found(false));
    p.regex(
        "codec",
        r"\b(?:dvix|mpeg2|divx|xvid|avc|(?<!-)av1)\b",
        "i",
        lowercase(),
        opts().remove().skip_if_already_found(false),
    );
    p.function("codec", |c| {
        if let Some(codec) = c.result.codec().filter(|codec| !codec.is_empty()) {
            let codec = codec.replacen([' ', '.', '-'], "", 1);
            c.result.set("codec", codec);
        }
        None
    });

    // Audio
    p.regex("audio", r"7\.1[. ]?Atmos\b", "i", value("7.1 Atmos"), opts().remove());
    p.regex("audio", r"\b(?:mp3|Atmos|DTS(?:-HD)?|TrueHD)\b", "i", lowercase(), opts());
    p.regex("audio", r"\bFLAC(?:\+?2\.0)?(?:x[2-4])?\b", "i", value("flac"), opts().remove());
    p.regex(
        "audio",
        r"\bE-?AC-?3(?:[. -]?[256]\.[01])?",
        "i",
        value("eac3"),
        opts().remove().skip_if_already_found(false),
    );
    p.regex(
        "audio",
        r"\bAC-?3(?:[.-]5\.1|x2\.?0?)?\b",
        "i",
        value("ac3"),
        opts().remove().skip_if_already_found(false),
    );
    p.regex(
        "audio",
        r"\b5\.1(?:x[2-4]+)?\+2\.0(?:x[2-4])?\b",
        "i",
        value("2.0"),
        opts().remove().skip_if_already_found(false),
    );
    p.regex(
        "audio",
        r"\b2\.0(?:x[2-4]|\+5\.1(?:x[2-4])?)\b",
        "i",
        value("2.0"),
        opts().remove().skip_if_already_found(false),
    );
    p.regex("audio", r"\b5\.1ch\b", "i", value("ac3"), opts().remove().skip_if_already_found(false));
    p.regex("audio", r"\bDD5[. ]?1\b", "i", value("dd5.1"), opts().remove());
    p.regex("audio", r"\bQ?AAC(?:[. ]?2[. ]0|x2)?\b", "", value("aac"), opts().remove());
    p.regex("audioChannels", r"\[[257][.-][01]]", "", lowercase(), opts().remove());
    p.regex("audioChannels", r"(?:\b(?:DTS(?:-?HD)?(?:[ .-]?MA)?|E?AC-?3|AAC|TrueHD|FLAC|Opus|L?PCM)[ .-]?|[[(])([257]\.[01])(?:ch)?(?=[\]) .,_-]|$)", "i", none(), opts().remove());

    // Container
    p.regex("container", r"\.?[[(]?\b(?<!\p{L})(MKV|AVI|MP4|WMV|MPG|MPEG)\b(?!\p{L})[\])]?", "iu", lowercase(), opts());

    // Volumes
    p.regex("volumes", r"\bvol(?:s|umes?)?[. -]*(?:\d{1,2}[., +/\\&-]+)+\d{1,2}\b", "i", range(), opts().remove());
    let volume = Regex::known(r"\bvol(?:ume)?[. -]*(\d{1,2})", "i");
    p.function("volumes", move |c| {
        let start = c.matched.get("year").map_or(0, |year| year.match_index);
        let title = slice_from(c.title, start);
        let m = volume.find(title)?;
        let raw_match = title[m.range()].to_owned();
        // like the js library the index is relative to the year, not to the title
        let index = utf16_index(title, m.start());
        c.matched.insert("volumes".to_owned(), MatchInfo { raw_match: None, match_index: index });
        if let Some(volumes) = array(integer())(group(&m, title, 1).unwrap_or_default(), None) {
            c.result.set("volumes", volumes);
        }
        Some(HandlerMatch { raw_match: Some(raw_match), match_index: index, remove: true, skip_from_title: false })
    });

    // Season
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?:\bseasons?[. ]?[([]?|\b(?=s\d))((?:s?\d{1,2})[. ]*-[. ]*s?\d{1,2})[)\]]?[. ]*[([]\d{1,2}[. ]?of[. ]?\d{1,2}[)\]]", "i", range(), opts().remove());
    p.regex(
        "seasons",
        r"(?:complete\W|seasons?\W(?:\d{1,2}[. ]*-[. ]*\d{1,2}[. ]+)?|\W|^)((?:s\d{1,2}[., +/\\&-]+)+s\d{1,2}\b)",
        "i",
        range(),
        opts().remove(),
    );
    p.regex("seasons", r"(?:complete\W|seasons?\W|\W|^)[([]?(s\d{2,}-\d{2,}\b)[)\]]?", "i", range(), opts().remove());
    p.regex("seasons", r"(?:complete\W|seasons?\W|\W|^)[([]?(s[1-9]-[2-9]\b)[)\]]?", "i", range(), opts().remove());
    p.regex("seasons", r"(?:[^\wЀ-ӿ-]|^)(\d{1,2}-\d{1,2})[xх](?!26[45]\b)\d{1,3}\b", "i", range(), opts());
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?:seasons?|[Сс]езони?|sezon|temporadas?|stagioni|s[äæ]song|sesong|staffel)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}(?: ?[,/\\&][, /\\&]*(?:and )?| ))+\d{1,2}\b)[)\]]?", "i", range(), opts().remove());
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?:seasons|[Сс]езони?|sezon|temporadas?|stagioni|s[äæ]song|sesong|staffel)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}[. -]+)+0?[1-9]\d?\b)(?![. _]?(?:seri|сери))[)\]]?", "i", range(), opts().remove());
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?<!\bopen[ ._])season[. ]?[([]?((?:\d{1,2}[. -]+)+[1-9]\d?\b)[)\]]?(?!.*\.\w{2,4}$)", "i", range(), opts().remove());
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?\b(?:seasons?|s[äæ]song|sesong|staffel)[. -]?(\d{1,2}[. -]?(?:to|thru|and|till|\+|:)[. -]?\d{1,2})\b", "i", range(), opts().remove());
    p.regex("seasons", r"\bfirst[. ](\d{1,2})[. ]seasons\b", "i", range_up_to(), opts());
    p.regex("seasons", r"\bS((?:19|20)\d{2})E0*[1-9]\d{0,2}\b", "i", array(integer()), opts());
    p.regex("seasons", r"\bS(\d{3})[. ]?E\d{1,4}\b", "i", array(integer()), opts());
    p.regex("seasons", r"(\d{1,2})(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?)(?:\W?\D|$)", "i", array(integer()), opts());
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete\W)?(?<!\bopen[ ._])(?:saison|seizoen|sezona|sezon(?:SO?)?|stagione|seasons?|series|temp(?:orada)?|sesong|s[äæ]song|sæson|staffel):?[. ]?(\d{1,2})(?!\d|(?<=seasons\W?\d+)[. ]?(?:episodes?|eps)\b(?![. -]*\d))", "i", array(integer()), opts());
    p.regex("seasons", r"[Сс]езон:?[. _]?№?(\d{1,2})(?!\d)", "i", array(integer()), opts());
    p.regex("seasons", r"(?:\D|^)(\d{1,2})Â?[°ºªa]?[. ]*temporada", "i", array(integer()), opts().remove());
    p.regex("seasons", r"(?<![a-z])t(\d{1,3})[ex]+", "i", array(integer()), opts().remove());
    p.regex("seasons", r"(?<=\s)(?<!\bby\s)t(\d{1,2})$", "i", array(integer()), opts().remove());
    p.regex("seasons", r"(?:(?:\bthe\W)?\bcomplete)?(?!(?<!s\d{1,2})-so?[1-9]\d(?:\.[a-z\d]{2,4}$|\]))(?:[^\w'’]|^)so?([01]?[0-5]?[1-9]|[1-5]0)(?:[\Wex]|\d{2}\b)", "i", array(integer()), opts().skip_if_already_found(false));
    p.regex("seasons", r"(?:[^\w-]|^)s(\d{1,2})$", "i", array(integer()), opts());
    p.regex(
        "seasons",
        r"(?:\W|^)s-(\d{1,2})(?=[. -]*e[. -]?\d{1,3}\b|[. ]+(?:d(?:isc)?[. -]?\d|dvd)\b|[. ]*$)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex(
        "seasons",
        r"(?:so?|t)(\d{1,2})[. ]?[xх-]?[. ]?(?:e|x|х|ep|-|\.)[. ]?\d{1,4}(?:[abc]|v0?[1-4]|\D|$)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex(
        "seasons",
        r"(?:(?:\bthe\W)?\bcomplete\W)?(?:\W|^)(\d{1,2})[. ]?(?:st|nd|rd|th)[. ]*season",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("seasons", r"第[ ]?([0-9０-９]{1,2}|[一二三四五六七八九十]{1,3})[ ]?(?:季|期(?=.*(?:[\u3040-\u30ff]|第\d+[話话])))(?:[(（][^)）]*[)）])?", "", array(cjk_integer()), opts().remove().skip_from_title());
    p.regex(
        "seasons",
        r"(?<=\[)(\d{1,2})(?:st|nd|rd|th)(?=[. ]*-[. ]*\d{1,4}\])",
        "i",
        array(integer()),
        opts().remove(),
    );
    p.regex("seasons", r"^(\d{1,2})[. ]season$", "i", array(integer()), opts());
    p.regex("seasons", r"(?:\D|^)(\d{1,2})[Xxх](?!26[45]\b)\d{1,3}(?:\D|$)", "", array(integer()), opts());
    p.regex(
        "seasons",
        r"^x(?!26[45]\b|(?:480|576|720|1080|2160)\b)(\d{1,2})\d{2}(?= - )",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("seasons", r"\bSn([1-9])(?:\D|$)", "", array(integer()), opts());
    p.regex("seasons", r"\bv(\d{1,2})e\d{1,3}\b", "i", array(integer()), opts());
    p.regex(
        "seasons",
        r"(?<=[a-z]\.)(?<!sezon\.)(\d{1,2})\.\(?\d{1,3}(?:-\d{1,3})?\.ser[ijya]{0,3}(?=\.)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("seasons", r"(?<=[a-z]\.)(\d{1,2})\.ser\.\d{1,3}(?=\.)", "i", array(integer()), opts());
    p.regex("seasons", r"(?:^|[ ._-])(\d{1,2})EP?\d{2,3}(?=[ ._-]|$)", "i", array(integer()), opts());
    p.regex("seasons", r"\bs\.(\d{1,2})[ .]ep\.\d{1,4}", "i", array(integer()), opts());
    p.regex("seasons", r"[[(](\d{1,2})\.(?!5[)\]])\d{1,3}[)\]]", "", array(integer()), opts());
    p.regex("seasons", r"-\s?(\d{1,2})\.\d{2,3}\s?-", "", array(integer()), opts());
    p.regex(
        "seasons",
        r"^(\d{1,2})\.\d{2,3} - ",
        "",
        array(integer()),
        opts().skip_if_before(&["year, source", "resolution"]),
    );
    p.regex(
        "seasons",
        r"^(\d{1,2})\.\d{2,3}[. ]+(?=[^\W\d]|[\u00c0-\u024f\u0400-\u04ff])",
        "",
        array(integer()),
        opts().skip_if_before(&["year"]),
    );
    p.regex("seasons", r"(?:^|\/)([1-9])(?=-0\d\b(?!-\d))", "", array(integer()), opts());
    p.regex(
        "seasons",
        r"(?:^|\/)(?!20-20)(\d{1,2}(?=-\d{2}\b(?!-\d))|(\d)(?=-(?:1\d{2}|\2\d{2})(?=[ ._]|$)))",
        "",
        array(integer()),
        opts().skip_if_before(&["year"]),
    );
    p.regex("seasons", r"[^\w-](\d{1,2})-\d{2}(?=\.\w{2,4}$)", "", array(integer()), opts());
    p.regex(
        "seasons",
        r"(?<!\bEp?(?:isode)? ?\d+\b.*)\b(\d{2})[ ._]\d{2}(?:.F)?\.\w{2,4}$",
        "",
        array(integer()),
        opts(),
    );
    p.regex("seasons", r"\bEp(?:isode)?\W+(?:#|(?!\d{1,2}\.5\b))(\d{1,2})\.\d{1,3}\b", "i", array(integer()), opts());

    // adds single season info if its there"s only single season
    p.function("season", |c| {
        if let Some(&[season]) = c.result.seasons() {
            c.result.set("season", season);
        }
        None
    });

    // Episode
    p.regex("episodes", r"\bS\d{1,2} \d{1,3}-(\d{3})(?=\.\w{2,4}$)", "i", array(integer()), opts());
    p.regex(
        "episodes",
        r"(?<=[a-z]\.)(?<!sezon\.)\d{1,2}\.(\d{1,3}(?:-\d{1,3})?)\.ser[ijya]{0,3}(?=\.)",
        "i",
        range(),
        opts(),
    );
    p.regex("episodes", r"(?<=[a-z]\.)\d{1,2}\.ser\.(\d{1,3})(?=\.)", "i", array(integer()), opts());
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)e[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|e){1,2}[ .]?\d{1,3})+)(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex("episodes", r"\bs\d{1,2}[ .-]?e(\d{1,3}-?e\d{1,3})(?:\W|$)", "i", list(), opts());
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)ep[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|ep){1,2}[ .]?\d{1,3})+)(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)\d+[xх][ .]?[([]?(\d{1,3}(?:[ .]?[xх][ .]?\d{1,3})+)(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)(?:episodes?|[Сс]ерии:?)[ .]?[([]?(\d{1,3}(?:[ .+]*[&+][ .]?\d{1,3})+)(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex("episodes", r"[([]?(?:\D|^)(\d{1,3}[ .]?ao[ .]?\d{1,3})[)\]]?(?:\W|$)", "i", range(), opts());
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)(?:e|eps?|episodes?|[Сс]ерии:?|\d+[xх])[ .]*[([]?(\d{1,3}(?:-\d{1,3})+)(?!\.\d)(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)(?:eps?|episodes?)[ .]*[([]?(?!(?:19|20)\d{2}-(?:19|20)\d{2}\b)(\d{3,4}-\d{3,4})(?!\.\d)(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?:[\W\d]|^)(?:eps?|episodes?)[ .]*[([]?(\d{1,3}[ .]+(?:to|thru|till)[ .]+\d{1,3})(?:\W|$)",
        "i",
        range(),
        opts(),
    );
    p.regex("episodes", r"[Сс]ерии:?\s+(\d{1,4})\s+из\s+\d{1,4}", "", range_up_to(), opts());
    p.regex("episodes", r"\[(\d{1,4})\s+из\s+\d{1,4}]", "", range_up_to(), opts());
    p.regex("episodes", r"(?<=\s)(\d{1,4})\/\1(?=\s*\[)", "", range_up_to(), opts());
    p.regex("episodes", r"(?:[\W\d]|^)(?:eps?|episodes?)[ .]*[([]?(\d{1,2}[ .]+-[ .]+\d{1,3}|\d{3,4}[ .]+-[ .]+\d{3,4})(?!\.\d|[ .]*-)(?:\W|$)", "i", range(), opts());
    p.regex("episodes", r"(?:\W|^)(\d{1,3}(?:[ .]*~[ .]*\d{1,3})+)(?:\W|$)", "i", range(), opts());
    p.regex(
        "episodes",
        r"(?:so?|t)\d{1,2}[. ]?[xх-]?[. ]?(?:e|x|х|ep)[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("episodes", r"(?:so?|t)\d{1,2}\s?[-.]\s?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)", "i", array(integer()), opts());
    p.regex("episodes", r"\b(?:so?|t)\d{2}(\d{2})\b", "i", array(integer()), opts());
    p.regex("episodes", r"\bS(?:19|20)\d{2}E(0*[1-9]\d{0,2})\b", "i", array(integer()), opts());
    p.regex("episodes", r"\bS\d{3}[. ]?E(\d{1,4})\b", "i", array(integer()), opts());
    p.regex("episodes", r"\bv\d{1,2}e(\d{1,3})\b", "i", array(integer()), opts());
    p.regex("episodes", r"\bs\.\d{1,2}[ .]ep\.(\d{1,4})", "i", array(integer()), opts());
    p.regex("episodes", r"-\s(\d{1,3}[ .]*-[ .]*\d{1,3})(?!-\d)(?:\W|$)", "i", range(), opts());
    p.regex("episodes", r"s\d{1,2}\s?\((\d{1,3}[ .]*-[ .]*\d{1,3})\)", "i", range(), opts());
    p.regex("episodes", r"^(01-02|02-03|03-04|04-05|05-06|06-07|07-08|08-09|09-10)(?=[ ._])", "", range(), opts());
    p.regex("episodes", r"(?:^|\/)[1-9]-(0\d)\b(?!-\d)", "", array(integer()), opts());
    p.regex(
        "episodes",
        r"(?:^|\/)(?!20-20)\d{1,2}-(\d{2})\b(?!-\d)",
        "",
        array(integer()),
        opts().skip_if_before(&["year"]),
    );
    p.regex(
        "episodes",
        r"(?:^|\/)(?:1-1|2-2|3-3|4-4|5-5|6-6|7-7|8-8|9-9)(\d{2})(?=[ ._]|$)",
        "",
        array(integer()),
        opts().skip_if_before(&["year"]),
    );
    p.regex("episodes", r"(?:^|\/)\d-(1\d{2})(?=[ ._]|$)", "", array(integer()), opts().skip_if_before(&["year"]));
    p.regex("episodes", r"(?<!\d-)\b\d{1,2}-(\d{2})(?=\.\w{2,4}$)", "", array(integer()), opts());
    p.regex(
        "episodes",
        r"(?<=^\[.+].+)(?:[. ]+episode)?[. ]+-[. ]+(\d{1,4})[. ]+(?=\W)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?<!(?:seasons?|series|[Сс]езони?)\W*)(?:[ .([-]|^)(\d{1,3}(?:[ .]?[,&+~][ .]?\d{1,3})+)(?:[ .)\]-]|$)",
        "i",
        range(),
        opts(),
    );
    p.regex(
        "episodes",
        r"^(?!20-20)(\d{1,3}(?:-\d{1,3})+)(?!\.\d)(?:[ .)(\]]|$)",
        "i",
        range(),
        opts().skip_if_before(&["year"]),
    );
    p.regex("episodes", r"(?<!^\d{1,3}(?:-\d{1,3})+[ .)(\]-].*)(?<!(?:seasons?|[Сс]езони?)\W*)(?!20-20)(?:[ .([]|(?<!^\d{1,3}(?:-\d{1,3})*)-)(\d{1,3}(?:-\d{1,3})+)(?!\.\d)(?:[ .)(\]]|-\D|$)", "i", range(), opts());
    p.regex("episodes", r"\bEp(?:isode)?\W+(?:#|(?!\d{1,2}\.5\b))\d{1,2}\.(\d{1,3})\b", "i", array(integer()), opts());
    p.regex("episodes", r"(?:\b(?<!\p{L})(?!(?<=(?:^|[^a-z])[a-z]\.)[ée]\.\s*[1-9])[ée]p?(?:isode)?|[Ээ]пизод|[Сс]ер(?:ии|ия|\.)?|seri[jy]a|caa?p(?:itulo)?|epis[oó]dio|epizoda|odcinek|avsnitt|aflevering|folge|jakso|puntata)[. ]?[-:#№]?[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\W|$)", "iu", array(integer()), opts());
    p.regex("episodes", r"\d{1,2}\.sezon\.(\d{1,3}\.\d{1,3})\.ser", "i", range(), opts());
    p.regex("episodes", r"\b(\d{1,3}-\d{1,3})[ ._]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)", "i", range(), opts());
    p.regex("episodes", r"(?<=\d[ ._]?[Сс]езон[ ._]+)(\d{1,4}-\d{1,4})(?![\d-])", "", range(), opts());
    p.regex(
        "episodes",
        r"\b(\d{1,3})(?:-?(?:я|ja|ya))?[ ._-]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?:\D|^)\d{1,2}[. ]?[Xxх][. ]?(?!26[45]\b)(\d{1,3})(?:[abc]|v0?[1-4]|\D|$)",
        "",
        array(integer()),
        opts(),
    );
    p.regex(
        "episodes",
        r"^x(?!26[45]\b|(?:480|576|720|1080|2160)\b)\d{1,2}(\d{2})(?= - )",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("episodes", r"[[(]\d{1,2}\.(?!5[)\]])(\d{1,3})[)\]]", "", array(integer()), opts());
    p.regex("episodes", r"(?:^|[ ._-])\d{1,2}EP?(\d{2,3})(?=[ ._-]|$)", "i", array(integer()), opts());
    p.regex("episodes", r"\b[Ss](?:eason\W?)?\d{1,2}[ .](\d{1,2})\b", "", array(integer()), opts());
    p.regex(
        "episodes",
        r"\bseries[ .]?\d{1,2}(?:, |\.| - ?|- | )(\d{1,3})(?=[ .]+[^\W\d])",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("episodes", r"-\s?\d{1,2}\.(\d{2,3})\s?-", "", array(integer()), opts());
    p.regex(
        "episodes",
        r"^\d{1,2}\.(\d{2,3}) - ",
        "",
        array(integer()),
        opts().skip_if_before(&["year, source", "resolution"]),
    );
    p.regex(
        "episodes",
        r"^\d{1,2}\.(\d{2,3})[. ]+(?=[^\W\d]|[\u00c0-\u024f\u0400-\u04ff])",
        "",
        array(integer()),
        opts().skip_if_before(&["year"]),
    );
    p.regex(
        "episodes",
        r"(?<=\D|^)(?!(?<=\d\.)5\b)(\d{1,3})[. ]?(?:of|из|iz)[. ]?\d{1,3}(?=\D|$)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex("episodes", r"\b\d{2}[ ._-](\d{2})(?:.F)?\.\w{2,4}$", "", array(integer()), opts());
    p.regex("episodes", r"(?<!^)\[(\d{2,3})](?!(?:\.\w{2,4})?$)", "", array(integer()), opts());
    p.regex("episodes", r"\bodc[. ]+(\d{1,3})\b", "i", array(integer()), opts());
    p.regex("episodes", r"(?<![a-z\d])(\d{1,3})ep(?![a-z\d]|[ ._-]?\d)", "i", array(integer()), opts());
    p.regex(
        "episodes",
        &[
            r"(?<=^(?:\[[^\]]+\])?[a-z]{3,})(?<!part|pt|cd|dvd|dis[ck]|vol|title|season)(0\d{1,2})(?=\.(?:",
            VIDEO_EXTENSIONS,
            "|",
            SUBTITLE_EXTENSIONS,
            ")$)",
        ]
        .concat(),
        "i",
        array(integer()),
        opts(),
    );
    p.regex("episodes", r"\b(\d{1,4})\.?[ ._-]*b[öo]l[üu]m\b", "i", array(integer()), opts());
    p.regex("episodes", r"\bb[öo]l[üu]m[ ._-]*(\d{1,4})\b", "i", array(integer()), opts());
    p.regex(
        "episodes",
        r"[ .]-[ .]\d{1,3}\.5[ ._]+(?:OVA|OAV|OAD|ONA)[ ._]*E?0*(\d{1,2})\b",
        "i",
        array(integer()),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?<=(?<!\d,\d+[ ._]*)\b(?:OVA|OAV|OAD|ONA)[ ._]*)E?0*(\d{1,2})(?:v\d)?\b(?![ .]*[-~&+.]\d)",
        "i",
        array(integer()),
        opts(),
    );
    p.regex(
        "episodes",
        r"(?<=(?:^|[ ._-])SP)0*(\d{1,2})(?=[ ._]*(?:[[(]|\.\w{2,4}$|$))",
        "i",
        array(integer()),
        opts(),
    );

    p.regex(
        "episodes",
        r"^(\d{3,4})[. ]+-[. ]+(?=[^\W\d])",
        "",
        array(integer()),
        opts().remove().skip_if_before(&["year"]),
    );

    p.regex("episodes", &marked_half_episode(), "i", array(integer()), opts());
    p.regex("episodes", &bare_half_episode(), "i", array(integer()), opts().skip_if_before(&["year"]));

    // can be both absolute episode and season+episode in format 101
    let separated_episode = Regex::known(
        r"(?<!movie\W*|film\W*|^)(?:[ .]+-[ .]+\d{1,4}(?:a|b|v\d)?[ .]*[([][ .]*(?!\d+-[a-z]+\))|[ .]+-[ .]+|[([][ .]*(?!\d+-[a-z]+\)))(\d{1,4})(?![. _]?(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?\b)(?![. _]?finali))(?:a|b|v\d|\.\d)?(?:\W|$)(?!movie|film|фильм|\d+)",
        "gi",
    );
    let leading_episode = Regex::known(
        r"^(?:[([-][ .]?)?(\d{1,4})(?![. _]?(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?\b)(?![. _]?finali))(?:a|b|v\d)?(?!\Wmovie|\Wfilm|\Wфильм|-\d)(?:\W|$)",
        "i",
    );
    p.function("episodes", move |c| {
        if has(c.result, "episodes") {
            return None;
        }
        let indexes = |fields: &[&str]| -> Vec<usize> {
            fields
                .iter()
                .filter_map(|&field| c.matched.get(field))
                .map(|m| m.match_index)
                .filter(|&index| index > 0)
                .collect()
        };
        let start_index = indexes(&["year", "seasons"]).into_iter().min().unwrap_or(0);
        let end_index =
            indexes(&["resolution", "source", "codec", "audio"]).into_iter().fold(utf16_len(c.title), usize::min);
        let end_index = byte_index(c.title, end_index);
        let beginning_title = &c.title[..end_index];
        let middle_title = &c.title[byte_index(c.title, start_index).min(end_index)..end_index];

        // try to match the episode inside the title with a separator, if not found include the start of the title as well
        let (raw_match, episode) = separated_episode
            .find_iter(beginning_title)
            .last()
            .map(|m| (&beginning_title[m.range()], group(&m, beginning_title, 1)))
            .or_else(|| {
                leading_episode.find(middle_title).map(|m| (&middle_title[m.range()], group(&m, middle_title, 1)))
            })?;
        let digits: String = episode.unwrap_or_default().chars().filter(char::is_ascii_digit).collect();
        c.result.set("episodes", vec![parse_int(&digits)?]);
        Some(HandlerMatch::at(c.title.find(raw_match).map_or(0, |byte| utf16_index(c.title, byte))))
    });
    p.regex(
        "episodes",
        &[
            r"(?<=^\[[^\]]+\][ .]?\S.*?(?<!\b(?:movie|film|pel[ií]cula|ova|oad|ona|part|pt|vol|volume|chapter|season|series)[ .]*)[ .](?<!\d\.))(\d{2,3})(?=[ .]+-[ .]+(?!S\d|.*\b(?:movie|film|pel[ií]cula)\b)\p{L}.*\.(?:",
            VIDEO_EXTENSIONS, ")$)",
        ]
        .concat(),
        "iu",
        array(integer()),
        opts().skip_if_before(&["year"]),
    );

    let season_dot_episode = Regex::known(r"(?<=[^\d\s][.-]|\s\.)(\d{1,2})\.(\d{2,3})(?=\.\w{2,4}$)", "");
    p.function("episodes", move |c| {
        if has(c.result, "episodes") || has(c.result, "seasons") || has(c.result, "year") {
            return None;
        }
        let m = season_dot_episode.find(c.title)?;
        let season = parse_int(group(&m, c.title, 1)?)?;
        let episode = parse_int(group(&m, c.title, 2)?)?;
        c.result.set("seasons", vec![season]);
        c.result.set("season", season);
        c.result.set("episodes", vec![episode]);
        Some(HandlerMatch::at(utf16_index(c.title, m.start())))
    });

    let episode_after_year = Regex::known(
        r"^(?:[ .]| - ?|\. (?=\d{1,3} ?(?:\[|$)))(?!(?:480|576|720)\b)(\d{1,3})(?:v\d)?-?(?=[ .([-]|$)(?![ .,-]*\d|[ .]*(?:cd|dvd|dis[ck]|bits?|films?|movies?|[mg]b)\b)",
        "i",
    );
    let episode_after_resolution = Regex::known(r"^ (\d{3,4}) (?=[^\W\d])", "");
    p.function("episodes", move |c| {
        if has(c.result, "episodes") || has(c.result, "date") {
            return None;
        }
        let after = |field: &str, regex: &Regex| {
            let index = c.matched.get(field)?.match_index;
            let title = slice_from(c.title, index);
            regex.find(title).and_then(|m| group(&m, title, 1)).and_then(parse_int)
        };
        let episode = after("year", &episode_after_year).or_else(|| after("resolution", &episode_after_resolution));
        if let Some(episode) = episode {
            c.result.set("episodes", vec![episode]);
        }
        None
    });
    p.regex("episodes", r"(?<!(?:audio|subtitles?|season|movie|film)\W*|\b\d{1,3}(?: -|\.) .*|\([^)]*)(?:^|[ ._])#(\d{1,3})(?![.,]?\d)(?=[ ._\])]|$)", "i", array(integer()), opts().skip_if_after(&["date"]));
    p.regex("episodes", r"第(\d{1,4}(?:-\d{1,4})?)[話话集]", "", range(), opts());
    p.regex("episodes", r"\bphase[ .](\d{2,3})\b", "i", array(integer()), opts());

    // the parsed episode written as `12.5`: an episode inserted after 12, i.e. a recap or special
    p.function("halfEpisode", |c| {
        let Some(&[episode]) = c.result.episodes() else {
            return None;
        };
        let episodes = c.matched.get("episodes");
        let start = episodes.map_or(0, |m| m.match_index);
        let length =
            episodes.and_then(|m| m.raw_match.as_deref()).filter(|raw| !raw.is_empty()).map_or(usize::MAX, utf16_len);
        let title = slice_from(c.title, start);
        // the pattern is built per episode, so skip compiling it when the episode is never followed by `.5`
        if !title.contains(&format!("{episode}.5")) {
            return None;
        }
        let half_episode =
            Regex::known(&[r"(?<!\d\.?|imdb[ .]?|#)0*", &episode.to_string(), HALF_EPISODE_END].concat(), "i");
        if let Some(m) = half_episode.find(title) {
            let text = &title[m.range()];
            let starts_with_number = matches!(text.as_bytes(), [b'1'..=b'9', b'.', ..]);
            let index = utf16_index(title, m.start());
            if index < length && !(start + index == 0 && starts_with_number) {
                c.result.set("halfEpisode", true);
            }
        }
        None
    });

    // adds single season info if its there's only single season
    p.function("episode", |c| {
        if let Some(&[episode]) = c.result.episodes() {
            c.result.set("episode", episode);
        }
        None
    });

    p.regex("source", r"\b(?:WEB|web)\b(?![ .-]*DL)", "", value("WEB"), opts().remove().skip_if_first());

    p.regex("complete", r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b", "i", boolean(), opts());
    p.regex(
        "complete",
        r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bmini[ .-]?series\b",
        "i",
        boolean(),
        opts(),
    );
    p.regex(
        "complete",
        r"(?:\bthe\W)?\b(?:complete|full|all)\b.*\b(?:series|seasons|collection|episodes|set|pack|movies)\b",
        "i",
        boolean(),
        opts(),
    );
    p.regex("complete", r"\b(?:series|seasons|movies?)\b.*\b(?:complete|collection)\b", "i", boolean(), opts());
    p.regex(
        "complete",
        r"(?:\bthe\W)?\bultimate\b[ .]\bcollection\b",
        "i",
        boolean(),
        opts().skip_if_already_found(false),
    );
    p.regex("complete", r"\bcollection\b.*\b(?:set|pack|movies)\b", "i", boolean(), opts());
    p.regex("complete", r"\bint[ée]grale\b", "i", boolean(), opts().skip_if_already_found(false));
    p.regex(
        "complete",
        r"\b(?:la |primera |segunda )?(?:s[ée]rie|temporada)[ .]complet[ao]s?\b",
        "i",
        boolean(),
        opts().skip_if_already_found(false),
    );
    p.regex("complete", r"[([{]seasons[)\]}]", "i", boolean(), opts().remove().skip_if_already_found(false));
    p.regex(
        "complete",
        r"[([{【](?:(?:full|complete|unofficial)[ ._-]?)?batch[)\]}】]",
        "i",
        boolean(),
        opts().remove().skip_from_title().skip_if_already_found(false),
    );
    p.regex(
        "complete",
        r"(?<![([{【][^)\]}】]*|\bbad[ ._-]?)\b(?:(?:full|complete|unofficial)[ ._-]?)?batch\b",
        "i",
        boolean(),
        opts().remove().skip_from_title().skip_if_already_found(false),
    );
    p.regex(
        "complete",
        r"\bcomplet[eo]s?\b(?=[ ._-]*(?:$|[[\]()]))",
        "i",
        boolean(),
        opts().skip_if_already_found(false),
    );
    p.regex("complete", r"\b(collection|completa)\b", "i", boolean(), opts().skip_from_title());
    p.regex("complete", r"\bkolekc(?:ja|e)\b(?:\Wfilm(?:y|ów|ow|ů|u)?)?", "i", boolean(), opts().remove());
    p.regex("complete", r"(?<![\p{L}\d])\d{1,3}[ .]+фильм(?:ов|а)(?!\p{L})", "iu", boolean(), opts().remove());
    p.regex(
        "complete",
        r"duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|heptalogy|anthology",
        "i",
        boolean(),
        opts().skip_if_already_found(false),
    );
    p.regex(
        "complete",
        r"\b(?:dualogy|(?:du[ao]|dy|tri|quadr[oi]|tetra|penta)log[ií]a)\b",
        "i",
        boolean(),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.regex(
        "complete",
        r"(?<!\p{L})(?:дилоги|трилоги|квадрологи|тетралоги|пенталоги|гексалоги|антологи)[яи](?!\p{L})",
        "iu",
        boolean(),
        opts().skip_if_already_found(false).skip_from_title(),
    );
    p.regex("complete", r"\bsaga\b", "i", boolean(), opts().skip_from_title());

    // Extras
    p.regex("extras", r"(?<=\S[ ._-]?)\[SP\d{1,2}]", "i", boolean(), opts().skip_if_already_found(false));
    p.regex(
        "extras",
        &[
            r"\s*\(?\s*(?:[+&]\s*",
            PACK_EXTRA,
            r"|\bplus\s+",
            PLUS_EXTRA,
            r")(?:\s*(?:[+&,]|and)\s*",
            PACK_EXTRA,
            r")*\)?",
        ]
        .concat(),
        "i",
        boolean(),
        opts().remove(),
    );

    // Trash for removal
    let trash = Regex::known(r"[[(](?:movie|anime|series?(?:[ .](?:anime|de[ .]tv|tv|dibujos))?)[)\]]", "i");
    p.function("trash", move |c| {
        let m = trash.find(c.title)?;
        Some(HandlerMatch {
            raw_match: Some(c.title[m.range()].to_owned()),
            match_index: utf16_index(c.title, m.start()),
            remove: true,
            skip_from_title: true,
        })
    });

    // Language
    p.regex(
        "languages",
        r"\bmulti(?:ple)?[ .-]*(?:su?$|sub\w*|dub\w*)\b|msub",
        "i",
        uniq_concat(value("multi subs")),
        opts().skip_if_already_found(false).remove(),
    );
    p.regex(
        "languages",
        r"\bmulti(?:ple)?[ .-]*(?:lang(?:uages?)?|audio|VF2)?\b",
        "i",
        uniq_concat(value("multi audio")),
        opts().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\btri(?:ple)?[ .-]*(?:audio|dub\w*)\b",
        "i",
        uniq_concat(value("multi audio")),
        opts().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\bdual[ .-]*(?:au?$|[aá]udio|line)\b",
        "i",
        uniq_concat(value("dual audio")),
        opts().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\bdual\b(?![ .-]*sub|!)",
        "i",
        uniq_concat(value("dual audio")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bengl?(?:sub[A-Z]*)?\b",
        "i",
        uniq_concat(value("english")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\beng?sub[A-Z]*\b",
        "i",
        uniq_concat(value("english")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bing(?:l[eéê]s)?\b",
        "i",
        uniq_concat(value("english")),
        opts().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\benglish\W+(?:subs?|sdh|hi|dub\w*)\b",
        "i",
        uniq_concat(value("english")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bEN\b",
        "i",
        uniq_concat(value("english")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("english?", "english", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\b(?:JP|JAP|JPN)\b",
        "i",
        uniq_concat(value("japanese")),
        opts().skip_if_already_found(false),
    );
    p.language_name("japanese|japon[eê]s", "japanese", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\b(?:KOR|kor[ .-]?sub)\b",
        "i",
        uniq_concat(value("korean")),
        opts().skip_if_already_found(false),
    );
    p.language_name("korean|coreano", "korean", opts().skip_if_first());
    p.regex(
        "languages",
        r"\b(?:traditional\W*chinese|chinese\W*traditional)(?:\Wchi)?\b",
        "i",
        uniq_concat(value("taiwanese")),
        opts().skip_if_already_found(false).remove(),
    );
    p.regex("languages", r"\bzh-hant\b", "i", uniq_concat(value("taiwanese")), opts().skip_if_already_found(false));
    p.standalone(
        "languages",
        r"\b(?:mand[ae]rin|ch[sn])\b",
        "i",
        uniq_concat(value("chinese")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bCH[IT]\b",
        "",
        uniq_concat(value("chinese")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("chinese|chin[eê]s", "chinese", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\bchi\b",
        "i",
        uniq_concat(value("chinese")),
        opts().skip_if_first().skip_if_already_found(false),
    );
    p.regex("languages", r"\bzh-hans\b", "i", uniq_concat(value("chinese")), opts().skip_if_already_found(false));
    p.standalone(
        "languages",
        r"\bFR(?:a|e|anc[eê]s)?\b",
        "i",
        uniq_concat(value("french")),
        opts().skip_if_already_found(false),
    );
    p.language_name("french", "french", opts());
    p.standalone(
        "languages",
        r"\b(?:Truefrench|VF[FI])\b",
        "i",
        uniq_concat(value("french")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?:VOST(?:FR?|A)?|SUBFRENCH)\b",
        "i",
        uniq_concat(value("french")),
        opts().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\bspanish\W?latin|american\W*(?:spa|esp?)",
        "i",
        uniq_concat(value("latino")),
        opts().skip_from_title().skip_if_already_found(false).remove(),
    );
    p.standalone(
        "languages",
        r"\b(?:audio.)?lat(?:i|ino)?\b",
        "i",
        uniq_concat(value("latino")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?:audio.)?(?:ESP(?![ .]+-[ .]+\d)|spa|(en[ .]+)?espa[nñ]ola?|castellano)\b",
        "i",
        uniq_concat(value("spanish")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bes(?=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})\b",
        "i",
        uniq_concat(value("spanish")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?<=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})es\b",
        "i",
        uniq_concat(value("spanish")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?<=[ .,/-]+[A-Z]{2}[ .,/-]+)es(?=[ .,/-]+[A-Z]{2}[ .,/-]+)\b",
        "i",
        uniq_concat(value("spanish")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bes(?=\.(?:ass|ssa|srt|sub|idx)$)",
        "i",
        uniq_concat(value("spanish")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\bspanish\W+(?:subs?|line|alta|calidad)\b",
        "i",
        uniq_concat(value("spanish")),
        opts().skip_if_already_found(false),
    );
    p.language_name("spanish|espanhol", "spanish", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\b(?:p[rt]|en|port)[. (\\/-]*BR\b",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_if_already_found(false).remove(),
    );
    p.standalone(
        "languages",
        r"\bbr(?:a|azil|azilian)\W+(?:pt|por)\b",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_if_already_found(false).remove(),
    );
    p.standalone(
        "languages",
        r"\b(?:leg(?:endado|endas?)?|dub(?:lado)?|portugu[eèê]se?)[. -]*BR\b",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bleg(?:endado|endas?)\b",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_if_already_found(false),
    );
    p.language_name("portugu[eèê]s[ea]?", "portuguese", opts());
    p.standalone(
        "languages",
        r"\bPT[. -]*(?:PT|ENG?|sub(?:s|titles?))\b",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bpt(?=\.(?:ass|ssa|srt|sub|idx)$)",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bpor\b",
        "i",
        uniq_concat(value("portuguese")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone("languages", r"\bITA\b", "i", uniq_concat(value("italian")), opts().skip_if_already_found(false));
    p.standalone(
        "languages",
        r"\b(?<!w{3}\.\w+\.)IT(?=[ .,/-]+(?:[a-zA-Z]{2}[ .,/-]+){2,})\b",
        "",
        uniq_concat(value("italian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bit(?=\.(?:ass|ssa|srt|sub|idx)$)",
        "i",
        uniq_concat(value("italian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("italiano?", "italian", opts().skip_if_first());
    p.regex(
        "languages",
        r"\bgreek[ .-]*(?:audio|lang(?:uage)?|subs?(?:titles?)?)\b",
        "i",
        uniq_concat(value("greek")),
        opts().skip_if_already_found(false),
    );
    p.language_name("greek", "greek", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\b(?:GER|DEU)\b",
        "i",
        uniq_concat(value("german")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bde(?=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})\b",
        "i",
        uniq_concat(value("german")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?<=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})de\b",
        "i",
        uniq_concat(value("german")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?<=[ .,/-]+[A-Z]{2}[ .,/-]+)de(?=[ .,/-]+[A-Z]{2}[ .,/-]+)\b",
        "i",
        uniq_concat(value("german")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bde(?=\.(?:ass|ssa|srt|sub|idx)$)",
        "i",
        uniq_concat(value("german")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("german|alem[aã]o", "german", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\bRUS?\b(?<!\w-RUS?(?=\s+(?!sub|dub|audio)[a-z{]))",
        "i",
        uniq_concat(value("russian")),
        opts().skip_if_already_found(false),
    );
    p.language_name("russian|russo", "russian", opts().skip_if_first());
    p.standalone("languages", r"\bUKR\b", "i", uniq_concat(value("ukrainian")), opts().skip_if_already_found(false));
    p.language_name("ukrainian", "ukrainian", opts().skip_if_first());
    p.standalone("languages", r"\bhin(?:di)?\b", "i", uniq_concat(value("hindi")), opts().skip_if_already_found(false));
    p.standalone(
        "languages",
        r"\b(?:(?<!w{3}\.\w+\.)tel(?!\W*aviv)|telugu)\b",
        "i",
        uniq_concat(value("telugu")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bt[aâ]m(?:il)?\b",
        "i",
        uniq_concat(value("tamil")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?<!YTS\.)LT\b",
        "",
        uniq_concat(value("lithuanian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("lithuanian", "lithuanian", opts().skip_if_first());
    p.language_name("latvian", "latvian", opts().skip_if_first());
    p.language_name("estonian", "estonian", opts().skip_if_first());
    p.regex(
        "languages",
        r"\b(?:PLDUB|Dubbing.PL|Lektor.PL|Film.Polski)\b",
        "i",
        uniq_concat(value("polish")),
        opts().skip_if_already_found(false).remove(),
    );
    p.regex(
        "languages",
        r"\b(?:Napisy.PL|PLSUB(?:BED)?)\b",
        "i",
        uniq_concat(value("polish")),
        opts().skip_if_already_found(false).remove(),
    );
    p.standalone(
        "languages",
        r"\b(?:(?<!w{3}\.\w+\.)PL|pol)\b",
        "i",
        uniq_concat(value("polish")),
        opts().skip_if_already_found(false),
    );
    p.language_name("polish|polon[eê]s|polaco", "polish", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\bCZ[EH]?\b",
        "i",
        uniq_concat(value("czech")),
        opts().skip_if_first().skip_if_already_found(false),
    );
    p.language_name("czech", "czech", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\bslo(?:vak|vakian|subs|[\]_)]?\.\w{2,4}$)\b",
        "i",
        uniq_concat(value("slovakian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bHU\b",
        "",
        uniq_concat(value("hungarian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bHUN\b",
        "i",
        uniq_concat(value("hungarian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("hungarian", "hungarian", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\bROM\b",
        "i",
        uniq_concat(value("romanian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("romanian", "romanian", opts().skip_from_title());
    p.regex(
        "languages",
        r"\bRO(?=[ .,/-]*(?:[A-Z]{2}[ .,/-]+)*sub)",
        "i",
        uniq_concat(value("romanian")),
        opts().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bbul\b",
        "i",
        uniq_concat(value("bulgarian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("bulgarian", "bulgarian", opts().skip_from_title());
    p.standalone("languages", r"\bsrp\b", "i", uniq_concat(value("serbian")), opts().skip_if_already_found(false));
    p.language_name("serbian", "serbian", opts());
    p.standalone("languages", r"\bHRV\b", "i", uniq_concat(value("croatian")), opts().skip_if_already_found(false));
    p.language_name("croatian", "croatian", opts());
    p.regex(
        "languages",
        r"\bHR(?=[ .,/-]*(?:[A-Z]{2}[ .,/-]+)*sub)\b",
        "i",
        uniq_concat(value("croatian")),
        opts().skip_if_already_found(false),
    );
    p.language_name("slovenian", "slovenian", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\b(?:(?<!w{3}\.\w+\.)NL|dut|holand[eê]s)\b",
        "i",
        uniq_concat(value("dutch")),
        opts().skip_if_already_found(false),
    );
    p.language_name("dutch", "dutch", opts().skip_from_title());
    p.language_name("flemish", "dutch", opts());
    p.standalone(
        "languages",
        r"\b(?:DK|danska|dansub|nordic)\b",
        "i",
        uniq_concat(value("danish")),
        opts().skip_if_already_found(false),
    );
    p.language_name("danish|dinamarqu[eê]s", "danish", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\bdan\b(?=.*\.(?:srt|vtt|ssa|ass|sub|idx)$)",
        "i",
        uniq_concat(value("danish")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\b(?:(?<!w{3}\.\w+\.|Sci-)FI|finsk|finsub|nordic)\b",
        "i",
        uniq_concat(value("finnish")),
        opts().skip_if_already_found(false),
    );
    p.language_name("finnish", "finnish", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\b(?:(?<!w{3}\.\w+\.)SE|swe|swesubs?|sv(?:ensk)?|nordic)\b",
        "i",
        uniq_concat(value("swedish")),
        opts().skip_if_already_found(false),
    );
    p.language_name("swedish|sueco", "swedish", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\b(?:NOR|norsk|norsub|nordic)\b",
        "i",
        uniq_concat(value("norwegian")),
        opts().skip_if_already_found(false),
    );
    p.language_name("norwegian|noruegu[eê]s|bokm[aå]l", "norwegian", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\b(?:nob|nor(?=[\]_)]?\.\w{2,4}$))\b",
        "i",
        uniq_concat(value("norwegian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("arabic|[aá]rabe", "arabic", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\bara\b",
        "i",
        uniq_concat(value("arabic")),
        opts().skip_if_first().skip_if_already_found(false),
    );
    p.regex(
        "languages",
        r"\barab.*(?:audio|lang(?:uage)?|sub(?:s|titles?)?)\b",
        "i",
        uniq_concat(value("arabic")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bar(?=\.(?:ass|ssa|srt|sub|idx)$)",
        "i",
        uniq_concat(value("arabic")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("turkish|turco", "turkish", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\btur\b",
        "i",
        uniq_concat(value("turkish")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("vietnamese", "vietnamese", opts().skip_from_title());
    p.standalone(
        "languages",
        r"\bvie(?=[\]_)]?\.\w{2,4}$)",
        "i",
        uniq_concat(value("vietnamese")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bind\b",
        "i",
        uniq_concat(value("indonesian")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("indonesian", "indonesian", opts().skip_from_title());
    p.language_name("thai|tailand[eê]s", "thai", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\b(THA|tha)\b",
        "",
        uniq_concat(value("thai")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("malay", "malay", opts().skip_if_first());
    p.standalone(
        "languages",
        r"\bmay(?=[\]_)]?\.\w{2,4}$)|(?<=subs?\([a-z,]+)may\b",
        "i",
        uniq_concat(value("malay")),
        opts().skip_if_first().skip_if_already_found(false),
    );
    p.standalone(
        "languages",
        r"\bheb\b",
        "i",
        uniq_concat(value("hebrew")),
        opts().skip_from_title().skip_if_already_found(false),
    );
    p.language_name("hebrew|hebraico", "hebrew", opts().skip_from_title());
    p.language_name("persian|persa", "persian", opts().skip_from_title());

    // infer pt language based on season/episode naming
    let capitulo = Regex::known("capitulo|ao", "i");
    let dublado = Regex::known("dublado", "i");
    p.function("languages", move |c| {
        let languages = c.result.languages().map(<[String]>::to_vec);
        let has_iberian = languages.as_ref().is_some_and(|l| l.iter().any(|l| l == "portuguese" || l == "spanish"));
        if !has_iberian {
            let episodes = c.matched.get("episodes").and_then(|m| m.raw_match.as_deref());
            if episodes.is_some_and(|raw| capitulo.is_match(raw)) || dublado.is_match(c.title) {
                let mut languages = languages.unwrap_or_default();
                languages.push("portuguese".to_owned());
                c.result.set("languages", languages);
            }
        }
        Some(HandlerMatch::at(0))
    });

    // Dubbed
    p.regex("dubbed", r"\b(?:DUBBED|dublado|dubbing|DUBS?)\b", "i", boolean(), opts());
    p.function("dubbed", |c| {
        let languages = c.result.languages().unwrap_or_default();
        if languages.iter().any(|l| l == "multi audio" || l == "dual audio") {
            c.result.set("dubbed", true);
        }
        Some(HandlerMatch::at(0))
    });

    // Group
    p.regex("group", r"- ?(?!\d+$|S\d+|\d+x|ep?\d+|[^[]+]$|(?:OVA|OAV|OAD|ONA|SP|Specials?)\d*(?:\.\w{2,4})?$|第\d+[話话集])([^\-. []+[^\-. [)\]\d][^\-. [)\]]*)(?:\[[\w.-]+])?(?=\.\w{2,4}$|$)", "i", none(), opts().skip_if_nothing_before());
    p.regex("group", r"^\[([^[\]]+)]", "", none(), opts());
    p.regex("group", r"\(([\w-]+)\)(?:$|\.\w{2,4}$)", "", none(), opts());
    let bracketed = Regex::known(r"^\[.+]$", "");
    p.function("group", move |c| {
        let group = c.matched.get("group");
        if let Some((index, raw)) = group.and_then(|g| Some((g.match_index, g.raw_match.as_deref()?))) {
            if bracketed.is_match(raw) {
                let end_index = index + utf16_len(raw);

                // remove anime group match if some other parameter is contained in it, since it's a false positive.
                if c.matched.values().any(|m| m.match_index != 0 && m.match_index < end_index) {
                    c.result.remove("group");
                }
            }
        }
        Some(HandlerMatch::at(0))
    });

    // Extension
    p.regex(
        "extension",
        &[r"\.(", VIDEO_EXTENSIONS, "|iso|vob|", SUBTITLE_EXTENSIONS, r"|idx|ttxt|txt|nfo|html)$"].concat(),
        "i",
        lowercase(),
        opts(),
    );

    if !exclude.is_empty() {
        p.0.remove_handlers(exclude);
    }
}

/// Mirrors js truthiness of `result[field]`.
fn has(result: &crate::ParsedTitle, field: &str) -> bool {
    result.get(field).is_some_and(Value::is_truthy)
}
