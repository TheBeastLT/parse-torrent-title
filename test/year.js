const { expect } = require("chai");
const parse = require("../index").parse;

describe("Paring year", () => {
    it("should detect the year correctly", () => {
        const releaseName = "Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO";

        expect(parse(releaseName)).to.deep.include({ year: 2014 });
    });

    it("should detect the year within braces correctly", () => {
        const releaseName = "Hercules (2014) 1080p BrRip H264 - YIFY";

        expect(parse(releaseName)).to.deep.include({ year: 2014 });
    });

    it("should detect the year within brackets correctly", () => {
        const releaseName = "One Shot [2014] DVDRip XViD-ViCKY";

        expect(parse(releaseName)).to.deep.include({ year: 2014 });
    });

    it("should detect the year but not the title if the title is a year", () => {
        const releaseName = "2012 2009 1080p BluRay x264 REPACK-METiS";

        expect(parse(releaseName)).to.deep.include({ year: 2009 });
    });

    it("should detect the year at the beginning if there is none", () => {
        const releaseName = "2008 The Incredible Hulk Feature Film.mp4'";
        expect(parse(releaseName)).to.deep.include({ year: 2008 });
    });

    it("should detect the year range", () => {
        const releaseName = "Harry Potter All Movies Collection 2001-2011 720p Dual KartiKing'";
        expect(parse(releaseName)).to.deep.include({ year: "2001-2011" });
    });

    it("should detect year range with simplified end year", () => {
        const releaseName = "Empty Nest Season 1 (1988 - 89) fiveofseven";
        expect(parse(releaseName)).to.deep.include({ year: "1988-1989" });
    });

    it("should not detect year from bitrate", () => {
        const releaseName = "04. Practice Two (1324mb 1916x1080 50fps 1970kbps x265 deef).mkv";
        expect(parse(releaseName)).to.not.have.property("year");
    });

    it("should not detect year spanish episode", () => {
        const releaseName = "Anatomia De Grey - Temporada 19 [HDTV][Cap.1905][Castellano][www.AtomoHD.nu].avi";
        expect(parse(releaseName)).to.not.have.property("year");
    });

    it("should detect a bracketed list of years as a year range", () => {
        const releaseName = "Багровые реки 1,2 / Les Rivières pourpres 1,2 [2000, 2004 г., триллер, криминал, детектив, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ year: "2000-2004" });
    });

    it("should detect a bracketed list of many years as a year range", () => {
        const releaseName = "Бетховен 1,2,3,4,5,6 / Beethoven 1,2,3,4,5,6 [1992,1993,2000,2001,2003,2008 г., Комедия, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ year: "1992-2008" });
    });

    it("should detect an em dash year range", () => {
        const releaseName = "Назад в будущее 1, 2, 3 / Back to the Future 1, 2, 3 [1985—1990, США, Комедии, Приключения]";
        expect(parse(releaseName)).to.deep.include({ year: "1985-1990" });
    });

    it("should detect a single bracketed year before other tags", () => {
        const releaseName = "4 дня в мае / 4 Tage im Mai [2011, BDRip]";
        expect(parse(releaseName)).to.deep.include({ year: 2011 });
    });

    it("should detect a space separated list of years as a year range", () => {
        const releaseName = "Перевозчик 1,2,3 / The Transporter 1,2,3 [2002 2005 2008, BDRip] Dub";
        expect(parse(releaseName)).to.deep.include({ year: "2002-2008" });
    });

    it("should detect a slash separated list of years as a year range", () => {
        const releaseName = "Миссия: Невыполнима [Трилогия] / Mission: Impossible [Trilogy] [1996 / 2000 / 2006 г., боевик]";
        expect(parse(releaseName)).to.deep.include({ year: "1996-2006" });
    });
});
