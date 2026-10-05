const { expect } = require("chai");
const parse = require("../index").parse;

describe("Random releases", () => {
    it("sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR", () => {
        const releaseName = "sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR";

        expect(parse(releaseName)).to.deep.equal({
            title: "sons of anarchy",
            resolution: "480p",
            seasons: [5],
            season: 5,
            episodes: [10],
            episode: 10,
            source: "BluRay",
            codec: "x264",
            group: "GAnGSteR",
        });
    });

    it("Color.Of.Night.Unrated.DC.VostFR.BRrip.x264", () => {
        const releaseName = "Color.Of.Night.Unrated.DC.VostFR.BRrip.x264";

        expect(parse(releaseName)).to.deep.equal({
            title: "Color Of Night",
            unrated: true,
            languages: ["french"],
            source: "BRRip",
            codec: "x264",
        });
    });

    it("Da Vinci Code DVDRip", () => {
        const releaseName = "Da Vinci Code DVDRip";

        expect(parse(releaseName)).to.deep.equal({
            title: "Da Vinci Code",
            source: "DVDRip",
        });
    });

    it("Some.girls.1998.DVDRip", () => {
        const releaseName = "Some.girls.1998.DVDRip";

        expect(parse(releaseName)).to.deep.equal({
            title: "Some girls",
            source: "DVDRip",
            year: 1998,
        });
    });

    it("Ecrit.Dans.Le.Ciel.1954.MULTI.DVDRIP.x264.AC3-gismo65", () => {
        const releaseName = "Ecrit.Dans.Le.Ciel.1954.MULTI.DVDRIP.x264.AC3-gismo65";

        expect(parse(releaseName)).to.deep.equal({
            title: "Ecrit Dans Le Ciel",
            source: "DVDRip",
            year: 1954,
            languages: ["multi audio"],
            dubbed: true,
            codec: "x264",
            audio: "ac3",
            group: "gismo65",
        });
    });

    it("2019 After The Fall Of New York 1983 REMASTERED BDRip x264-GHOULS", () => {
        const releaseName = "2019 After The Fall Of New York 1983 REMASTERED BDRip x264-GHOULS";

        expect(parse(releaseName)).to.deep.equal({
            title: "2019 After The Fall Of New York",
            source: "BDRip",
            remastered: true,
            year: 1983,
            codec: "x264",
            group: "GHOULS",
        });
    });

    it("Ghost In The Shell 2017 720p HC HDRip X264 AC3-EVO", () => {
        const releaseName = "Ghost In The Shell 2017 720p HC HDRip X264 AC3-EVO";

        expect(parse(releaseName)).to.deep.equal({
            title: "Ghost In The Shell",
            source: "HDRip",
            hardcoded: true,
            year: 2017,
            resolution: "720p",
            codec: "x264",
            audio: "ac3",
            group: "EVO",
        });
    });

    it("Rogue One 2016 1080p BluRay x264-SPARKS", () => {
        const releaseName = "Rogue One 2016 1080p BluRay x264-SPARKS";

        expect(parse(releaseName)).to.deep.equal({
            title: "Rogue One",
            source: "BluRay",
            year: 2016,
            resolution: "1080p",
            codec: "x264",
            group: "SPARKS",
        });
    });

    it("Desperation 2006 Multi Pal DvdR9-TBW1973", () => {
        const releaseName = "Desperation 2006 Multi Pal DvdR9-TBW1973";

        expect(parse(releaseName)).to.deep.equal({
            title: "Desperation",
            source: "DVD",
            year: 2006,
            languages: ["multi audio"],
            dubbed: true,
            region: "R9",
            group: "TBW1973",
        });
    });

    it("Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG", () => {
        const releaseName = "Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG";

        expect(parse(releaseName)).to.deep.equal({
            title: "Maman, j'ai raté l'avion",
            source: "BluRay",
            year: 1990,
            audio: "dts",
            resolution: "1080p",
            languages: ["french"],
            codec: "x265",
            group: "HTG",
        });
    });

    it("Game of Thrones - The Complete Season 3 [HDTV]", () => {
        const releaseName = "Game of Thrones - The Complete Season 3 [HDTV]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Game of Thrones",
            seasons: [3],
            season: 3,
            source: "HDTV"
        });
    });

    it("The Sopranos: The Complete Series (Season 1,2,3,4,5&6) + Extras", () => {
        const releaseName = "The Sopranos: The Complete Series (Season 1,2,3,4,5&6) + Extras";
        expect(parse(releaseName)).to.deep.equal({
            title: "The Sopranos",
            seasons: [1, 2, 3, 4, 5, 6],
            complete: true
        });
    });

    it("Skins Season S01-S07 COMPLETE UK Soundtrack 720p WEB-DL", () => {
        const releaseName = "Skins Season S01-S07 COMPLETE UK Soundtrack 720p WEB-DL";
        expect(parse(releaseName)).to.deep.equal({
            seasons: [1, 2, 3, 4, 5, 6, 7],
            title: "Skins",
            resolution: "720p",
            source: "WEB-DL"
        });
    });

    it("Futurama.COMPLETE.S01-S07.720p.BluRay.x265-HETeam", () => {
        const releaseName = "Futurama.COMPLETE.S01-S07.720p.BluRay.x265-HETeam";
        expect(parse(releaseName)).to.deep.equal({
            title: "Futurama",
            seasons: [1, 2, 3, 4, 5, 6, 7],
            resolution: "720p",
            source: "BluRay",
            codec: "x265",
            group: "HETeam"
        });
    });

    it("You.[Uncut].S01.SweSub.1080p.x264-Justiso", () => {
        const releaseName = "You.[Uncut].S01.SweSub.1080p.x264-Justiso";
        expect(parse(releaseName)).to.deep.equal({
            title: "You",
            seasons: [1],
            season: 1,
            languages: ["swedish"],
            resolution: "1080p",
            codec: "x264",
            group: "Justiso",
        });
    });

    it("Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]", () => {
        const releaseName = "Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Stephen Colbert",
            date: "2019-10-25",
            resolution: "480p",
            codec: "x264",
        });
    });

    it("House MD Season 7 Complete MKV", () => {
        const releaseName = "House MD Season 7 Complete MKV";
        expect(parse(releaseName)).to.deep.equal({
            title: "House MD",
            season: 7,
            seasons: [7],
            container: "mkv"
        });
    });

    it("2008 The Incredible Hulk Feature Film.mp4", () => {
        const releaseName = "2008 The Incredible Hulk Feature Film.mp4";
        expect(parse(releaseName)).to.deep.equal({
            title: "The Incredible Hulk Feature Film",
            year: 2008,
            container: "mp4",
            extension: "mp4"
        });
    });

    it("【4月/悠哈璃羽字幕社】[UHA-WINGS][不要输！恶之军团][Makeruna!! Aku no Gundan!][04][1080p AVC_AAC][简繁外挂][sc_tc]", () => {
        const releaseName = "【4月/悠哈璃羽字幕社】[UHA-WINGS][不要输！恶之军团][Makeruna!! Aku no Gundan!][04][1080p AVC_AAC][简繁外挂][sc_tc]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Makeruna!! Aku no Gundan!",
            episodes: [4],
            episode: 4,
            resolution: "1080p",
            codec: "avc",
            audio: "aac",
        });
    });

    it("[GM-Team][国漫][西行纪之集结篇][The Westward Ⅱ][2019][17][AVC][GB][1080P]", () => {
        const releaseName = "[GM-Team][国漫][西行纪之集结篇][The Westward Ⅱ][2019][17][AVC][GB][1080P]";
        expect(parse(releaseName)).to.deep.equal({
            title: "The Westward Ⅱ",
            year: 2019,
            episodes: [17],
            episode: 17,
            resolution: "1080p",
            codec: "avc",
            group: "GM-Team"
        });
    });

    it("Черное зеркало / Black Mirror / Сезон 4 / Серии 1-6 (6) [2017, США, WEBRip 1080p] MVO + Eng Sub", () => {
        const releaseName = "Черное зеркало / Black Mirror / Сезон 4 / Серии 1-6 (6) [2017, США, WEBRip 1080p] MVO + Eng Sub";
        expect(parse(releaseName)).to.deep.equal({
            title: "Black Mirror",
            year: 2017,
            seasons: [4],
            season: 4,
            episodes: [1, 2, 3, 4, 5, 6],
            languages: ["english"],
            resolution: "1080p",
            source: "WEBRip",
        });
    });

    it("[neoHEVC] Student Council's Discretion / Seitokai no Ichizon [Season 1] [BD 1080p x265 HEVC AAC]", () => {
        const releaseName = "[neoHEVC] Student Council's Discretion / Seitokai no Ichizon [Season 1] [BD 1080p x265 HEVC AAC]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Student Council's Discretion / Seitokai no Ichizon",
            seasons: [1],
            season: 1,
            resolution: "1080p",
            source: "BDRip",
            audio: "aac",
            codec: "hevc",
            group: "neoHEVC"
        });
    });

    it("[Commie] Chihayafuru 3 - 21 [BD 720p AAC] [5F1911ED].mkv", () => {
        const releaseName = "[Commie] Chihayafuru 3 - 21 [BD 720p AAC] [5F1911ED].mkv";
        expect(parse(releaseName)).to.deep.equal({
            title: "Chihayafuru 3",
            episodes: [21],
            episode: 21,
            resolution: "720p",
            source: "BDRip",
            audio: "aac",
            container: "mkv",
            extension: "mkv",
            episodeCode: "5F1911ED",
            group: "Commie"
        });
    });

    it("[DVDRip-ITA]The Fast and the Furious: Tokyo Drift [CR-Bt]", () => {
        const releaseName = "[DVDRip-ITA]The Fast and the Furious: Tokyo Drift [CR-Bt]";
        expect(parse(releaseName)).to.deep.equal({
            title: "The Fast and the Furious: Tokyo Drift",
            source: "DVDRip",
            languages: ["italian"]
        });
    });

    it("[BluRay Rip 720p ITA AC3 - ENG AC3 SUB] Hostel[2005]-LIFE[ultimafrontiera]", () => {
        const releaseName = "[BluRay Rip 720p ITA AC3 - ENG AC3 SUB] Hostel[2005]-LIFE[ultimafrontiera]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Hostel",
            year: 2005,
            resolution: "720p",
            source: "BRRip",
            audio: "ac3",
            languages: ["english", "italian"],
            group: "LIFE"
        });
    });

    it("[OFFICIAL ENG SUB] Soul Land Episode 121-125 [1080p][Soft Sub][Web-DL][Douluo Dalu][斗罗大陆]", () => {
        const releaseName = "[OFFICIAL ENG SUB] Soul Land Episode 121-125 [1080p][Soft Sub][Web-DL][Douluo Dalu][斗罗大陆]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Soul Land",
            episodes: [121, 122, 123, 124, 125],
            resolution: "1080p",
            source: "WEB-DL",
            languages: ["english"]
        });
    });

    it("[720p] The God of Highschool Season 1", () => {
        const releaseName = "[720p] The God of Highschool Season 1";
        expect(parse(releaseName)).to.deep.equal({
            title: "The God of Highschool",
            seasons: [1],
            season: 1,
            resolution: "720p"
        });
    });

    it("Heidi Audio Latino DVDRip [cap. 3 Al 18]", () => {
        const releaseName = "Heidi Audio Latino DVDRip [cap. 3 Al 18]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Heidi",
            episodes: [3],
            episode: 3,
            source: "DVDRip",
            languages: ["latino"]
        });
    });

    it("Sprint.2024.S01.COMPLETE.1080p.WEB.h264-EDITH[TGx]", () => {
        const releaseName = "Sprint.2024.S01.COMPLETE.1080p.WEB.h264-EDITH[TGx]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Sprint",
            year: 2024,
            seasons: [1],
            season: 1,
            resolution: "1080p",
            source: "WEB",
            codec: "h264",
            group: "EDITH"
        });
    });

    it("High Heat *2022* [BRRip.XviD] Lektor PL", () => {
        const releaseName = "High Heat *2022* [BRRip.XviD] Lektor PL";
        expect(parse(releaseName)).to.deep.equal({
            title: "High Heat",
            year: 2022,
            codec: "xvid",
            source: "BRRip",
            languages: ["polish"]
        });
    });

    it("Ghost Busters *1984-2021* [720p] [BDRip] [AC-3] [XviD] [Lektor + Dubbing PL] [DYZIO]", () => {
        const releaseName = "Ghost Busters *1984-2021* [720p] [BDRip] [AC-3] [XviD] [Lektor + Dubbing PL] [DYZIO]";
        expect(parse(releaseName)).to.deep.equal({
            title: "Ghost Busters",
            year: "1984-2021",
            resolution: "720p",
            audio: "ac3",
            codec: "xvid",
            source: "BDRip",
            languages: ["polish"]
        });
    });

    it("20-20.2024.11.15.Fatal.Disguise.XviD-AFG[EZTVx.to].avi", () => {
        const releaseName = "20-20.2024.11.15.Fatal.Disguise.XviD-AFG[EZTVx.to].avi";
        expect(parse(releaseName)).to.deep.equal({
            title: "20-20",
            date: "2024-11-15",
            codec: "xvid",
            source: "XviD",
            group: "AFG",
            container: "avi",
            extension: "avi"
        });
    });
});
