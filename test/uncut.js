const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing uncut", () => {
    it("should detect an uncut release before its year", () => {
        const releaseName = "The.Terminator.Uncut.1984.1080p.BluRay.x264";

        expect(parse(releaseName)).to.deep.include({ title: "The Terminator", uncut: true, year: 1984 });
    });

    it("should detect an uncut release before its year in a spaced name", () => {
        const releaseName = "Aliens Uncut 1986 1080p BluRay x264";

        expect(parse(releaseName)).to.deep.include({ title: "Aliens", uncut: true, year: 1986 });
    });

    it("should detect a bracketed uncut release", () => {
        const releaseName = "Dawn of the Dead (Uncut) 1978 1080p BluRay";

        expect(parse(releaseName)).to.deep.include({ title: "Dawn of the Dead", uncut: true, year: 1978 });
    });

    it("should detect an uncut release before its resolution", () => {
        const releaseName = "Scarface Uncut 1080p BluRay x264";

        expect(parse(releaseName)).to.deep.include({ title: "Scarface", uncut: true });
    });

    it("should keep uncut starting a title", () => {
        const releaseName = "Uncut Gems 2019 1080p WEB-DL x264";

        expect(parse(releaseName)).to.deep.include({ title: "Uncut Gems", year: 2019 });
        expect(parse(releaseName)).to.not.have.property("uncut");
    });

    it("should keep uncut starting a dotted title", () => {
        const releaseName = "Uncut.Gems.2019.1080p.BluRay";

        expect(parse(releaseName)).to.deep.include({ title: "Uncut Gems", year: 2019 });
        expect(parse(releaseName)).to.not.have.property("uncut");
    });

    it("should keep a title that is only uncut", () => {
        const releaseName = "Uncut (1997) DVDRip";

        expect(parse(releaseName)).to.deep.include({ title: "Uncut", year: 1997 });
        expect(parse(releaseName)).to.not.have.property("uncut");
    });

    it("should keep uncut followed by more title words", () => {
        const releaseName = "The Uncut Story of Mewtwos Origin 2000 DVDRip";

        expect(parse(releaseName)).to.deep.include({ title: "The Uncut Story of Mewtwos Origin", year: 2000 });
        expect(parse(releaseName)).to.not.have.property("uncut");
    });
});
