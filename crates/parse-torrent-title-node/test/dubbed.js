const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing dubbed", () => {
    it("should detect the english dubbed language correctly", () => {
        const releaseName = "Yo-Kai Watch S01E71 DUBBED 720p HDTV x264-W4F";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dub correctly", () => {
        const releaseName = "[Golumpa] Kochoki - 11 (Kochoki - Wakaki Nobunaga) [English Dub] [FuniDub 720p x264 AAC] [MKV] [4FA0D898]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dub correctly", () => {
        const releaseName = "[Golumpa] Kochoki - 11 (Kochoki - Wakaki Nobunaga) [English Dub] [FuniDub 720p x264 AAC] [MKV] [4FA0D898]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dubs correctly", () => {
        const releaseName = "[Aomori-Raws] Juushinki Pandora (01-13) [Dubs & Subs]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dual audio correctly", () => {
        const releaseName = "[LostYears] Tsuredure Children (WEB 720p Hi10 AAC) [Dual-Audio]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dual-audio correctly", () => {
        const releaseName = "[DB] Gamers! [Dual Audio 10bit 720p][HEVC-x265]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect multi-audio correctly", () => {
        const releaseName = "[DragsterPS] Yu-Gi-Oh! S02 [480p] [Multi-Audio] [Multi-Subs]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dublado correctly", () => {
        const releaseName = "A Freira (2018) Dublado HD-TS 720p";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dual correctly", () => {
        const releaseName = "Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should detect dubbing correctly", () => {
        const releaseName = "Vaiana Skarb oceanu / Moana (2016) [720p] [WEB-DL] [x264] [Dubbing]";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });

    it("should not detect dual subs", () => {
        const releaseName = "[Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR";

        expect(parse(releaseName)).to.not.have.property("dubbed");
    });

    it("should not detect multi-dub", () => {
        const releaseName = "[IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][HEVC 10Bits] 800p BD";

        expect(parse(releaseName)).to.deep.include({ dubbed: true });
    });
});
