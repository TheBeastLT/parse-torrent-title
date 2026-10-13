const { expect } = require("chai");
const ptt = require("../index");
const { Parser, addDefaults, createParser, Transformers } = ptt;

function names(parser) {
    return parser.getHandlers().map(handler => handler.name);
}

function simpleParser() {
    const parser = new Parser();
    parser.addHandler("resolution", /\b(720)p\b/, Transformers.value("$1p"));
    parser.addHandler("resolution", /\b(1080)p\b/, Transformers.value("$1p"));
    parser.addHandler("codec", /\bx26[45]\b/);
    return parser;
}

describe("Parser customization", () => {

    describe("getHandlers", () => {
        it("lists handlers with their category index and position", () => {
            const parser = simpleParser();
            parser.addHandler("custom", () => null);

            const handlers = parser.getHandlers();

            expect(handlers.map(({ name, index, position }) => ({ name, index, position }))).to.deep.equal([
                { name: "resolution", index: 0, position: 0 },
                { name: "resolution", index: 1, position: 1 },
                { name: "codec", index: 0, position: 2 },
                { name: "custom", index: 0, position: 3 }
            ]);
            expect(handlers[1].regExp.source).to.equal("\\b(1080)p\\b");
            expect(handlers[1].options).to.include({ skipIfAlreadyFound: true, remove: false });
            expect(handlers[3]).to.not.have.property("regExp");
        });

        it("names unnamed function handlers as unknown", () => {
            const parser = new Parser();
            parser.addHandler(() => null);

            expect(names(parser)).to.deep.equal(["unknown"]);
        });

        it("returns copies which do not modify the parser", () => {
            const parser = simpleParser();
            const handlers = parser.getHandlers();
            handlers.pop();
            handlers[0].options.remove = true;

            expect(names(parser)).to.deep.equal(["resolution", "resolution", "codec"]);
            expect(parser.getHandlers()[0].options.remove).to.equal(false);
        });
    });

    describe("addHandler positioning", () => {
        it("appends to the end by default", () => {
            const parser = simpleParser();
            parser.addHandler("network", /\bNF\b/);

            expect(names(parser)).to.deep.equal(["resolution", "resolution", "codec", "network"]);
        });

        it("inserts after the last handler of a category", () => {
            const parser = simpleParser();
            parser.addHandler("network", /\bNF\b/, { after: "resolution" });

            expect(names(parser)).to.deep.equal(["resolution", "resolution", "network", "codec"]);
        });

        it("inserts before the first handler of a category", () => {
            const parser = simpleParser();
            parser.addHandler("network", /\bNF\b/, Transformers.uppercase, { before: "resolution" });

            expect(names(parser)).to.deep.equal(["network", "resolution", "resolution", "codec"]);
        });

        it("inserts relative to a specific handler index of a category", () => {
            const parser = simpleParser();
            parser.addHandler("after0", /a/, { after: { name: "resolution", index: 0 } });
            parser.addHandler("before1", /b/, { before: { name: "resolution", index: 1 } });

            expect(names(parser)).to.deep.equal(["resolution", "after0", "before1", "resolution", "codec"]);
        });

        it("targets the first and last handler of a non contiguous category", () => {
            const parser = new Parser();
            parser.addHandler("a", /a1/);
            parser.addHandler("b", /b/);
            parser.addHandler("a", /a2/);
            parser.addHandler("first", /f/, { before: "a" });
            parser.addHandler("last", /l/, { after: "a" });

            expect(names(parser)).to.deep.equal(["first", "a", "b", "a", "last"]);
        });

        it("targets handlers counted from the end with a negative index", () => {
            const parser = simpleParser();
            parser.addHandler("network", /\bNF\b/, { before: { name: "resolution", index: -2 } });

            expect(names(parser)).to.deep.equal(["network", "resolution", "resolution", "codec"]);
        });

        it("targets the first or last handler of a category when the index is omitted", () => {
            const parser = simpleParser();
            parser.addHandler("first", /f/, { before: { name: "resolution" } });
            parser.addHandler("last", /l/, { after: { name: "resolution" } });

            expect(names(parser)).to.deep.equal(["first", "resolution", "resolution", "last", "codec"]);
        });

        it("positions function handlers", () => {
            const parser = simpleParser();
            parser.addHandler("named", () => null, { before: "codec" });
            parser.addHandler(() => null, { before: "resolution" });

            expect(names(parser)).to.deep.equal(["unknown", "resolution", "resolution", "named", "codec"]);
        });

        it("does not pass positioning keys to the handler options", () => {
            const parser = simpleParser();
            parser.addHandler("network", /\bNF\b/, { after: "codec", remove: true });

            const { options } = parser.getHandlers()[3];
            expect(options).to.not.have.any.keys("after", "before");
            expect(options.remove).to.equal(true);
        });

        it("runs inserted handlers in their position", () => {
            const parser = simpleParser();
            parser.addHandler("resolution", /\b(4k)\b/i, Transformers.value("4k"), { before: "resolution" });

            expect(parser.parse("Movie 1080p 4k x264").resolution).to.equal("4k");
        });

        it("throws for an unknown category", () => {
            const parser = simpleParser();

            expect(() => parser.addHandler("network", /NF/, { after: "missing" })).to.throw(/missing/);
        });

        it("throws for an out of range category index", () => {
            const parser = simpleParser();

            expect(() => parser.addHandler("network", /NF/, { before: { name: "codec", index: 1 } })).to.throw(/codec/);
        });

        it("throws when both before and after are given", () => {
            const parser = simpleParser();

            expect(() => parser.addHandler("network", /NF/, { before: "codec", after: "codec" })).to.throw(/before.*after/);
        });
    });

    describe("removeHandlers", () => {
        it("removes all handlers of a category", () => {
            const parser = simpleParser();

            expect(parser.removeHandlers("resolution")).to.equal(2);
            expect(names(parser)).to.deep.equal(["codec"]);
        });

        it("removes handlers of several categories", () => {
            const parser = simpleParser();

            expect(parser.removeHandlers(["resolution", "codec"])).to.equal(3);
            expect(names(parser)).to.deep.equal([]);
        });

        it("removes handlers matching a filter", () => {
            const parser = simpleParser();

            expect(parser.removeHandlers(handler => handler.name === "resolution" && handler.index === 1)).to.equal(1);
            expect(parser.getHandlers().map(handler => handler.regExp.source)).to.deep.equal(["\\b(720)p\\b", "\\bx26[45]\\b"]);
        });

        it("returns zero when nothing matches", () => {
            const parser = simpleParser();

            expect(parser.removeHandlers("missing")).to.equal(0);
        });

        it("stops parsing removed categories", () => {
            const parser = createParser();
            parser.removeHandlers("group");

            expect(parser.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU")).to.not.have.property("group");
        });
    });

    describe("clone", () => {
        it("copies handlers without sharing changes", () => {
            const parser = simpleParser();
            const copy = parser.clone();
            copy.removeHandlers("codec");
            copy.addHandler("network", /NF/);
            parser.addHandler("other", /O/);

            expect(names(parser)).to.deep.equal(["resolution", "resolution", "codec", "other"]);
            expect(names(copy)).to.deep.equal(["resolution", "resolution", "network"]);
            expect(copy).to.be.instanceOf(Parser);
        });
    });

    describe("createParser", () => {
        it("creates a parser with the default handlers", () => {
            const parser = createParser();

            expect(parser.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU"))
                .to.deep.equal(ptt.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU"));
        });

        it("does not modify the default parser", () => {
            const parser = createParser();
            parser.removeHandlers("resolution");

            expect(ptt.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU").resolution).to.equal("720p");
        });
    });

    describe("addDefaults", () => {
        it("excludes categories", () => {
            const parser = new Parser();
            addDefaults(parser, { exclude: ["site", "languages"] });

            expect(names(parser)).to.not.include.members(["site", "languages"]);
            expect(names(parser)).to.include("resolution");
        });
    });

    describe("bug fixes", () => {
        it("forwards transformer and options of module level addHandler", () => {
            ptt.addHandler("customBarTag", /\bBAR\b/, Transformers.lowercase, { remove: true, before: "site" });

            try {
                expect(ptt.parse("BAR Movie 2010")).to.include({ customBarTag: "bar", title: "Movie" });
            } finally {
                ptt.removeHandlers("customBarTag");
            }
        });

        it("applies options given in place of the transformer", () => {
            const parser = createParser();
            parser.addHandler("tag", /\bFOO\b/, { remove: true, before: "site" });

            expect(parser.parse("FOO Movie 2010")).to.include({ tag: "FOO", title: "Movie" });
        });

        it("parses with an unbound parse function", () => {
            const { parse } = createParser();

            expect(parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU").title).to.equal("Game of Thrones");
        });
    });
});
