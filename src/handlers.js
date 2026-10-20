const { none, value, integer, cjkInteger, boolean, lowercase, uppercase, date, range, rangeUpTo, list, yearRange, array, uniqConcat } = require("./transformers");

const VIDEO_EXTENSIONS = "3g2|3gp|avi|flv|mkv|mk3d|mov|mp2|mp4|m4v|mpe|mpeg|mpg|mpv|webm|wmv|ogm|divx|ts|m2ts";
const SUBTITLE_EXTENSIONS = "sub|smi|srt|ssa|ass|vtt";
const PACK_EXTRA = "(?:\\d+\\s*)?(?:OVAs?|OAVs?|OADs?|ONAs?|Specials?|SPs?|Movies?|Films?|Extras?|Bonus(?:es)?|Novels?|Mangas?|OSTs?|Soundtracks?|Artbooks?|OP|ED|NC ?(?:OP|ED)s?)\\b";
const PLUS_EXTRA = "(?:\\d+\\s*)?(?:Extras?|Bonus(?:es)?)\\b";
const EPISODE_CODE = "(?:\\bS\\d{1,2}E\\d{1,3}|\\b\\d{1,2}x\\d{1,3}\\b)";
const HALF_EPISODE_END = "\\.5(?:v\\d)?(?=[ ._\\-)\\][]|$)(?![ .-]\\d)(?![ ._]?(?:gb|mb|ch|kbps|fps|bit|x\\d|th\\b|km\\b|dimensional))" +
    "(?![._-](?:minut[eio]s?|seconds?|hours?|days?|weeks?|months?|years?|anni|o'?clock|[ap]m|to|million|ways|signs|stages|" +
    "steps|times|reasons|things|babies|quarts|acres|stars?|miles|miglia|lit(?:ri|ers|res)|b[oö]l[uü]m|jigen)\\b)";
const MARKED_HALF_EPISODE = `(?<!${EPISODE_CODE}.*)(?:\\bs\\d{1,2}e|\\be|\\bep(?:isode)?[ .]*|[ .]-[ .]?|^(?=\\d{1,3}\\.5(?:v\\d)? - \\D))0*(\\d{1,3})${HALF_EPISODE_END}(?!.*${EPISODE_CODE})`;
const BARE_HALF_EPISODE = `(?<!${EPISODE_CODE}.*|\\d - .*|(?:imdb|part|seasons?|sezon|сезон|ios|android|version)[ ._]?|\\b(?:19|20)\\d{2})(?:[ .]|\\[)(?=\\d{2})0*(\\d{1,3})${HALF_EPISODE_END}(?!.*(?:${EPISODE_CODE}| - \\d))`;

function standalone(regExp) {
    return new RegExp(`(?<!\\p{L})(?:${regExp.source})(?!\\p{L})`, `${regExp.flags}u`);
}

