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

    it("should detect two bracketed years of films joined by an ampersand as a year range", () => {
        const releaseName = "Wonder Woman(2017) & Wonder Woman 1984(2020) [1080p] [H264] [Dubbing,Lektor PL] SYRIX";
        expect(parse(releaseName)).to.deep.include({ title: "Wonder Woman", year: "2017-2020" });
    });

    it("should detect two bracketed years of numbered films joined by an ampersand as a year range", () => {
        const releaseName = "Legion samobójców 1(2016) & Legion samobójców 2(2021) [1080p] [H264] [Lektor] SYRIX";
        expect(parse(releaseName)).to.deep.include({ year: "2016-2021" });
    });

    it("should not detect a year range in a title with an ampersand and one year", () => {
        const releaseName = "Tom & Jerry (2021) 1080p WEBRip";
        expect(parse(releaseName)).to.deep.include({ title: "Tom & Jerry", year: 2021 });
    });

    it("should detect three bracketed years of films joined by ampersands as a year range", () => {
        const releaseName = "John Wick 1(2014)  &  John Wick 2(2017) &  John Wick 3(2019) [1080p] [H264] [Lektor PL] SYRIX";
        expect(parse(releaseName)).to.deep.include({ year: "2014-2019" });
    });

    it("should keep seasons joined by an ampersand with their own years", () => {
        const releaseName = "[Ari-M] Berserk Memorial Edition + Berserk S1 (2016) & S2 (2017) [CR Web-dl 1080p]";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect the bracketed year after a title year and an alternative title", () => {
        const releaseName = "Захват самолёта / Hijack 1971 / Haijaeking (2024) WEB-DL 1080p от Scar";
        expect(parse(releaseName)).to.deep.include({ title: "Hijack 1971 / Haijaeking", year: 2024 }).and.not.have.property("episodes");
    });

    it("should detect the bracketed year after Wonder Woman 1984 and an alternative title", () => {
        const releaseName = "Чудо-женщина 1984 / Wonder Woman 1984 / WW84 (2020) BDRip";
        expect(parse(releaseName)).to.deep.include({ title: "Wonder Woman 1984 / WW84", year: 2020 }).and.not.have.property("episodes");
    });

    it("should detect the bracketed year after a year in the original title", () => {
        const releaseName = "Забастовка / Sulis 1907 / The Riot (2023) BDRip от MegaPeer | P2 | ViruseProject";
        expect(parse(releaseName)).to.deep.include({ title: "Sulis 1907 / The Riot", year: 2023 }).and.not.have.property("episodes");
    });

    it("should read the second of two years as the year and keep the first in the title", () => {
        const releaseName = "Hijack.1971.2024.1080p.WEB-DL.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Hijack 1971", year: 2024 });
    });

    it("should keep a title year before a dotted release year without a trailing dot", () => {
        const releaseName = "Chernobyl 1986.2021.1080p.WEBRip.x264-EVO";
        expect(parse(releaseName)).to.deep.include({ title: "Chernobyl 1986", year: 2021 });
    });

    it("should read a release year older than the year in the title", () => {
        const releaseName = "Godzilla.2000.1999.720p.BluRay.x264-WaLMaRT";
        expect(parse(releaseName)).to.deep.include({ title: "Godzilla 2000", year: 1999 });
    });

    it("should not keep a repeated year in the title", () => {
        const releaseName = "Silent House 2011 2011 480p BRRiP XViD AC3-LEGi0N";
        expect(parse(releaseName)).to.deep.include({ title: "Silent House", year: 2011 });
    });

    it("should read the first of three years of a trilogy as the year", () => {
        const releaseName = "Poltergeist Trilogy 1982 1986 1988 720p BRRip x264 -MgB";
        expect(parse(releaseName)).to.deep.include({ year: 1982 });
    });

    it("should read the first year of a collection with two years", () => {
        const releaseName = "House On Haunted Hill Complete Collection - Horror 1959 2007 Eng Subs 720p [H264-mp4]";
        expect(parse(releaseName)).to.deep.include({ year: 1959 });
    });

    it("should read the first year of two films joined by and", () => {
        const releaseName = "Doom And Doom Annihilation - Unrated Extended 2005 2019 Eng Rus Multi Subs 720p [H264-mp4]";
        expect(parse(releaseName)).to.deep.include({ year: 2005 });
    });

    it("should not read a resolution width after a year as the year", () => {
        const releaseName = "Arrival 2016 1920 x 808 x264 Phun Psyz";
        expect(parse(releaseName)).to.deep.include({ title: "Arrival", year: 2016 });
    });
});
