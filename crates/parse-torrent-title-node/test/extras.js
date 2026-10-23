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

    it("should end the title before a numbered special section", () => {
        const releaseName = "[Moozzi2] Aria The Animation [SP01] TV-Spot - 01 (BD 1440x1080 x.264 Flac).mkv";

        expect(parse(releaseName)).to.deep.include({ title: "Aria The Animation", extras: true });
    });

    it("should end the title before a numbered special section with a named special", () => {
        const releaseName = "[BananaBoat] Yuyushiki [SP03] Nyanyashiki - 01 (BD 1920x1080 x.264 Flac).mkv";

        expect(parse(releaseName)).to.deep.include({ title: "Yuyushiki", extras: true });
    });

    it("should not detect extras from a leading special section", () => {
        const releaseName = "[SP01] Show - 01.mkv";

        expect(parse(releaseName)).to.deep.include({ title: "Show" });
        expect(parse(releaseName)).to.not.have.property("extras");
    });

    it("should not detect extras from a special word without a number", () => {
        const releaseName = "[Group] Show [SP] - 01.mkv";

        expect(parse(releaseName)).to.not.have.property("extras");
    });

    it("should not detect extras from other in an ampersand title", () => {
        const releaseName = "[uba] Love, Chunibyo & Other Delusions! (2012) - S02 v3 (BD Remux)";

        expect(parse(releaseName)).to.deep.include({ title: "Love, Chunibyo & Other Delusions!" });
    });

    it("should detect extras added with plus and keep them out of the title", () => {
        const releaseName = "Magic Boy plus Extra Princess Kuro | 1959 | DVDRip | 720p | x265 | AC3 | German";

        expect(parse(releaseName)).to.deep.include({ title: "Magic Boy", year: 1959, extras: true });
    });

    it("should keep plus in a title when followed by a movie", () => {
        const releaseName = "Macross Plus Movie Edition 1995 720p BluRay x264-GeneMige";

        expect(parse(releaseName)).to.deep.include({ title: "Macross Plus Movie Edition", year: 1995 });
    });

    it("should keep plus in a title when followed by a special", () => {
        const releaseName = "Amagami SS Plus Special";

        expect(parse(releaseName).title).to.equal("Amagami SS Plus Special");
    });

    it("should keep plus in a title when followed by an ova", () => {
        const releaseName = "[SpurdoMux] Macross Plus OVA 1-2 BD 720p (Dual audio)";

        expect(parse(releaseName).title).to.equal("Macross Plus OVA");
    });
});
