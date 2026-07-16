const { none, value, integer, boolean, lowercase, uppercase, date, range, yearRange, array, uniqConcat } = require("./transformers");

exports.addDefaults = /** @type Parser */ parser => {

    // Episode code
    parser.addHandler("episodeCode", /[[(]([a-z0-9]{8}|[A-Z0-9]{8})[\])](?=\.[a-zA-Z0-9]{1,5}$|$)/, uppercase, { remove: true });
    parser.addHandler("episodeCode", /\[(?=[A-Z]+\d|\d+[A-Z])([A-Z0-9]{8})]/, uppercase, { remove: true });

    // Resolution
    parser.addHandler("resolution", /\b[([]?4k[)\]]?\b/i, value("4k"), { remove: true });
    parser.addHandler("resolution", /21600?[pi]/i, value("4k"), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("resolution", /[([]?3840x\d{4}[)\]]?/i, value("4k"), { remove: true });
    parser.addHandler("resolution", /[([]?1920x\d{3,4}[)\]]?/i, value("1080p"), { remove: true });
    parser.addHandler("resolution", /[([]?1280x\d{3}[)\]]?/i, value("720p"), { remove: true });
    parser.addHandler("resolution", /[([]?\d{3,4}x(\d{3,4})[)\]]?/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(480|720|1080)0[pi]/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(?:BD|HD|M)(720|1080|2160)/, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(480|576|720|1080|2160)[pi]/i, value("$1p"), { remove: true });
    parser.addHandler("resolution", /(?:^|\D)(\d{3,4})[pi]/i, value("$1p"), { remove: true });

    // Year
    parser.addHandler("date", /(?<=\W|^)([([]?(?:19[6-9]|20[012])[0-9]([. \-/\\])(?:0[1-9]|1[012])\2(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYY MM DD"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)/, date("DD MM YYYY"), { remove: true });
    parser.addHandler("date", /(?<=\W)([([]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:19[6-9]|20[012])[0-9][)\]]?)(?=\W|$)/, date("MM DD YYYY"), { remove: true });
    parser.addHandler("date", /(?<=\W)([([]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/, date("MM DD YY"), { remove: true });
    parser.addHandler("date", /(?<=\W)([([]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:[0][1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/, date("DD MM YY"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:19[7-9]|20[012])[0-9][)\]]?)(?=\W|$)/i, date("DD MMM YYYY"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:0[1-9]|[0126789][0-9])[)\]]?)(?=\W|$)/i, date("DD MMM YY"), { remove: true });
    parser.addHandler("date", /(?<=\W|^)([([]?20[012][0-9](?:0[1-9]|1[012])(?:0[1-9]|[12][0-9]|3[01])[)\]]?)(?=\W|$)/, date("YYYYMMDD"), { remove: true });

    // Year
    parser.addHandler("year", /[([*]?[ .]?((?:19\d|20[012])\d[ .]?-[ .]?(?:19\d|20[012])\d)(?:\s?[*)\]])?/, yearRange, { remove: true });
    parser.addHandler("year", /[([*][ .]?((?:19\d|20[012])\d[ .]?-[ .]?\d{2})(?:\s?[*)\]])?/, yearRange, { remove: true });
    parser.addHandler("year", /[([*]?(?!^)(?<!\d|Cap[. ]?)((?:19\d|20[012])\d)(?!\d|kbps)[*)\]]?/i, integer, { remove: true });
    parser.addHandler("year", /^[([]?((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?/i, integer, { remove: true });

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

    // RemasteredREKONSTRUKCJA
    parser.addHandler("remastered", /\bRemaster(?:ed)?\b/i, boolean);
    parser.addHandler("remastered", /\b[[(]?REKONSTRUKCJA[\])]?\b/i, boolean);

    // Unrated
    parser.addHandler("unrated", /\bunrated|uncensored\b/i, boolean);

    // Region
    parser.addHandler("region", /R\d\b/, none, { skipIfFirst: true });

    // Source
    parser.addHandler("source", /\b(?:H[DQ][ .-]*)?CAM(?:H[DQ])?(?:[ .-]*Rip)?\b/i, value("CAM"), { remove: true });
    parser.addHandler("source", /\b(?:H[DQ][ .-]*)?S[ .-]+print/i, value("CAM"), { remove: true });
    parser.addHandler("source", /\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b/i, value("TeleSync"), { remove: true });
    parser.addHandler("source", /\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b/, value("TeleCine"), { remove: true });
    parser.addHandler("source", /\bBlu[ .-]*Ray\b(?=.*remux)/i, value("BluRay REMUX"), { remove: true });
    parser.addHandler("source", /(?:BD|BR|UHD)[- ]?remux/i, value("BluRay REMUX"), { remove: true });
    parser.addHandler("source", /(?<=remux.*)\bBlu[ .-]*Ray\b/i, value("BluRay REMUX"), { remove: true });
    parser.addHandler("source", /\bBlu[ .-]*Ray\b(?![ .-]*Rip)/i, value("BluRay"), { remove: true });
    parser.addHandler("source", /\bUHD[ .-]*Rip\b/i, value("UHDRip"), { remove: true });
    parser.addHandler("source", /\bHD[ .-]*Rip\b/i, value("HDRip"), { remove: true });
    parser.addHandler("source", /\bMicro[ .-]*HD\b/i, value("HDRip"), { remove: true });
    parser.addHandler("source", /\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b/i, value("BRRip"), { remove: true });
    parser.addHandler("source", /\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|[[(]BD[\]) .,-]/i, value("BDRip"), { remove: true });
    parser.addHandler("source", /\b(?:HD[ .-]*)?DVD[ .-]*Rip\b/i, value("DVDRip"), { remove: true });
    parser.addHandler("source", /\bVHS[ .-]*Rip\b/i, value("DVDRip"), { remove: true });
    parser.addHandler("source", /\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b/i, value("SCR"), { remove: true });
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
    parser.addHandler("source", /\b(?:DL|WEB|BD|BR)MUX\b/i, { remove: true });
    parser.addHandler("source", /\b(DivX|XviD)\b/, { remove: true });

    // Video depth
    parser.addHandler("bitDepth", /(?:8|10|12)[- ]?bit/i, lowercase, { remove: true });
    parser.addHandler("bitDepth", /\bhevc\s?10\b/i, value("10bit"));
    parser.addHandler("bitDepth", /\bhdr10\b/i, value("10bit"));
    parser.addHandler("bitDepth", /\bhi10\b/i, value("10bit"));
    parser.addHandler("bitDepth", ({ result }) => {
        if (result.bitDepth) {
            result.bitDepth = result.bitDepth.replace(/[ -]/, "");
        }
    });

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
    parser.addHandler("codec", /\b(?:dvix|mpeg2|divx|xvid|avc)\b/i, lowercase, { remove: true, skipIfAlreadyFound: false });
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

    // Group
    parser.addHandler("group", /- ?(?!\d+$|S\d+|\d+x|ep?\d+|[^[]+]$)([^\-. []+[^\-. [)\]\d][^\-. [)\]]*)(?:\[[\w.-]+])?(?=\.\w{2,4}$|$)/i, { remove: true });

    // Container
    parser.addHandler("container", /\.?[[(]?\b(MKV|AVI|MP4|WMV|MPG|MPEG)\b[\])]?/i, lowercase);

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
    // Detect multi-season episode ranges like S01E001-S04E126 and extract explicit
    // season list and episode-range bounds.  This produces richer metadata so
    // callers can decide whether to treat the upload as a multi-season pack.
    parser.addHandler("seasons", ({ title, result }) => {
        if (result.seasons) return null;
        const m = title.match(/S(\d{1,2})E(\d{1,3})\s*[-–~]\s*S(\d{1,2})E?(\d{1,3})/i);
        if (m) {
            const startSeason = parseInt(m[1], 10);
            const startEp = parseInt(m[2], 10);
            const endSeason = parseInt(m[3], 10);
            const endEp = parseInt(m[4], 10);
            if ([startSeason, startEp, endSeason, endEp].every(n => !Number.isNaN(n))) {
                // build seasons array [start..end]
                const seasons = [];
                for (let s = startSeason; s <= endSeason; s++) seasons.push(s);
                result.seasons = seasons;
                // prefer the first season/episode as the representative single-episode
                // (keeps existing tests that expect S1 E1), but also expose full range
                result.season = startSeason;
                result.episode = startEp;
                result.episodeRangeStart = startEp;
                result.episodeRangeEnd = endEp;
                result.seasonConfidence = "range";
                result.batch = true;
                result.isBatch = true;
                return { rawMatch: m[0], matchIndex: m.index, remove: true };
            }
        }
        return null;
    });

    // Handle trailing single-pipe numeric markers like "| 041" as episode numbers.
    // Avoid accidentally matching common resolution numbers (720, 1080, 2160, etc.).
    parser.addHandler("episodes", ({ title, result }) => {
        if (result.episodes) return null;
        const m = title.match(/\|\s*0*(\d{1,4})\s*$/);
        if (m) {
            const n = parseInt(m[1], 10);
            const skipRes = new Set([480, 576, 720, 1080, 1440, 2160, 3840]);
            if (skipRes.has(n)) return null;
            result.episodes = [n];
            return { rawMatch: m[0], matchIndex: m.index, remove: true };
        }
        return null;
    });
    parser.addHandler("seasons", /(?:complete\W|seasons?\W|\W|^)((?:s\d{1,2}[., +/\\&-]+)+s\d{1,2}\b)/i, range, { remove: true });
    parser.addHandler("seasons", /(?:complete\W|seasons?\W|\W|^)[([]?(s\d{2,}-\d{2,}\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:complete\W|seasons?\W|\W|^)[([]?(s[1-9]-[2-9]\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?(?:seasons?|[Сс]езони?|sezon|temporadas?|stagioni)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}[, /\\&]+)+\d{1,2}\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?(?:seasons|[Сс]езони?|sezon|temporadas?|stagioni)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}[. -]+)+0?[1-9]\d?\b)[)\]]?/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?season[. ]?[([]?((?:\d{1,2}[. -]+)+[1-9]\d?\b)[)\]]?(?!.*\.\w{2,4}$)/i, range, { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?\bseasons?\b[. -]?(\d{1,2}[. -]?(?:to|thru|and|\+|:)[. -]?\d{1,2})\b/i, range, { remove: true });
    parser.addHandler("seasons", /(\d{1,2})(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?)(?:\W?\D|$)/i, array(integer));
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?(?:saison|seizoen|sezon(?:SO?)?|stagione|season|series|temp(?:orada)?):?[. ]?(\d{1,2})/i, array(integer));
    parser.addHandler("seasons", /[Сс]езон:?[. _]?№?(\d{1,2})(?!\d)/i, array(integer));
    parser.addHandler("seasons", /(?:\D|^)(\d{1,2})Â?[°ºªa]?[. ]*temporada/i, array(integer), { remove: true });
    parser.addHandler("seasons", /t(\d{1,3})(?:[ex]+|$)/i, array(integer), { remove: true });
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete)?(?:\W|^)so?([01]?[0-5]?[1-9])(?:[\Wex]|\d{2}\b)/i, array(integer), { skipIfAlreadyFound: false });
    parser.addHandler("seasons", /(?:so?|t)(\d{1,2})[. ]?[xх-]?[. ]?(?:e|x|х|ep|-|\.)[. ]?\d{1,4}(?:[abc]|v0?[1-4]|\D|$)/i, array(integer));
    parser.addHandler("seasons", /(?:(?:\bthe\W)?\bcomplete\W)?(?:\W|^)(\d{1,2})[. ]?(?:st|nd|rd|th)[. ]*season/i, array(integer));
    parser.addHandler("seasons", /(?:\D|^)(\d{1,2})[Xxх]\d{1,3}(?:\D|$)/, array(integer));
    parser.addHandler("seasons", /\bSn([1-9])(?:\D|$)/, array(integer));
    parser.addHandler("seasons", /[[(](\d{1,2})\.\d{1,3}[)\]]/, array(integer));
    parser.addHandler("seasons", /-\s?(\d{1,2})\.\d{2,3}\s?-/, array(integer));
    parser.addHandler("seasons", /^(\d{1,2})\.\d{2,3} - /, array(integer), { skipIfBefore: ["year, source", "resolution"] });
    parser.addHandler("seasons", /(?:^|\/)(?!20-20)(\d{1,2})-\d{2}\b(?!-\d)/, array(integer));
    parser.addHandler("seasons", /[^\w-](\d{1,2})-\d{2}(?=\.\w{2,4}$)/, array(integer));
    parser.addHandler("seasons", /(?<!\bEp?(?:isode)? ?\d+\b.*)\b(\d{2})[ ._]\d{2}(?:.F)?\.\w{2,4}$/, array(integer));
    parser.addHandler("seasons", /\bEp(?:isode)?\W+(\d{1,2})\.\d{1,3}\b/i, array(integer));

    // Donghua: Chinese season marker 第X季 (e.g. [百炼成神 第3季])
    // skipFromTitle prevents the match position from corrupting endOfTitle
    parser.addHandler("seasons", /第(\d{1,2})季/, array(integer), { skipFromTitle: true });

    // Donghua: Roman numeral after arc/arc-title section: "- Arc Name III" before episode range
    // e.g. "Battle Through The Heavens - Nian Fan III (106-157)" → season 3
    // Only unambiguous Roman numerals (III+) to avoid false positives
    parser.addHandler("seasons", ({ title, result }) => {
        if (result.seasons) return null;
        const ROMAN_MAP = { 'XIII': 13, 'XII': 12, 'XI': 11, 'IX': 9, 'VIII': 8, 'VII': 7, 'VI': 6, 'IV': 4, 'III': 3 };
        const m = title.match(/-\s+[A-Za-z][A-Za-z\s.'-]{2,}\s+(XIII|XII|XI|IX|VIII|VII|VI|IV|III)\s*[([\[]/);
        if (m) {
            const season = ROMAN_MAP[m[1]];
            if (season) {
                result.seasons = [season];
                return { matchIndex: title.indexOf(m[0]) };
            }
        }
        return null;
    });

    // Donghua: Inline Roman numeral II before dash separator
    // e.g. "Douluo Dalu II - Soul Land 2 - 112" → season 2
    parser.addHandler("seasons", ({ title, result }) => {
        if (result.seasons) return null;
        const m = title.match(/\s(II)\s+-\s+[A-Za-z]/);
        if (m) {
            result.seasons = [2];
            return { matchIndex: title.indexOf(m[0]) };
        }
        return null;
    });

    // Bare trailing Roman-numeral season: "Sword Art Online II - 01", "Title III 12".
    // The bracketed/dash-before-letter handlers above miss a lone Roman numeral that
    // sits between the title and the episode number. Restricted to unambiguous
    // multi-char numerals (II–XIII) preceded by a title word, so single-letter
    // I/V/X can't false-positive on ordinary title tokens.
    parser.addHandler("seasons", ({ title, result }) => {
        if (result.seasons) return null;
        const ROMAN_MAP = { XIII: 13, XII: 12, XI: 11, IX: 9, VIII: 8, VII: 7, VI: 6, IV: 4, III: 3, II: 2 };
        const m = title.match(/\w[\w.'-]*\s+(XIII|XII|XI|IX|VIII|VII|VI|IV|III|II)(\s*[-.]?\s*)(\d{1,4})(?:\D|$)/);
        if (m && ROMAN_MAP[m[1]]) {
            result.seasons = [ROMAN_MAP[m[1]]];
            // Only claim the episode for a purely space-separated number ("Douluo Dalu
            // II 26") that the downstream episode handlers can't parse. Dash/dot forms
            // ("... II - 01") are left to those handlers, which also trim the episode
            // from the title — claiming it here would leave "- 01" stuck in the title.
            const separatorIsSpaceOnly = /^\s+$/.test(m[2]);
            if (separatorIsSpaceOnly && !result.episodes) {
                const ep = parseInt(m[3], 10);
                if (ep >= 1 && ep <= 9999 && !(ep >= 1900 && ep <= 2099)) {
                    result.episodes = [ep];
                }
            }
            return { matchIndex: 0 };
        }
        return null;
    });

    // adds single season info if its there"s only single season
    parser.addHandler("season", ({ result }) => {
        if (result.seasons && result.seasons.length === 1) {
            result.season = result.seasons[0];
        }
    });

    // Fan-sub Pattern 1: Bracket-enclosed romanized title ending with a Roman numeral
    // e.g. [Jade Dynasty Ⅱ] → season 2 | [Sword Art Online III] → season 3
    // Handles both Unicode numerals (Ⅱ Ⅲ…) and ASCII Roman numerals (II III IV…)
    // Intentionally omits bare 'I' to avoid false positives on single-letter suffixes
    parser.addHandler("seasons", ({ title, result }) => {
        if (result.seasons) return null;

        const unicodeRomanMap = {
            '\u2160': 1, '\u2161': 2, '\u2162': 3, '\u2163': 4, '\u2164': 5,
            '\u2165': 6, '\u2166': 7, '\u2167': 8, '\u2168': 9, '\u2169': 10
        };
        const asciiRomanMap = {
            'XIII': 13, 'XII': 12, 'XI': 11, 'IX': 9, 'VIII': 8, 'VII': 7,
            'VI': 6, 'IV': 4, 'III': 3, 'II': 2, 'X': 10, 'V': 5
        };

        // Unicode Roman numerals first — most unambiguous (Ⅱ Ⅲ Ⅳ …)
        const unicodeMatch = title.match(/\[[A-Za-z][A-Za-z0-9\s:'.-]{2,?}\s+([\u2160-\u2169])\]/);
        if (unicodeMatch) {
            const season = unicodeRomanMap[unicodeMatch[1]];
            if (season) {
                result.seasons = [season];
                return { matchIndex: title.indexOf(unicodeMatch[0]) };
            }
        }

        // ASCII Roman numerals — longest alternatives first to prevent II matching inside III
        const asciiMatch = title.match(/\[[A-Za-z][A-Za-z0-9\s:'.-]{2,?}\s+(XIII|XII|XI|IX|VIII|VII|VI|IV|III|II|X{1,3}|V)\]/);
        if (asciiMatch) {
            const season = asciiRomanMap[asciiMatch[1]];
            if (season) {
                result.seasons = [season];
                return { matchIndex: title.indexOf(asciiMatch[0]) };
            }
        }

        return null;
    });

    // Fan-sub Pattern 2: Parenthesized title ending with a 1–2 digit season number
    // e.g. (Sword of Coming 2) → season 2
    // Requires ≥3 alpha chars before the number to avoid matching (720p) or (2024)
    // remove:true strips the matched group so the trailing number isn't re-parsed as an episode
    parser.addHandler("seasons", /\([A-Za-z][A-Za-z\s:'.-]{2,}\s+(\d{1,2})\)/, array(integer), { remove: true });

    // Bare "S\d" suffix — e.g. "Apotheosis S2", "Battle Through the Heavens S4"
    // None of the above handlers catch a lone S\d at end-of-title (they all require an episode
    // component or a multi-season range following the S).  This handler fires last as a safety
    // net; skipIfAlreadyFound ensures it cannot override a season already extracted above.
    parser.addHandler("seasons", /(?:^|\s)S(\d{1,2})(?:\s|$)/i, array(integer), { skipIfAlreadyFound: true });

    // Ensure season (singular) is updated after Fan-sub Patterns 1 & 2 which run after the first
    // season singular handler — covers cases like (Sword of Coming 2) setting seasons=[2] late.
    parser.addHandler("season", ({ result }) => {
        if (!result.season && result.seasons && result.seasons.length === 1) {
            result.season = result.seasons[0];
        }
    });

    // Donghua SP/Special episode: "Title SP1" → episode 1, strips SP tag from title
    parser.addHandler("episodes", /\bSP(\d{1,2})\b/, array(integer), { remove: true, skipIfAlreadyFound: false });

    // Donghua subtitle-episode: "[Group] Title/Alt - SubTitle N (resolution)"
    // e.g. "[Impromptu] Ling Long/ Spirit Cage/ Ling Cage - Incarnation 10 (1080p)" → episode 10
    parser.addHandler("episodes", /^\[[^\]]+\](?:[^[\]|-]+\/)+[^[\]|-]+-\s+[A-Za-z][A-Za-z\s]{1,25}\s+(\d{1,3})\s*[([]/i, array(integer), { skipIfAlreadyFound: true });

    // Episode
    parser.addHandler("episodes", /(?:[\W\d]|^)e[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|e){1,2}[ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)ep[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|ep){1,2}[ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)\d+[xх][ .]?[([]?(\d{1,3}(?:[ .]?[xх][ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:episodes?|[Сс]ерии:?)[ .]?[([]?(\d{1,3}(?:[ .+]*[&+][ .]?\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /[([]?(?:\D|^)(\d{1,3}[ .]?ao[ .]?\d{1,3})[)\]]?(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:[\W\d]|^)(?:e|eps?|episodes?|[Сс]ерии:?|\d+[xх])[ .]*[([]?(\d{1,4}(?:-\d{1,4})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /(?:so?|t)\d{1,2}[. ]?[xх-]?[. ]?(?:e|x|х|ep)[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)/i, array(integer));
    parser.addHandler("episodes", /(?:so?|t)\d{1,2}\s?[-.]\s?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)/i, array(integer));
    parser.addHandler("episodes", /\b(?:so?|t)\d{2}(\d{2})\b/i, array(integer));
    parser.addHandler("episodes", /(?:\W|^)(\d{1,3}(?:[ .]*~[ .]*\d{1,3})+)(?:\W|$)/i, range);
    parser.addHandler("episodes", /-\s(\d{1,3}[ .]*-[ .]*\d{1,3})(?!-\d)(?:\W|$)/i, range);
    parser.addHandler("episodes", /s\d{1,2}\s?\((\d{1,3}[ .]*-[ .]*\d{1,3})\)/i, range);
    parser.addHandler("episodes", /(?:^|\/)(?!20-20)\d{1,2}-(\d{2})\b(?!-\d)/, array(integer));
    parser.addHandler("episodes", /(?<!\d-)\b\d{1,2}-(\d{2})(?=\.\w{2,4}$)/, array(integer));
    parser.addHandler("episodes", /(?<=^\[.+].+)[. ]+-[. ]+(\d{1,4})[. ]+(?=\W)/i, array(integer));
    parser.addHandler("episodes", /(?<!(?:seasons?|[Сс]езони?)\W*)(?:[ .([-]|^)(\d{1,3}(?:[ .]?[,&+~][ .]?\d{1,3})+)(?:[ .)\]-]|$)/i, range);
    parser.addHandler("episodes", /(?<!(?:seasons?|[Сс]езони?)\W*)(?!20-20)(?:[ .([-]|^)(\d{1,3}(?:-\d{1,3})+)(?:[ .)(\]]|-\D|$)/i, range);
    // Bracketed numeric ranges with optional trailing markers like "Fin" or "End"
    // e.g. [01-17 Fin], [01-26], [01-17 End]
    parser.addHandler("episodes", /\[(\s*0*\d{1,3}\s*(?:[-–~]|to)\s*0*\d{1,3}(?:\s*(?:fin|end|fin\.|end\.)?)?\s*)\]/i, range, { remove: true });
    // Absolute-episode range hint: a parenthetical/bracketed 3+ digit range that
    // annotates a per-season batch pack, e.g. "Apotheosis S2 01-10 (053-062)" →
    // {start:53,end:62}. Coexists with the main per-season range ("01-10") without
    // replacing it, so it's stored as a separate observational field (no remove,
    // matchIndex 0). 2-digit ranges are intentionally excluded — ambiguous with an
    // ordinary per-season range. Callers use this to match flat-catalog absolute
    // episode requests against season-labelled packs.
    parser.addHandler("absoluteRangeHint", ({ title, result }) => {
        if (result.absoluteRangeHint) return null;
        const m = title.match(/[([]\s*(\d{3,})\s*-\s*(\d{3,})\s*[)\]]/);
        if (m) {
            const start = parseInt(m[1], 10);
            const end = parseInt(m[2], 10);
            if (start < end) {
                result.absoluteRangeHint = { start, end };
                return { matchIndex: 0 };
            }
        }
        return null;
    });

    // Large zero-padded ranges in parens/brackets, incl. 4-digit: "(0001-1000)", "[001-500]".
    // The range handlers above cap at 3 digits and don't accept parens, so long-running
    // batch packs (One Piece, Naruto absolute numbering) slip through. Stored as a
    // two-element [start, end] sentinel — episodeRangeStart/End derives the bounds — to
    // avoid materializing a thousand-element episodes array.
    parser.addHandler("episodes", ({ title, result }) => {
        if (result.episodes) return null;
        const m = title.match(/[([]\s*0*(\d{1,4})\s*[-–~]\s*0*(\d{1,4})\s*[)\]]/);
        if (m) {
            const start = parseInt(m[1], 10);
            const end = parseInt(m[2], 10);
            if (start > 0 && end > 0 && start < end && end <= 9999) {
                result.episodes = [start, end];
                result.episodeRangeStart = start;
                result.episodeRangeEnd = end;
                return { rawMatch: m[0], matchIndex: m.index, remove: true };
            }
        }
        return null;
    });
    // Bare numeric range with no keyword/brackets, e.g. "One Piece 001-1000",
    // "Show 01-26". Restricted to zero-padded (0N) or both-sides ≥3-digit values so
    // it can't misread "Part 2 - 11" (small, unpadded, spaced) as a range — that
    // must stay a single episode. Stored as a [start, end] sentinel.
    parser.addHandler("episodes", ({ title, result }) => {
        if (result.episodes) return null;
        const m = title.match(/(?:^|[\s([])(\d{1,4})\s*[-–]\s*(\d{1,4})(?=[\s)\]]|$)/);
        if (m) {
            const [a, b] = [m[1], m[2]];
            const start = parseInt(a, 10);
            const end = parseInt(b, 10);
            const zeroPadded = /^0\d/.test(a) || /^0\d/.test(b);
            const bothLarge = start >= 100 && end >= 100;
            if ((zeroPadded || bothLarge) && start > 0 && end > 0 && start < end && end <= 9999) {
                result.episodes = [start, end];
                result.episodeRangeStart = start;
                result.episodeRangeEnd = end;
                return { rawMatch: m[0], matchIndex: m.index };
            }
        }
        return null;
    });
    parser.addHandler("episodes", /\bEp(?:isode)?\W+\d{1,2}\.(\d{1,3})\b/i, array(integer));
    parser.addHandler("episodes", /(?:\b[ée]p?(?:isode)?|[Ээ]пизод|[Сс]ер(?:ии|ия|\.)?|caa?p(?:itulo)?|epis[oó]dio)[. ]?[-:#№]?[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\W|$)/i, array(integer));
    parser.addHandler("episodes", /\b(\d{1,3})(?:-?я)?[ ._-]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)/i, array(integer));
    parser.addHandler("episodes", /(?:\D|^)\d{1,2}[. ]?[Xxх][. ]?(\d{1,3})(?:[abc]|v0?[1-4]|\D|$)/, array(integer));
    parser.addHandler("episodes", /[[(]\d{1,2}\.(\d{1,3})[)\]]/, array(integer));
    parser.addHandler("episodes", /\b[Ss](?:eason\W?)?\d{1,2}[ .](\d{1,2})\b/, array(integer));
    parser.addHandler("episodes", /-\s?\d{1,2}\.(\d{2,3})\s?-/, array(integer));
    parser.addHandler("episodes", /^\d{1,2}\.(\d{2,3}) - /, array(integer), { skipIfBefore: ["year, source", "resolution"] });
    parser.addHandler("episodes", /(?<=\D|^)(\d{1,3})[. ]?(?:of|из|iz)[. ]?\d{1,3}(?=\D|$)/i, array(integer));
    parser.addHandler("episodes", /\b\d{2}[ ._-](\d{2})(?:.F)?\.\w{2,4}$/, array(integer));
    parser.addHandler("episodes", /(?<!^)\[(\d{2,3})](?!(?:\.\w{2,4})?$)/, array(integer));
    parser.addHandler("episodes", /\bodc[. ]+(\d{1,3})\b/i, array(integer));
    parser.addHandler("episodes", /\|\s*0*(\d{1,4})\s*\|/, array(integer), {
        skipIfAlreadyFound: false,
        remove: true,
    });

    // Anime arc pattern: "Title: Arc Name - 01" (colon not preceded by S followed by digits)
    // Only match colons followed by arc name text (not season indicators like "S3")
    // Uses a negative lookahead to avoid matching when the arc section contains a
    // range like "- 01 - 12" (which indicates a multi-episode pack, not a single arc episode)
    parser.addHandler("episodes", /(?<!S\d)(?<!S\d\d):(?!\s*[^:\[\]]*?-\s*\d{1,4}\s*-\s*\d{1,4})\s*[^:\[\]]*?\s+-\s+(\d{1,3})(?:\s*\[|$|\s+(?:\d{3,4}p|HEVC|x264|x265|WEB|BD|DL))/i, array(integer), { skipIfAlreadyFound: false });

    // Arc / subtitle name (anime multi-arc shows).
    // Captures the subtitle that follows a colon when an episode marker (" - NN")
    // trails it — the exact shape the anime-arc episode handler above matches, e.g.
    //   "Sword Art Online: Alicization - War of Underworld - 05" → arc "Alicization - War of Underworld"
    // Tying arc to the trailing "- NN" keeps it from firing on plain colon titles
    // like "The Fast and the Furious: Tokyo Drift" (no episode) — those have no arc.
    // Observational only (matchIndex 0): never mutates title/episode parsing.
    parser.addHandler("arc", ({ title, result }) => {
        if (result.arc) return null;
        const m = title.match(/(?<!S\d)(?<!S\d\d):\s*([^:\[\]]+?)\s+-\s+\d{1,4}(?:\D|$)/i);
        if (m) {
            const arc = m[1].trim().replace(/\s{2,}/g, " ");
            if (arc && !/^\d+$/.test(arc)) {
                result.arc = arc;
                return { matchIndex: 0 };
            }
        }
        return null;
    });

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
            const matches = Array.from(beginningTitle.matchAll(/(?<!movie\W*|film\W*|^)(?:[ .]+-[ .]+|[([][ .]*)(\d{1,4})(?:a|b|v\d|\.\d)?(?:\W|$)(?!movie|film|\d+)/gi)).pop() ||
                middleTitle.match(/^(?:[([-][ .]?)?(\d{1,4})(?:a|b|v\d)?(?!\Wmovie|\Wfilm|-\d)(?:\W|$)/i);

            if (matches) {
                result.episodes = [matches[matches.length - 1]]
                    .map(group => group.replace(/\D/g, ""))
                    .map(group => parseInt(group, 10));
                return { matchIndex: title.indexOf(matches[0]) };
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

    parser.addHandler("episodeRangeStart", ({ result }) => {
        if (result.episodes && result.episodes.length > 1) {
            result.episodeRangeStart = result.episodes[0];
            result.episodeRangeEnd = result.episodes[result.episodes.length - 1];
        }
    });

    parser.addHandler("complete", /(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b/i, boolean);
    parser.addHandler("complete", /(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bmini[ .-]?series\b/i, boolean);
    parser.addHandler("complete", /(?:\bthe\W)?(?:\bcomplete|full|all)\b.*\b(?:series|seasons|collection|episodes|set|pack|movies)\b/i, boolean);
    parser.addHandler("complete", /\b(?:series|seasons|movies?)\b.*\b(?:complete|collection)\b/i, boolean);
    parser.addHandler("complete", /(?:\bthe\W)?\bultimate\b[ .]\bcollection\b/i, boolean, { skipIfAlreadyFound: false });
    parser.addHandler("complete", /\bcollection\b.*\b(?:set|pack|movies)\b/i, boolean);
    parser.addHandler("complete", /\b(collection|completa)\b/i, boolean, { skipFromTitle: true });
    parser.addHandler("complete", /\bkolekcja\b(?:\Wfilm(?:y|ów|ow)?)?/i, boolean, { remove: true });
    parser.addHandler("complete", /duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|heptalogy|anthology|saga/i, boolean, { skipIfAlreadyFound: false });

    // Batch marker — detect labelled "batch" packs (e.g., "[Group] Title (batch)", "[Group] Title [BATCH]")
    // We set a lightweight flag so callers can treat these uploads as multi-episode packs
    parser.addHandler("batch", ({ title, result }) => {
        if (result.episodes || result.batch) return null;
        const m = title.match(/\bbatch\b/i);
        if (m) {
            result.batch = true;
            return { matchIndex: m.index };
        }
        return null;
    });

    parser.addHandler("isBatch", ({ result, title }) => {
        if (result.isMovie) return;
        if (
            (result.seasons?.length > 0 && !result.episodes?.length) ||
            result.batch ||
            (result.episodes?.length > 1) ||
            result.complete ||
            /\b(?:batch|pack|complete)\b/i.test(title)
        ) {
            result.isBatch = true;
        }
    });

    parser.addHandler("isMovie", ({ result, title }) => {
        if (/\b(?:movie|film|collection|compil)\b/i.test(title) && !result.seasons?.length) {
            result.isMovie = true;
        }
    });

    // Language
    parser.addHandler("languages", /\bmulti(?:ple)?[ .-]*(?:su?$|sub\w*|dub\w*)\b|msub/i, uniqConcat(value("multi subs")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\bmulti(?:ple)?[ .-]*(?:lang(?:uages?)?|audio|VF2)?\b/i, uniqConcat(value("multi audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\btri(?:ple)?[ .-]*(?:audio|dub\w*)\b/i, uniqConcat(value("multi audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bdual[ .-]*(?:au?$|[aá]udio|line)\b/i, uniqConcat(value("dual audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bdual\b(?![ .-]*sub)/i, uniqConcat(value("dual audio")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bengl?(?:sub[A-Z]*)?\b/i, uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\beng?sub[A-Z]*\b/i, uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bing(?:l[eéê]s)?\b/i, uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\benglish\W+(?:subs?|sdh|hi)\b/i, uniqConcat(value("english")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bEN\b/i, uniqConcat(value("english")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\benglish?\b/i, uniqConcat(value("english")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:JP|JAP|JPN)\b/i, uniqConcat(value("japanese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(japanese|japon[eê]s)\b/i, uniqConcat(value("japanese")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:KOR|kor[ .-]?sub)\b/i, uniqConcat(value("korean")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(korean|coreano)\b/i, uniqConcat(value("korean")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:traditional\W*chinese|chinese\W*traditional)(?:\Wchi)?\b/i, uniqConcat(value("taiwanese")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\bzh-hant\b/i, uniqConcat(value("taiwanese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:mand[ae]rin|ch[sn])\b/i, uniqConcat(value("chinese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bCH[IT]\b/, uniqConcat(value("chinese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(chinese|chin[eê]s|chi)\b/i, uniqConcat(value("chinese")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bzh-hans\b/i, uniqConcat(value("chinese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bFR(?:ench|a|e|anc[eê]s)?\b/i, uniqConcat(value("french")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(Truefrench|VF[FI])\b/i, uniqConcat(value("french")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(VOST(?:FR?|A)?|SUBFRENCH)\b/i, uniqConcat(value("french")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bspanish\W?latin|american\W*(?:spa|esp?)/i, uniqConcat(value("latino")), { skipFromTitle: true, skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\b(?:audio.)?lat(?:i|ino)?\b/i, uniqConcat(value("latino")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:audio.)?(?:ESP|spa|(en[ .]+)?espa[nñ]ola?|castellano)\b/i, uniqConcat(value("spanish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bes(?=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})\b/i, uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?<=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})es\b/i, uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?<=[ .,/-]+[A-Z]{2}[ .,/-]+)es(?=[ .,/-]+[A-Z]{2}[ .,/-]+)\b/i, uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bes(?=\.(?:ass|ssa|srt|sub|idx)$)/i, uniqConcat(value("spanish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bspanish\W+subs?\b/i, uniqConcat(value("spanish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(spanish|espanhol)\b/i, uniqConcat(value("spanish")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:p[rt]|en|port)[. (\\/-]*BR\b/i, uniqConcat(value("portuguese")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\bbr(?:a|azil|azilian)\W+(?:pt|por)\b/i, uniqConcat(value("portuguese")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\b(?:leg(?:endado|endas?)?|dub(?:lado)?|portugu[eèê]se?)[. -]*BR\b/i, uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bleg(?:endado|endas?)\b/i, uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bportugu[eèê]s[ea]?\b/i, uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bPT[. -]*(?:PT|ENG?|sub(?:s|titles?))\b/i, uniqConcat(value("portuguese")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bpt(?=\.(?:ass|ssa|srt|sub|idx)$)/i, uniqConcat(value("portuguese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bpor\b/i, uniqConcat(value("portuguese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bITA\b/i, uniqConcat(value("italian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?<!w{3}\.\w+\.)IT(?=[ .,/-]+(?:[a-zA-Z]{2}[ .,/-]+){2,})\b/, uniqConcat(value("italian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bit(?=\.(?:ass|ssa|srt|sub|idx)$)/i, uniqConcat(value("italian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bitaliano?\b/i, uniqConcat(value("italian")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bgreek[ .-]*(?:audio|lang(?:uage)?|subs?(?:titles?)?)?\b/i, uniqConcat(value("greek")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:GER|DEU)\b/i, uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bde(?=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})\b/i, uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?<=[ .,/-]+(?:[A-Z]{2}[ .,/-]+){2,})de\b/i, uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?<=[ .,/-]+[A-Z]{2}[ .,/-]+)de(?=[ .,/-]+[A-Z]{2}[ .,/-]+)\b/i, uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bde(?=\.(?:ass|ssa|srt|sub|idx)$)/i, uniqConcat(value("german")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(german|alem[aã]o)\b/i, uniqConcat(value("german")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bRUS?\b/i, uniqConcat(value("russian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(russian|russo)\b/i, uniqConcat(value("russian")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bUKR\b/i, uniqConcat(value("ukrainian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bukrainian\b/i, uniqConcat(value("ukrainian")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bhin(?:di)?\b/i, uniqConcat(value("hindi")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:(?<!w{3}\.\w+\.)tel(?!\W*aviv)|telugu)\b/i, uniqConcat(value("telugu")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bt[aâ]m(?:il)?\b/i, uniqConcat(value("tamil")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?<!YTS\.)LT\b/, uniqConcat(value("lithuanian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\blithuanian\b/i, uniqConcat(value("lithuanian")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\blatvian\b/i, uniqConcat(value("latvian")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bestonian\b/i, uniqConcat(value("estonian")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:PLDUB|Dubbing.PL|Lektor.PL|Film.Polski)\b/i, uniqConcat(value("polish")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\b(?:Napisy.PL|PLSUB(?:BED)?)\b/i, uniqConcat(value("polish")), { skipIfAlreadyFound: false, remove: true });
    parser.addHandler("languages", /\b(?:(?<!w{3}\.\w+\.)PL|pol)\b/i, uniqConcat(value("polish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(polish|polon[eê]s|polaco)\b/i, uniqConcat(value("polish")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bCZ[EH]?\b/i, uniqConcat(value("czech")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bczech\b/i, uniqConcat(value("czech")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bslo(?:vak|vakian|subs|[\]_)]?\.\w{2,4}$)\b/i, uniqConcat(value("slovakian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bHU\b/, uniqConcat(value("hungarian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bHUN(?:garian)?\b/i, uniqConcat(value("hungarian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bROM(?:anian)?\b/i, uniqConcat(value("romanian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bRO(?=[ .,/-]*(?:[A-Z]{2}[ .,/-]+)*sub)/i, uniqConcat(value("romanian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bbul(?:garian)?\b/i, uniqConcat(value("bulgarian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:srp|serbian)\b/i, uniqConcat(value("serbian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:HRV|croatian)\b/i, uniqConcat(value("croatian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bHR(?=[ .,/-]*(?:[A-Z]{2}[ .,/-]+)*sub)\b/i, uniqConcat(value("croatian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bslovenian\b/i, uniqConcat(value("slovenian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:(?<!w{3}\.\w+\.)NL|dut|holand[eê]s)\b/i, uniqConcat(value("dutch")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bdutch\b/i, uniqConcat(value("dutch")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bflemish\b/i, uniqConcat(value("dutch")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:DK|danska|dansub|nordic)\b/i, uniqConcat(value("danish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(danish|dinamarqu[eê]s)\b/i, uniqConcat(value("danish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bdan\b(?=.*\.(?:srt|vtt|ssa|ass|sub|idx)$)/i, uniqConcat(value("danish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:(?<!w{3}\.\w+\.)FI|finsk|finsub|nordic)\b/i, uniqConcat(value("finnish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bfinnish\b/i, uniqConcat(value("finnish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:(?<!w{3}\.\w+\.)SE|swe|swesubs?|sv(?:ensk)?|nordic)\b/i, uniqConcat(value("swedish")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(swedish|sueco)\b/i, uniqConcat(value("swedish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:NOR|norsk|norsub|nordic)\b/i, uniqConcat(value("norwegian")), { skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(norwegian|noruegu[eê]s|bokm[aå]l|nob|nor(?=[\]_)]?\.\w{2,4}$))\b/i, uniqConcat(value("norwegian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:arabic|[aá]rabe|ara)\b/i, uniqConcat(value("arabic")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\barab.*(?:audio|lang(?:uage)?|sub(?:s|titles?)?)\b/i, uniqConcat(value("arabic")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bar(?=\.(?:ass|ssa|srt|sub|idx)$)/i, uniqConcat(value("arabic")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:turkish|tur(?:co)?)\b/i, uniqConcat(value("turkish")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bvietnamese\b|\bvie(?=[\]_)]?\.\w{2,4}$)/i, uniqConcat(value("vietnamese")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bind(?:onesian)?\b/i, uniqConcat(value("indonesian")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(thai|tailand[eê]s)\b/i, uniqConcat(value("thai")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(THA|tha)\b/, uniqConcat(value("thai")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(?:malay|may(?=[\]_)]?\.\w{2,4}$)|(?<=subs?\([a-z,]+)may)\b/i, uniqConcat(value("malay")), { skipIfFirst: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\bheb(?:rew|raico)?\b/i, uniqConcat(value("hebrew")), { skipFromTitle: true, skipIfAlreadyFound: false });
    parser.addHandler("languages", /\b(persian|persa)\b/i, uniqConcat(value("persian")), { skipFromTitle: true, skipIfAlreadyFound: false });

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
    parser.addHandler("extension", /\.(3g2|3gp|avi|flv|mkv|mk3d|mov|mp2|mp4|m4v|mpe|mpeg|mpg|mpv|webm|wmv|ogm|divx|ts|m2ts|iso|vob|sub|idx|ttxt|txt|smi|srt|ssa|ass|vtt|nfo|html")$/i, lowercase);
};
