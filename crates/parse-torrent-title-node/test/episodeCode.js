const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing episode code", () => {
    it("[Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39].mkv", () => {
        const releaseName = "[Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39].mkv";

        expect(parse(releaseName)).to.deep.include({ episodeCode: "5E46AC39" });
    });

    it("[Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv", () => {
        const releaseName = "[Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv";

        expect(parse(releaseName)).to.deep.include({ episodeCode: "41858470" });
    });

    it("[ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv", () => {
        const releaseName = "[ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv";

        expect(parse(releaseName)).to.deep.include({ episodeCode: "9E199846" });
    });

    it("[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]", () => {
        const releaseName = "[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]";

        expect(parse(releaseName)).to.deep.include({ episodeCode: "436E0E90" });
    });

    it("Gankutsuou.-.The.Count.Of.Monte.Cristo[2005].-.04.-.[720p.BD.HEVC.x265].[FLAC].[Jd].[DHD].[b6e6e648].mkv", () => {
        const releaseName = "Gankutsuou.-.The.Count.Of.Monte.Cristo[2005].-.04.-.[720p.BD.HEVC.x265].[FLAC].[Jd].[DHD].[b6e6e648].mkv";

        expect(parse(releaseName)).to.deep.include({ episodeCode: "B6E6E648" });
    });

    it("[D0ugyB0y] Nanatsu no Taizai Fundo no Shinpan - 01 (1080p WEB NF x264 AAC[9CC04E06]).mkv", () => {
        const releaseName = "[D0ugyB0y] Nanatsu no Taizai Fundo no Shinpan - 01 (1080p WEB NF x264 AAC[9CC04E06]).mkv";

        expect(parse(releaseName)).to.deep.include({ episodeCode: "9CC04E06" });
    });

    it("should not detect episode code not at the end", () => {
        const releaseName = "Lost.[Perdidos].6x05.HDTV.XviD.[www.DivxTotaL.com].avi";
        expect(parse(releaseName)).to.not.have.property("episodeCode");
    });

    it("should not detect episode code when it's a word", () => {
        const releaseName = "Lost - Stagioni 01-06 (2004-2010) [COMPLETA] SD x264 AAC ITA SUB ITA";
        expect(parse(releaseName)).to.not.have.property("episodeCode");
    });

    it("should not detect episode code when it's only numbers", () => {
        const releaseName = "The voice of Holland S05E08 [20141017]  NL Battles 1.mp4";
        expect(parse(releaseName)).to.not.have.property("episodeCode");
    });
});
