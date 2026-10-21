const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing audio", () => {
    it("should detect the dts audio correctly", () => {
        const releaseName = "Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2";

        expect(parse(releaseName)).to.deep.include({ audio: "dts" });
    });

    it("should detect the DTS-HD audio correctly", () => {
        const releaseName = "Gold 2016 1080p BluRay DTS-HD MA 5 1 x264-HDH";

        expect(parse(releaseName)).to.deep.include({ audio: "dts-hd" });
    });

    it("should detect the AAC audio correctly", () => {
        const releaseName = "Rain Man 1988 REMASTERED 1080p BRRip x264 AAC-m2g";

        expect(parse(releaseName)).to.deep.include({ audio: "aac" });
    });

    it("should convert the AAC2.0 audio to AAC", () => {
        const releaseName = "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN";

        expect(parse(releaseName)).to.deep.include({ audio: "aac" });
    });

    it("should detect the dd5 audio correctly", () => {
        const releaseName = "Jimmy Kimmel 2017 05 03 720p HDTV DD5 1 MPEG2-CTL";

        expect(parse(releaseName)).to.deep.include({ audio: "dd5.1" });
    });

    it("should detect the AC3 audio correctly", () => {
        const releaseName = "A Dog's Purpose 2016 BDRip 720p X265 Ac3-GANJAMAN";

        expect(parse(releaseName)).to.deep.include({ audio: "ac3" });
    });

    it("should convert the AC-3 audio to AC3", () => {
        const releaseName = "Retroactive 1997 BluRay 1080p AC-3 HEVC-d3g";

        expect(parse(releaseName)).to.deep.include({ audio: "ac3" });
    });

    it("should detect the mp3 audio correctly", () => {
        const releaseName = "Tempete 2016-TrueFRENCH-TVrip-H264-mp3";

        expect(parse(releaseName)).to.deep.include({ audio: "mp3" });
    });

    xit("should detect the MD audio correctly", () => {
        const releaseName = "Detroit.2017.BDRip.MD.GERMAN.x264-SPECTRE";

        expect(parse(releaseName)).to.deep.include({ audio: "md" });
    });

    it("should detect the eac3 5.1 audio correctly", () => {
        const releaseName = "The Blacklist S07E04 (1080p AMZN WEB-DL x265 HEVC 10bit EAC-3 5.1)[Bandi]";

        expect(parse(releaseName)).to.deep.include({ audio: "eac3" });
    });

    it("should detect the eac3 6.0 audio correctly", () => {
        const releaseName = "Condor.S01E03.1080p.WEB-DL.x265.10bit.EAC3.6.0-Qman[UTR].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "eac3" });
    });

    it("should detect the eac3 2.0 audio correctly 2", () => {
        const releaseName = "The 13 Ghosts of Scooby-Doo (1985) S01 (1080p AMZN Webrip x265 10bit EAC-3 2.0 - Frys) [TAoE]";

        expect(parse(releaseName)).to.deep.include({ audio: "eac3" });
    });

    it("should not detect mp3 audio inside a word", () => {
        const releaseName = "[Thund3r3mp3ror] Attack on Titan - 23.mp4";

        expect(parse(releaseName)).to.not.have.property("audio");
    });

    it("should detect 2.0x2 audio", () => {
        const releaseName = "Buttobi!! CPU - 02 (DVDRip 720x480p x265 HEVC AC3x2 2.0x2)(Dual Audio)[sxales].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "2.0" });
    });

    it("should detect qaac2 audio", () => {
        const releaseName = "[naiyas] Fate Stay Night - Unlimited Blade Works Movie [BD 1080P HEVC10 QAACx2 Dual Audio]";

        expect(parse(releaseName)).to.deep.include({ audio: "aac" });
    });

    it("should detect 2.0x5.1 audio", () => {
        const releaseName = "Sakura Wars the Movie (2001) (BDRip 1920x1036p x265 HEVC FLACx2, AC3 2.0+5.1x2)(Dual Audio)[sxales].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "2.0" });
    });

    it("should detect 5.1x2.0 audio", () => {
        const releaseName = "Macross ~ Do You Remember Love (1984) (BDRip 1920x1036p x265 HEVC DTS-HD MA, FLAC, AC3x2 5.1+2.0x3)(Dual Audio)[sxales].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "2.0" });
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect 5.1x2+2.0x3 audio", () => {
        const releaseName = "Escaflowne (2000) (BDRip 1896x1048p x265 HEVC TrueHD, FLACx3, AC3 5.1x2+2.0x3)(Triple Audio)[sxales].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "2.0" });
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect FLAC2.0x2 audio", () => {
        const releaseName = "[SAD] Inuyasha - The Movie 4 - Fire on the Mystic Island [BD 1920x1036 HEVC10 FLAC2.0x2] [84E9A4A1].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "flac" });
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect FLACx2 2.0x3 audio", () => {
        const releaseName = "Outlaw Star - 23 (BDRip 1440x1080p x265 HEVC AC3, FLACx2 2.0x3)(Dual Audio)[sxales].mkv";

        expect(parse(releaseName)).to.deep.include({ audio: "2.0", episode: 23 });
    });

    it("should detect 7.1 Atmos audio", () => {
        const releaseName = "Spider-Man.No.Way.Home.2021.2160p.BluRay.REMUX.HEVC.TrueHD.7.1.Atmos-FraMeSToR";

        expect(parse(releaseName)).to.deep.include({ audio: "7.1 Atmos" });
    });

    it("should detect hyphenated E-AC3 audio", () => {
        const releaseName = "Tu corazón será destrozado Lat-Rus Subt Esp H264 E-AC3 5.1 BDRip Hd 1080p";

        expect(parse(releaseName)).to.deep.include({ audio: "eac3" });
    });

    it("should detect hyphenated E-AC-3 audio", () => {
        const releaseName = "The Devil's Climb (2024) (1080p DS4K BluRay x265 10-bit SDR E-AC-3 5.1) [WeSLeY]";

        expect(parse(releaseName)).to.deep.include({ audio: "eac3" });
    });
});
