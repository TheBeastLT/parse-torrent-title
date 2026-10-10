const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing season", () => {
    it("should detect regular season correctly", () => {
        const releaseName = "The Simpsons S28E21 720p HDTV x264-AVS";
        expect(parse(releaseName)).to.deep.include({ season: 28 });
    });

    it("should detect regular season with lowercase correctly", () => {
        const releaseName = "breaking.bad.s01e01.720p.bluray.x264-reward";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect regular season with high number", () => {
        const releaseName = "48 Hours S51E15 720p WEB x264-CookieMonster";
        expect(parse(releaseName)).to.deep.include({ season: 51 });
    });

    it("should detect regular season with high number v2", () => {
        const releaseName = "Bargain.Hunt.S66E02.Builth Wells4.720p.WEB.H264-BeechyBoy.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 66 });
    });

    it("should detect regular season with O instead of zero", () => {
        const releaseName = "Arrested Development SO2E04.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("should detect regular season with 3 digits", () => {
        const releaseName = "S011E16.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 11 });
    });

    it("should detect regular season with a space between", () => {
        const releaseName = "Dragon Ball Super S01 E23 French 1080p HDTV H264-Kesni";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect regular season with a letter a suffix", () => {
        const releaseName = "The Twilight Zone 1985 S01E23a Shadow Play.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect regular season with a letter b suffix", () => {
        const releaseName = "Mash S10E01b Thats Show Biz Part 2 1080p H.264 (moviesbyrizzo upload).mp4";
        expect(parse(releaseName)).to.deep.include({ season: 10 });
    });

    it("should detect regular season with a letter c suffix", () => {
        const releaseName = "The Twilight Zone 1985 S01E22c The Library.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect regular episode without e separator", () => {
        const releaseName = "Desperate.Housewives.S0615.400p.WEB-DL.Rus.Eng.avi";
        expect(parse(releaseName)).to.deep.include({ season: 6 });
    });

    it("should detect season with SxEE format correctly", () => {
        const releaseName = "Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV";
        expect(parse(releaseName)).to.deep.include({ season: 8 });
    });

    it("should detect season when written as such", () => {
        const releaseName = "Orange Is The New Black Season 5 Episodes 1-10 INCOMPLETE (LEAKED)";
        expect(parse(releaseName)).to.deep.include({ season: 5 });
    });

    it("should detect season with parenthesis prefix and x separator", () => {
        const releaseName = "Smallville (1x02 Metamorphosis).avi";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect season with x separator and letter on left", () => {
        const releaseName = "The.Man.In.The.High.Castle1x01.HDTV.XviD[www.DivxTotaL.com].avi";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect season with x separator and letter on right", () => {
        const releaseName = "clny.3x11m720p.es[www.planetatorrent.com].mkv";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect multiple seasons separated with comma", () => {
        const releaseName = "Game Of Thrones Complete Season 1,2,3,4,5,6,7 406p mkv + Subs";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7] });
    });

    it("should detect multiple seasons separated with space with redundant digit suffix", () => {
        const releaseName = "Futurama Season 1 2 3 4 5 6 7 + 4 Movies - threesixtyp";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7] });
    });

    it("should detect multiple season separated with spaces and comma", () => {
        const releaseName = "Breaking Bad Complete Season 1 , 2 , 3, 4 ,5 ,1080p HEVC";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5] });
    });

    it("should detect multiple season separated with comma, space and symbol at the end", () => {
        const releaseName = "True Blood Season 1, 2, 3, 4, 5 & 6 + Extras BDRip TSV";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6] });
    });

    it("How I Met Your Mother Season 1, 2, 3, 4, 5, & 6 + Extras DVDRip", () => {
        const releaseName = "How I Met Your Mother Season 1, 2, 3, 4, 5, & 6 + Extras DVDRip";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6] });
    });

    it("should detect multiple seasons separated with space", () => {
        const releaseName = "The Simpsons Season 20 21 22 23 24 25 26 27 - threesixtyp";
        expect(parse(releaseName)).to.deep.include({ seasons: [20, 21, 22, 23, 24, 25, 26, 27] });
    });

    it("should detect multiple seasons separated with space an spanish season name", () => {
        const releaseName = "Perdidos: Lost: Castellano: Temporadas 1 2 3 4 5 6 (Serie Com";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6] });
    });

    it("should detect multiple seasons with with unequal separators", () => {
        const releaseName = "The Boondocks Season 1, 2 & 3";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3] });
    });

    it("should detect multiple seasons with with space and plus symbol", () => {
        const releaseName = "Boondocks, The - Seasons 1 + 2";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect multiple seasons with implied range without s prefix", () => {
        const releaseName = "The Boondocks Seasons 1-4 MKV";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4] });
    });

    it("should detect multiple seasons separated with space plus and symbol", () => {
        const releaseName = "The Expanse Complete Seasons 01 & 02 1080p";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect multiple seasons with s prefix and implied range", () => {
        const releaseName = "Friends.Complete.Series.S01-S10.720p.BluRay.2CH.x265.HEVC-PSA";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] });
    });

    it("should detect multiple seasons with s prefix separated with slash", () => {
        const releaseName = "Stargate Atlantis ALL Seasons - S01 / S02 / S03 / S04 / S05";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5] });
    });

    it("should detect multiple seasons with season and parenthesis prefix", () => {
        const releaseName = "Stargate Atlantis Complete (Season 1 2 3 4 5) 720p HEVC x265";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5] });
    });

    it("should detect multiple seasons with s prefix separated with hyphen", () => {
        const releaseName = "Skam.S01-S02-S03.SweSub.720p.WEB-DL.H264";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3] });
    });

    it("should detect detect correct title with multiple season definitions", () => {
        const releaseName = "Seinfeld S02 Season 2 720p WebRip ReEnc-DeeJayAhmed";
        expect(parse(releaseName)).to.deep.include({ title: "Seinfeld", season: 2 });
    });

    it("should detect correct title with multiple season definitions", () => {
        const releaseName = "Seinfeld Season 2 S02 720p AMZN WEBRip x265 HEVC Complete";
        expect(parse(releaseName)).to.deep.include({ title: "Seinfeld", season: 2 });
    });

    it("should detect multiple season when given implied range inside parenthesis without s prefix", () => {
        const releaseName = "House MD All Seasons (1-8) 720p Ultra-Compressed";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7, 8] });
    });

    it("should detect multiple season when given implied range with season prefix", () => {
        const releaseName = "Teen Titans Season 1-5";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5] });
    });

    it("should detect multiple season when given implied range y words with season prefix", () => {
        const releaseName = "Game Of Thrones - Season 1 to 6 (Eng Subs)";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6] });
    });

    it("should detect multiple season with and separator", () => {
        const releaseName = "Travelers - Seasons 1 and 2 - Mp4 x264 AC3 1080p";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect multiple season when given implied range separated with colon", () => {
        const releaseName = "Naruto Shippuden Season 1:11";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] });
    });

    it("should detect multiple season when given implied range separated with colon and space", () => {
        const releaseName = "South Park Complete Seasons 1: 11";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] });
    });

    it("should detect multiple season when title is with numbers", () => {
        const releaseName = "24 Season 1-8 Complete with Subtitles";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7, 8] });
    });

    it("should detect single season when contains non related number range", () => {
        const releaseName = "One Punch Man 01 - 12 Season 1 Complete [720p] [Eng Subs] [Xerxe:16";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect season withing brackets with dot separator", () => {
        const releaseName = "[5.01] Weight Loss.avi";
        expect(parse(releaseName)).to.deep.include({ season: 5 });
    });

    it("should detect season withing brackets with dot separator and episode in hunderds", () => {
        const releaseName = "Dragon Ball [5.134] Preliminary Peril.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 5 });
    });

    it("should detect season with s prefix and single digit", () => {
        const releaseName = "Bron - S4 - 720P - SweSub.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 4 });
    });

    it("Mad Men S02 Season 2 720p 5.1Ch BluRay ReEnc-DeeJayAhmed", () => {
        const releaseName = "Mad Men S02 Season 2 720p 5.1Ch BluRay ReEnc-DeeJayAhmed";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("Friends S04 Season 4 1080p 5.1Ch BluRay ReEnc-DeeJayAhmed", () => {
        const releaseName = "Friends S04 Season 4 1080p 5.1Ch BluRay ReEnc-DeeJayAhmed";
        expect(parse(releaseName)).to.deep.include({ season: 4 });
    });

    it("should detect multiple season with double seperators", () => {
        const releaseName = "Doctor Who S01--S07--Complete with holiday episodes";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7] });
    });

    it("should detect season with a dot and hyphen separator", () => {
        const releaseName = "My Little Pony FiM - 6.01 - No Second Prances.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 6 });
    });

    it("should detect season with a dot and epsiode prefix", () => {
        const releaseName = "Desperate Housewives - Episode 1.22 - Goodbye for now.avi";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect season with a dot and episode prefix v2", () => {
        const releaseName = "All of Us Are Dead . 2022 . S01 EP #1.2.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect season with a year range afterwards", () => {
        const releaseName = "Empty Nest Season 1 (1988 - 89) fiveofseven";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect multiple seasons with russian season and hyphen separator", () => {
        const releaseName = "Game of Thrones / Сезон: 1-8 / Серии: 1-73 из 73 [2011-2019, США, BDRip 1080p] MVO (LostFilm)";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7, 8] });
    });

    it("should detect multiple seasons with russian season and comma separator", () => {
        const releaseName = "Друзья / Friends / Сезон: 1, 2 / Серии: 1-24 из 24 [1994-1999, США, BDRip 720p] MVO";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect season with russian season word", () => {
        const releaseName = "Друзья / Friends / Сезон: 1 / Серии: 1-24 из 24 [1994-1995, США, BDRip 720p] MVO + Original + Sub (Rus, Eng)";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect season with russian season word as folder name", () => {
        const releaseName = "Сезон 5/Серия 11.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 5 });
    });

    it("should detect season with russian season word followed by the number", () => {
        const releaseName = "Разрушители легенд. MythBusters. Сезон 15. Эпизод 09. Скрытая угроза (2015).avi";
        expect(parse(releaseName)).to.deep.include({ season: 15 });
    });

    it("should detect season with russian season word followed by the number v2", () => {
        const releaseName = "Леди Баг и Супер-Кот – Сезон 3, Эпизод 21 – Кукловод 2 [1080p].mkv";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect episode with full russian season name with case suffix", () => {
        const releaseName = "Проклятие острова ОУК_ 5-й сезон 09-я серия_ Прорыв Дэна.avi";
        expect(parse(releaseName)).to.deep.include({ season: 5 });
    });

    it("should detect season with russian season word with number at front", () => {
        const releaseName = "2 сезон 24 серия.avi";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("should detect season with russian season word with number at front and nothing else", () => {
        const releaseName = "3 сезон";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect season with russian season word and underscore", () => {
        const releaseName = "2. Discovery-Kak_ustroena_Vselennaya.(2.sezon_8.serii.iz.8).2012.XviD.HDTVRip.Krasnodarka";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("should detect season with russian season shortened word", () => {
        const releaseName = "Otchayannie.domochozyaiki.(8.sez.21.ser.iz.23).2012.XviD.HDTVRip.avi";
        expect(parse(releaseName)).to.deep.include({ season: 8 });
    });

    it("should detect season with russian season word and no prefix", () => {
        const releaseName = "Интерны. Сезон №9. Серия №180.avi";
        expect(parse(releaseName)).to.deep.include({ season: 9 });
    });

    it("should detect season with russian x separator", () => {
        const releaseName = "Discovery. Парни с Юкона / Yokon Men [06х01-08] (2017) HDTVRip от GeneralFilm | P1";
        expect(parse(releaseName)).to.deep.include({ season: 6 });
    });

    it("should detect season range with russian x separator", () => {
        const releaseName = "Пока-пока, Земля / Bye Bye, Earth [01-02х01-19 из 20] (2024) WEBRip 1080p by VLDeshka | L | DreamCast, JAMClub";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect season range with x separator", () => {
        const releaseName = "Show [01-03x01-30 из 30] (2020) WEBRip 1080p";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3] });
    });

    it("should not detect season range from a digit in the title before x separator", () => {
        const releaseName = "stargate-sg1-10x01-maso-a-krev.avi";
        expect(parse(releaseName)).to.deep.include({ seasons: [10], title: "stargate-sg1" });
    });

    it("should detect season with russian season word in araic letters", () => {
        const releaseName = "Zvezdnie.Voiny.Voina.Klonov.3.sezon.22.seria.iz.22.XviD.HDRip.avi";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect season with hyphen separator between episode", () => {
        const releaseName = "2-06. Девичья сила.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("should detect season with hyphen separator between episode v2", () => {
        const releaseName = "4-13 Cursed (HD).m4v";
        expect(parse(releaseName)).to.deep.include({ season: 4 });
    });

    it("should detect season with hyphen separator between episode v3", () => {
        const releaseName = "Доктор Хаус 03-20.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect episodes with hyphen separator between episode v4", () => {
        const releaseName = "Комиссар Рекс 11-13.avi";
        expect(parse(releaseName)).to.deep.include({ season: 11 });
    });

    it("should not detect season with hyphen separator when it's the title", () => {
        const releaseName = "13-13-13 2013 DVDrip x264 AAC-MiLLENiUM";
        expect(parse(releaseName)).to.not.have.property("seasons");
        expect(parse(releaseName)).to.deep.include({ title: "13-13-13" });
    });

    it("should detect correct season with eps prefix and hyphen separator", () => {
        const releaseName = "MARATHON EPISODES/Orphan Black S3 Eps.05-08.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect multiple seasons with end season without s symbol", () => {
        const releaseName = "Once Upon a Time [S01-07] (2011-2017) WEB-DLRip by Generalfilm";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7] });
    });

    it("should detect multiple seasons with one space and hyphen separator", () => {
        const releaseName = "[F-D] Fairy Tail Season 1 -6 + Extras [480P][Dual-Audio]";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6] });
    });

    it("should detect multiple seasons with spaces and hyphen separator", () => {
        const releaseName = "Coupling Season 1 - 4 Complete DVDRip - x264 - MKV by RiddlerA";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4] });
    });

    it("should single season with spaces and hyphen separator", () => {
        const releaseName = "[HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ";
        expect(parse(releaseName)).to.deep.include({ seasons: [4] });
    });

    it("Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]", () => {
        const releaseName = "Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]";
        expect(parse(releaseName)).to.deep.include({ seasons: [2] });
    });

    it("Ace of the Diamond: 1st Season", () => {
        const releaseName = "Ace of the Diamond: 1st Season";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("Ace of the Diamond: 2nd Season", () => {
        const releaseName = "Ace of the Diamond: 2nd Season";
        expect(parse(releaseName)).to.deep.include({ seasons: [2] });
    });

    it("Adventure Time 10 th season", () => {
        const releaseName = "Adventure Time 10 th season";
        expect(parse(releaseName)).to.deep.include({ seasons: [10] });
    });

    it("Kyoukai no Rinne (TV) 3rd Season - 23 [1080p]", () => {
        const releaseName = "Kyoukai no Rinne (TV) 3rd Season - 23 [1080p]";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("[Erai-raws] Granblue Fantasy The Animation Season 2 - 08 [1080p][Multiple Subtitle].mkv", () => {
        const releaseName = "[Erai-raws] Granblue Fantasy The Animation Season 2 - 08 [1080p][Multiple Subtitle].mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [2] });
    });

    it("The Nile Egypts Great River with Bettany Hughes Series 1 4of4 10", () => {
        const releaseName = "The Nile Egypts Great River with Bettany Hughes Series 1 4of4 10";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("Teen Wolf - 04ª Temporada 720p", () => {
        const releaseName = "Teen Wolf - 04ª Temporada 720p";
        expect(parse(releaseName)).to.deep.include({ seasons: [4] });
    });

    it("Vikings 3 Temporada 720p", () => {
        const releaseName = "Vikings 3 Temporada 720p";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("Eu, a Patroa e as Crianças  4° Temporada Completa - HDTV - Dublado", () => {
        const releaseName = "Eu, a Patroa e as Crianças  4° Temporada Completa - HDTV - Dublado";
        expect(parse(releaseName)).to.deep.include({ seasons: [4] });
    });

    it("Merl - Temporada 1", () => {
        const releaseName = "Merl - Temporada 1";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("Elementar 3º Temporada Dublado", () => {
        const releaseName = "Elementar 3º Temporada Dublado";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("Beavis and Butt-Head - 1a. Temporada", () => {
        const releaseName = "Beavis and Butt-Head - 1a. Temporada";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("3Âº Temporada Bob esponja Pt-Br", () => {
        const releaseName = "3Âº Temporada Bob esponja Pt-Br";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("Juego de Tronos - Temp.2 [ALTA DEFINICION 720p][Cap.209][Spanish].mkv", () => {
        const releaseName = "Juego de Tronos - Temp.2 [ALTA DEFINICION 720p][Cap.209][Spanish].mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [2] });
    });

    it("Los Simpsons Temp 7 DVDrip Espanol De Espana", () => {
        const releaseName = "Los Simpsons Temp 7 DVDrip Espanol De Espana";
        expect(parse(releaseName)).to.deep.include({ seasons: [7] });
    });

    it("should detect spanish season range with & separator", () => {
        const releaseName = "The Walking Dead [Temporadas 1 & 2 Completas Em HDTV E Legena";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2] });
    });

    it("should detect spanish short season identifier", () => {
        const releaseName = "My Little Pony - A Amizade é Mágica - T02E22.mp4";
        expect(parse(releaseName)).to.deep.include({ seasons: [2] });
    });

    it("should detect spanish short season identifier with xe separator", () => {
        const releaseName = "30 M0N3D4S ESP T01XE08.mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("should detect sn naming scheme", () => {
        const releaseName = "Sons of Anarchy Sn4 Ep14 HD-TV - To Be, Act 2, By Cool Release";
        expect(parse(releaseName)).to.deep.include({ seasons: [4] });
    });

    it("should detect single season and not range in filename", () => {
        const releaseName = "[FFA] Kiratto Pri☆chan Season 3 - 11 [1080p][HEVC].mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("should detect single season and not range in filename v2", () => {
        const releaseName = "[Erai-raws] Granblue Fantasy The Animation Season 2 - 10 [1080p][Multiple Subtitle].mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [2] });
    });

    it("should detect single season and not range in filename v3", () => {
        const releaseName = "[SCY] Attack on Titan Season 3 - 11 (BD 1080p Hi10 FLAC) [1FA13150].mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("should detect single zero season", () => {
        const releaseName = "DARKER THAN BLACK - S00E04 - Darker Than Black Gaiden OVA 3.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 0 });
    });

    it("should detect nl season word", () => {
        const releaseName = "Seizoen 22 - Zon & Maan Ultra Legendes/afl.18 Je ogen op de bal houden!.mp4";
        expect(parse(releaseName)).to.deep.include({ season: 22 });
    });

    it("should detect italian season word", () => {
        const releaseName = "Nobody Wants This - Stagione 1 (2024) [COMPLETA] 720p H264 ITA AAC 2.0-Zer0landia";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect italian season range", () => {
        const releaseName = "Red Oaks - Stagioni 01-03 (2014-2017) [COMPLETA] SD x264 AAC ITA SUB ITA - mkeagle3";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3] });
    });

    it("should detect polish season", () => {
        const releaseName = "Star.Wars.Skeleton.Crew.Sezon01.PLDUB.480p.DSNP.WEB-DL.H264.DDP5.1-K83";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("should detect polish season with S prefix", () => {
        const releaseName = "Bitten.SezonSO3.PL.480p.NF.WEB-DL.DD5.1.XviD-Ralf";
        expect(parse(releaseName)).to.deep.include({ seasons: [3] });
    });

    it("should detect regular season with year range before", () => {
        const releaseName = "'Lucky.Luke.1983-1992.S01E04.PL.720p.WEB-DL.H264-zyl.mkv'";
        expect(parse(releaseName)).to.deep.include({ seasons: [1] });
    });

    it("should detect polish season range", () => {
        const releaseName = "Rizzoli & Isles 2010-2016 [Sezon 01-07] [1080p.WEB-DL.H265.EAC3-FT][Alusia]";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3, 4, 5, 6, 7] });
    });

    it("should detect season episode when not in boundary", () => {
        const releaseName = "Those.About.to.DieS01E06.MULTi.720p.AMZN.WEB-DL.H264.DDP5.1-K83.mkv";
        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [6] });
    });

    it("should not detect season when it's part of the name", () => {
        const releaseName = "Ranma-12-86.mp4";
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should not detect season when it's part of group", () => {
        const releaseName = "The Killer's Game 2024 PL 1080p WEB-DL H264 DD5.1-S56";
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should not detect season when it's part of group v2", () => {
        const releaseName = "Apollo 13 (1995) [1080p] [WEB-DL] [x264] [E-AC3-S78] [Lektor PL]";
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should not detect season dot episode notation when it's a title", () => {
        const releaseName = "3.10 to Yuma (2007).mkv";
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should not detect season dot episode notation when there is year in name", () => {
        const releaseName = "18.11 - A Code Of Secrecy (2014) x264 1080p-AAC-ESUB [Parth].mkv";
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should detect norwegian season folder", () => {
        const releaseName = "Sesong 2";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("should detect german season folder", () => {
        const releaseName = "Staffel 1";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect swedish season folder", () => {
        const releaseName = "pepparkornen säsong 3";
        expect(parse(releaseName)).to.deep.include({ season: 3 });
    });

    it("should detect croatian season folder", () => {
        const releaseName = "Sezona 5 + Bonus";
        expect(parse(releaseName)).to.deep.include({ season: 5 });
    });

    it("should detect season number before season word", () => {
        const releaseName = "2 Season";
        expect(parse(releaseName)).to.deep.include({ season: 2 });
    });

    it("should not detect year range end before season word as season", () => {
        const releaseName = "Tottenham Hotspur 2012-13 Season Review.mp4";
        expect(parse(releaseName)).to.not.have.property("season");
    });

    it("should detect bare season folder", () => {
        const releaseName = "S04";
        expect(parse(releaseName)).to.deep.include({ season: 4 });
    });

    it("should detect season at the end of the title", () => {
        const releaseName = "Peaky Blinders S04";
        expect(parse(releaseName)).to.deep.include({ title: "Peaky Blinders", season: 4 });
    });

    it("should not expand comma separated seasons into a range", () => {
        const releaseName = "Горец (Сезон 1, 22 серии) / Highlander [1992, DVDRip]";
        expect(parse(releaseName)).to.deep.include({ season: 1 });
    });

    it("should detect swedish season range", () => {
        const releaseName = "Bäst I Test Säsong 1-3 (1080p), Bast I Test";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3] });
    });

    it("should not detect season range across a removed year", () => {
        const releaseName = "Stargate SG-1. Season 9 (2005) 20.avi";
        expect(parse(releaseName)).to.deep.include({ seasons: [9], episode: 20 });
    });

    it("should detect season with three digits padded", () => {
        const releaseName = "One.Piece.S004E111.Dash.For.a.Miracle!.Alabasta.Animal.Land!.1080p.NF.WEB-DL.DDP2.0.x264-KQRM.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 4, episode: 111 });
    });

    it("should detect three digit season", () => {
        const releaseName = "house.hunters.s216e12.1080p.web.h264-spamneggs.mkv";
        expect(parse(releaseName)).to.deep.include({ season: 216, episode: 12 });
    });

    it("should not detect zero season from zero padded complete series episode range", () => {
        const releaseName = "Naruto Shippuden (Complete Series 001-500) [720p][HEVC][x265]";
        expect(parse(releaseName)).to.not.have.property("seasons");
    });

    it("should detect single season with plural seasons keyword", () => {
        const releaseName = "DareDevil - Seasons 2 - Episodes 10 thru 13";
        expect(parse(releaseName)).to.deep.include({ title: "DareDevil", seasons: [2], season: 2 });
    });

    it("should detect season list with oxford comma and", () => {
        const releaseName = "Skam - Season 1, 2, and 3";
        expect(parse(releaseName)).to.deep.include({ seasons: [1, 2, 3] });
    });

    it("should detect seasons from first n seasons", () => {
        const releaseName = "The Boondocks 2005 Complete First 3 Seasons Burntodisc";
        expect(parse(releaseName)).to.deep.include({ title: "The Boondocks", seasons: [1, 2, 3] });
    });
});
