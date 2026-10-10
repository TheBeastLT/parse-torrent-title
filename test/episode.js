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

    it("should detect regular season with O instead of zero", () => {
        const releaseName = "Arrested Development SO2E04.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect regular episode with a space between", () => {
        const releaseName = "Dragon Ball Super S01 E23 French 1080p HDTV H264-Kesni";
        expect(parse(releaseName)).to.deep.include({ episode: 23 });
    });

    it("should detect regular episode above 1000", () => {
        const releaseName = "One.Piece.S01E1116.Lets.Go.Get.It!.Buggys.Big.Declaration.2160p.B-Global.WEB-DL.JPN.AAC2.0.H.264.MSubs-ToonsHub.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1116 });
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

    it("should detect regular episode without break at the end", () => {
        const releaseName = "Desperate_housewives_S03E02Le malheur aime la compagnie.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect regular episode with a letter b suffix", () => {
        const releaseName = "Mash S10E01b Thats Show Biz Part 2 1080p H.264 (moviesbyrizzo upload).mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect regular episode with a letter c suffix", () => {
        const releaseName = "The Twilight Zone 1985 S01E22c The Library.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 22 });
    });

    it("should detect regular episode without e separator", () => {
        const releaseName = "Desperate.Housewives.S0615.400p.WEB-DL.Rus.Eng.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 15 });
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

    it("should detect episode with x separator and letter on left", () => {
        const releaseName = "The.Man.In.The.High.Castle1x01.HDTV.XviD[www.DivxTotaL.com].avi";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect episode with x separator and letter on right", () => {
        const releaseName = "clny.3x11m720p.es[www.planetatorrent.com].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 11 });
    });

    it("should detect episode when similar digits included", () => {
        const releaseName = "Friends.S07E20.The.One.With.Rachel's.Big.Kiss.720p.BluRay.2CH.x265.HEVC-PSA.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 7, episode: 20 });
    });

    it("should detect episode when separated with x and inside brackets", () => {
        const releaseName = "Friends - [8x18] - The One In Massapequa.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 8, episode: 18 });
    });

    it("should detect episode when separated with X", () => {
        const releaseName = "Archivo 81 1X7 HDTV XviD Castellano.avi";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 7 });
    });

    it("should detect multiple episodes with x prefix and hyphen separator", () => {
        const releaseName = "Friends - [7x23-24] - The One with Monica and Chandler's Wedding + Audio Commentary.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 7, episodes: [23, 24] });
    });

    it("should detect episode when separated with x and has three digit episode", () => {
        const releaseName = "Yu-Gi-Oh 3x089 - Awakening of Evil (Part 4).avi";
        expect(parse(releaseName)).to.deep.include({ season: 3, episodes: [89] });
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

    it("should detect episode in hundreds withing brackets with dot separator", () => {
        const releaseName = "Dragon Ball [5.134] Preliminary Peril.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 134 });
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

    it("should detect detect anime episode when title contains number range", () => {
        const releaseName = "[Erai-raws] 2-5 Jigen no Ririsa - 08 [480p][Multiple Subtitle][972D0669].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 8 });
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

    it("should detect season with a dot and episode prefix", () => {
        const releaseName = "Desperate Housewives - Episode 1.22 - Goodbye for now.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 22 });
    });

    it("should detect season with a dot and episode prefix v2", () => {
        const releaseName = "All of Us Are Dead . 2022 . S01 EP #1.2.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
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

    it("should detect episode with of separator v1", () => {
        const releaseName = "Witches Of Salem - 2Of4 - Road To Hell - Gr.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect episode with of separator v2", () => {
        const releaseName = "Das Boot Miniseries Original Uncut-Reevel Cd2 Of 3.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
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

    it("should detect episode with E symbols without season v2", () => {
        const releaseName = "E5.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 5 });
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

    it("should detect season with russian episode shortened word", () => {
        const releaseName = "Otchayannie.domochozyaiki.(8.sez.21.ser.iz.23).2012.XviD.HDTVRip.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 21 });
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

    it("should detect episodes with hyphen separator between episode v4", () => {
        const releaseName = "Комиссар Рекс 11-13.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 13 });
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

    it("should detect spanish partial episode identifier", () => {
        const releaseName = "Juego de Tronos - Temp.2 [ALTA DEFINICION 720p][Cap.209][Spanish].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 209 });
    });

    it("should detect spanish partial long episode identifier", () => {
        const releaseName = "Blue Bloods - Temporada 11 [HDTV 720p][Cap.1103][AC3 5.1 Castellano][www.PCTmix.com].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1103 });
    });

    it("should detect spanish partial episode identifier with common typo", () => {
        const releaseName = "Para toda la humanidad [4k 2160p][Caap.406](wolfmax4k.com).mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 406 });
    });

    it("should detect spanish partial episode identifier v2", () => {
        const releaseName = "Mazinger-Z-Cap-52.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 52 });
    });

    it("should detect latino full episode identifier", () => {
        const releaseName = "Yu-Gi-Oh! ZEXAL Temporada 1 Episodio 009 Dual Latino e Inglés [B3B4970E].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 9 });
    });

    it("should detect spanish multiple episode identifier", () => {
        const releaseName = "Bleach 10º Temporada - 215 ao 220 - [DB-BR]";
        expect(parse(releaseName)).to.deep.include({ episodes: [215, 216, 217, 218, 219, 220] });
    });

    it("should detect spanish short season identifier", () => {
        const releaseName = "My Little Pony - A Amizade é Mágica - T02E22.mp4";
        expect(parse(releaseName)).to.deep.include({ episodes: [22] });
    });

    it("should detect spanish short season identifier with xe separator", () => {
        const releaseName = "30 M0N3D4S ESP T01XE08.mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [8] });
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

    it("should detect anime episode with hyphen number in title", () => {
        const releaseName = "[SubsPlease] Fairy Tail - 100 Years Quest - 05 (1080p) [1107F3A9].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 5 });
    });

    it("should detect anime episode recap episode", () => {
        const releaseName = "[KH] Sword Art Online II - 14.5 - Debriefing.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 14 });
    });

    it("should detect four digit anime episode", () => {
        const releaseName = "[SSA] Detective Conan - 1001 [720p].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1001 });
    });

    it("should detect season_episode pattern", () => {
        const releaseName = "Pwer-04_05.avi";
        expect(parse(releaseName)).to.deep.include({ season: 4, episode: 5 });
    });

    it("should detect season_episode pattern v2", () => {
        const releaseName = "SupNat-11_06.avi";
        expect(parse(releaseName)).to.deep.include({ season: 11, episode: 6 });
    });

    it("should detect season_episode pattern v3", () => {
        const releaseName = "office_03_19.avi";
        expect(parse(releaseName)).to.deep.include({ season: 3, episode: 19 });
    });

    it("should detect season_episode pattern with years in title", () => {
        const releaseName = "Spergrl-2016-02_04.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 4 });
    });

    it("should detect final with dash season_episode pattern with years in title", () => {
        const releaseName = "Iron-Fist-2017-01_13-F.avi";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 13 });
    });

    it("should detect final with dot season_episode pattern with years in title", () => {
        const releaseName = "Lgds.of.Tmrow-02_17.F.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 17 });
    });

    it("should detect season.episode pattern", () => {
        const releaseName = "Ozk.02.09.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 9 });
    });

    it("should detect final season.episode pattern", () => {
        const releaseName = "Ozk.02.10.F.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 10 });
    });

    it("should detect not detect season_episode pattern when other pattern present", () => {
        const releaseName = "Cestovatelé_S02E04_11_27.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 4 });
    });

    it("should detect not detect season_episode pattern when it's additional info", () => {
        const releaseName = "S03E13_91.avi";
        expect(parse(releaseName)).to.deep.include({ season: 3, episode: 13 });
    });

    it("should detect not detect season.episode pattern (not working yet)", () => {
        const releaseName = "wwe.nxt.uk.11.26.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 11, episode: 26 });
    });

    it("should detect not detect season.episode pattern when other pattern present", () => {
        const releaseName = "Chernobyl.S01E01.1.23.45.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 1 });
    });

    it("should detect season.episode pattern with S identifier", () => {
        const releaseName = "The.Witcher.S01.07.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 7 });
    });

    it("should detect season episode pattern with S identifier", () => {
        const releaseName = "Breaking Bad S02 03.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 3 });
    });

    it("should detect season episode pattern with Season prefix", () => {
        const releaseName = "NCIS Season 11 01.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 11, episode: 1 });
    });

    it("should detect not detect season.episode pattern when it's a date", () => {
        const releaseName = "Top Gear - 3x05 - 2003.11.23.avi";
        expect(parse(releaseName)).to.deep.include({ season: 3, episode: 5 });
    });

    it("should detect episode in brackets", () => {
        const releaseName = "[KTKJ]_[BLEACH]_[DVDRIP]_[116]_[x264_640x480_aac].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 116 });
    });

    it("should detect episode in brackets but not years", () => {
        const releaseName = "[GM-Team][国漫][绝代双骄][Legendary Twins][2022][08][HEVC][GB][4K].mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 8 });
    });

    it("should not detect season-episode pattern when with dot split", () => {
        const releaseName = "SG-1. Season 4.16. (2010).avi";
        expect(parse(releaseName)).to.deep.include({ season: 4, episode: 16 });
    });

    it("should not detect polish episode", () => {
        const releaseName = "Reset- odc. 10.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 10 });
    });

    xit("should not detect season-episode pattern when it's a date", () => {
        const releaseName = "8-6 2006.07.16.avi";
        expect(parse(releaseName)).to.deep.include({ season: 8, episode: 6 });
    });

    it("should not detect season episode pattern but absolute episode", () => {
        const releaseName = "523 23.mp4";
        expect(parse(releaseName)).to.not.have.property("season");
        expect(parse(releaseName)).to.deep.include({ episode: 523 });
    });

    it("should detect only episode", () => {
        const releaseName = "Chernobyl E02 1 23 45.mp4";
        expect(parse(releaseName)).to.not.have.property("season");
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect regular episode with year range before", () => {
        const releaseName = "Lucky.Luke.1983-1992.S01E04.PL.720p.WEB-DL.H264-zyl.mkv";
        expect(parse(releaseName)).to.deep.include({ episodes: [4] });
    });

    it("should detect season dot episode format", () => {
        const releaseName = "11.14 - The List.mp4";
        expect(parse(releaseName)).to.deep.include({ seasons: [11], episodes: [14] });
    });

    it("should detect only episode v2", () => {
        const releaseName = "Watch Gary And His Demons Episode 10 - 0.00.07-0.11.02.mp4";
        expect(parse(releaseName)).to.not.have.property("season");
        expect(parse(releaseName)).to.deep.include({ episode: 10 });
    });

    it("should detect only episode v3", () => {
        const releaseName = "523 23.mp4";
        expect(parse(releaseName)).to.not.have.property("season");
        expect(parse(releaseName)).to.deep.include({ episode: 523 });
    });

    it("should not detect season.episode pattern when it's a date without other pattern", () => {
        const releaseName = "wwf.raw.is.war.18.09.00.avi";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episode");
    });

    it("should not detect episodes when it's 2.0 sound", () => {
        const releaseName = "The Rat Race (1960) [1080p] [WEB-DL] [x264] [DD] [2-0] [DReaM] [LEKTOR PL]";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episodes when it's 2.0 sound v2", () => {
        const releaseName = "Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [2.0] [Lektor PL]";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episodes when it's 5.1 sound", () => {
        const releaseName = "A Quiet Place: Day One (2024) [1080p] [WEB-DL] [x264] [AC3] [DD] [5-1] [LEKTOR PL]";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episodes when it's 5.1 sound v2", () => {
        const releaseName = "Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [5.1] [Lektor PL]";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episodes when it's 7.1 sound", () => {
        const releaseName = "Frequency (2000) [1080p] [BluRay] [REMUX] [AVC] [DTS] [HD] [MA] [7-1] [MR]";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episodes when it's 7.1 sound v2", () => {
        const releaseName = "Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [7.1] [Lektor PL]";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect four digit episode with E prefix instead of year", () => {
        const releaseName = "M.jak.Milosc.E1944.1080p.WEB.DL.x264.PL.GhN.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 1944 });
        expect(parse(releaseName)).to.not.have.property("year");
    });

    it("should detect four digit episode with ep prefix instead of year", () => {
        const releaseName = "Santa_Barbara_ep_1900_vhsrip_rus_xvid.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 1900 });
        expect(parse(releaseName)).to.not.have.property("year");
    });

    it("should detect season and episode with dot separator before extension", () => {
        const releaseName = "Mangust.2.01.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 1 });
    });

    it("should detect season and three digit episode with dot separator before extension", () => {
        const releaseName = "Glukhar-3.036.avi";
        expect(parse(releaseName)).to.deep.include({ season: 3, episode: 36 });
    });

    it("should not detect runtime suffix as season and episode", () => {
        const releaseName = "sokol'nichiy_tomas_dvdrip_1.46.avi";
        expect(parse(releaseName)).to.not.have.property("season");
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect season dot episode in the middle of a time", () => {
        const releaseName = "Elements of Chernobyl Episode 1 1.23.45.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should detect season and episode with dot separator at the start", () => {
        const releaseName = "3.001.Кизиловый щербет.WEBRip.1080p.RG.Russkie.serialy.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 3, episode: 1 });
    });

    it("should detect season and episode with dot separator at the start followed by title", () => {
        const releaseName = "1.01 Shattered Vows.avi";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 1 });
    });

    it("should not detect number in parentheses with a suffix as episode", () => {
        const releaseName = "[Asakura] Kaiko Sareta Ankoku Heishi (30-dai) no Slow na Second Life 01 [BDRip 1920x1080 x265 10bit FLAC].mkv";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should still detect episode after title with number in parentheses with a dash", () => {
        const releaseName = "[ASW] Kaiko sareta Ankoku Heishi (30-dai) no Slow na Second Life - 01 [1080p HEVC][8FAB46AC].mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect transliterated russian episode", () => {
        const releaseName = "Shejh.Badijar.Istorija.ljubvi.i.predatelstva.1978.(4-ja.serija.iz.4).XviD.TVRip.avi";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect serbian episode", () => {
        const releaseName = "Lud, Zbunjen, Normalan (2014) Epizoda 170.flv";
        expect(parse(releaseName)).to.deep.include({ episode: 170 });
    });

    it("should not expand comma separated decimal into a range", () => {
        const releaseName = "Yuragi-Sou No Yuuna-San - 03,5 OVA1 uncen  [1080p].mkv";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not expand comma separated duration into a range", () => {
        const releaseName = "Следствие ведут знатоки фильм 1 Черный маклер 1,46.avi";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should still detect comma separated consecutive episodes", () => {
        const releaseName = "LVDB 1,2.avi";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2] });
    });

    it("should not detect episode range when end is a decimal number", () => {
        const releaseName = "Astrid.et.Raphaelle-S04E03-10.000.metres.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 4, episodes: [3] });
    });

    it("should not detect episode range from time range", () => {
        const releaseName = "The.Pitt.1x15.Quindicesima.Ora.21.00-22.00.ITA.ENG.1080p.DLMux.AAC.x265-Pir8.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1, episodes: [15] });
    });

    it("should detect episode with year numbered season", () => {
        const releaseName = "Looney Tunes - S1940E24 - A Wild Hare.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Looney Tunes", season: 1940, episode: 24 });
        expect(parse(releaseName)).to.not.have.property("year");
    });

    it("should detect episode with year numbered season and separate year", () => {
        const releaseName = "Tom and Jerry S1947E27 - Cat Fishin (1947) [1080p BluRay REMUX AVC FLAC 1.0].mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1947, episode: 27, year: 1947 });
    });

    it("should detect episode with s.N ep.N notation", () => {
        const releaseName = "[apreder]Les_Pays_d'en_Haut_s.4_ep.01(2019)DVB.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Les Pays d'en Haut", season: 4, episode: 1 });
    });

    it("should detect episode with three digit s.N ep.N notation", () => {
        const releaseName = "[apreder]Un_si_grand_soleil_s.8_ep.001(2025)DVB.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 8, episode: 1 });
    });

    it("should detect episode before bracketed resolution", () => {
        const releaseName = "Kamen Rider Black - 24 [GSG.Persona99][480].rus.jpn.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 24, resolution: "480p" });
    });

    it("should detect absolute episode before resolution in parentheses", () => {
        const releaseName = "37 Yali Capkini  (1080).mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 37, resolution: "1080p" });
    });

    it("should still detect year glued to episode", () => {
        const releaseName = "Insiders.2023E43.Sarah.Hanson-Young.Greens.Senator.720p";
        expect(parse(releaseName)).to.deep.include({ year: 2023, episode: 43 });
    });

    it("should not detect episode from year numbered season without episode", () => {
        const releaseName = "MythBusters S2009 Complete 720p WEBRips x264 [i c]";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect absolute episode at the beginning followed by title", () => {
        const releaseName = "1125 - One piece [Sub] 1080p";
        expect(parse(releaseName)).to.deep.include({ episode: 1125 });
    });

    it("should detect episode right after the year", () => {
        const releaseName = "[NoobSubs] Berserk 2016 12 (720p Blu-ray 8bit AAC).mp4";
        expect(parse(releaseName)).to.deep.include({ year: 2016, episode: 12 });
    });

    it("should detect concatenated episode right after the year", () => {
        const releaseName = "the.flash.2014.113.hdtv-lol.mp4";
        expect(parse(releaseName)).to.deep.include({ year: 2014, episode: 113 });
    });

    it("should detect episode right after the year in parentheses", () => {
        const releaseName = "[Some-Stuffs]_Pocket_Monsters_(2019)_088_(1080p)_[0AA4D76E].mkv";
        expect(parse(releaseName)).to.deep.include({ year: 2019, episode: 88 });
    });

    it("should detect episode right after the year followed by episode title", () => {
        const releaseName = "Sherlock.Holmes.(1954).11.The Red Headed League.mp4";
        expect(parse(releaseName)).to.deep.include({ year: 1954, episode: 11 });
    });

    it("should detect dash wrapped episode right after the year", () => {
        const releaseName = "Flash Gordon 1996 -24- The Wrath of Ming [Sub+Eng+Fre].mkv";
        expect(parse(releaseName)).to.deep.include({ year: 1996, episode: 24 });
    });

    it("should detect concatenated episode right after the resolution", () => {
        const releaseName = "WhenCallsTheHeart720p_204_WWW.NEWPCT1.COM.mkv";
        expect(parse(releaseName)).to.deep.include({ resolution: "720p", episode: 204 });
    });

    it("should detect series word episode", () => {
        const releaseName = "My Family, Series 1, 08 Much Ado About Ben 720p WEB-DL HEVC x265 BONE.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 8 });
    });

    it("should detect series word episode with dot separator", () => {
        const releaseName = "Timmy Time Series 2.01 Timmy Learns Magic.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 2, episode: 1 });
    });

    it("should detect polish episode word", () => {
        const releaseName = "Simpsonowie - The Simpsons - Sezon 35 Odcinek 11 .mp4";
        expect(parse(releaseName)).to.deep.include({ season: 35, episode: 11 });
    });

    it("should detect swedish episode word", () => {
        const releaseName = "Avsnitt 02 - Dishonor.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 2 });
    });

    it("should detect dutch episode word", () => {
        const releaseName = "'t Schaep met de 5 pooten aflevering 4.mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect german episode word", () => {
        const releaseName = "Super Dragonball Heroes (Deutsch) [Folge 21] (720p_30fps_H264-192kbit_AAC).mp4";
        expect(parse(releaseName)).to.deep.include({ episode: 21 });
    });

    it("should detect finnish episode word", () => {
        const releaseName = "Silkkitie.30.päivässä.-.Jakso-04.flv";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should detect italian episode word", () => {
        const releaseName = "Catch 22 - Puntata 4 (2019).HDTV.720p.H264.italian.Ac3-2.0-BaMax71.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 4 });
    });

    it("should not detect channels after the year as episode", () => {
        const releaseName = "Godzilla.vs.Kong.2021.1080p.10bit.WEBRip.6CH.x265.HEVC-PSA.mkv";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect audio after the year as episode", () => {
        const releaseName = "Nobody (2021) 1080p 5.1 - 2.0 x264 Phun Psyz.mp4";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect cd number after the year as episode", () => {
        const releaseName = "Jism 2 (2012) 1 CD DVD Rip XivD MP3 E-SUB Team DST";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect size after the year as episode", () => {
        const releaseName = "Mission Impossible 5 Rogue Nation 2015 700 MB";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect date after the year as episode", () => {
        const releaseName = "The Bold and the Beautiful 2015 2 27";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect episode range with tilde starting from zero", () => {
        const releaseName = "[Erai-raws] Boku no Hero Academia S2 - 00~25 [1080p][Multiple Subtitle]";
        expect(parse(releaseName)).to.deep.include({ episodes: Array.from({ length: 26 }, (_, i) => i) });
    });

    it("should detect episode range in brackets with ep prefix", () => {
        const releaseName = "Naruto Complete [Ep 01 - 220][English][480p]";
        expect(parse(releaseName)).to.deep.include({ episodes: Array.from({ length: 220 }, (_, i) => i + 1) });
    });

    it("should detect season and episode without s prefix", () => {
        const releaseName = "Vikkatakavi 01E06.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1, episode: 6 });
    });

    it("should detect season and episode with series and ep prefixes", () => {
        const releaseName = "Steptoe And Son Series 05EP01 A Death In The Family.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 5, episode: 1 });
    });

    it("should detect ova episode with underscores", () => {
        const releaseName = "[Cleo]Monster_Musume_no_Iru_Nichijou_-_OVA_01_(Dual Audio_10bit_BD720p_x265).mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 1 });
    });

    it("should detect russian episode count of total as range", () => {
        const releaseName = "Викинги / Vikings / Сезон: 5 / Серии: 5 из 20 [2017, WEB-DL 1080p] MVO";
        expect(parse(releaseName)).to.deep.include({ episodes: [1, 2, 3, 4, 5] });
    });

    it("should detect russian full episode count in brackets as range", () => {
        const releaseName = "Клинок, рассекающий демонов (ТВ-1) / Kimetsu no Yaiba / Demon Slayer [TV] [26 из 26] [RUS(ext), ENG, JAP+Sub]";
        expect(parse(releaseName)).to.deep.include({ episodes: Array.from({ length: 26 }, (_, i) => i + 1) });
    });

    it("should detect turkish bolum episode", () => {
        const releaseName = "Kaderimin Yazıldığı Gün 26. Bölüm.mkv";
        expect(parse(releaseName)).to.deep.include({ episode: 26 });
    });

    it("should not detect season number before russian season word as episode", () => {
        const releaseName = "01 - Дело о золотом таланте - Приключения Пети и Волка - 4 Сезон - D2T.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 4 });
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect transliterated russian serija episode", () => {
        const releaseName = "01. Griffiny - Sezon 21 - Serija 1 - Oskary.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 21, episode: 1 });
    });

    it("should not detect series number as part of episode list", () => {
        const releaseName = "My Family, Series 1, 02 Pain In The Class 720p WEB-DL HEVC x265 BONE.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1, episodes: [2] });
    });

    it("should not detect episode from bracketed audio channels", () => {
        const releaseName = "Deadpool (2016) [2160p] [7.1 AAC ENG] [5.1 AAC ENG FRE GER ITA SPA] [COMMENTARY]";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect episode range with thru", () => {
        const releaseName = "DareDevil - Seasons 2 - Episodes 10 thru 13";
        expect(parse(releaseName)).to.deep.include({ episodes: [10, 11, 12, 13] });
    });

    it("should detect episode from bracketed x of y", () => {
        const releaseName = "BBC Planet Earth [3 of 11] Caves.avi";
        expect(parse(releaseName)).to.deep.include({ title: "BBC Planet Earth", episodes: [3] });
    });

    it("should not detect episode from disc numbering in multi season pack", () => {
        const releaseName = "Forensic Files Complete Series Seasons 01-06 [1 of 4]";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episode from disc numbering after season range", () => {
        expect(parse("Forensic Files Seasons 01-06 [1 of 4]")).to.not.have.property("episodes");
        expect(parse("Forensic Files S01-S06 [1 of 4]")).to.not.have.property("episodes");
    });

    it("should detect season and three digit episode at the start of a file name", () => {
        expect(parse("3-104 Ралли в Клубе Микки Мауса часть 2 (Road Rally Part 2).avi")).to.deep.include({ seasons: [3], episodes: [104] });
    });

    it("should not detect season and episode from a movie title starting with numbers", () => {
        const result = parse("3-10 to Yuma (2007) 1080p BluRay x264.mkv");
        expect(result).to.deep.include({ title: "3-10 to Yuma", year: 2007 });
        expect(result).to.not.have.property("episodes");
        expect(parse("3-10.to.Yuma.2007.1080p.BluRay.x264.mkv")).to.not.have.property("episodes");
    });

    it("should detect episode before cyrillic resolution marker", () => {
        expect(parse("«Эрнест и Селестина» 1 сезон 05 серия 720р.mkv")).to.deep.include({ seasons: [1], episodes: [5], resolution: "720p" });
    });

    it("should detect both episodes of a descending double episode code", () => {
        expect(parse("Its.Pony.S01E31E22.Annie-Versary.Teachers.Pet.1080p.AMZN.WEB-DL.DDP5.1.H.264.mkv")).to.deep.include({ seasons: [1], episodes: [22, 31] });
        expect(parse("Show.S01E01E02.1080p.mkv")).to.deep.include({ episodes: [1, 2] });
    });

    it("should detect zero padded episode glued to the name of a file", () => {
        expect(parse("esperanca001.avi")).to.deep.include({ title: "esperanca", episodes: [1] });
        expect(parse("GodHandTeru05.avi")).to.deep.include({ title: "GodHandTeru", episodes: [5] });
    });

    it("should not detect episode from a number glued to a movie title", () => {
        const releaseName = "Apollo13.mkv";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episode from a number glued to a movie title v2", () => {
        const releaseName = "Terminator2.avi";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should not detect episode from a number glued to a movie title v3", () => {
        const releaseName = "Agent47.mkv";
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect episode with a number in a glued title", () => {
        const releaseName = "Numb3rs.S01E01.mkv";
        expect(parse(releaseName)).to.deep.include({ title: "Numb3rs", seasons: [1], episodes: [1] });
    });

    it("should detect episode glued to ep suffix", () => {
        expect(parse("AniPlay_Kumiho_6ep_dub.mp4")).to.deep.include({ episodes: [6] });
        expect(parse("AniPlay_Kumiho_10ep_dub.mp4")).to.deep.include({ episodes: [10] });
    });

    it("should detect season and episode from volume episode naming", () => {
        expect(parse("v2e09. Morning, Noon and Night.rus.eng.avi")).to.deep.include({ seasons: [2], episodes: [9] });
        expect(parse("V01E14 - Heart Of Ice.mkv")).to.deep.include({ seasons: [1], episodes: [14] });
    });

    it("should detect season and episode before transliterated series word", () => {
        expect(parse("Morskje.Dyvolj.3.01.serij.iz.16.2009.DivX.DVDRip.Kinozal.TV.avi")).to.deep.include({ seasons: [3], episodes: [1] });
        expect(parse("Vsegda.govori.vsegda.6.1-3.ser.iz.8.avi")).to.deep.include({ seasons: [6], episodes: [1, 2, 3] });
        expect(parse("Vsegda.govori.vsegda.5.02.ser.iz.10.avi")).to.deep.include({ seasons: [5], episodes: [2] });
    });

    it("should not detect season from a single number before transliterated series word", () => {
        expect(parse("Brigada.01.seriya.avi")).to.not.have.property("seasons");
        expect(parse("Ulica.razbityh.fonarej.2.serija.avi")).to.not.have.property("seasons");
    });

    it("should detect season and episode from x prefixed code at the start", () => {
        expect(parse("x1001 - The Return Of Chef.avi")).to.deep.include({ seasons: [10], episodes: [1] });
        expect(parse("x302 - Some Title.avi")).to.deep.include({ seasons: [3], episodes: [2] });
    });

    it("should not detect season and episode from codec or resolution with x prefix", () => {
        expect(parse("x264 - Some Movie 2010.mkv")).to.not.have.property("episodes");
        expect(parse("x1080 - Some Movie 2010.mkv")).to.not.have.property("episodes");
    });

    it("should not detect episode range from the middle of a hyphenated number chain", () => {
        expect(parse("90-60-90.modelos_001.avi")).to.not.have.property("episodes");
    });

    it("should detect season prefixed three digit episode code at the start of a file name", () => {
        expect(parse("3-307 The Last Shout (1).mp4")).to.deep.include({ seasons: [3], episodes: [7] });
        expect(parse("2-208 Documentary - How To Be Absolutely Fabulous.mp4")).to.deep.include({ seasons: [2], episodes: [8] });
    });

    it("should detect episode range instead of season from two digit start with three digit end", () => {
        expect(parse("96-100.mp4")).to.deep.include({ episodes: [96, 97, 98, 99, 100] }).and.to.not.have.property("seasons");
        expect(parse("99-100 Douluo Dalu.mkv")).to.deep.include({ episodes: [99, 100] }).and.to.not.have.property("seasons");
    });

    it("should not detect season and episode from a phone number at the start", () => {
        expect(parse("Extras/1-900 Hotline")).to.not.have.property("seasons");
        expect(parse("1-800 Hotline.mkv")).to.not.have.property("seasons");
    });

    it("should not detect episode from a part number glued to the name", () => {
        expect(parse("SupercutALTverWithRecaps720pPart01.mkv")).to.not.have.property("episodes");
        expect(parse("dvd01.avi")).to.not.have.property("episodes");
    });

    it("should not detect episode from a season number glued to ep before the episode number", () => {
        expect(parse("Istoriya.Zatoichi.s.1ep.03.1974.DVDRip.avi").episodes).to.not.deep.equal([1]);
    });

    it("should not detect episode range from a hyphenated name at the start", () => {
        expect(parse("1-900-Beavis.avi")).to.not.have.property("episodes");
    });

    it("should detect dot separated episode pair after season before transliterated season word", () => {
        expect(parse("Doktor.Kuinn.Zhenshina.vrach.(2.sezon.26.27.serija.iz.27).1994.DivX.DVDRip.avi")).to.deep.include({ seasons: [2], episodes: [26, 27] });
        expect(parse("Sekretnye.materialy.sezon.1.24.seriya.iz.24.avi")).to.deep.include({ seasons: [1], episodes: [24] });
    });

    it("should detect episode range before russian series word", () => {
        expect(parse("Черепашки ниндзя 3 сезон 12-13 серия[Saint Sound].mkv")).to.deep.include({ seasons: [3], episodes: [12, 13] });
    });

    it("should detect episode after year followed by dot and space", () => {
        expect(parse("STEPonee Хороший доктор Good Doctor 2013. 5 [озвучка ]_xvid.avi")).to.deep.include({ episodes: [5] });
        expect(parse("STEPonee Хороший доктор Good Doctor 2013. 19 [озвучка ].avi")).to.deep.include({ episodes: [19] });
        expect(parse("Istoriya.Zatoichi.s.1ep.03.1974.DVDRip_4_prob4 .mkv")).to.not.have.property("episodes");
    });

    it("should detect zero padded episode glued to the name after a release group", () => {
        expect(parse("[alliance]Queens01.avi")).to.deep.include({ episodes: [1] });
        expect(parse("[clubfate]sandglass02.avi")).to.deep.include({ episodes: [2] });
        expect(parse("[group]Apollo13.mkv")).to.not.have.property("episodes");
    });
});
