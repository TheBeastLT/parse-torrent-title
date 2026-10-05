const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing source", () => {
    it("should detect the Bluray source correctly", () => {
        const releaseName = "Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay" });
    });

    it("should detect the HDTV source correctly", () => {
        const releaseName = "doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov";
        expect(parse(releaseName)).to.deep.include({ source: "HDTV" });
    });

    it("should detect the HDTVRip source correctly", () => {
        const releaseName = "Rebecca.1940.720p.HDTVRip.HDCLUB";
        expect(parse(releaseName)).to.deep.include({ source: "HDTV" });
    });

    it("should detect the SATRip source correctly", () => {
        const releaseName = "Gossip Girl - 1ª Temporada. (SAT-Rip)";
        expect(parse(releaseName)).to.deep.include({ source: "SATRip" });
    });

    it("should detect the DVDRip source correctly", () => {
        const releaseName = "A Stable Life S01E01 DVDRip x264-Ltu";
        expect(parse(releaseName)).to.deep.include({ source: "DVDRip" });
    });

    it("should detect the Web-DL source correctly", () => {
        const releaseName = "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN";
        expect(parse(releaseName)).to.deep.include({ source: "WEB-DL" });
    });

    it("should detect the WebRip source correctly", () => {
        const releaseName = "Brown Nation S01E05 1080p WEBRip x264-JAWN";
        expect(parse(releaseName)).to.deep.include({ source: "WEBRip" });
    });

    it("should detect the TeleSync source correctly", () => {
        const releaseName = "Star Wars The Last Jedi 2017 TeleSync AAC x264-MiniMe";
        expect(parse(releaseName)).to.deep.include({ source: "TeleSync" });
    });

    it("should detect the DVDScr source correctly", () => {
        const releaseName = "The.Shape.of.Water.2017.DVDScr.XVID.AC3.HQ.Hive-CM8";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("should detect the PPVRip source correctly", () => {
        const releaseName = "Cloudy With A Chance Of Meatballs 2 2013 720p PPVRip x264 AAC-FooKaS";
        expect(parse(releaseName)).to.deep.include({ source: "PPVRip" });
    });

    it("should detect the WEBMux source correctly", () => {
        const releaseName = "The.OA.1x08.L.Io.Invisibile.ITA.WEBMux.x264-UBi.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "WEBMux" });
    });

    it("should detect the BD source correctly", () => {
        const releaseName = "[UsifRenegade] Cardcaptor Sakura [BD][Remastered][1080p][HEVC_10Bit][Dual] + Movies";
        expect(parse(releaseName)).to.deep.include({ source: "BDRip" });
    });

    it("should detect the BD-RM source correctly", () => {
        const releaseName = "[UsifRenegade] Cardcaptor Sakura - 54 [BD-RM][1080p][x265_10Bit][Dual_AAC].mkv";
        expect(parse(releaseName)).to.deep.include({ source: "BDRip" });
    });

    it("should detect the MicroHD source correctly", () => {
        const releaseName = "Elvis & Nixon (MicroHD-1080p)";
        expect(parse(releaseName)).to.deep.include({ source: "HDRip" });
    });

    it("should detect the UHDrip source correctly", () => {
        const releaseName = "Bohemian Rhapsody 2018.2160p.UHDrip.x265.HDR.DD+.5.1-DTOne";
        expect(parse(releaseName)).to.deep.include({ source: "UHDRip" });
    });

    it("should detect the UltraHD source correctly", () => {
        const releaseName = "Blade.Runner.2049.2017.4K.UltraHD.BluRay.2160p.x264.TrueHD.Atmos";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay" });
    });

    it("should detect the UHD source correctly", () => {
        const releaseName = "Terminator.Dark.Fate.2019.2160p.UHD.BluRay.X265.10bit.HDR.TrueHD";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay" });
    });

    it("When We Were Boys 2013 BD Rip x264 titohmr", () => {
        const releaseName = "When We Were Boys 2013 BD Rip x264 titohmr";
        expect(parse(releaseName)).to.deep.include({ source: "BDRip" });
    });

    it("Key.and.Peele.s03e09.720p.web.dl.mrlss.sujaidr (pimprg)", () => {
        const releaseName = "Key.and.Peele.s03e09.720p.web.dl.mrlss.sujaidr (pimprg)";
        expect(parse(releaseName)).to.deep.include({ source: "WEB-DL" });
    });

    it("Godzilla 2014 HDTS HC XVID AC3 ACAB", () => {
        const releaseName = "Godzilla 2014 HDTS HC XVID AC3 ACAB";
        expect(parse(releaseName)).to.deep.include({ source: "TeleSync" });
    });

    it("Harry Potter And The Half Blood Prince 2009 telesync aac -- king", () => {
        const releaseName = "Harry Potter And The Half Blood Prince 2009 telesync aac -- king";
        expect(parse(releaseName)).to.deep.include({ source: "TeleSync" });
    });

    it("Capitao.America.2.TS.BrunoG", () => {
        const releaseName = "Capitao.America.2.TS.BrunoG";
        expect(parse(releaseName)).to.deep.include({ source: "TeleSync" });
    });

    it("Star Trek TS-Screener Spanish Alta-Calidad 2da Version 2009 - Me", () => {
        const releaseName = "Star Trek TS-Screener Spanish Alta-Calidad 2da Version 2009 - Me";
        expect(parse(releaseName)).to.deep.include({ source: "TeleSync" });
    });

    it("Solo: A Star Wars Story (2018) English 720p TC x264 900MBTEAM TR", () => {
        const releaseName = "Solo: A Star Wars Story (2018) English 720p TC x264 900MBTEAM TR";
        expect(parse(releaseName)).to.deep.include({ source: "TeleCine" });
    });

    it("Alita Battle Angel 2019 720p HDTC-1XBET", () => {
        const releaseName = "Alita Battle Angel 2019 720p HDTC-1XBET";
        expect(parse(releaseName)).to.deep.include({ source: "TeleCine" });
    });

    it("My.Super.Ex.Girlfriend.FRENCH.TELECINE.XViD-VCDFRV", () => {
        const releaseName = "My.Super.Ex.Girlfriend.FRENCH.TELECINE.XViD-VCDFRV";
        expect(parse(releaseName)).to.deep.include({ source: "TeleCine" });
    });

    it("You're Next (2013) cam XVID", () => {
        const releaseName = "You're Next (2013) cam XVID";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("Shes the one_2013(camrip)__TOPSIDER [email protected]", () => {
        const releaseName = "Shes the one_2013(camrip)__TOPSIDER [email protected]";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("Blair Witch 2016 HDCAM UnKnOwN", () => {
        const releaseName = "Blair Witch 2016 HDCAM UnKnOwN";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("Thor : Love and Thunder (2022) Hindi HQCAM x264 AAC - QRips.mkv", () => {
        const releaseName = "Thor : Love and Thunder (2022) Hindi HQCAM x264 AAC - QRips.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("Avatar The Way of Water (2022) 1080p HQ S-Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv", () => {
        const releaseName = "Avatar The Way of Water (2022) 1080p HQ S-Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("Avatar The Way of Water (2022) 1080p S Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv", () => {
        const releaseName = "Avatar The Way of Water (2022) 1080p S Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("Good Deeds 2012 SCR XViD-KiNGDOM", () => {
        const releaseName = "Good Deeds 2012 SCR XViD-KiNGDOM";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("Genova DVD-Screener Spanish 2008", () => {
        const releaseName = "Genova DVD-Screener Spanish 2008";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("El Albergue Rojo BR-Screener Spanish 2007", () => {
        const releaseName = "El Albergue Rojo BR-Screener Spanish 2007";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("The.Mysteries.of.Pittsburgh.LIMITED.SCREENER.XviD-COALiTiON [NOR", () => {
        const releaseName = "The.Mysteries.of.Pittsburgh.LIMITED.SCREENER.XviD-COALiTiON [NOR";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("El.curioso.caso.de.benjamin.button-BRScreener-[EspaDivx.com].rar", () => {
        const releaseName = "El.curioso.caso.de.benjamin.button-BRScreener-[EspaDivx.com].rar";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("Thor- Love and Thunder (2022) Original Hindi Dubbed 1080p HQ PreDVD Rip x264 AAC [1.7 GB]- CineVood.mkv", () => {
        const releaseName = "Thor- Love and Thunder (2022) Original Hindi Dubbed 1080p HQ PreDVD Rip x264 AAC [1.7 GB]- CineVood.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("Black Panther Wakanda Forever 2022 Hindi 1080p PDVDRip x264 AAC CineVood.mkv", () => {
        const releaseName = "Black Panther Wakanda Forever 2022 Hindi 1080p PDVDRip x264 AAC CineVood.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "SCR" });
    });

    it("Vampire in Vegas (2009) NL Subs DVDR DivXNL-Team", () => {
        const releaseName = "Vampire in Vegas (2009) NL Subs DVDR DivXNL-Team";
        expect(parse(releaseName)).to.deep.include({ source: "DVD" });
    });

    it("should parse WEB-DLRip source", () => {
        const releaseName = "Звонок из прошлого / Kol / The Call (2020) WEB-DLRip | ViruseProject";
        expect(parse(releaseName)).to.deep.include({ source: "WEB-DL" });
    });

    it("should parse BluRay Rip source", () => {
        const releaseName = "La nube (2020) [BluRay Rip][AC3 5.1 Castellano][www.maxitorrent.com]";
        expect(parse(releaseName)).to.deep.include({ source: "BRRip" });
    });

    it("should parse BluRay remux together source", () => {
        const releaseName = "Joker.2019.2160p.BluRay.REMUX.HEVC.DTS-HD.MA.TrueHD.7.1.Atmos-FGT";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse Blu-Ray remux source", () => {
        const releaseName = "Warcraft 2016 1080p Blu-ray Remux AVC TrueHD Atmos-KRaLiMaRKo";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse BluRay remux ahead source", () => {
        const releaseName = "Joker.2019.UHD.BluRay.2160p.TrueHD.Atmos.7.1.HEVC.REMUX-JAT";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse BluRay remux before source", () => {
        const releaseName = "Spider-Man No Way Home.2022.REMUX.1080p.Bluray.DTS-HD.MA.5.1.AVC-EVO[TGx]";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse BDRemux before source", () => {
        const releaseName = "Son of God 2014 HDR BDRemux 1080p.mkv";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse UHDRemux before source", () => {
        const releaseName = "Peter Rabbit 2 [4K UHDremux][2160p][HDR10][DTS-HD 5.1 Castellano-TrueHD 7.1-Ingles+Subs][ES-EN]";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse 4kUHDRemux before source", () => {
        const releaseName = "Snatch cerdos y diamantes [4KUHDremux 2160p][Castellano AC3 5.1-Ingles TrueHD 7.1+Subs]";
        expect(parse(releaseName)).to.deep.include({ source: "BluRay REMUX" });
    });

    it("should parse HDDVDRip source", () => {
        const releaseName = " Троя / Troy [2004 HDDVDRip-AVC] Dub + Original + Sub]";
        expect(parse(releaseName)).to.deep.include({ source: "DVDRip" });
    });

    it("should parse VHSRip source", () => {
        const releaseName = "Структура момента (Расим Исмайлов) [1980, Драма, VHSRip]";
        expect(parse(releaseName)).to.deep.include({ source: "DVDRip" });
    });

    it("should parse VHS source", () => {
        const releaseName = "Мужчины без женщин (Альгимантас Видугирис) [1981, Драма, VHS]";
        expect(parse(releaseName)).to.deep.include({ source: "DVD" });
    });

    it("should parse DVB source", () => {
        const releaseName = "Преферанс по пятницам (Игорь Шешуков) [1984, Детектив, DVB]";
        expect(parse(releaseName)).to.deep.include({ source: "HDTV" });
    });

    it("should parse WEB-DLRip source", () => {
        const releaseName = "Соперницы (Алексей Дмитриев) [1929, драма, WEB-DLRip]";
        expect(parse(releaseName)).to.deep.include({ source: "WEB-DL" });
    });

    it("should parse HDTSRip source", () => {
        const releaseName = "Dragon Blade (2015) HDTSRip Exclusive";
        expect(parse(releaseName)).to.deep.include({ source: "TeleSync" });
    });

    it("should parse HDTCRip source", () => {
        const releaseName = "Criminal (2016) Hindi Dubbed HDTCRip";
        expect(parse(releaseName)).to.deep.include({ source: "TeleCine" });
    });

    it("should parse CAMHD source", () => {
        const releaseName = "Avatar La Voie de l'eau.FRENCH.CAMHD.H264.AAC";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("should not parse VHS in title", () => {
        const releaseName = "VHS 3 Viral (2014)PL.mp4";
        expect(parse(releaseName)).to.not.have.property("source");
    });

    it("should detect CAM source attached to year", () => {
        const releaseName = "A Very Harold & Kumar 3D Christmas (2011)CAM(900mb)Nl subs N";
        expect(parse(releaseName)).to.deep.include({ source: "CAM" });
    });

    it("should detect WEB source", () => {
        const releaseName = "The.Last.of.Us.S01E08.1080p.WEB.H264-CAKES[TGx]";
        expect(parse(releaseName)).to.deep.include({ source: "WEB" });
    });
});
