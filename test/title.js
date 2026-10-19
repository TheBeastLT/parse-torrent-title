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

    it("should keep a star inside the title after a group section", () => {
        const releaseName = "[UCCUSS] BLACK★ROCK SHOOTER ブラック★ロックシューター 全8話 + OVA (BD 1920x1080p AVC FLAC SUPx2)";
        expect(parse(releaseName)).to.deep.include({ title: "BLACK★ROCK SHOOTER" });
    });

    it("should keep a star inside the title before a trailing group section", () => {
        const releaseName = "Lucky☆Star ★Hi10★ [1080p]";
        expect(parse(releaseName)).to.deep.include({ title: "Lucky☆Star" });
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

    it("should not include all seasons prefix in title", () => {
        const releaseName = "House MD All Seasons (1-8) 720p Ultra-Compressed";
        expect(parse(releaseName)).to.deep.include({ title: "House MD", seasons: [1, 2, 3, 4, 5, 6, 7, 8] });
    });

    it("should not include complete keyword before all seasons in title", () => {
        const releaseName = "Breaking Bad Complete (All Seasons 1 to 5) -nelly-";
        expect(parse(releaseName)).to.deep.include({ title: "Breaking Bad", seasons: [1, 2, 3, 4, 5] });
    });

    it("should not include trailing comma in title", () => {
        const releaseName = "The Golden Girls, Seasons 1 thru 7 Complete, X264";
        expect(parse(releaseName)).to.deep.include({ title: "The Golden Girls", seasons: [1, 2, 3, 4, 5, 6, 7] });
    });

    it("should use the latin title when cyrillic season info precedes it", () => {
        const releaseName = "Клиника / Сезон 8 / Серия 1-12 (HDTV 720p) / Scrubs / Season 8/ Episode 1-12 [2009, HDTV, Sub]";
        expect(parse(releaseName)).to.deep.include({ title: "Scrubs", seasons: [8] });
    });

    it("should use the latin title when cyrillic episode info precedes it", () => {
        const releaseName = "Балда / серии 01-25 (25) / The Baldy Man [1996, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ title: "The Baldy Man" });
    });

    it("should use the latin title when cyrillic season brackets precede it", () => {
        const releaseName = "Отчаянные дегустаторы отправляются... (Сезон 1 Серий 1-6 из 7) / The Supersizers go [2007, PDTVRip]";
        expect(parse(releaseName)).to.deep.include({ title: "The Supersizers go", seasons: [1] });
    });

    it("should use the latin title when cyrillic season and episode segments precede it", () => {
        const releaseName = "Леди Баг и Супер-Кот / Сезон: 2 / Серии: 1-26 из 26 / Miraculous: Tales of Ladybug & Cat Noir [2018, WEB-DL]";
        expect(parse(releaseName)).to.deep.include({ title: "Miraculous: Tales of Ladybug & Cat Noir", seasons: [2] });
    });

    it("should use the latin title when cyrillic episode brackets precede it", () => {
        const releaseName = "Лето Нашей Тайны (1-20 серия (125)) / Corpo Dourado [1998, TVRip]";
        expect(parse(releaseName)).to.deep.include({ title: "Corpo Dourado" });
    });

    it("should keep the cyrillic title when no latin title is present", () => {
        const releaseName = "Мастершеф / Сезон: 2 / Серии: 1, 2 [2014, DVB-T2]";
        expect(parse(releaseName)).to.deep.include({ title: "Мастершеф", seasons: [2] });
    });

    it("should remove cast parenthesis before episode count of total", () => {
        const releaseName = "Хочу влюбиться / Koi ga Shitai x3 (Katayama Osamu, Yoshida Ken) 11/11 [Япония, 2001 г., романтика, драма, TVRip] [RAW] [JAP+Sub Rus]";
        expect(parse(releaseName)).to.deep.include({ title: "Koi ga Shitai x3", year: 2001 });
    });

    it("should remove cast parenthesis before episode count of total with details block", () => {
        const releaseName = "Наш учебник / Watashitachi no Kyokasho (Kawake Shunsaku, Hayama Hiroki) 12/12 [Япония, 2007 г., детектив, школа, TVRip]";
        expect(parse(releaseName)).to.deep.include({ title: "Watashitachi no Kyokasho", year: 2007 });
    });

    it("should remove single name cast parenthesis before episode count of total", () => {
        const releaseName = "Песня солнцу / Taiyo no uta (Yamamuro Daisuke) 10/10 [Япония, 2006 г., романтика, драма, TVRip]";
        expect(parse(releaseName)).to.deep.include({ title: "Taiyo no uta", year: 2006 });
    });

    it("should remove cast parenthesis with multiple alt titles before episode count of total", () => {
        const releaseName = "Якша / Яша / Yasha (Sato Shimako) 11/11 [Япония, 2000 г., Фантастика, драма, DVDRip][RAW][JAP+Sub Rus]";
        expect(parse(releaseName)).to.deep.include({ title: "Yasha", year: 2000 });
    });

    it("should remove cast parenthesis before episode count of total with short details block", () => {
        const releaseName = "Отец невесты / Hanayome to Papa (Sato Yuichi, Ishikawa Junichi) 12/12 [Япония, 2007 г., семейная]";
        expect(parse(releaseName)).to.deep.include({ title: "Hanayome to Papa", year: 2007 });
    });

    it("should remove cast parenthesis before details block", () => {
        const releaseName = "Наш учебник / Watashitachi no Kyokasho (Kawake Shunsaku, Hayama Hiroki) [Япония, 2007 г.]";
        expect(parse(releaseName)).to.deep.include({ title: "Watashitachi no Kyokasho", year: 2007 });
    });

    it("should keep slash numbers in title", () => {
        const releaseName = "Fahrenheit 9/11 (2004) 1080p";
        expect(parse(releaseName)).to.deep.include({ title: "Fahrenheit 9/11", year: 2004 });
    });

    it("should keep slash numbers title", () => {
        const releaseName = "24/7 2022 WEB-DL";
        expect(parse(releaseName)).to.deep.include({ title: "24/7", year: 2022 });
    });

    it("should keep accented first letter", () => {
        const releaseName = "Último Alvo 2024 WEB-DL 1080p x264 DUAL 5.1";
        expect(parse(releaseName)).to.deep.include({ title: "Último Alvo", year: 2024 });
    });

    it("should keep accented first letter with brackets", () => {
        const releaseName = "Área 15 2018 (720p) DUBLADO WWW.BLUDV.COM";
        expect(parse(releaseName)).to.deep.include({ title: "Área 15", year: 2018 });
    });

    it("should prefer latin only alt title over accented one", () => {
        const releaseName = "Полный рабочий день / À plein temps / Full Time (2021) BDRip 720p от msltel";
        expect(parse(releaseName)).to.deep.include({ title: "Full Time", year: 2021 });
    });

    it("should keep accented title when it is the only one", () => {
        const releaseName = "Безумно грустная принцесса / Šíleně smutná princezna [1968, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ title: "Šíleně smutná princezna", year: 1968 });
    });

    it("should keep ESP as part of the title before an episode dash", () => {
        const releaseName = "[HorribleSubs] Tokyo ESP - 08 [720p].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Tokyo ESP", episodes: [8] });
    });

    it("should skip tag sections after the release group", () => {
        const releaseName = "[BonoboSubs][4k]Renegade Immortal - Xian Ni Episode 100";
        expect(parse(releaseName)).to.deep.include({ title: "Renegade Immortal - Xian Ni", episodes: [100] });
    });

    it("should skip a year section after the release group", () => {
        const releaseName = "[Fansubber] [1995] Hyper Doll - Dub[EN,ES,IT,JP] & Sub[EN,FR]";
        expect(parse(releaseName)).to.deep.include({ title: "Hyper Doll", year: 1995 });
    });

    it("should keep the longest section when the name is only bracketed sections", () => {
        const releaseName = "[Wu Dong Qian Kun][2022][S3E06][1080p+] ENG SUB (Softsub)";
        expect(parse(releaseName)).to.deep.include({ title: "Wu Dong Qian Kun" });
    });

    it("should take the longest section when only a short tag is left outside the sections", () => {
        const releaseName = "[Kurina][Dou Luo Da Lu][Soul Land][120][1080P]EN";
        expect(parse(releaseName)).to.deep.include({ title: "Dou Luo Da Lu", episodes: [120] });
    });

    it("should not take the release group as title when other text remains", () => {
        expect(parse("[tvN] 미생.E20.END.141220.HDTV.H264.720p-WITH.mp4").title).to.not.equal("tvN");
        expect(parse("[philosophy-raws][COWBOY BEBOP][TV 01-26+Movie+SP]][BDRIP][1080p 10bit][Three Audio]").title).to.not.equal("philosophy-raws");
    });

    it("should not take a full width bracket section after the tags as title", () => {
        expect(parse("[SweetSub][小書痴的下剋上 領主的養女][Honzuki no Gekokujou S04][16][WebRip][1080P][AVC 8bit][繁日雙語]（第四季）")).to.deep.include({ title: "Honzuki no Gekokujou", seasons: [4], episodes: [16] });
        expect(parse("[Group][Show Name][05][1080P]【Season 2】").title).to.equal("Show Name");
    });

    it("should keep a short title after the release group", () => {
        expect(parse("[HorribleSubs] K - 11 [720p].mkv")).to.deep.include({ title: "K", episodes: [11] });
        expect(parse("[HDTV - Eng - SubITA] 24 S07E04 [CR-Bt]")).to.deep.include({ title: "24", episodes: [4] });
        expect(parse("[NC-Raws] 谎颜 - 10 (B-Global Donghua 1920x1080 HEVC AAC MKV)")).to.deep.include({ title: "谎颜", episodes: [10] });
    });

    it("should not keep a dangling sub marker before language codes", () => {
        const releaseName = "[Fansubber] [1998] Gekiganger 3 - Sub[EN,ES]";
        expect(parse(releaseName)).to.deep.include({ title: "Gekiganger 3", year: 1998 });
    });

    it("should keep sub when it is part of the title", () => {
        const releaseName = "The.Sub.2019.1080p.WEBRip.x264";
        expect(parse(releaseName)).to.deep.include({ title: "The Sub", year: 2019 });
    });

    it("should keep a hyphenated Ru as part of the title", () => {
        const releaseName = "[PUNCH] To Love-Ru Darkness 2nd - 10 (1080p) [rich_jc].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "To Love-Ru Darkness 2nd", episodes: [10] });
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should keep the Code Geass R2 season in the title", () => {
        const releaseName = "Code Geass R2 - 218 - Final Battle Tokyo II {C_P}.avi";
        expect(parse(releaseName)).to.deep.include({ title: "Code Geass R2", episodes: [218] });
    });

    it("should not keep the episode word before an episode dash", () => {
        const releaseName = "[GoGoAnime.net] One Piece episode - 436 [English Sub][480p].avi";
        expect(parse(releaseName)).to.deep.include({ title: "One Piece", episodes: [436] });
    });

    it("should end the title before a dual numbered episode", () => {
        const releaseName = "Dragon Ball Z - 058 (072) (Toonami) [P][C][marktrade].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Dragon Ball Z" });
    });

    it("should keep a dashed part number in the title before a dual numbered episode", () => {
        const releaseName = "Show - Part 2 - 05 (17).mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Show - Part 2" });
    });

    it("should end the title before an unspaced dual numbered episode", () => {
        const releaseName = "[Orphan] Smash Hit! - 03(1.5) (DVD) [632E4274].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Smash Hit!" });
    });

    it("should end the title before a dual numbered episode with a part letter", () => {
        const releaseName = "Dragon Ball Z - 117A (132A) (TV3) [P][DragoonClawNZ].mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Dragon Ball Z", episodes: [132] });
    });

    it("should remove a chinese season marker from the title", () => {
        const releaseName = "[ANi] 葬送的芙莉蓮 第二季 - 03 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4";
        expect(parse(releaseName)).to.deep.include({ title: "葬送的芙莉蓮", seasons: [2], episodes: [3] });
    });

    it("should keep the romaji title after a chinese title with a season marker", () => {
        const releaseName = "[晚街与灯][Re：从零开始的异世界生活 第四季 / Re:Zero kara Hajimeru Isekai Seikatsu 4th Season][15 / 总第81][简日双语][1080P]";
        expect(parse(releaseName)).to.deep.include({ title: "Re:Zero kara Hajimeru Isekai Seikatsu", seasons: [4], episodes: [15] });
    });

    it("should drop a quoted episode title and a glued year suffix after a japanese title", () => {
        const releaseName = "NNNドキュメント’26「縁組み家族 こどもの権利にオトナはどうこたえる」";
        expect(parse(releaseName)).to.deep.include({ title: "NNN" });
    });

    it("should keep a year suffix glued to a russian title", () => {
        const releaseName = "Берлин'23 / Серии: 1-7 из 8 [2026, WEBRip-AVC]";
        expect(parse(releaseName)).to.deep.include({ title: "Берлин'23" });
    });

    it("should drop a trailing quoted japanese episode title", () => {
        const releaseName = "ガリレオX「躍進するフィジカルAI開発 次世代ロボットは何ができるのか？」";
        expect(parse(releaseName)).to.deep.include({ title: "ガリレオX" });
    });

    it("should drop a trailing quoted edition name", () => {
        const releaseName = "[hydes] Patlabor: The Movie『Enthusiast's Collected Edition』JPBD Remux｜5xAudio/5xSub｜Extras";
        expect(parse(releaseName)).to.deep.include({ title: "Patlabor: The Movie" });
    });

    it("should drop a trailing quoted subtitle with the dash before it", () => {
        const releaseName = "[JMAX] [2026.08.18] 学園アイドルマスター - 「ねえ、言っちゃうよ。」 [FLAC 96kHz/24bit]";
        expect(parse(releaseName)).to.deep.include({ title: "学園アイドルマスター" });
    });

    it("should keep a quoted title after a movie prefix", () => {
        const releaseName = "Movie『 Your Name. 』2016 1080p.MP4";
        expect(parse(releaseName).title).to.not.equal("Movie");
    });

    it("should keep removing chinese chars glued to a latin title", () => {
        const releaseName = "[LoliHouse] MAO摩绪 - 20 [WebRip 1080p HEVC-10bit AAC][简繁内封字幕]";
        expect(parse(releaseName)).to.deep.include({ title: "MAO" });
    });

    it("should remove a channel number prefix from the title", () => {
        const releaseName = "Ch5 The Rise and Fall of the King of Spain 2021 1080p HDTV x265-MVGroup";
        expect(parse(releaseName)).to.deep.include({ title: "The Rise and Fall of the King of Spain", year: 2021 });
    });

    it("should remove a dotted channel 4 prefix from the title", () => {
        const releaseName = "Ch4.Graysons.Art.Club.Series.2.1080p.HDTV.x265.AAC.MVGroup.org.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Graysons Art Club" });
    });

    it("should keep a language name followed by title words before the year in the title", () => {
        const releaseName = "A Serbian Film 2010 1080p BluRay";
        expect(parse(releaseName)).to.deep.include({ title: "A Serbian Film", year: 2010 }).and.not.have.property("languages");
    });

    it("should keep a dot separated language name followed by title words before the year in the title", () => {
        const releaseName = "The.French.Connection.1971.x264.DTS-WAF";
        expect(parse(releaseName)).to.deep.include({ title: "The French Connection", year: 1971 }).and.not.have.property("languages");
    });

    it("should keep a language name followed by title words before any tag in the title without a year", () => {
        const releaseName = "A.Serbian.Film.1080p.Multi.Subs.AAC.x264-DLKING.mp4";
        expect(parse(releaseName)).to.deep.include({ title: "A Serbian Film", languages: ["multi subs"] });
    });

    it("should keep a language name before the season in the title while reading the caps language tag", () => {
        const releaseName = "A.French.Village.S02.FRENCH.1080p.AMZN.WEBRip.DDP2.0.x264-SbR[rartv]";
        expect(parse(releaseName)).to.deep.include({ title: "A French Village", seasons: [2], languages: ["french"] });
    });

    it("should keep a leading language name in the title of a series", () => {
        const releaseName = "Swedish.Dicks.S01E01.WEB-DL.x264-RARBG";
        expect(parse(releaseName)).to.deep.include({ title: "Swedish Dicks", seasons: [1] }).and.not.have.property("languages");
    });

    it("should keep a language name in the middle of a title before the year", () => {
        const releaseName = "The French Dispatch 2021 WEB-DL Sonata Premiere";
        expect(parse(releaseName)).to.deep.include({ title: "The French Dispatch", year: 2021 }).and.not.have.property("languages");
    });

    it("should keep a language surname followed by title words before the year", () => {
        const releaseName = "Dawn.French.Live-Thirty.Million.Minutes.2016.1080p.WEBRip.x265-RARBG";
        expect(parse(releaseName)).to.deep.include({ title: "Dawn French Live-Thirty Million Minutes", year: 2016 }).and.not.have.property("languages");
    });

    it("should end the title at a scene caps language tag after a mixed case title", () => {
        const releaseName = "Vuoi.Sapere.Un.Segreto.iTALiAN.DVDRip.XviD-Republic";
        expect(parse(releaseName)).to.deep.include({ title: "Vuoi Sapere Un Segreto", languages: ["italian"] });
    });

    it("should end the title at a caps language tag before the year", () => {
        const releaseName = "The Reader ITALIAN 2009-BY NERO69.avi";
        expect(parse(releaseName)).to.deep.include({ title: "The Reader", year: 2009, languages: ["italian"] });
    });

    it("should keep a caps language word inside an all caps title", () => {
        const releaseName = "THE ITALIAN JOB 2003 1080p BluRay x264";
        expect(parse(releaseName)).to.deep.include({ title: "THE ITALIAN JOB", year: 2003 }).and.not.have.property("languages");
    });

    it("should not detect a language code inside an accented word", () => {
        const releaseName = "Mistrovský plán / Masterplan (2026)(CZ/EN)[2160p][HDR10+/DV][HEVC]";
        expect(parse(releaseName)).to.deep.include({ title: "Masterplan", year: 2026, languages: ["english", "czech"] });
    });

    it("should not detect an episode marker inside an accented word", () => {
        const releaseName = "Sonríe 2 (2024).1080p.dual-lat.cinecalidad.day.mp4";
        expect(parse(releaseName)).to.deep.include({ title: "Sonríe 2", year: 2024 }).and.not.have.property("episodes");
    });

    it("should not detect a container inside an accented word", () => {
        const releaseName = "Aviões 2 (2014) BluRay 1080p 5.1 Dublado - S4F";
        expect(parse(releaseName)).to.deep.include({ title: "Aviões 2", year: 2014 }).and.not.have.property("container");
    });

    it("should keep a dashed subtitle of a bare file name in the title", () => {
        const releaseName = "Thor - Ragnarok.mp4";
        expect(parse(releaseName)).to.deep.include({ title: "Thor - Ragnarok" });
    });

    it("should keep a hyphenated bare file name whole in the title", () => {
        const releaseName = "Ant-Man.mp4";
        expect(parse(releaseName)).to.deep.include({ title: "Ant-Man" });
    });
});
