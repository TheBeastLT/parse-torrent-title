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

    it("should detect title before complete series in parentheses", () => {
        const releaseName = "Dragon Ball Z (Complete Series) [1080p] [MP4] [English Audio]";
        expect(parse(releaseName)).to.deep.include({ title: "Dragon Ball Z" });
    });

    it("should detect title without sample flag", () => {
        const releaseName = "The Vanishing Of Sidney Hall 2017 Movies HDRip x264 5.1 +Sample";
        expect(parse(releaseName)).to.deep.include({ title: "The Vanishing Of Sidney Hall" });
    });

    it("should detect title before season in parentheses", () => {
        const releaseName = "[Judas] Vinland Saga (Season 2) [1080p][HEVC x265 10bit][Multi-Subs]";
        expect(parse(releaseName)).to.deep.include({ title: "Vinland Saga" });
    });

    it("should detect title with dash in it", () => {
        const releaseName = "Furiosa - A Mad Max Saga (2024) 2160p H265 HDR10 D V iTA EnG AC3 5 1 Sub iTA EnG NUiTA NUEnG AsPiDe-MIRCrew mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Furiosa - A Mad Max Saga" });
    });

    it("should detect short dotted title", () => {
        const releaseName = "Court.Cam.S02E01.720p.WEB.h264-ROBOTS[rartv]";
        expect(parse(releaseName)).to.deep.include({ title: "Court Cam", season: 2, episode: 1 });
    });

    it("should detect short title before episode name", () => {
        const releaseName = "Body Cam S08E09 A Hail of Bullets 1080p MAX WEB-DL DD 2 0 H 264-playWEB";
        expect(parse(releaseName)).to.deep.include({ title: "Body Cam", source: "WEB-DL" });
    });

    it("should detect non english title with year", () => {
        const releaseName = "La jeune fille et les loup 2008 1080p BluRay DTS5.1 x264-SbR.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "La jeune fille et les loup", year: 2008 });
    });

    it("should detect title with audio channels in parentheses", () => {
        const releaseName = "Deadpool (2016) (1080p) (7.1).mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Deadpool", year: 2016 });
    });

    it("should detect year title", () => {
        const releaseName = "1923 S02E01 The Killing Season 1080p AMZN WEB-DL DDP5 1 H 264-FLUX[TGx]";
        expect(parse(releaseName)).to.deep.include({ title: "1923", season: 2, episode: 1 });
    });

    it("should detect title after leading episode brackets", () => {
        const releaseName = "[S8.Ep4] Trailer Park Boys - Orangie's Pretty F++kin' Tough.mp4";
        expect(parse(releaseName)).to.deep.include({ title: "Trailer Park Boys - Orangie's Pretty F++kin' Tough" });
    });

    it("should detect title after leading hi10 tag", () => {
        const releaseName = "(Hi10)_Re_Zero_-_23_(BD_1080p)_(SCY)_(D62F164A).mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Re Zero", episode: 23, bitDepth: "10bit" });
    });

    it("should detect dotted title after site brackets", () => {
        const releaseName = "[BEST-TORRENTS.COM] The.Penguin.S01E07.MULTi.1080p.AMZN.WEB-DL.H264.DDP5.1.Atmos-K83";
        expect(parse(releaseName)).to.deep.include({ title: "The Penguin" });
    });

    it("should detect english title in parentheses after russian title", () => {
        const releaseName = "О мышах и людях (Of Mice and Men) 1992 BDRip 1080p.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Of Mice and Men" });
    });

    it("should detect title after site prefix", () => {
        const releaseName = "www.1TamilMV.phd - Oppenheimer (2023) English BluRay - 1080p - x264 - (DTS 5.1) - 7.3GB - ESub.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Oppenheimer", year: 2023 });
    });

    it("should detect title with year in it", () => {
        const releaseName = "Wonder Woman 1984 (2020) [UHDRemux 2160p DoVi P8 Es-DTSHD AC3 En-AC3].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Wonder Woman 1984", year: 2020 });
    });
});