exports.addDefaults = /** @type Parser */ (parser, { exclude } = {}) => {

    // Site
    parser.addHandler("site", /^(www?[., ][\w-]+[. ][\w-]+(?:[. ][\w-]+)?)\s+-\s*/i, none, { skipFromTitle: true, remove: true });

    // Uncut
    parser.addHandler("uncut", /(?<=\w[ ._-]+)[[(]?\buncut\b[\])]?(?=[ ._-]*(?:[[(]?(?:19\d|20[012])\d\b|\d{3,4}p\b|S\d|$))/i, boolean);

    // Episode code
    parser.addHandler("episodeCode", /[[(]([a-z0-9]{8}|[A-Z0-9]{8})[\])](?=\.[a-zA-Z0-9]{1,5}$|$)/, uppercase, { remove: true });
    parser.addHandler("episodeCode", /\[(?=[A-Z]+\d|\d+[A-Z])([A-Z0-9]{8})]/, uppercase, { remove: true });

    // Resolution
    parser.addHandler("resolution", /(?<=\b(?:4k|2160p)\b.*)\b(480|720|1080)p\b/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /\b[([]?4k[)\]]?\b/i, value("4k"), { remove: true });
    parser.addHandler("resolution", /21600?[pi]/i, value("4k"), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("resolution", /[([]?3840x\d{4}[)\]]?/i, value("4k"), { remove: true });
    parser.addHandler("resolution", /[([]?1920x\d{3,4}[)\]]?/i, value("1080p"), { remove: true });
    parser.addHandler("resolution", /[([]?1280x\d{3}[)\]]?/i, value("720p"), { remove: true });
    parser.addHandler("resolution", /[([]?\d{3,4}x(\d{3,4})[)\]]?/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(480|720|1080)0[pi]/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(?:BD|HD|M)(720|1080|2160)/, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(480|576|720|1080|2160)[piр]/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(?:^|\D)(\d{3,4})[pi]/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /[([](480|576|720|1080|2160)[)\]]/, value("$1p"), { remove: true });

    // Year
    parser.addHandler("date", /(?<=\W|^)([([]?(?:19[6-9]|20[012])[0-9]([. \-/\\])(?:0[1-9]|1[012])\2(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYY MM DD"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)/, date("DD MM YYYY"), { remove: true });
    parser.addHandler("date", /(?<=\W)([([]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)/, date("MM DD YYYY"), { remove: true });
    parser.addHandler("date", /(?<=\W)([([]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/, date("MM DD YY"), { remove: true });
    parser.addHandler("date", /(?<=\W)([([]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/, date("DD MM YY"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:19[7-9]|20[012])[0-9][)\]]?)(?=\W|$)/i, date("DD MMM YYYY"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:0[1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/i, date("DD MMM YY"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?20[012][0-9](?:0[1-9]|1[012])(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYYMMDD"), { remove: true });
    parser.addHandler("date", /^([([]?(?:19[6-9]|20[012])[0-9] (?:0[1-9]|1[012])-(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYY MM DD"), { remove: true });

    // Year
    parser.addHandler("year", /[([*]?[ .]?(?<!\bE)((?:19\d|20[012])\d[ .]?-[ .]?(?:19\d|20[012])\d)(?:\s?[*)\]])?/, yearRange, { remove: true });
    parser.addHandler("year", /[([*][ .]?((?:19\d|20[012])\d[ .]?-[ .]?\d{2})(?:\s?[*)\]])?/, yearRange, { remove: true });
    parser.addHandler("year", /[([][ .]?((?:19\d|20[012])\d(?:(?:[ .]?[,—–/][ .]?|[ .])(?:19\d|20[012])\d)+)(?!\d)/, yearRange, { remove: true });
    parser.addHandler("year", /[([]((?:19\d|20[012])\d(?:[)\]][ .]*&(?:(?!\bS\d)[^()[\]])*[([](?:19\d|20[012])\d)+)[)\]]/i, yearRange, { remove: true });
    parser.addHandler("year", /(?<=[ .](?:19\d|20[012])\d[ .]+)[([]((?:19\d|20[012])\d)[)\]]/, integer, { remove: true });
    parser.addHandler("year", /[([*]?(?!^)(?<!\d|Cap[. ]?|\bep[. ]?|\bE|\bS(?=\d{4}E\d))((?:19\d|20[012])\d)(?!\d|kbps)[*)\]]?/i, integer, { remove: true });
    parser.addHandler("year", /^[([]?((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?(?![ ._]+(?:S\d{1,2}[ .]?E\d|E\d{1,3}\b))/i, integer, { remove: true });

    // Extended
    parser.addHandler("extended", /EXTENDED/, boolean);
    parser.addHandler("extended", /- Extended/i, boolean);

    // Convert
    parser.addHandler("convert", /CONVERT/, boolean);

    // Hardcoded
    parser.addHandler("hardcoded", /HC|HARDCODED/, boolean);

    // Proper
    parser.addHandler("proper", /(?:REAL.)?PROPER/, boolean);

    // Repack
    parser.addHandler("repack", /REPACK|RERIP/, boolean);

    // Retail
    parser.addHandler("retail", /\bRetail\b/i, boolean);

    // Remastered
    parser.addHandler("remastered", /\bRemaster(?:ed)?\b/i, boolean);
    parser.addHandler("remastered", /\b[[(]?REKONSTRUKCJA[\])]?\b/i, boolean);

    // Unrated
    parser.addHandler("unrated", /\bunrated|uncensored\b/i, boolean);

    // Region
    parser.addHandler("region", /R\d\b(?![ ._]+(?:-[ ._]*)?(?:[Ee][Pp]\.?[ ._]*)?\d)/, none, { skipIfFirst: true });

    // Video depth
    parser.addHandler("bitDepth", /(?:8|10|12)[- ]?bit/i, lowercase, { remove: true });
    parser.addHandler("bitDepth", /\bhevc\s?10\b/i, value("10bit"));
    parser.addHandler("bitDepth", /\bhdr10\b/i, value("10bit"));
    parser.addHandler("bitDepth", /\bhi10\b/i, value("10bit"), { remove: true, skipFromTitle: true });
    parser.addHandler("bitDepth", ({ result }) => {
        if (result.bitDepth) {
            result.bitDepth = result.bitDepth.replace(/[ -]/, "");
        }
    });

    // Source
    parser.addHandler("source", /\b(?:H[DQ][ .-]*)?CAM(?:H[DQ])?(?:[ .-]*Rip)?\b(?![ .-]*(?:S\d|E\d|\((?:19|20)\d{2}\)))/i, value("CAM"), { remove: true });
    parser.addHandler("source", /\b(?:H[DQ][ .-]*)?S[ .-]+print/i, value("CAM"), { remove: true });
    parser.addHandler("source", /\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b/i, value("TeleSync"), { remove: true });
    parser.addHandler("source", /\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b/, value("TeleCine"), { remove: true });
    parser.addHandler("source", /\bBlu[ .-]*Ray\b(?=.*remux)/i, value("BluRay REMUX"), { remove: true });
    parser.addHandler("source", /(?:BD|BR|UHD)[- ]?remux/i, value("BluRay REMUX"), { remove: true });
    parser.addHandler("source", /(?<=remux.*)\bBlu[ .-]*Ray\b/i, value("BluRay REMUX"), { remove: true });
    parser.addHandler("source", /\bBlu[ .-]*Ray\b(?![ .-]*Rip)/i, value("BluRay"), { remove: true });
    parser.addHandler("source", /\bUHD[ .-]*Rip\b/i, value("UHDRip"), { remove: true });
    parser.addHandler("source", /\bP(?:re)?-HD(?:Rip)?\b/i, value("SCR"), { remove: true });
    parser.addHandler("source", /\bHD[ .-]*Rip\b/i, value("HDRip"), { remove: true });
    parser.addHandler("source", /\bMicro[ .-]*HD\b/i, value("HDRip"), { remove: true });
    parser.addHandler("source", /\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b/i, value("BRRip"), { remove: true });
    parser.addHandler("source", /\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|[[(]BD[\]) .,-]/i, value("BDRip"), { remove: true });
    parser.addHandler("source", /\b(?:HD[ .-]*)?DVD[ .-]*Rip\b/i, value("DVDRip"), { remove: true });
    parser.addHandler("source", /\bVHS[ .-]*Rip\b/i, value("DVDRip"), { remove: true });
    parser.addHandler("source", /\b(?:DVD?|BD|BR|HD)?[ .-]*Scr(?:eener)?\b/i, value("SCR"), { remove: true });
    parser.addHandler("source", /\bP(?:re)?DVD(?:Rip)?\b/i, value("SCR"), { remove: true });
    parser.addHandler("source", /\bDVD(?:R\d?)?\b/i, value("DVD"), { remove: true });
    parser.addHandler("source", /\bVHS\b/i, value("DVD"), { remove: true, skipIfFirst: true });
    parser.addHandler("source", /\bPPVRip\b/i, value("PPVRip"), { remove: true });
    parser.addHandler("source", /\bHD[ .-]*TV(?:Rip)?\b/i, value("HDTV"), { remove: true });
    parser.addHandler("source", /\bDVB[ .-]*(?:Rip)?\b/i, value("HDTV"), { remove: true });
    parser.addHandler("source", /\bSAT[ .-]*Rips?\b/i, value("SATRip"), { remove: true });
    parser.addHandler("source", /\bTVRips?\b/i, value("TVRip"), { remove: true });
    parser.addHandler("source", /\bR5\b/i, value("R5"), { remove: true });
    parser.addHandler("source", /\bWEB[ .-]*DL(?:Rip)?\b/i, value("WEB-DL"), { remove: true });
    parser.addHandler("source", /\bWEB[ .-]*Rip\b/i, value("WEBRip"), { remove: true });
    parser.addHandler("source", /\b(?:DL|WEB|BD|BR)MUX\b/i);
    parser.addHandler("source", /\b(DivX|XviD)\b/);

    // HDR
    parser.addHandler("hdr", /\bDV\b|dolby.?vision|\bDoVi\b/i, uniqConcat(value("DV")), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("hdr", /HDR10(?:\+|plus)/i, uniqConcat(value("HDR10+")), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("hdr", /\bHDR(?:10)?\b/i, uniqConcat(value("HDR")), { remove: true, skipIfAlreadyFound: false });

    // 3D
    parser.addHandler("threeD", /\b(3D)\b.*\b(Half-?SBS|H[-\\/]?SBS)\b/i, value("3D HSBS"));
    parser.addHandler("threeD", /\bHalf.Side.?By.?Side\b/i, value("3D HSBS"));
    parser.addHandler("threeD", /\b(3D)\b.*\b(Full-?SBS|SBS)\b/i, value("3D SBS"));
    parser.addHandler("threeD", /\bSide.?By.?Side\b/i, value("3D SBS"));
    parser.addHandler("threeD", /\b(3D)\b.*\b(Half-?OU|H[-\\/]?OU)\b/i, value("3D HOU"));
    parser.addHandler("threeD", /\bHalf.?Over.?Under\b/i, value("3D HOU"));
    parser.addHandler("threeD", /\b(3D)\b.*\b(OU)\b/i, value("3D OU"));
    parser.addHandler("threeD", /\bOver.?Under\b/i, value("3D OU"));
    parser.addHandler("threeD", /\b((?:BD)?3D)\b/i, value("3D"), { skipIfFirst: true });

    // Codec
    parser.addHandler("codec", /\b[xh][-. ]?26[45]/i, lowercase, { remove: true });
    parser.addHandler("codec", /\bhevc(?:\s?10)?\b/i, value("hevc"), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("codec", /\b(?:dvix|mpeg2|divx|xvid|avc|(?<!-)av1)\b/i, lowercase, { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("codec", ({ result }) => {
        if (result.codec) {
            result.codec = result.codec.replace(/[ .-]/, "");
        }
    });

    // Audio
    parser.addHandler("audio", /7\.1[. ]?Atmos\b/i, value("7.1 Atmos"), { remove: true });
    parser.addHandler("audio", /\b(?:mp3|Atmos|DTS(?:-HD)?|TrueHD)\b/i, lowercase);
    parser.addHandler("audio", /\bFLAC(?:\+?2\.0)?(?:x[2-4])?\b/i, value("flac"), { remove: true });
    parser.addHandler("audio", /\bEAC-?3(?:[. -]?[256]\.[01])?/i, value("eac3"), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("audio", /\bAC-?3(?:[.-]5\.1|x2\.?0?)?\b/i, value("ac3"), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("audio", /\b5\.1(?:x[2-4]+)?\+2\.0(?:x[2-4])?\b/i, value("2.0"), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("audio", /\b2\.0(?:x[2-4]|\+5\.1(?:x[2-4])?)\b/i, value("2.0"), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("audio", /\b5\.1ch\b/i, value("ac3"), { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("audio", /\bDD5[. ]?1\b/i, value("dd5.1"), { remove: true });
    parser.addHandler("audio", /\bQ?AAC(?:[. ]?2[. ]0|x2)?\b/, value("aac"), { remove: true });
    parser.addHandler("audioChannels", /\[[257][.-][01]]/, lowercase, { remove: true });
    parser.addHandler("audioChannels", /(?:\b(?:DTS(?:-?HD)?(?:[ .-]?MA)?|E?AC-?3|AAC|TrueHD|FLAC|Opus|L?PCM)[ .-]?|[[(])([257]\.[01])(?:ch)?(?=[\]) .,_-]|$)/i, none, { remove: true });


    // Container
    parser.addHandler("container", /\.?[[(]?\b(?<!\p{L})(MKV|AVI|MP4|WMV|MPG|MPEG)\b(?!\p{L})[\])]?/iu, lowercase);

    // Volumes
    parser.addHandler("volumes", /\bvol(?:s|umes?)?[. -]*(?:\d{1,2}[., +/\\&-]+)+\d{1,2}\b/i, range, { remove: true });
    parser.addHandler("volumes", ({ title, result, matched }) => {
        const startIndex = matched.year && matched.year.matchIndex || 0;
        const match = title.slice(startIndex).match(/\bvol(?:ume)?[. -]*(\d{1,2})/i);

        if (match) {
            matched.volumes = { match: match[0], matchIndex: match.index };
            result.volumes = array(integer)(match[1]);
            return { rawMatch: match[0], matchIndex: match.index, remove: true };
        }
        return null;
    });

    // Season
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?:\bseasons?[. ]?[([]?|\b(?=s\d))((?:s?\d{1,2})[. ]*-[. ]*s?\d{1,2})[)\]]?[. ]*[([]\d{1,2}[. ]?of[. ]?\d{1,2}[)\]]/i, range, { remove: true });
    parser.addHandler("seasons", /(?:complete\W|seasons?\W(?:\d{1,2}[. ]*-[. ]*\d{1,2}[. ]+)?|\W|^)((?:s\d{1,2}[., +/\\&-]+)+s\d{1,2}\b)/i, range, { remove: true });
    parser.addHandler("seasons", /(?:complete\W|seasons?\W|\W|^)[([]?(s\d{2,}-\d{2,}\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:complete\W|seasons?\W|\W|^)[([]?(s[1-9]-[2-9]\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:[^\wЀ-ӿ-]|^)(\d{1,2}-\d{1,2})[xх](?!26[45]\b)\d{1,3}\b/i, range);
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?:seasons?|[Сс]езони?|sezon|temporadas?|stagioni|s[äæ]song|sesong|staffel)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}(?: ?[,/\\&][, /\\&]*(?:and )?| ))+\d{1,2}\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?:seasons|[Сс]езони?|sezon|temporadas?|stagioni|s[äæ]song|sesong|staffel)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}[. -]+)+0?[1-9]\d?\b)(?![. _]?(?:seri|сери))[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?(?<!\bopen[ ._])season[. ]?[([]?((?:\d{1,2}[. -]+)+[1-9]\d?\b)[)\]]?(?!.*\.\w{2,4}$)/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W+)?(?:\ball\W)?\b(?:seasons?|s[äæ]song|sesong|staffel)[. -]?(\d{1,2}[. -]?(?:to|thru|and|till|\+|:)[. -]?\d{1,2})\b/i, range, { remove: true });
    parser.addHandler("seasons", /\bfirst[. ](\d{1,2})[. ]seasons\b/i, rangeUpTo);
    parser.addHandler("seasons", /\bS((?:19|20)\d{2})E0*[1-9]\d{0,2}\b/i, array(integer));
    parser.addHandler("seasons", /\bS(\d{3})[. ]?E\d{1,4}\b/i, array(integer));
    parser.addHandler("seasons", /(\d{1,2})(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?)(?:\W?\D|$)/i, array(integer));
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?(?<!\bopen[ ._])(?:saison|seizoen|sezona|sezon(?:SO?)?|stagione|seasons?|series|temp(?:orada)?|sesong|s[äæ]song|sæson|staffel):?[. ]?(\d{1,2})(?!\d|(?<=seasons\W?\d+)[. ]?(?:episodes?|eps)\b(?![. -]*\d))/i, array(integer));
    parser.addHandler("seasons", /[Сс]езон:?[. _]?№?(\d{1,2})(?!\d)/i, array(integer));
    parser.addHandler("seasons", /(?:\D|^)(\d{1,2})Â?[°ºªa]?[. ]*temporada/i, array(integer), { remove: true });
    parser.addHandler("seasons", /(?<![a-z])t(\d{1,3})[ex]+/i, array(integer), { remove: true });
    parser.addHandler("seasons", /(?<=\s)(?<!\bby\s)t(\d{1,2})$/i, array(integer), { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete)?(?!(?<!s\d{1,2})-so?[1-9]\d(?:\.[a-z\d]{2,4}$|\]))(?:[^\w'’]|^)so?([01]?[0-5]?[1-9]|[1-5]0)(?:[\Wex]|\d{2}\b)/i, array(integer), { skipIfAlreadyFound: false });
    parser.addHandler("seasons", /(?:[^\w-]|^)s(\d{1,2})$/i, array(integer));
    parser.addHandler("seasons", /(?:\W|^)s-(\d{1,2})(?=[. -]*e[. -]?\d{1,3}\b|[. ]+(?:d(?:isc)?[. -]?\d|dvd)\b|[. ]*$)/i, array(integer));
    parser.addHandler("seasons", /(?:so?|t)(\d{1,2})[. ]?[xх-]?[. ]?(?:e|x|х|ep|-|\.)[. ]?\d{1,4}(?:[abc]|v0?[1-4]|\D|$)/i, array(integer));
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?(?:\W|^)(\d{1,2})[. ]?(?:st|nd|rd|th)[. ]*season/i, array(integer));
    parser.addHandler("seasons", /第[ ]?([0-9０-９]{1,2}|[一二三四五六七八九十]{1,3})[ ]?(?:季|期(?=.*(?:[\u3040-\u30ff]|第\d+[話话])))(?:[(（][^)）]*[)）])?/, array(cjkInteger), { remove: true, skipFromTitle: true });
    parser.addHandler("seasons", /(?<=\[)(\d{1,2})(?:st|nd|rd|th)(?=[. ]*-[. ]*\d{1,4}\])/i, array(integer), { remove: true });
    parser.addHandler("seasons", /^(\d{1,2})[. ]season$/i, array(integer));
    parser.addHandler("seasons", /(?:\D|^)(\d{1,2})[Xxх](?!26[45]\b)\d{1,3}(?:\D|$)/, array(integer));
    parser.addHandler("seasons", /^x(?!26[45]\b|(?:480|576|720|1080|2160)\b)(\d{1,2})\d{2}(?= - )/i, array(integer));
    parser.addHandler("seasons", /\bSn([1-9])(?:\D|$)/, array(integer));
    parser.addHandler("seasons", /\bv(\d{1,2})e\d{1,3}\b/i, array(integer));
    parser.addHandler("seasons", /(?<=[a-z]\.)(?<!sezon\.)(\d{1,2})\.\(?\d{1,3}(?:-\d{1,3})?\.ser[ijya]{0,3}(?=\.)/i, array(integer));
    parser.addHandler("seasons", /(?<=[a-z]\.)(\d{1,2})\.ser\.\d{1,3}(?=\.)/i, array(integer));
    parser.addHandler("seasons", /(?:^|[ ._-])(\d{1,2})EP?\d{2,3}(?=[ ._-]|$)/i, array(integer));
    parser.addHandler("seasons", /\bs\.(\d{1,2})[ .]ep\.\d{1,4}/i, array(integer));
    parser.addHandler("seasons", /[[(](\d{1,2})\.(?!5[)\]])\d{1,3}[)\]]/, array(integer));
    parser.addHandler("seasons", /-\s?(\d{1,2})\.\d{2,3}\s?-/, array(integer));
    parser.addHandler("seasons", /^(\d{1,2})\.\d{2,3} - /, array(integer), { skipIfBefore: ["year, source", "resolution"] });
    parser.addHandler("seasons", /^(\d{1,2})\.\d{2,3}[. ]+(?=[^\W\d]|[\u00c0-\u024f\u0400-\u04ff])/, array(integer), { skipIfBefore: ["year"] });
    parser.addHandler("seasons", /(?:^|\/)([1-9])(?=-0\d\b(?!-\d))/, array(integer));
    parser.addHandler("seasons", /(?:^|\/)(?!20-20)(\d{1,2}(?=-\d{2}\b(?!-\d))|(\d)(?=-(?:1\d{2}|\2\d{2})(?=[ ._]|$)))/, array(integer), { skipIfBefore: ["year"] });
    parser.addHandler("seasons", /[^\w-](\d{1,2})-\d{2}(?=\.\w{2,4}$)/, array(integer));
    parser.addHandler("seasons", /(?<!\bEp?(?:isode)? ?\d+\b.*)\b(\d{2})[ ._]\d{2}(?:.F)?\.\w{2,4}$/, array(integer));
    parser.addHandler("seasons", /\bEp(?:isode)?\W+(?:#|(?!\d{1,2}\.5\b))(\d{1,2})\.\d{1,3}\b/i, array(integer));

    // adds single season info if its there"s only single season
    parser.addHandler("season", ({ result }) => {
        if (result.seasons && result.seasons.length === 1) {
            result.season = result.seasons[0];
        }
    });

    // Episode
    parser.addHandler("episodes", /\bS\d{1,2} \d{1,3}-(\d{3})(?=\.\w{2,4}$)/i, array(integer));
    parser.addHandler("episodes", /(?<=[a-z]\.)(?<!sezon\.)\d{1,2}\.(\d{1,3}(?:-\d{1,3})?)\.ser[ijya]{0,3}(?=\.)/i, range);
    parser.addHandler("episodes", /(?<=[a-z]\.)\d{1,2}\.ser\.(\d{1,3})(?=\.)/i, array(integer));
    parser.addHandler("episodes", /(?:[\W\d]|^)e[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|e){1,2}[ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /\bs\d{1,2}[ .-]?e(\d{1,3}-?e\d{1,3})(?:\W|$)/i, list);
    parser.addHandler("episodes", /(?:[\W\d]|^)ep[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|ep){1,2}[ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)\d+[xх][ .]?[([]?(\d{1,3}(?:[ .]?[xх][ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:episodes?|[Сс]ерии:?)[ .]?[([]?(\d{1,3}(?:[ .+]*[&+][ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /[([]?(?:\D|^)(\d{1,3}[ .]?ao[ .]?\d{1,3})[)\]]?(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:e|eps?|episodes?|[Сс]ерии:?|\d+[xх])[ .]*[([]?(\d{1,3}(?:-\d{1,3})+)(?!\.\d)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:eps?|episodes?)[ .]*[([]?(?!(?:19|20)\d{2}-(?:19|20)\d{2}\b)(\d{3,4}-\d{3,4})(?!\.\d)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:eps?|episodes?)[ .]*[([]?(\d{1,3}[ .]+(?:to|thru|till)[ .]+\d{1,3})(?:\W|$)/i, range);
    parser.addHandler("episodes", /[Сс]ерии:?\s+(\d{1,4})\s+из\s+\d{1,4}/, rangeUpTo);
    parser.addHandler("episodes", /\[(\d{1,4})\s+из\s+\d{1,4}]/, rangeUpTo);
    parser.addHandler("episodes", /(?<=\s)(\d{1,4})\/\1(?=\s*\[)/, rangeUpTo);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:eps?|episodes?)[ .]*[([]?(\d{1,2}[ .]+-[ .]+\d{1,3}|\d{3,4}[ .]+-[ .]+\d{3,4})(?!\.\d|[ .]*-)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:\W|^)(\d{1,3}(?:[ .]*~[ .]*\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:so?|t)\d{1,2}[. ]?[xх-]?[. ]?(?:e|x|х|ep)[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)/i, array(integer));
    parser.addHandler("episodes", /(?:so?|t)\d{1,2}\s?[-.]\s?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)/i, array(integer));
    parser.addHandler("episodes", /\b(?:so?|t)\d{2}(\d{2})\b/i, array(integer));
    parser.addHandler("episodes", /\bS(?:19|20)\d{2}E(0*[1-9]\d{0,2})\b/i, array(integer));
    parser.addHandler("episodes", /\bS\d{3}[. ]?E(\d{1,4})\b/i, array(integer));
    parser.addHandler("episodes", /\bv\d{1,2}e(\d{1,3})\b/i, array(integer));
    parser.addHandler("episodes", /\bs\.\d{1,2}[ .]ep\.(\d{1,4})/i, array(integer));
    parser.addHandler("episodes", /-\s(\d{1,3}[ .]*-[ .]*\d{1,3})(?!-\d)(?:\W|$)/i, range);
    parser.addHandler("episodes", /s\d{1,2}\s?\((\d{1,3}[ .]*-[ .]*\d{1,3})\)/i, range);
    parser.addHandler("episodes", /^(01-02|02-03|03-04|04-05|05-06|06-07|07-08|08-09|09-10)(?=[ ._])/, range);
    parser.addHandler("episodes", /(?:^|\/)[1-9]-(0\d)\b(?!-\d)/, array(integer));
    parser.addHandler("episodes", /(?:^|\/)(?!20-20)\d{1,2}-(\d{2})\b(?!-\d)/, array(integer), { skipIfBefore: ["year"] });
    parser.addHandler("episodes", /(?:^|\/)(?:1-1|2-2|3-3|4-4|5-5|6-6|7-7|8-8|9-9)(\d{2})(?=[ ._]|$)/, array(integer), { skipIfBefore: ["year"] });
    parser.addHandler("episodes", /(?:^|\/)\d-(1\d{2})(?=[ ._]|$)/, array(integer), { skipIfBefore: ["year"] });
    parser.addHandler("episodes", /(?<!\d-)\b\d{1,2}-(\d{2})(?=\.\w{2,4}$)/, array(integer));
    parser.addHandler("episodes", /(?<=^\[.+].+)(?:[. ]+episode)?[. ]+-[. ]+(\d{1,4})[. ]+(?=\W)/i, array(integer));
    parser.addHandler("episodes", /(?<!(?:seasons?|series|[Сс]езони?)\W*)(?:[ .([-]|^)(\d{1,3}(?:[ .]?[,&+~][ .]?\d{1,3})+)(?:[ .)\]-]|$)/i, range);
    parser.addHandler("episodes", /^(?!20-20)(\d{1,3}(?:-\d{1,3})+)(?!\.\d)(?:[ .)(\]]|$)/i, range, { skipIfBefore: ["year"] });
    parser.addHandler("episodes", /(?<!^\d{1,3}(?:-\d{1,3})+[ .)(\]-].*)(?<!(?:seasons?|[Сс]езони?)\W*)(?!20-20)(?:[ .([]|(?<!^\d{1,3}(?:-\d{1,3})*)-)(\d{1,3}(?:-\d{1,3})+)(?!\.\d)(?:[ .)(\]]|-\D|$)/i, range);
    parser.addHandler("episodes", /\bEp(?:isode)?\W+(?:#|(?!\d{1,2}\.5\b))\d{1,2}\.(\d{1,3})\b/i, array(integer));
    parser.addHandler("episodes", /(?:\b(?<!\p{L})(?!(?<=(?:^|[^a-z])[a-z]\.)[ée]\.\s*[1-9])[ée]p?(?:isode)?|[Ээ]пизод|[Сс]ер(?:ии|ия|\.)?|seri[jy]a|caa?p(?:itulo)?|epis[oó]dio|epizoda|odcinek|avsnitt|aflevering|folge|jakso|puntata)[. ]?[-:#№]?[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\W|$)/iu, array(integer));
    parser.addHandler("episodes", /\d{1,2}\.sezon\.(\d{1,3}\.\d{1,3})\.ser/i, range);
    parser.addHandler("episodes", /\b(\d{1,3}-\d{1,3})[ ._]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)/i, range);
    parser.addHandler("episodes", /(?<=\d[ ._]?[Сс]езон[ ._]+)(\d{1,4}-\d{1,4})(?![\d-])/, range);
    parser.addHandler("episodes", /\b(\d{1,3})(?:-?(?:я|ja|ya))?[ ._-]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)/i, array(integer));
    parser.addHandler("episodes", /(?:\D|^)\d{1,2}[. ]?[Xxх][. ]?(?!26[45]\b)(\d{1,3})(?:[abc]|v0?[1-4]|\D|$)/, array(integer));
    parser.addHandler("episodes", /^x(?!26[45]\b|(?:480|576|720|1080|2160)\b)\d{1,2}(\d{2})(?= - )/i, array(integer));
    parser.addHandler("episodes", /[[(]\d{1,2}\.(?!5[)\]])(\d{1,3})[)\]]/, array(integer));
    parser.addHandler("episodes", /(?:^|[ ._-])\d{1,2}EP?(\d{2,3})(?=[ ._-]|$)/i, array(integer));
    parser.addHandler("episodes", /\b[Ss](?:eason\W?)?\d{1,2}[ .](\d{1,2})\b/, array(integer));
    parser.addHandler("episodes", /\bseries[ .]?\d{1,2}(?:, |\.| - ?|- | )(\d{1,3})(?=[ .]+[^\W\d])/i, array(integer));
    parser.addHandler("episodes", /-\s?\d{1,2}\.(\d{2,3})\s?-/, array(integer));
    parser.addHandler("episodes", /^\d{1,2}\.(\d{2,3}) - /, array(integer), { skipIfBefore: ["year, source", "resolution"] });
    parser.addHandler("episodes", /^\d{1,2}\.(\d{2,3})[. ]+(?=[^\W\d]|[\u00c0-\u024f\u0400-\u04ff])/, array(integer), { skipIfBefore: ["year"] });
    parser.addHandler("episodes", /(?<=\D|^)(?!(?<=\d\.)5\b)(\d{1,3})[. ]?(?:of|из|iz)[. ]?\d{1,3}(?=\D|$)/i, array(integer));
    parser.addHandler("episodes", /\b\d{2}[ ._-](\d{2})(?:.F)?\.\w{2,4}$/, array(integer));
    parser.addHandler("episodes", /(?<!^)\[(\d{2,3})](?!(?:\.\w{2,4})?$)/, array(integer));
    parser.addHandler("episodes", /\bodc[. ]+(\d{1,3})\b/i, array(integer));
    parser.addHandler("episodes", /(?<![a-z\d])(\d{1,3})ep(?![a-z\d]|[ ._-]?\d)/i, array(integer));
    parser.addHandler("episodes", new RegExp(`(?<=^(?:\\[[^\\]]+\\])?[a-z]{3,})(?<!part|pt|cd|dvd|dis[ck]|vol|title|season)(0\\d{1,2})(?=\\.(?:${VIDEO_EXTENSIONS}|${SUBTITLE_EXTENSIONS})$)`, "i"), array(integer));
    parser.addHandler("episodes", /\b(\d{1,4})\.?[ ._-]*b[öo]l[üu]m\b/i, array(integer));
    parser.addHandler("episodes", /\bb[öo]l[üu]m[ ._-]*(\d{1,4})\b/i, array(integer));
    parser.addHandler("episodes", /[ .]-[ .]\d{1,3}\.5[ ._]+(?:OVA|OAV|OAD|ONA)[ ._]*E?0*(\d{1,2})\b/i, array(integer));
    parser.addHandler("episodes", /(?<=(?<!\d,\d+[ ._]*)\b(?:OVA|OAV|OAD|ONA)[ ._]*)E?0*(\d{1,2})(?:v\d)?\b(?![ .]*[-~&+.]\d)/i, array(integer));
    parser.addHandler("episodes", /(?<=(?:^|[ ._-])SP)0*(\d{1,2})(?=[ ._]*(?:[[(]|\.\w{2,4}$|$))/i, array(integer));

    parser.addHandler("episodes", /^(\d{3,4})[. ]+-[. ]+(?=[^\W\d])/, array(integer), { remove: true, skipIfBefore: ["year"] });

    parser.addHandler("episodes", new RegExp(MARKED_HALF_EPISODE, "i"), array(integer));
    parser.addHandler("episodes", new RegExp(BARE_HALF_EPISODE, "i"), array(integer), { skipIfBefore: ["year"] });

    // can be both absolute episode and season+episode in format 101
    parser.addHandler("episodes", ({ title, result, matched }) => {
        if (!result.episodes) {
            const startIndexes = [matched.year, matched.seasons]
                .filter(component => component)
                .map(component => component.matchIndex)
                .filter(index => index > 0);
            const endIndexes = [matched.resolution, matched.source, matched.codec, matched.audio]
                .filter(component => component)
                .map(component => component.matchIndex)
                .filter(index => index > 0);
            const startIndex = startIndexes.length ? Math.min(...startIndexes) : 0;
            const endIndex = Math.min(...endIndexes, title.length);
            const beginningTitle = title.slice(0, endIndex);
            const middleTitle = title.slice(startIndex, endIndex);

            // try to match the episode inside the title with a separator, if not found include the start of the title as well
            const matches = Array.from(beginningTitle.matchAll(/(?<!movie\W*|film\W*|^)(?:[ .]+-[ .]+\d{1,4}(?:a|b|v\d)?[ .]*[([][ .]*(?!\d+-[a-z]+\))|[ .]+-[ .]+|[([][ .]*(?!\d+-[a-z]+\)))(\d{1,4})(?![. _]?(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?\b)(?![. _]?finali))(?:a|b|v\d|\.\d)?(?:\W|$)(?!movie|film|фильм|\d+)/gi)).pop() ||
                middleTitle.match(/^(?:[([-][ .]?)?(\d{1,4})(?![. _]?(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?\b)(?![. _]?finali))(?:a|b|v\d)?(?!\Wmovie|\Wfilm|\Wфильм|-\d)(?:\W|$)/i);

            if (matches) {
                result.episodes = [matches[matches.length - 1]]
                    .map(group => group.replace(/\D/g, ""))
                    .map(group => parseInt(group, 10));
                return { matchIndex: title.indexOf(matches[0]) };
            }
        }
        return null;
    });

    parser.addHandler("episodes", ({ title, result }) => {
        if (!result.episodes && !result.seasons && !result.year) {
            const match = title.match(/(?<=[^\d\s][.-]|\s\.)(\d{1,2})\.(\d{2,3})(?=\.\w{2,4}$)/);
            if (match) {
                result.seasons = [parseInt(match[1], 10)];
                result.season = result.seasons[0];
                result.episodes = [parseInt(match[2], 10)];
                return { matchIndex: match.index };
            }
        }
        return null;
    });

    parser.addHandler("episodes", ({ title, result, matched }) => {
        if (!result.episodes && !result.date) {
            const match = matched.year && title.slice(matched.year.matchIndex)
                .match(/^(?:[ .]| - ?|\. (?=\d{1,3} ?(?:\[|$)))(?!(?:480|576|720)\b)(\d{1,3})(?:v\d)?-?(?=[ .([-]|$)(?![ .,-]*\d|[ .]*(?:cd|dvd|dis[ck]|bits?|films?|movies?|[mg]b)\b)/i) ||
                matched.resolution && title.slice(matched.resolution.matchIndex).match(/^ (\d{3,4}) (?=[^\W\d])/);
            if (match) {
                result.episodes = [parseInt(match[1], 10)];
            }
        }
        return null;
    });
    parser.addHandler("episodes", /(?<!(?:audio|subtitles?|season|movie|film)\W*|\b\d{1,3}(?: -|\.) .*|\([^)]*)(?:^|[ ._])#(\d{1,3})(?![.,]?\d)(?=[ ._\])]|$)/i, array(integer), { skipIfAfter: ["date"] });
    parser.addHandler("episodes", /第(\d{1,4}(?:-\d{1,4})?)[話话集]/, range);
    parser.addHandler("episodes", /\bphase[ .](\d{2,3})\b/i, array(integer));

    // the parsed episode written as `12.5`: an episode inserted after 12, i.e. a recap or special
    parser.addHandler("halfEpisode", ({ title, result, matched }) => {
        if (result.episodes && result.episodes.length === 1) {
            const start = matched.episodes ? matched.episodes.matchIndex : 0;
            const length = matched.episodes && matched.episodes.rawMatch ? matched.episodes.rawMatch.length : Infinity;
            const match = title.slice(start).match(new RegExp(`(?<!\\d\\.?|imdb[ .]?|#)0*${result.episodes[0]}${HALF_EPISODE_END}`, "i"));
            if (match && match.index < length && !(start + match.index === 0 && /^[1-9]\./.test(match[0]))) {
                result.halfEpisode = true;
            }
        }
        return null;
    });

    // adds single season info if its there's only single season
    parser.addHandler("episode", ({ result }) => {
        if (result.episodes && result.episodes.length === 1) {
            result.episode = result.episodes[0];
        }
    });

    parser.addHandler("source", /\b(?:WEB|web)\b(?![ .-]*DL)/, value("WEB"), { remove: true, skipIfFirst: true });

    parser.addHandler("complete", /(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b/i, boolean);
    parser.addHandler("complete", /(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bmini[ .-]?series\b/i, boolean);
    parser.addHandler("complete", /(?:\bthe\W)?\b(?:complete|full|all)\b.*\b(?:series|seasons|collection|episodes|set|pack|movies)\b/i, boolean);
    parser.addHandler("complete", /\b(?:series|seasons|movies?)\b.*\b(?:complete|collection)\b/i, boolean);
    parser.addHandler("complete", /(?:\bthe\W)?\bultimate\b[ .]\bcollection\b/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("complete", /\bcollection\b.*\b(?:set|pack|movies)\b/i, boolean);
    parser.addHandler("complete", /\bint[ée]grale\b/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("complete", /\b(?:la |primera |segunda )?(?:s[ée]rie|temporada)[ .]complet[ao]s?\b/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("complete", /[([{]seasons[)\]}]/i, boolean, { remove: true, skipIfAlreadyFound: false });
    parser.addHandler("complete", /[([{【](?:(?:full|complete|unofficial)[ ._-]?)?batch[)\]}】]/i, boolean, { remove: true, skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("complete", /(?<![([{【][^)\]}】]*|\bbad[ ._-]?)\b(?:(?:full|complete|unofficial)[ ._-]?)?batch\b/i, boolean, { remove: true, skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("complete", /\bcomplet[eo]s?\b(?=[ ._-]*(?:$|[[\]()]))/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("complete", /\b(collection|completa)\b/i, boolean, { skipFromTitle: true });
    parser.addHandler("complete", /\bkolekc(?:ja|e)\b(?:\Wfilm(?:y|ów|ow|ů|u)?)?/i, boolean, { remove: true });
    parser.addHandler("complete", /(?<![\p{L}\d])\d{1,3}[ .]+фильм(?:ов|а)(?!\p{L})/iu, boolean, { remove: true });
    parser.addHandler("complete", /duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|heptalogy|anthology/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("complete", /\b(?:dualogy|(?:du[ao]|dy|tri|quadr[oi]|tetra|penta)log[ií]a)\b/i, boolean, { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("complete", /(?<!\p{L})(?:дилоги|трилоги|квадрологи|тетралоги|пенталоги|гексалоги|антологи)[яи](?!\p{L})/iu, boolean, { skipIfAlreadyFound: false, skipFromTitle: true });
    parser.addHandler("complete", /\bsaga\b/i, boolean, { skipFromTitle: true });

    parser.addHandler("extras", /(?<=\S[ ._-]?)\[SP\d{1,2}]/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("extras", new RegExp(`\\s*\\(?\\s*(?:[+&]\\s*${PACK_EXTRA}|\\bplus\\s+${PLUS_EXTRA})(?:\\s*(?:[+&,]|and)\\s*${PACK_EXTRA})*\\)?`, "i"), boolean, { remove: true });

    // Language
    function addLanguageName(name, language, { skipIfFirst, ...options } = {}) {
        const caps = name.replace(/[a-z]/g, letter => (letter === "i" ? "[iI]" : letter.toUpperCase()));
        parser.addHandler("languages", new RegExp(`(?<=\\p{Ll}.*)\\b(?:${caps})\\b`, "u"), uniqConcat(value(language)), { skipIfAlreadyFound: false, ...options });
        parser.addHandler("languages", new RegExp(`\\b(?:${name})\\b(?![ .]+(?!(?:sub(?:title)?s?|language|dub\\w*)\\b)\\p{L}+(?![\\p{L}\\p{N}]|$))`, "iu"), uniqConcat(value(language)), { skipIfFirst, skipIfAlreadyFound: false, ...options });
        parser.addHandler("languages", new RegExp(`\\b(?:${name})\\b`, "iu"), uniqConcat(value(language)), { skipIfFirst: true, skipIfBefore: ["year"], skipIfAlreadyFound: false, ...options });
    }
    parser.addHandler("languages", /\bmulti(?:ple)?[ .-]*(?:su?$|sub\w*|dub\w*)\b|msub/i, uniqConcat(value("multi subs")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\bmulti(?:ple)?[ .-]*(?:lang(?:uages?)?|audio|VF2)?\b/i, uniqConcat(value("multi audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\btri(?:ple)?[ .-]*(?:audio|dub\w*)\b/i, uniqConcat(value("multi audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bdual[ .-]*(?:au?$|[aá]udio|line)\b/i, uniqConcat(value("dual audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bdual\b(?![ .-]*sub|!)/i, uniqConcat(value("dual audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bengl?(?:sub[A-Z]*)?\b/i), uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\beng?sub[A-Z]*\b/i), uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bing(?:l[eéê]s)?\b/i), uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\benglish\W+(?:subs?|sdh|hi|dub\w*)\b/i, uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bEN\b/i), uniqConcat(value("english")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("english?", "english", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\b(?:JP|JAP|JPN)\b/i), uniqConcat(value("japanese")), { skipIfAlreadyFound: false });
    addLanguageName("japanese|japon[eê]s", "japanese", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\b(?:KOR|kor[ .-]?sub)\b/i), uniqConcat(value("korean")), { skipIfAlreadyFound: false });
    addLanguageName("korean|coreano", "korean", { skipIfFirst: true });
    parser.addHandler("languages", /\b(?:traditional\W*chinese|chinese\W*traditional)(?:\Wchi)?\b/i, uniqConcat(value("taiwanese")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\bzh-hant\b/i, uniqConcat(value("taiwanese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?:mand[ae]rin|ch[sn])\b/i), uniqConcat(value("chinese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bCH[IT]\b/), uniqConcat(value("chinese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("chinese|chin[eê]s", "chinese", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bchi\b/i), uniqConcat(value("chinese")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bzh-hans\b/i, uniqConcat(value("chinese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bFR(?:a|e|anc[eê]s)?\b/i), uniqConcat(value("french")), { skipIfAlreadyFound: false });
    addLanguageName("french", "french");
    parser.addHandler("languages", standalone(/\b(?:Truefrench|VF[FI])\b/i), uniqConcat(value("french")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?:VOST(?:FR?|A)?|SUBFRENCH)\b/i), uniqConcat(value("french")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bspanish\W?latin|american\W*(?:spa|esp?)/i, uniqConcat(value("latino")), { skipFromTitle: true, skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", standalone(/\b(?:audio.)?lat(?:i|ino)?\b/i), uniqConcat(value("latino")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?:audio.)?(?:ESP(?![ .]+-[ .]+\d)|spa|(en[ .]+)?espa[nñ]ola?|castellano)\b/i), uniqConcat(value("spanish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bes(?=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})\b/i), uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?<=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})es\b/i), uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?<=[ .,/-]+[A-Z]{2}[ .,/-]+)es(?=[ .,/-]+[A-Z]{2}[ .,/-]+)\b/i), uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bes(?=\.(?:ass|ssa|srt|sub|idx)$)/i), uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bspanish\W+(?:subs?|line|alta|calidad)\b/i, uniqConcat(value("spanish")), { skipIfAlreadyFound: false });
    addLanguageName("spanish|espanhol", "spanish", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\b(?:p[rt]|en|port)[. (\\/-]*BR\b/i), uniqConcat(value("portuguese")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", standalone(/\bbr(?:a|azil|azilian)\W+(?:pt|por)\b/i), uniqConcat(value("portuguese")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", standalone(/\b(?:leg(?:endado|endas?)?|dub(?:lado)?|portugu[eèê]se?)[. -]*BR\b/i), uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bleg(?:endado|endas?)\b/i), uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    addLanguageName("portugu[eèê]s[ea]?", "portuguese");
    parser.addHandler("languages", standalone(/\bPT[. -]*(?:PT|ENG?|sub(?:s|titles?))\b/i), uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bpt(?=\.(?:ass|ssa|srt|sub|idx)$)/i), uniqConcat(value("portuguese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bpor\b/i), uniqConcat(value("portuguese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bITA\b/i), uniqConcat(value("italian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?<!w{3}\.\w+\.)IT(?=[ .,/-]+(?:[a-zA-Z]{2}[ .,/-]+){2,})\b/), uniqConcat(value("italian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bit(?=\.(?:ass|ssa|srt|sub|idx)$)/i), uniqConcat(value("italian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("italiano?", "italian", { skipIfFirst: true });
    parser.addHandler("languages", /\bgreek[ .-]*(?:audio|lang(?:uage)?|subs?(?:titles?)?)\b/i, uniqConcat(value("greek")), { skipIfAlreadyFound: false });
    addLanguageName("greek", "greek", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\b(?:GER|DEU)\b/i), uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bde(?=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})\b/i), uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?<=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})de\b/i), uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?<=[ .,/-]+[A-Z]{2}[ .,/-]+)de(?=[ .,/-]+[A-Z]{2}[ .,/-]+)\b/i), uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bde(?=\.(?:ass|ssa|srt|sub|idx)$)/i), uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("german|alem[aã]o", "german", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bRUS?\b(?<!\w-RUS?(?=\s+(?!sub|dub|audio)[a-z{]))/i), uniqConcat(value("russian")), { skipIfAlreadyFound: false });
    addLanguageName("russian|russo", "russian", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bUKR\b/i), uniqConcat(value("ukrainian")), { skipIfAlreadyFound: false });
    addLanguageName("ukrainian", "ukrainian", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bhin(?:di)?\b/i), uniqConcat(value("hindi")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?:(?<!w{3}\.\w+\.)tel(?!\W*aviv)|telugu)\b/i), uniqConcat(value("telugu")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bt[aâ]m(?:il)?\b/i), uniqConcat(value("tamil")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?<!YTS\.)LT\b/), uniqConcat(value("lithuanian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("lithuanian", "lithuanian", { skipIfFirst: true });
    addLanguageName("latvian", "latvian", { skipIfFirst: true });
    addLanguageName("estonian", "estonian", { skipIfFirst: true });
    parser.addHandler("languages", /\b(?:PLDUB|Dubbing.PL|Lektor.PL|Film.Polski)\b/i, uniqConcat(value("polish")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\b(?:Napisy.PL|PLSUB(?:BED)?)\b/i, uniqConcat(value("polish")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", standalone(/\b(?:(?<!w{3}\.\w+\.)PL|pol)\b/i), uniqConcat(value("polish")), { skipIfAlreadyFound: false });
    addLanguageName("polish|polon[eê]s|polaco", "polish", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bCZ[EH]?\b/i), uniqConcat(value("czech")), { skipIfFirst: true, skipIfAlreadyFound: false });
    addLanguageName("czech", "czech", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bslo(?:vak|vakian|subs|[\]_)]?\.\w{2,4}$)\b/i), uniqConcat(value("slovakian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bHU\b/), uniqConcat(value("hungarian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bHUN\b/i), uniqConcat(value("hungarian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("hungarian", "hungarian", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\bROM\b/i), uniqConcat(value("romanian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("romanian", "romanian", { skipFromTitle: true });
    parser.addHandler("languages", /\bRO(?=[ .,/-]*(?:[A-Z]{2}[ .,/-]+)*sub)/i, uniqConcat(value("romanian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bbul\b/i), uniqConcat(value("bulgarian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("bulgarian", "bulgarian", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\bsrp\b/i), uniqConcat(value("serbian")), { skipIfAlreadyFound: false });
    addLanguageName("serbian", "serbian");
    parser.addHandler("languages", standalone(/\bHRV\b/i), uniqConcat(value("croatian")), { skipIfAlreadyFound: false });
    addLanguageName("croatian", "croatian");
    parser.addHandler("languages", /\bHR(?=[ .,/-]*(?:[A-Z]{2}[ .,/-]+)*sub)\b/i, uniqConcat(value("croatian")), { skipIfAlreadyFound: false });
    addLanguageName("slovenian", "slovenian", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\b(?:(?<!w{3}\.\w+\.)NL|dut|holand[eê]s)\b/i), uniqConcat(value("dutch")), { skipIfAlreadyFound: false });
    addLanguageName("dutch", "dutch", { skipFromTitle: true });
    addLanguageName("flemish", "dutch");
    parser.addHandler("languages", standalone(/\b(?:DK|danska|dansub|nordic)\b/i), uniqConcat(value("danish")), { skipIfAlreadyFound: false });
    addLanguageName("danish|dinamarqu[eê]s", "danish", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\bdan\b(?=.*\.(?:srt|vtt|ssa|ass|sub|idx)$)/i), uniqConcat(value("danish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\b(?:(?<!w{3}\.\w+\.|Sci-)FI|finsk|finsub|nordic)\b/i), uniqConcat(value("finnish")), { skipIfAlreadyFound: false });
    addLanguageName("finnish", "finnish", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\b(?:(?<!w{3}\.\w+\.)SE|swe|swesubs?|sv(?:ensk)?|nordic)\b/i), uniqConcat(value("swedish")), { skipIfAlreadyFound: false });
    addLanguageName("swedish|sueco", "swedish", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\b(?:NOR|norsk|norsub|nordic)\b/i), uniqConcat(value("norwegian")), { skipIfAlreadyFound: false });
    addLanguageName("norwegian|noruegu[eê]s|bokm[aå]l", "norwegian", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\b(?:nob|nor(?=[\]_)]?\.\w{2,4}$))\b/i), uniqConcat(value("norwegian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("arabic|[aá]rabe", "arabic", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bara\b/i), uniqConcat(value("arabic")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\barab.*(?:audio|lang(?:uage)?|sub(?:s|titles?)?)\b/i, uniqConcat(value("arabic")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bar(?=\.(?:ass|ssa|srt|sub|idx)$)/i), uniqConcat(value("arabic")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("turkish|turco", "turkish", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\btur\b/i), uniqConcat(value("turkish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("vietnamese", "vietnamese", { skipFromTitle: true });
    parser.addHandler("languages", standalone(/\bvie(?=[\]_)]?\.\w{2,4}$)/i), uniqConcat(value("vietnamese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bind\b/i), uniqConcat(value("indonesian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("indonesian", "indonesian", { skipFromTitle: true });
    addLanguageName("thai|tailand[eê]s", "thai", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\b(THA|tha)\b/), uniqConcat(value("thai")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("malay", "malay", { skipIfFirst: true });
    parser.addHandler("languages", standalone(/\bmay(?=[\]_)]?\.\w{2,4}$)|(?<=subs?\([a-z,]+)may\b/i), uniqConcat(value("malay")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", standalone(/\bheb\b/i), uniqConcat(value("hebrew")), { skipFromTitle: true, skipIfAlreadyFound: false });
    addLanguageName("hebrew|hebraico", "hebrew", { skipFromTitle: true });
    addLanguageName("persian|persa", "persian", { skipFromTitle: true });

    // infer pt language based on season/episode naming
    parser.addHandler("languages", ({ title, result, matched }) => {
        if (!result.languages || ["portuguese", "spanish"].every(l => !result.languages.includes(l))) {
            if ((matched.episodes && matched.episodes.rawMatch.match(/capitulo|ao/i)) ||
                title.match(/dublado/i)) {
                result.languages = (result.languages || []).concat("portuguese");
            }
        }
        return { matchIndex: 0 };
    });

    // Dubbed
    parser.addHandler("dubbed", /\b(?:DUBBED|dublado|dubbing|DUBS?)\b/i, boolean);
    parser.addHandler("dubbed", ({ result }) => {
        if (result.languages && ["multi audio", "dual audio"].some(l => result.languages.includes(l))) {
            result.dubbed = true;
        }
        return { matchIndex: 0 };
    });

    // Group
    parser.addHandler("group", /- ?(?!\d+$|S\d+|\d+x|ep?\d+|[^[]+]$|(?:OVA|OAV|OAD|ONA|SP|Specials?)\d*(?:\.\w{2,4})?$|第\d+[話话集])([^\-. []+[^\-. [)\]\d][^\-. [)\]]*)(?:\[[\w.-]+])?(?=\.\w{2,4}$|$)/i, { skipIfNothingBefore: true });
    parser.addHandler("group", /^\[([^[\]]+)]/);
    parser.addHandler("group", /\(([\w-]+)\)(?:$|\.\w{2,4}$)/);
    parser.addHandler("group", ({ result, matched }) => {
        if (matched.group && matched.group.rawMatch.match(/^\[.+]$/)) {
            const endIndex = matched.group && matched.group.matchIndex + matched.group.rawMatch.length || 0;

            // remove anime group match if some other parameter is contained in it, since it's a false positive.
            if (Object.keys(matched)
                .some(key => matched[key].matchIndex && matched[key].matchIndex < endIndex)) {
                delete result.group;
            }
        }
        return { matchIndex: 0 };
    });

    // Extension
    parser.addHandler("extension", new RegExp(`\\.(${VIDEO_EXTENSIONS}|iso|vob|${SUBTITLE_EXTENSIONS}|idx|ttxt|txt|nfo|html)$`, "i"), lowercase);

    if (exclude) {
        parser.removeHandlers(exclude);
    }
};
