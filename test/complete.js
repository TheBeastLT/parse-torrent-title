const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing complete collection", () => {
    it("should detect complete series with full seasons", () => {
        const releaseName = "[Furi] Avatar - The Last Airbender [720p] (Full 3 Seasons + Extr";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection", () => {
        const releaseName = "Harry.Potter.Complete.Collection.2001-2011.1080p.BluRay.DTS-ETRG";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with all seasons", () => {
        const releaseName = "Game of Thrones All 7 Seasons 1080p ~∞~ .HakunaMaKoko";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with full series", () => {
        const releaseName = "Avatar: The Last Airbender Full Series 720p";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with ultimate collection", () => {
        const releaseName = "Dora the Explorer - Ultimate Collection";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with complete pack", () => {
        const releaseName = "Mr Bean Complete Pack (Animated, Tv series, 2 Movies) DVDRIP (WA";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with complete set", () => {
        const releaseName = "American Pie - Complete set (8 movies) 720p mkv - YIFY";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with complete filmography", () => {
        const releaseName = "Charlie Chaplin - Complete Filmography (87 movies)";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection with movies complete", () => {
        const releaseName = "Monster High Movies Complete 2014";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection all movies", () => {
        const releaseName = "Harry Potter All Movies Collection 2001-2011 720p Dual KartiKing";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete movie collection", () => {
        const releaseName = "The Clint Eastwood Movie Collection";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete collection movies", () => {
        const releaseName = "Clint Eastwood Collection - 15 HD Movies";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect complete movies collection", () => {
        const releaseName = "Official  IMDb  Top  250  Movies  Collection  6/17/2011";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect collection", () => {
        const releaseName = "The Texas Chainsaw Massacre Collection (1974-2017) BDRip 1080p";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect duology", () => {
        const releaseName = "Snabba.Cash.I-II.Duology.2010-2012.1080p.BluRay.x264.anoXmous";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect trilogy", () => {
        const releaseName = "Star Wars Original Trilogy 1977-1983 Despecialized 720p";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect quadrology", () => {
        const releaseName = "The.Wong.Kar-Wai.Quadrology.1990-2004.1080p.BluRay.x264.AAC.5.1-";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect quadrilogy", () => {
        const releaseName = "Lethal.Weapon.Quadrilogy.1987-1992.1080p.BluRay.x264.anoXmous";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect tetralogy", () => {
        const releaseName = "X-Men.Tetralogy.BRRip.XviD.AC3.RoSubbed-playXD";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect pentalogy", () => {
        const releaseName = "Mission.Impossible.Pentalogy.1996-2015.1080p.BluRay.x264.AAC.5.1";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect hexalogy", () => {
        const releaseName = "Mission.Impossible.Hexalogy.1996-2018.SweSub.1080p.x264-Justiso";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect hexalogy", () => {
        const releaseName = "American.Pie.Heptalogy.SWESUB.DVDRip.XviD-BaZZe";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect anthalogy", () => {
        const releaseName = "The Exorcist 1, 2, 3, 4, 5 - Complete Horror Anthology 1973-2005";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect saga", () => {
        const releaseName = "Harry.Potter.Complete.Saga. I - VIII .1080p.Bluray.x264.anoXmous";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should detect italian complete", () => {
        const releaseName = "Inganno - Miniserie (2024) [COMPLETA] SD H264 ITA AAC-UBi";
        expect(parse(releaseName)).to.deep.include({ complete: true });
    });

    it("should not remove collection from title", () => {
        const releaseName = "[Erai-raws] Ninja Collection - 05 [720p][Multiple Subtitle].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Ninja Collection", complete: true });
    });

    it("should not remove kolekcja from title", () => {
        const releaseName = "Kolekcja Halloween (1978-2022) [720p] [BRRip] [XviD] [AC3-ELiTE] [Lektor PL]";
        expect(parse(releaseName)).to.deep.include({ title: "Halloween", complete: true });
    });

    it("should detect trailing complete and keep it out of the title", () => {
        const releaseName = "Gurren Lagann Complete [BD 1080p]";

        expect(parse(releaseName)).to.deep.include({ title: "Gurren Lagann", complete: true });
    });

    it("should detect spanish complete series with an episode count", () => {
        const releaseName = "Mazinger Z La Serie Completa 92 Capitulos [DVDRip]";

        expect(parse(releaseName)).to.deep.include({ title: "Mazinger Z", complete: true });
    });

    it("should detect portuguese completo", () => {
        const releaseName = "Dragon Ball Kai Completo (2011) [Dual Audio]";

        expect(parse(releaseName)).to.deep.include({ title: "Dragon Ball Kai", complete: true });
    });

    it("should detect french integrale in parentheses", () => {
        const releaseName = "Death Note (Integrale) [VOSTFR 1080p]";

        expect(parse(releaseName)).to.deep.include({ title: "Death Note", complete: true });
    });

    it("should detect spanish complete season", () => {
        const releaseName = "Ghost in the Shell -Primera Temporada Completa";

        expect(parse(releaseName)).to.deep.include({ title: "Ghost in the Shell", complete: true });
    });

    it("should keep complete in a complete edition title", () => {
        const releaseName = "World Masterpiece Theater Complete Edition - Anne of Green Gables [DVDRip]";

        expect(parse(releaseName).title).to.include("Complete Edition");
    });

    it("should strip trailing full batch", () => {
        const releaseName = "Onigashima Paced - Full Batch";

        expect(parse(releaseName)).to.deep.include({ title: "Onigashima Paced", complete: true });
    });

    it("should strip bracketed batch", () => {
        const releaseName = "[Golumpa] 18if (Batch) [English Dub] [FuniDub 1080p x264 AAC] [MKV]";

        expect(parse(releaseName)).to.deep.include({ title: "18if", complete: true });
    });

    it("should strip leading unofficial batch", () => {
        const releaseName = "(Unofficial Batch) Cross Ange (BD)(Hi10)(720)(AAC)";

        expect(parse(releaseName)).to.deep.include({ title: "Cross Ange", complete: true });
    });

    it("should strip complete batch", () => {
        const releaseName = "Shinryaku! Ika Musume (Complete Batch) [10bit BD 720p]";

        expect(parse(releaseName)).to.deep.include({ title: "Shinryaku! Ika Musume", complete: true });
    });

    it("should not strip batch inside a longer bracket", () => {
        const releaseName = "[blad761] Princess Principal. [Batch 01 - 12] 720p [MKV Hi444PP AAC]";

        expect(parse(releaseName)).to.deep.include({ title: "Princess Principal." });
    });

    it("should not strip bad batch title", () => {
        const releaseName = "Star.Wars.The.Bad.Batch.S02E03.1080p";

        expect(parse(releaseName)).to.deep.include({ title: "Star Wars The Bad Batch" });
        expect(parse(releaseName)).to.not.have.property("complete");
    });
});
