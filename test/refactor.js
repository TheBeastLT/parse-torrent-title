const { expect } = require("chai");
const ptt = require("../index");
const parse = ptt.parse;

// Additions from the Nyaa title-extraction refactor (Phase 1):
//   - arc / subtitle separation for multi-arc anime
//   - bare trailing Roman-numeral season detection
//   - normalizeResolution / normalizeCodec display helpers
//   - parseFilename (batch file-list episode extraction)

describe("Arc / subtitle separation", () => {
    it("extracts the arc from a colon subtitle with trailing episode", () => {
        const r = parse("Sword Art Online: Alicization - War of Underworld - 05 [1080p]");
        expect(r.title).to.equal("Sword Art Online");
        expect(r.arc).to.equal("Alicization - War of Underworld");
        expect(r.episode).to.equal(5);
    });

    it("keeps the arc in the title when there is no colon (token-set matching handles it)", () => {
        const r = parse("[Judas] Sword Art Online Alicization - 01 [1080p][x265]");
        expect(r.title).to.equal("Sword Art Online Alicization");
        expect(r.arc).to.be.undefined;
        expect(r.episode).to.equal(1);
    });

    it("does not set arc for plain colon titles without an episode", () => {
        expect(parse("The Fast and the Furious: Tokyo Drift [CR-Bt]").arc).to.be.undefined;
        expect(parse("The Sopranos: The Complete Series (Season 1,2,3,4,5&6) + Extras").arc).to.be.undefined;
    });
});

describe("Bare Roman-numeral season", () => {
    it("detects season from 'Title II - NN'", () => {
        const r = parse("[Judas] Sword Art Online II - 01 [BD 1080p]");
        expect(r.season).to.equal(2);
        expect(r.episode).to.equal(1);
    });

    it("detects season when the title ends in a number ('Mob Psycho 100 II - 13')", () => {
        const r = parse("Mob Psycho 100 II - 13 [1080p][x265]");
        expect(r.season).to.equal(2);
        expect(r.episode).to.equal(13);
    });

    it("does not invent a season for ordinary titles", () => {
        expect(parse("Sword Art Online - S01E01 [1080p]").season).to.equal(1);
        expect(parse("Attack on Titan - 25 [1080p]").season).to.be.undefined;
    });
});

describe("normalizeResolution", () => {
    const cases = { "4k": "4K", "2160p": "4K", "1080p": "1080p", "720p": "720p", "480p": "480p" };
    Object.entries(cases).forEach(([input, expected]) => {
        it(`${input} → ${expected}`, () => expect(ptt.normalizeResolution(input)).to.equal(expected));
    });
    it("returns null for empty input", () => expect(ptt.normalizeResolution(null)).to.equal(null));
    it("passes through unknown values", () => expect(ptt.normalizeResolution("144p")).to.equal("144p"));
});

describe("normalizeCodec", () => {
    const cases = { hevc: "HEVC", h265: "x265", x265: "x265", x264: "x264", h264: "x264", avc: "AVC", av1: "AV1" };
    Object.entries(cases).forEach(([input, expected]) => {
        it(`${input} → ${expected}`, () => expect(ptt.normalizeCodec(input)).to.equal(expected));
    });
    it("returns null for empty input", () => expect(ptt.normalizeCodec("")).to.equal(null));
});

describe("absoluteRangeHint", () => {
    it("extracts the parenthetical 3-digit hint alongside a per-season range", () => {
        const r = parse("[x39] Apotheosis S2 01-10 (053-062) [1080p HEVC AAC]");
        expect(r.absoluteRangeHint).to.deep.equal({ start: 53, end: 62 });
    });
    it("extracts a bracketed hint", () => {
        expect(parse("[SanKyuu] Bai Lian Cheng Shen S1 [001-052]").absoluteRangeHint).to.deep.equal({ start: 1, end: 52 });
    });
    it("tolerates spaces inside the parens", () => {
        expect(parse("[Group] Show S2 ( 063 - 072 ) [4K]").absoluteRangeHint).to.deep.equal({ start: 63, end: 72 });
    });
    it("ignores 2-digit ranges (ambiguous with per-season range)", () => {
        expect(parse("[SanKyuu] Apotheosis (01-52) [1080p]").absoluteRangeHint).to.be.undefined;
    });
    it("ignores single numbers and descending ranges", () => {
        expect(parse("[GM-Team][Apotheosis][64][4K HEVC 10Bit]").absoluteRangeHint).to.be.undefined;
        expect(parse("[Group] Title (062-053) [1080p]").absoluteRangeHint).to.be.undefined;
    });
});

// Corrected behaviors adopted in Phase 2 (formerly buggy in the Nyaa scraper).
// Locked here so the fixes can't silently regress.
describe("Corrected batch / episode behaviors", () => {
    it("'Part N - NN' is a single episode, not a batch range", () => {
        const r = parse("[SubsPlease] Spy x Family Part 2 - 13 (1080p)");
        expect(r.episode).to.equal(13);
        expect(r.episodeRangeStart).to.be.undefined;
        expect(r.isBatch).to.not.equal(true);
    });
    it("season 'Part N - NN' keeps the season and single episode", () => {
        const r = parse("Attack on Titan Season 3 Part 2 - 10 [1080p]");
        expect(r.season).to.equal(3);
        expect(r.episode).to.equal(10);
        expect(r.isBatch).to.not.equal(true);
    });
    it("flags '(batch)' / '[Batch]' labels as batches", () => {
        expect(parse("[Judas] Kimetsu no Yaiba (batch) [1080p]").isBatch).to.equal(true);
        expect(parse("[Coalgirls] Toradora! (1920x1080 Blu-ray FLAC) [Batch]").isBatch).to.equal(true);
    });
    it("parses a 4-digit parenthesized batch range", () => {
        const r = parse("[Anime Time] One Piece (0001-1000) [Dual Audio][1080p][HEVC][Batch]");
        expect(r.episodeRangeStart).to.equal(1);
        expect(r.episodeRangeEnd).to.equal(1000);
        expect(r.title).to.equal("One Piece");
    });
});

describe("parseFilename", () => {
    const cases = [
        ["[Group] Show Title - 01 [720p].mkv", 1],
        ["Show.Title.S01E05.mkv", 5],
        ["[Group] Show Title 05 (BD 1080p).mkv", 5],
        ["第01話 Show Title [1080p].mkv", 1],
        ["[Group] Show - Episode 137 [1080p].mkv", 137],
    ];
    cases.forEach(([input, expected]) => {
        it(`"${input}" → ${expected}`, () => expect(ptt.parseFilename(input)).to.equal(expected));
    });
    it("returns null when no episode number is present", () => {
        expect(ptt.parseFilename("[Group] Show Title (1080p).mkv")).to.equal(null);
    });
    // Known limitation carried over verbatim from the Nyaa port: a CJK episode
    // suffix with no keyword prefix ("01화", "01話") is not recognized. Documented
    // here so the behavior is intentional, not an accidental regression. Candidate
    // for a follow-up enhancement (out of Phase 1's faithful-port scope).
    it("does not yet handle a bare CJK episode suffix (documented limitation)", () => {
        expect(ptt.parseFilename("Show Title 01화 [720p].mkv")).to.equal(null);
    });
});
