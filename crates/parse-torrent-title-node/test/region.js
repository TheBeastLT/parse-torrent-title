const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing region", () => {

    it("should detect the R5 region correctly", () => {
        const releaseName = "Welcome to New York 2014 R5 XviD AC3-SUPERFAST";

        expect(parse(releaseName)).to.deep.include({ region: "R5" });
    });

    it("should not detect region in the title", () => {
        const releaseName = "[Coalgirls]_Code_Geass_R2_06_(1920x1080_Blu-ray_FLAC)_[F8C7FE25].mkv";

        expect(parse(releaseName)).to.not.have.property("region");
    });
});
