const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing episode", () => {
    it("should detect regular episode correctly", () => {
        const releaseName = "The Simpsons S28E21 720p HDTV x264-AVS";
        expect(parse(releaseName)).to.deep.include({ episode: 21 });
    });

    it("should detect regular episode with lowercase correctly", () => {
        const releaseName = "breaking.bad.s01e01.720p.bluray.x264-reward";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect regular episode with a space between", () => {
        const releaseName = "Dragon Ball Super S01 E23 French 1080p HDTV H264-Kesni";
        expect(parse(releaseName)).to.deep.include({ episode: 23 });
    });

    it("should detect regular episode without e symbol after season", () => {
        const releaseName = "The.Witcher.S01.07.2019.Dub.AVC.ExKinoRay.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 7 });
    });

    it("should detect regular episode with season symbol but wihout episode symbol", () => {
        const releaseName = "Vikings.s02.09.AVC.tahiy.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 9 });
    });

    it("should detect regular episode with a letter a suffix", () => {
        const releaseName = "The Twilight Zone 1985 S01E23a Shadow Play.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 23 });
    });

    it("should detect regular episode with a letter b suffix", () => {
        const releaseName = "Mash S10E01b Thats Show Biz Part 2 1080p H.264 (moviesbyrizzo upload).mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect regular episode with a letter c suffix", () => {
        const releaseName = "The Twilight Zone 1985 S01E22c The Library.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 22 });
    });

    it("should detect episode with SxEE format correctly", () => {
        const releaseName = "Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV";
        expect(parse(releaseName)).to.deep.include({ episode: 11 });
    });

    it("should detect episode when written as such", () => {
        const releaseName = "Anubis saison 01 episode 38 tvrip FR";
        expect(parse(releaseName)).to.deep.include({ episode: 38 });
    });

    it("should detect episode when written as such shortened", () => {
        const releaseName = "Le Monde Incroyable de Gumball - Saison 5 Ep 14 - L'extérieur";
        expect(parse(releaseName)).to.deep.include({ episode: 14 });
    });

    it("should detect episode with parenthesis prefix and x separator", () => {
        const releaseName = "Smallville (1x02 Metamorphosis).avi";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect episode when similar digits included", () => {
        const releaseName = "Friends.S07E20.The.One.With.Rachel's.Big.Kiss.720p.BluRay.2CH.x265.HEVC-PSA.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 7, episode: 20 });
    });

    it("should detect episode when separated with x and inside brackets", () => {
        const releaseName = "Friends - [8x18] - The One In Massapequa.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 8, episode: 18 });
    });

    it("should detect multiple episodes with x prefix and hyphen separator", () => {
        const releaseName = "Friends - [7x23-24] - The One with Monica and Chandler's Wedding + Audio Commentary.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 7, episodes: [23, 24] });
    });

    it("should detect multiple episodes with hyphen no spaces separator", () => {
        const releaseName = "611-612 - Desperate Measures, Means & Ends.mp4";
        expect(parse(releaseName)).to.deep.include({ episodes: [611, 612] });
    });

    it("should detect multiple single episode with 10-bit notation in it", () => {
        const releaseName = "[Final8]Suisei no Gargantia - 05 (BD 10-bit 1920x1080 x264 FLAC)[E0B15ACF].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 5 });
    });

    it("should detect multiple episodes with episodes prefix and hyphen separator", () => {
        const releaseName = "Orange Is The New Black Season 5 Episodes 1-10 INCOMPLETE (LEAKED)";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] });
    });

    it("should detect multiple episodes with ep prefix and hyphen separator inside parentheses", () => {
        const releaseName = "Vikings.Season.05.Ep(01-10).720p.WebRip.2Ch.x265.PSA";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] });
    });

    it("should detect multiple episodes with hyphen separator and follower by open parenthesis", () => {
        const releaseName = "[TBox] Dragon Ball Z Full 1-291(Subbed Jap Vers)";
        expect(parse(releaseName)).to.deep.include({ episodes: [...Array(291).keys()].map(i => i + 1) });
    });

    it("should detect multiple episodes with e prefix and hyphen separator", () => {
        const releaseName = "Marvel's.Agents.of.S.H.I.E.L.D.S02E01-03.Shadows.1080p.WEB-DL.DD5.1";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3] });
    });

    it("should detect absolute episode with ep prefix", () => {
        const releaseName = "Naruto Shippuden Ep 107 - Strange Bedfellows.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 107 });
    });

    it("should detect absolute episode in middle with hyphen dividers", () => {
        const releaseName = "Naruto Shippuden - 107 - Strange Bedfellows.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 107 });
    });

    it("should detect absolute episode in middle with similar resolution value", () => {
        const releaseName = "[AnimeRG] Naruto Shippuden - 107 [720p] [x265] [pseudo].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 107 });
    });

    it("should detect multiple absolute episodes separated by hyphen", () => {
        const releaseName = "Naruto Shippuuden - 006-007.mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [6, 7] });
    });

    it("should detect absolute episode correctly not hindered by title digits with hashtag", () => {
        const releaseName = "321 - Family Guy Viewer Mail #1.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 321 });
    });

    it("should detect absolute episode correctly not hindered by title digits with apostrophe", () => {
        const releaseName = "512 - Airport '07.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 512 });
    });

    it("should detect absolute episode at the begining even though its mixed with season", () => {
        const releaseName = "102 - The Invitation.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 102 });
    });

    it("should detect absolute episode double digit at the beginning", () => {
        const releaseName = "02 The Invitation.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect absolute episode triple digit at the beginning with zero padded", () => {
        const releaseName = "004 - Male Unbonding - [DVD].avi";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect multiple absolute episodes separated by comma", () => {
        const releaseName = "The Amazing World of Gumball - 103, 104 - The Third - The Debt.mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [103, 104] });
    });

    it("should detect absolute episode with a possible match at the end", () => {
        const releaseName = "The Amazing World of Gumball - 103 - The End - The Dress (720p.x264.ac3-5.1) [449].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 103 });
    });

    it("should detect absolute episode with a divided episode into a part", () => {
        const releaseName = "The Amazing World of Gumball - 107a - The Mystery (720p.x264.ac3-5.1) [449].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 107 });
    });

    it("should detect absolute episode with a divided episode into b part", () => {
        const releaseName = "The Amazing World of Gumball - 107b - The Mystery (720p.x264.ac3-5.1) [449].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 107 });
    });

    it("should detect episode withing brackets with dot separator", () => {
        const releaseName = "[5.01] Weight Loss.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect episode with spaces and hyphen separator", () => {
        const releaseName = "S01 - E03 - Fifty-Fifty.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 3 });
    });

    it("should detect multiple episodes separated with plus", () => {
        const releaseName = "The Office S07E25+E26 Search Committee.mp4";
        expect(parse(releaseName)).to.deep.include({ episodes: [25, 26] });
    });

    it("[animeawake] Naruto Shippuden - 124 - Art_2.mkv", () => {
        const releaseName = "[animeawake] Naruto Shippuden - 124 - Art_2.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 124 });
    });

    it("[animeawake] Naruto Shippuden - 072 - The Quietly Approaching Threat_2.mkv", () => {
        const releaseName = "[animeawake] Naruto Shippuden - 072 - The Quietly Approaching Threat_2.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 72 });
    });

    it("[animeawake] Naruto Shippuden - 120 - Kakashi Chronicles. Boys' Life on the Battlefield. Part 2.mkv", () => {
        const releaseName = "[animeawake] Naruto Shippuden - 120 - Kakashi Chronicles. Boys' Life on the Battlefield. Part 2.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 120 });
    });

    it("should detect single episode even when a possible range identifier hyphen is present", () => {
        const releaseName = "Supernatural - S03E01 - 720p BluRay x264-Belex - Dual Audio + Legenda.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should not detect episodes when the range is for season", () => {
        const releaseName = "[F-D] Fairy Tail Season 1 -6 + Extras [480P][Dual-Audio]";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episodes when the range is for seasons", () => {
        const releaseName = "House MD All Seasons (1-8) 720p Ultra-Compressed";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episode when it indicates sequence of the movie in between hyhen separators", () => {
        const releaseName = "Dragon Ball Z Movie - 09 - Bojack Unbound - 1080p";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episode when it indicates sequence of the movie at the start", () => {
        const releaseName = "09 Movie - Dragon Ball Z - Bojack Unbound";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect detect episode with x separator and e prefix", () => {
        const releaseName = "24 - S01xE03.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 3 });
    });

    it("should detect detect episode correctly and not episode range ", () => {
        const releaseName = "24 - S01E04 - x264 - dilpill.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect detect episode correctly and not episode range with two codecs", () => {
        const releaseName = "24.Legacy.S01E05.720p.HEVC.x265-MeGusta";
        expect(parse(releaseName)).to.deep.include({ episode: 5 });
    });

    it("should detect detect absolute episode with a version", () => {
        const releaseName = "[F-D] Fairy.Tail.-.004v2.-. [480P][Dual-Audio].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect detect absolute episode with a version and ep suffix", () => {
        const releaseName = "[Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect detect absolute episode and not detect any season modifier", () => {
        const releaseName = "[a-s]_fairy_tail_-_003_-_infiltrate_the_everlue_mansion__rs2_[1080p_bd-rip][4CB16872].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 3 });
        expect(parse(releaseName)).to.not.have.property("seasons");
    });

    it("should detect episode after season with separator", () => {
        const releaseName = "Food Wars! Shokugeki No Souma S4 - 11 (1080p)(HEVC x265 10bit)";
        expect(parse(releaseName)).to.deep.include({ episode: 11 });
    });

    it("should not detect episode range for mismatch episode marker e vs ep", () => {
        const releaseName = "Dragon Ball Super S05E53 - Ep.129.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 53 });
    });

    it("should not detect episode range with other parameter ending", () => {
        const releaseName = "DShaun.Micallefs.MAD.AS.HELL.S10E03.576p.x642-YADNUM.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 3 });
    });

    it("should not detect episode range with spaced hyphen separator", () => {
        const releaseName = "The Avengers (EMH) - S01 E15 - 459 (1080p - BluRay).mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 15 });
    });

    it("should detect episode with a dot and hyphen separator", () => {
        const releaseName = "My Little Pony FiM - 6.01 - No Second Prances.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect episode with number in a title", () => {
        const releaseName = "Mob Psycho 100 - 09 [1080p].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 9 });
    });

    it("should detect episode with number and a hyphen after it in a title", () => {
        const releaseName = "3-Nen D-Gumi Glass no Kamen - 13 [480p]";
        expect(parse(releaseName)).to.deep.include({ episode: 13 });
    });

    it("should detect episode with of separator", () => {
        const releaseName = "BBC Indian Ocean with Simon Reeve 5of6 Sri Lanka to Bangladesh.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 5 });
    });

    it("should detect multiple episodes with multiple E sign and no separator", () => {
        const releaseName = "Stargate Universe S01E01E02E03.mp4";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3] });
    });

    it("should detect multiple episodes with multiple E sign and hyphen separator", () => {
        const releaseName = "Stargate Universe S01E01-E02-E03.mp4";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3] });
    });

    it("should detect multiple episodes with eps prefix and hyphen separator", () => {
        const releaseName = "MARATHON EPISODES/Orphan Black S3 Eps.05-08.mp4";
        expect(parse(releaseName)).to.deep.include({ episodes: [5, 6, 7, 8] });
    });

    it("should detect multiple episodes with E sign and hyphen spaced separator", () => {
        const releaseName = "Pokemon Black & White E10 - E17 [CW] AVI";
        expect(parse(releaseName)).to.deep.include({ episodes: [10, 11, 12, 13, 14, 15, 16, 17] });
    });

    it("should detect multiple episodes with E sign and hyphen separator", () => {
        const releaseName = "Pokémon.S01E01-E04.SWEDISH.VHSRip.XviD-aka";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4] });
    });

    it("should detect episode with single episode and not range", () => {
        const releaseName = "[HorribleSubs] White Album 2 - 06 [1080p].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 6 });
    });

    it("should detect episode with E symbols without season", () => {
        const releaseName = "Mob.Psycho.100.II.E10.720p.WEB.x264-URANiME.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 10 });
    });

    it("should detect episode without season", () => {
        const releaseName = "[OMDA] Bleach - 002 (480p x264 AAC) [rich_jc].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect episode with a episode code including multiple numbers", () => {
        const releaseName = "[ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 19 });
    });

    it("should detect multiple episodes with x episode marker and hyphen separator", () => {
        const releaseName = "BoJack Horseman [06x01-08 of 16] (2019-2020) WEB-DLRip 720p";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8] });
    });

    it("should detect multiple episodes with russian episode marker and hyphen separator", () => {
        const releaseName = "Мистер Робот / Mr. Robot / Сезон: 2 / Серии: 1-5 (12) [2016, США, WEBRip 1080p] MVO";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5] });
    });

    it("should detect episode with russian episode marker and single episode", () => {
        const releaseName = "Викинги / Vikings / Сезон: 5 / Серии: 1 [2017, WEB-DL 1080p] MVO";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect episode with russian episode marker and single episode and with total episodes value", () => {
        const releaseName = "Викинги / Vikings / Сезон: 5 / Серии: 1 из 20 [2017, WEB-DL 1080p] MVO";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect episode with russian arabic total episodes value separator", () => {
        const releaseName = "Prehistoric park.3iz6.Supercroc.DVDRip.Xvid.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 3 });
    });

    it("should detect episode with shortened russian episode name", () => {
        const releaseName = "Меч (05 сер.) - webrip1080p.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 5 });
    });

    it("should detect episode with full russian episode name", () => {
        const releaseName = "Серия 11.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 11 });
    });

    it("should detect episode with full different russian episode name", () => {
        const releaseName = "Разрушители легенд. MythBusters. Сезон 15. Эпизод 09. Скрытая угроза (2015).avi";
        expect(parse(releaseName)).to.deep.include({ episode: 9 });
    });

    it("should detect episode with full different russian episode name v2", () => {
        const releaseName = "Леди Баг и Супер-Кот – Сезон 3, Эпизод 21 – Кукловод 2 [1080p].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 21 });
    });

    it("should detect episode with full russian episode name with case suffix", () => {
        const releaseName = "Проклятие острова ОУК_ 5-й сезон 09-я серия_ Прорыв Дэна.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 9 });
    });

    it("should detect episode with full russian episode name and no prefix", () => {
        const releaseName = "Интерны. Сезон №9. Серия №180.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 180 });
    });

    it("should detect episode with russian episode name in non kirilica", () => {
        const releaseName = "Tajny.sledstviya-20.01.serya.WEB-DL.(1080p).by.lunkin.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect episode with russian episode name in non kirilica alternative 2", () => {
        const releaseName = "Zvezdnie.Voiny.Voina.Klonov.3.sezon.22.seria.iz.22.XviD.HDRip.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 22 });
    });

    it("should detect episode with russian episode name in non kirilica alternative 3", () => {
        const releaseName = "MosGaz.(08.seriya).2012.WEB-DLRip(AVC).ExKinoRay.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 8 });
    });

    it("should detect episode with russian episode name in non kirilica alternative 5", () => {
        const releaseName = "Tajny.sledstvija.(2.sezon.12.serija.iz.12).2002.XviD.DVDRip.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 12 });
    });

    it("should detect episodes with russian x separator", () => {
        const releaseName = "Discovery. Парни с Юкона / Yokon Men [06х01-08] (2017) HDTVRip от GeneralFilm | P1";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8] });
    });

    it("should detect episodes with hyphen separator between episode", () => {
        const releaseName = "2-06. Девичья сила.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 6 });
    });

    it("should detect episodes with hyphen separator between episode v2", () => {
        const releaseName = "4-13 Cursed (HD).m4v";
        expect(parse(releaseName)).to.deep.include({ episode: 13 });
    });

    it("should detect not episodes with hyphen separator between episode when it's date", () => {
        const releaseName = "The Ed Show 10-19-12.mp4";
        expect(parse(releaseName)).to.not.have.property("episodes");
        expect(parse(releaseName)).to.deep.include({ date: "2012-10-19" });
    });

    it("should detect not episodes with hyphen separator between episode when it's not supported date", () => {
        const releaseName = "Hogan's Heroes - 516 - Get Fit or Go Flight - 1-09-70.divx";
        expect(parse(releaseName)).to.deep.include({ episode: 516 });
    });

    it("should detect episodes with hyphen separator between episode v3", () => {
        const releaseName = "Доктор Хаус 03-20.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 20 });
    });

    it("should detect episode after ordinal season and hyphen separator", () => {
        const releaseName = "Kyoukai no Rinne (TV) 3rd Season - 23 [1080p]";
        expect(parse(releaseName)).to.deep.include({ episode: 23 });
    });

    it("should detect episode after ordinal season and hyphen separator and multiple spaces", () => {
        const releaseName = "[224] Shingeki no Kyojin - S03 - Part 1 -  13 [BDRip.1080p.x265.FLAC].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 13 });
    });

    it("should detect spanish full episode identifier", () => {
        const releaseName = "El Chema Temporada 1 Capitulo 25";
        expect(parse(releaseName)).to.deep.include({ episode: 25 });
    });

    it("should detect latino full episode identifier", () => {
        const releaseName = "Yu-Gi-Oh! ZEXAL Temporada 1 Episodio 009 Dual Latino e Inglés [B3B4970E].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 9 });
    });

    it("should detect spanish multiple episode identifier", () => {
        const releaseName = "Bleach 10º Temporada - 215 ao 220 - [DB-BR]";
        expect(parse(releaseName)).to.deep.include({ episodes: [215, 216, 217, 218, 219, 220] });
    });

    it("should not detect episode in episode checksum code", () => {
        const releaseName = "[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [11] });
    });

    it("should not detect episode in episode checksum code without container", () => {
        const releaseName = "[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]";
        expect(parse(releaseName)).to.deep.include({ episodes: [11] });
    });

    it("should not detect episode in episode checksum code with paranthesis", () => {
        const releaseName = "(Hi10)_Re_Zero_Shin_Henshuu-ban_-_02v2_(720p)_(DDY)_(72006E34).mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [2] });
    });

    it("should not detect episode before season", () => {
        const releaseName = "22-7 (Season 1) (1080p)(HEVC x265 10bit)(Eng-Subs)-Judas[TGx] ⭐";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect multiple episode with tilde separator", () => {
        const releaseName = "[Erai-raws] Carole and Tuesday - 01 ~ 12 [1080p][Multiple Subtitle]";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] });
    });

    it("should detect multiple episode with tilde separator and season prefix", () => {
        const releaseName = "[Erai-raws] 3D Kanojo - Real Girl 2nd Season - 01 ~ 12 [720p]";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] });
    });

    it("should detect multiple episode with hyphen separator", () => {
        const releaseName = "[FFA] Koi to Producer: EVOL×LOVE - 01 - 12 [1080p][HEVC][AAC]";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] });
    });

    it("should detect multiple episode with hyphen separator between parenthesis", () => {
        const releaseName = "[BenjiD] Quan Zhi Gao Shou (The King’s Avatar) / Full-Time Master S01 (01 - 12) [1080p x265] [Soft sub] V2";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] });
    });

    it("[HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ", () => {
        const releaseName = "[HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ";
        expect(parse(releaseName)).to.deep.include({ episodes: [24] });
    });

    it("Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]", () => {
        const releaseName = "Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]";
        expect(parse(releaseName)).to.deep.include({ episodes: [7] });
    });

    it("black-ish.S05E02.1080p..x265.10bit.EAC3.6.0-Qman[UTR].mkv", () => {
        const releaseName = "black-ish.S05E02.1080p..x265.10bit.EAC3.6.0-Qman[UTR].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [2] });
    });

    it("[Eng Sub] Rebirth Ep #36 [8CF3ADFA].mkv", () => {
        const releaseName = "[Eng Sub] Rebirth Ep #36 [8CF3ADFA].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [36] });
    });

    it("[92 Impatient Eilas & Miyafuji] Strike Witches - Road to Berlin - 01 [1080p][BCDFF6A2].mkv", () => {
        const releaseName = "[92 Impatient Eilas & Miyafuji] Strike Witches - Road to Berlin - 01 [1080p][BCDFF6A2].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [1] });
    });

    it("[224] Darling in the FranXX - 14 [BDRip.1080p.x265.FLAC].mkv", () => {
        const releaseName = "[224] Darling in the FranXX - 14 [BDRip.1080p.x265.FLAC].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [14] });
    });

    it("[Erai-raws] Granblue Fantasy The Animation Season 2 - 10 [1080p][Multiple Subtitle].mkv", () => {
        const releaseName = "[Erai-raws] Granblue Fantasy The Animation Season 2 - 10 [1080p][Multiple Subtitle].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [10] });
    });

    it("[Erai-raws] Shingeki no Kyojin Season 3 - 11 [1080p][Multiple Subtitle].mkv", () => {
        const releaseName = "[Erai-raws] Shingeki no Kyojin Season 3 - 11 [1080p][Multiple Subtitle].mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [11] });
    });

    it("should detect single zero episode", () => {
        const releaseName = "DARKER THAN BLACK - S00E00.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 0 });
    });

    it("should detect anime episode when title contain similar pattern", () => {
        const releaseName = "[Erai-raws] 22-7 - 11 .mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 11 });
    });

    it("should detect anime episode after year in title", () => {
        const releaseName = "[Golumpa] Star Blazers 2202 - 22 (Uchuu Senkan Yamato 2022) [FuniDub 1080p x264 AAC] [A24B89C8].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 22 });
    });

    it("should detect anime episode after year property", () => {
        const releaseName = "[SubsPlease] Digimon Adventure (2020) - 35 (720p) [4E7BA28A].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 35 });
    });

    it("should detect anime episode recap episode", () => {
        const releaseName = "[KH] Sword Art Online II - 14.5 - Debriefing.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 14 });
    });

    it("should detect four digit anime episode", () => {
        const releaseName = "[SSA] Detective Conan - 1001 [720p].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1001 });
    });
});
