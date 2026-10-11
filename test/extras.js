const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing extras", () => {
    it("should detect added ova and keep it out of the title", () => {
        const releaseName = "[Judas] Elfen Lied + OVA (BD 1080p)";

        expect(parse(releaseName)).to.deep.include({ title: "Elfen Lied", extras: true });
    });

    it("should detect added specials without spaces", () => {
        const releaseName = "[Mysteria] Ao Haru Ride+OVA [BD 1080p] (HEVC x265 10-bit AAC)";

        expect(parse(releaseName)).to.deep.include({ title: "Ao Haru Ride", extras: true });
    });

    it("should detect a chain of added extras", () => {
        const releaseName = "[Pog42] Shirobako + Specials + OVAs (BD 1080p HEVC x265 10-bit)";

        expect(parse(releaseName)).to.deep.include({ title: "Shirobako", extras: true });
    });

    it("should detect added specials and extras joined by and", () => {
        const releaseName = "Shiki + Specials and Extras (2010-2011)";

        expect(parse(releaseName)).to.deep.include({ title: "Shiki", extras: true });
    });

    it("should detect added extras in parentheses", () => {
        const releaseName = "Hyouka (+ OVA) [BD]";

        expect(parse(releaseName)).to.deep.include({ title: "Hyouka", extras: true });
    });

    it("should detect added counted movies", () => {
        const releaseName = "Naruto Shippuuden 1-15+ 3 Movies";

        expect(parse(releaseName)).to.deep.include({ extras: true });
    });

    it("should detect added extras for a non anime series", () => {
        const releaseName = "Star Trek Original Series TOS +extras 720p EN-SUB";

        expect(parse(releaseName)).to.deep.include({ title: "Star Trek Original Series TOS", extras: true });
    });

    it("should not detect extras from an ova title", () => {
        const releaseName = "Hyouka OVA.mkv";

        expect(parse(releaseName)).to.not.have.property("extras");
    });

    it("should not detect extras from a dash separated ova", () => {
        const releaseName = "Kyochuu Rettou - OVA (Uncut)";

        expect(parse(releaseName)).to.not.have.property("extras");
    });

    it("should not detect extras from an ampersand in the title", () => {
        const releaseName = "Panty & Stocking with Garterbelt [BD]";

        expect(parse(releaseName)).to.not.have.property("extras");
    });

    it("should not detect extras from a movie title with an ampersand", () => {
        const releaseName = "Tom & Jerry Movie";

        expect(parse(releaseName)).to.not.have.property("extras");
    });

    it("should detect added soundtracks and books", () => {
        const releaseName = "Hyouka_TV+Novel+Manga+Music+Other; dub jpn sub chs (2019-10-06)";

        expect(parse(releaseName)).to.deep.include({ title: "Hyouka TV", extras: true });
    });

    it("should not detect extras from other in an ampersand title", () => {
        const releaseName = "[uba] Love, Chunibyo & Other Delusions! (2012) - S02 v3 (BD Remux)";

        expect(parse(releaseName)).to.deep.include({ title: "Love, Chunibyo & Other Delusions!" });
    });
});
