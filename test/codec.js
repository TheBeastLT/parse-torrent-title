const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing codec", () => {
    it("should detect the hevc codec correctly", () => {
        const releaseName = "Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2";

        expect(parse(releaseName)).to.deep.include({ codec: "hevc" });
    });

    it("should detect the x264 codec correctly", () => {
        const releaseName = "doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov";

        expect(parse(releaseName)).to.deep.include({ codec: "x264" });
    });

    it("should detect the Web-DL codec correctly", () => {
        const releaseName = "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN";

        expect(parse(releaseName)).to.deep.include({ codec: "h264" });
    });

    it("should detect the XviD codec correctly", () => {
        const releaseName = "Gotham S03E17 XviD-AFG";

        expect(parse(releaseName)).to.deep.include({ codec: "xvid" });
    });

    it("should detect the Mpeg2 codec correctly", () => {
        const releaseName = "Jimmy Kimmel 2017 05 03 720p HDTV DD5 1 MPEG2-CTL";

        expect(parse(releaseName)).to.deep.include({ codec: "mpeg2" });
    });

    it("should detect the HEVC10bit codec correctly", () => {
        const releaseName = "[Anime Time] Re Zero kara Hajimeru Isekai Seikatsu (Season 2 Part 1) [1080p][HEVC10bit x265][Multi Sub]";

        expect(parse(releaseName)).to.deep.include({ codec: "hevc", bitDepth: "10bit" });
    });

    it("should detect the HEVC10 codec correctly", () => {
        const releaseName = "[naiyas] Fate Stay Night - Unlimited Blade Works Movie [BD 1080P HEVC10 QAACx2 Dual Audio]";

        expect(parse(releaseName)).to.deep.include({ codec: "hevc", bitDepth: "10bit" });
    });

    it("should not detect 264 codec from episode number", () => {
        const releaseName = "[DB]_Bleach_264_[012073FE].avi";

        expect(parse(releaseName)).to.not.have.property("codec");
    });

    it("should not detect 265 codec from episode number", () => {
        const releaseName = "[DB]_Bleach_265_[B4A04EC9].avi";

        expect(parse(releaseName)).to.not.have.property("codec");
    });

    it("should detect av1 codec", () => {
        const releaseName = "Love.Island.S03E32.1080p.AV1.10bit-MeGusta.mkv";
        expect(parse(releaseName)).to.deep.include({ codec: "av1" });
    });
});
