const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing aspect ratio", () => {
    it("Sleeping with the Enemy 16X9 (Altair-Starhome).m4v", () => {
        expect(parse("Sleeping with the Enemy 16X9 (Altair-Starhome).m4v")).to.deep.include({"aspectRatio": "16:9", "title": "Sleeping with the Enemy"});
    });

    it("01 Star Wars (1977) 1080p (16x9).mp4", () => {
        expect(parse("01 Star Wars (1977) 1080p (16x9).mp4")).to.deep.include({"aspectRatio": "16:9"});
    });

    it("THE_MATRIX_16X9LB_N_AMERICA", () => {
        expect(parse("THE_MATRIX_16X9LB_N_AMERICA")).to.deep.include({"aspectRatio": "16:9", "title": "THE MATRIX"});
    });

    it("ЦИРК ПРИЕХАЛ 1 16х9 (Kultura 53).mkv", () => {
        expect(parse("ЦИРК ПРИЕХАЛ 1 16х9 (Kultura 53).mkv")).to.deep.include({"aspectRatio": "16:9", "title": "ЦИРК ПРИЕХАЛ 1"});
    });

    it("La Caduta Degli Dei [1969,DVDRip,4X3].avi", () => {
        expect(parse("La Caduta Degli Dei [1969,DVDRip,4X3].avi")).to.deep.include({"aspectRatio": "4:3"});
    });

    it("Smart House (1999) [720p 4x3 DD5.1 Eng CC] [Squee].ts", () => {
        expect(parse("Smart House (1999) [720p 4x3 DD5.1 Eng CC] [Squee].ts")).to.deep.include({"aspectRatio": "4:3"});
    });

    it("Sailor Moon R The Movie 4х3 DVDRemux.mkv", () => {
        expect(parse("Sailor Moon R The Movie 4х3 DVDRemux.mkv")).to.deep.include({"aspectRatio": "4:3", "title": "Sailor Moon R The Movie"});
    });

    it("Zeitgeist 1 (2007) The Movie (4x3).mp4", () => {
        expect(parse("Zeitgeist 1 (2007) The Movie (4x3).mp4")).to.deep.include({"aspectRatio": "4:3"});
    });

    it("[Venny] Dragon Ball Z - Movie 13 [R2FR] [x264] [4x3].mkv", () => {
        expect(parse("[Venny] Dragon Ball Z - Movie 13 [R2FR] [x264] [4x3].mkv")).to.deep.include({"aspectRatio": "4:3"});
    });

    it("Heroes 4x3.avi", () => {
        expect(parse("Heroes 4x3.avi")).to.not.have.property("aspectRatio");
    });

    it("Czarna Żmija [4x3] Sztuka kamuflażu.avi", () => {
        expect(parse("Czarna Żmija [4x3] Sztuka kamuflażu.avi")).to.not.have.property("aspectRatio");
    });

    it("Buffy cazavampiros 4x3 HDTV XviD Castellano.avi", () => {
        expect(parse("Buffy cazavampiros 4x3 HDTV XviD Castellano.avi")).to.not.have.property("aspectRatio");
    });

    it("16x9.2014.03.29.A.Fathers.Justice.HDTV.x264-CLDD", () => {
        expect(parse("16x9.2014.03.29.A.Fathers.Justice.HDTV.x264-CLDD")).to.not.have.property("aspectRatio");
    });
});
