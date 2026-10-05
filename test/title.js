const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing title", () => {
    it("should return the title", () => {
        const releaseName = "La famille bélier";

        expect(parse(releaseName)).to.deep.include({ title: "La famille bélier" });
    });

    it("should remove dots", () => {
        const releaseName = "La.famille.bélier";

        expect(parse(releaseName)).to.deep.include({ title: "La famille bélier" });
    });

    it("should not remove dots when they are part of the title", () => {
        const releaseName = "Mr. Nobody";

        expect(parse(releaseName)).to.deep.include({ title: "Mr. Nobody" });
    });

    it("should remove underscores", () => {
        const releaseName = "doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov";

        expect(parse(releaseName)).to.deep.include({ title: "doctor who" });
    });

    it("should remove unnecessary translations", () => {
        const releaseName = "[GM-Team][国漫][太乙仙魔录 灵飞纪 第3季][Magical Legend of Rise to immortality Ⅲ][01-26][AVC][GB][1080P]";
        expect(parse(releaseName)).to.deep.include({ title: "Magical Legend of Rise to immortality Ⅲ" });
    });

    it("should remove unnecessary translations not included in brackets", () => {
        const releaseName = "【喵萌奶茶屋】★01月新番★[Rebirth][01][720p][简体][招募翻译]";
        expect(parse(releaseName)).to.deep.include({ title: "Rebirth" });
    });

    it("should remove japanese alt titles", () => {
        const releaseName = "【喵萌奶茶屋】★01月新番★[別對映像研出手！/映像研には手を出すな！/Eizouken ni wa Te wo Dasu na!][01][1080p][繁體]";
        expect(parse(releaseName)).to.deep.include({ title: "Eizouken ni wa Te wo Dasu na!" });
    });

    it("should remove japanese alt titles when the main one is in the middle", () => {
        const releaseName = "【喵萌奶茶屋】★01月新番★[別對映像研出手！/Eizouken ni wa Te wo Dasu na!/映像研には手を出すな！][01][1080p][繁體]";
        expect(parse(releaseName)).to.deep.include({ title: "Eizouken ni wa Te wo Dasu na!" });
    });

    it("should remove japanese alt titles without separators", () => {
        const releaseName = "[Seed-Raws] 劇場版 ペンギン・ハイウェイ Penguin Highway The Movie (BD 1280x720 AVC AACx4 [5.1+2.0+2.0+2.0]).mp4";
        expect(parse(releaseName)).to.deep.include({ title: "Penguin Highway The Movie" });
    });

    it("should not split slash separated title", () => {
        const releaseName = "[SweetSub][Mutafukaz / MFKZ][Movie][BDRip][1080P][AVC 8bit][简体内嵌]";
        expect(parse(releaseName)).to.deep.include({ title: "Mutafukaz / MFKZ" });
    });

    it("should clean release group tag title", () => {
        const releaseName = "[Erai-raws] Kingdom 3rd Season - 02 [1080p].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Kingdom" });
    });

    it("should detect remove russian alt title", () => {
        const releaseName = "Голубая волна / Blue Crush (2002) DVDRip";
        expect(parse(releaseName)).to.deep.include({ title: "Blue Crush" });
    });

    it("should not remove non english title if its the only thing left", () => {
        const releaseName = "Жихарка (2007) DVDRip";
        expect(parse(releaseName)).to.deep.include({ title: "Жихарка" });
    });

    it("should not remove non english title with digits in it", () => {
        const releaseName = "3 Миссия невыполнима 3 2006г. BDRip 1080p.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "3 Миссия невыполнима 3" });
    });

    it("should not remove russian movie numbering with dot and space", () => {
        const releaseName = "1. Детские игры. 1988. 1080p. HEVC. 10bit..mkv";
        expect(parse(releaseName)).to.deep.include({ title: "1. Детские игры." });
    });

    it("should not remove russian movie numbering with number in title", () => {
        const releaseName = "01. 100 девчонок и одна в лифте 2000 WEBRip 1080p.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "01. 100 девчонок и одна в лифте" });
    });

    it("should not remove russian movie numbering with dot", () => {
        const releaseName = "08.Планета.обезьян.Революция.2014.BDRip-HEVC.1080p.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "08 Планета обезьян Революция" });
    });

    it("should clear russian cast info from title", () => {
        const releaseName = "Американские животные / American Animals (Барт Лэйтон / Bart Layton) [2018, Великобритания, США, драма, криминал, BDRip] MVO (СВ Студия)";
        expect(parse(releaseName)).to.deep.include({ title: "American Animals" });
    });

    it("should clear cast info from russian title", () => {
        const releaseName = "Греческая смоковница / Griechische Feigen / The Fruit Is Ripe (Зиги Ротемунд / Sigi Rothemund (as Siggi Götz)) [1976, Германия (ФРГ), эротика, комедия, приключения, DVDRip] 2 VO";
        expect(parse(releaseName)).to.deep.include({ title: "Griechische Feigen / The Fruit Is Ripe" });
    });

    it("should clear cast info from russian title v2", () => {
        const releaseName = "Греческая смоковница / The fruit is ripe / Griechische Feigen (Siggi Götz) [1976, Германия, Эротическая комедия, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ title: "The fruit is ripe / Griechische Feigen" });
    });

    it("should clear cast info from russian title v3", () => {
        const releaseName = "Бастер / Buster (Дэвид Грин / David Green) [1988, Великобритания, Комедия, мелодрама, драма, приключения, криминал, биография, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ title: "Buster" });
    });

    it("should detect title even when year is in beginning with paranthesis", () => {
        const releaseName = "(2000) Le follie dell'imperatore - The Emperor's New Groove (DvdRip Ita Eng AC3 5.1).avi";
        expect(parse(releaseName)).to.deep.include({ title: "Le follie dell'imperatore - The Emperor's New Groove" });
    });

    it("should remove chinese alt title", () => {
        const releaseName = "[NC-Raws] 间谍过家家 / SPY×FAMILY - 04 (B-Global 1920x1080 HEVC AAC MKV)";
        expect(parse(releaseName)).to.deep.include({ title: "SPY×FAMILY" });
    });

    it("should remove ep range in parenthesis", () => {
        const releaseName = "GTO (Great Teacher Onizuka) (Ep. 1-43) Sub 480p lakshay";
        expect(parse(releaseName)).to.deep.include({ title: "GTO (Great Teacher Onizuka)" });
    });

    it("should not fully remove partially russian title", () => {
        const releaseName = "Книгоноши / Кнiганошы (1987) TVRip от AND03AND | BLR";
        expect(parse(releaseName)).to.deep.include({ title: "Кнiганошы" });
    });

    it("should remove extension fully", () => {
        const releaseName = "Yurusarezaru_mono2.srt";
        expect(parse(releaseName)).to.deep.include({ title: "Yurusarezaru mono2" });
    });

    it("should not detect season prefix in title", () => {
        const releaseName = "COMPASS2.0.ANIMATION.PROJECT.S01E02.Will.You.Be.My.Partner.1080p.CR.WEB-DL.JPN.AAC2.0.H.264.MSubs-ToonsHub.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "COMPASS2 0 ANIMATION PROJECT" });
    });

    it("should detect title with absolute episode at the beginning", () => {
        const releaseName = "1125 - One piece [Sub] 1080p";
        expect(parse(releaseName)).to.deep.include({ title: "One piece" });
    });
});
