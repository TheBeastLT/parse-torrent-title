const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing language", () => {
    it("should detect the russian language correctly", () => {
        const releaseName = "Deadpool 2016 1080p BluRay DTS Rus Ukr 3xEng HDCL";

        expect(parse(releaseName)).to.deep.include({ languages: ["russian", "ukrainian"] });
    });

    it("should detect the netherlands language correctly", () => {
        const releaseName = "VAIANA: MOANA (2017) NL-Retail [2D] EAGLE";

        expect(parse(releaseName)).to.deep.include({ languages: ["dutch"] });
    });

    it("should detect the flemish language correctly", () => {
        const releaseName = "De Noodcentrale S02E05 FLEMISH 540p WEBRip AAC H 264";

        expect(parse(releaseName)).to.deep.include({ languages: ["dutch"] });
    });

    it("should detect the truefrench language correctly", () => {
        const releaseName = "The Intern 2015 TRUEFRENCH 720p BluRay x264-PiNKPANTERS";

        expect(parse(releaseName)).to.deep.include({ languages: ["french"] });
    });

    it("should detect the vff language correctly", () => {
        const releaseName = "After Earth 2013 VFF BDrip x264 YJ";

        expect(parse(releaseName)).to.deep.include({ languages: ["french"] });
    });

    it("should detect the french language correctly", () => {
        const releaseName = "127.Heures.FRENCH.DVDRip.AC3.XViD-DVDFR";

        expect(parse(releaseName)).to.deep.include({ languages: ["french"] });
    });

    it("should detect the vostfr language with lowercase correctly", () => {
        const releaseName = "Color.Of.Night.Unrated.DC.VostFR.BRrip.x264";

        expect(parse(releaseName)).to.deep.include({ languages: ["french"] });
    });

    it("should detect the multi language correctly", () => {
        const releaseName = "Le Labyrinthe 2014 Multi-VF2 1080p BluRay x264-PopHD";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi audio"] });
    });

    it("should detect the VFI language correctly", () => {
        const releaseName = "Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG";

        expect(parse(releaseName)).to.deep.include({ languages: ["french"] });
    });

    it("should detect the italian language correctly", () => {
        const releaseName = "South.Park.S21E10.iTALiAN.FiNAL.AHDTV.x264-NTROPiC";

        expect(parse(releaseName)).to.deep.include({ languages: ["italian"] });
    });

    it("2- English- {SDH}.srt", () => {
        const releaseName = "2- English- {SDH}.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["english"] });
    });

    it("1_ English-Subs.srt", () => {
        const releaseName = "1_ English-Subs.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["english"] });
    });

    it("House S 1 CD 1-6 svensk, danska, norsk, finsk sub", () => {
        const releaseName = "House S 1 CD 1-6 svensk, danska, norsk, finsk sub";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish", "finnish", "swedish", "norwegian"] });
    });

    it("Svein.Og.Rotta.NORSK.Nordic.Subs.2006", () => {
        const releaseName = "Svein.Og.Rotta.NORSK.Nordic.Subs.2006";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish", "finnish", "swedish", "norwegian"] });
    });

    it("Borat med Norsk Undertekst", () => {
        const releaseName = "Borat med Norsk Undertekst";

        expect(parse(releaseName)).to.deep.include({ languages: ["norwegian"] });
    });

    it("Subs/21_Bokmal.srt", () => {
        const releaseName = "Subs/21_Bokmal.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["norwegian"] });
    });

    it("Subs/nob.srt", () => {
        const releaseName = "Subs/nob.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["norwegian"] });
    });

    it("Subs/5_nor.srt", () => {
        const releaseName = "Subs/5_nor.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["norwegian"] });
    });

    it("Curious.George.2.Follow.That.Monkey.2009.DK.SWE.UK.PAL.DVDR-CATC", () => {
        const releaseName = "Curious.George.2.Follow.That.Monkey.2009.DK.SWE.UK.PAL.DVDR-CATC";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish", "swedish"] });
    });

    it("Yes.Man.Dk-Subs.2009.dingel", () => {
        const releaseName = "Yes.Man.Dk-Subs.2009.dingel";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish"] });
    });

    it("Dan-SDH.srt", () => {
        const releaseName = "Dan-SDH.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish"] });
    });

    it("dan.srt", () => {
        const releaseName = "dan.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish"] });
    });

    it("Red Riding 1974 [2009 PAL DVD][En Subs[Sv.No.Fi]", () => {
        const releaseName = "Red Riding 1974 [2009 PAL DVD][En Subs[Sv.No.Fi]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "finnish", "swedish"] }); // @TODO does not include no
    });

    it("Comme une Image (Look at Me) [2004 PAL DVD][Fr Subs[Sv.Da.No]", () => {
        const releaseName = "Comme une Image (Look at Me) [2004 PAL DVD][Fr Subs[Sv.Da.No]";

        expect(parse(releaseName)).to.deep.include({ languages: ["french", "swedish"] }); // @TODO does not include da,no
    });

    it("A.Good.Day.To.Die.Hard.2013.SWESUB.DANSUB.FiNSUB.720p.WEB-DL.-Ro", () => {
        const releaseName = "A.Good.Day.To.Die.Hard.2013.SWESUB.DANSUB.FiNSUB.720p.WEB-DL.-Ro";

        expect(parse(releaseName)).to.deep.include({ languages: ["danish", "finnish", "swedish"] });
    });

    it("The.Prisoner.1967-1968.Complete.Series.Subs.English+Nordic", () => {
        const releaseName = "The.Prisoner.1967-1968.Complete.Series.Subs.English+Nordic";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "danish", "finnish", "swedish", "norwegian"] });
    });

    it("Royal.Pains.S05E02.HDTV.subtitulado.esp.sc.avi", () => {
        const releaseName = "Royal.Pains.S05E02.HDTV.subtitulado.esp.sc.avi";

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("Desmembrados (2006) [HDrip-XviD-AC3][Castellano]", () => {
        const releaseName = "Desmembrados (2006) [HDrip-XviD-AC3][Castellano]";

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("10_Spanish-Subs.srt", () => {
        const releaseName = "10_Spanish-Subs.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("Patriot Games [1992] Eng, Ger, Cze, Hun, Pol + multisub  DVDrip", () => {
        const releaseName = "Patriot Games [1992] Eng, Ger, Cze, Hun, Pol + multisub  DVDrip";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "english", "german", "polish", "czech", "hungarian"] });
    });

    it("Elvis Presley - La via del Male (King creole) - IT EN FR DE ES", () => {
        const releaseName = "Elvis Presley - La via del Male (King creole) - IT EN FR DE ES";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "french", "spanish", "italian", "german"] });
    });

    it("FernGully [H264 - Ita Dut Fre Ger Eng Spa Aac - MultiSub]", () => {
        const releaseName = "FernGully [H264 - Ita Dut Fre Ger Eng Spa Aac - MultiSub]";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "english", "french", "spanish", "italian", "german", "dutch"] });
    });

    it("Jesus de Montreal / Jesus of Montreal - subtitulos espanol", () => {
        const releaseName = "Jesus de Montreal / Jesus of Montreal - subtitulos espanol";

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("Los.Vengadores.DVDR español ingles clon", () => {
        const releaseName = "Los.Vengadores.DVDR español ingles clon";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish"] });
    });

    it("EMPIRE STATE 2013 DVDRip TRNC English and Española Latin", () => {
        const releaseName = "EMPIRE STATE 2013 DVDRip TRNC English and Española Latin";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish"] });
    });

    it("Mary Poppins Returns 2019 DVDRip LATINO-1XBET", () => {
        const releaseName = "Mary Poppins Returns 2019 DVDRip LATINO-1XBET";

        expect(parse(releaseName)).to.deep.include({ languages: ["latino"] });
    });

    it("Los Simpsons S18 E01 Latino", () => {
        const releaseName = "Los Simpsons S18 E01 Latino";

        expect(parse(releaseName)).to.deep.include({ languages: ["latino"] });
    });

    it("Spider-Man (2002) Blu-Ray [720p] Dual Ingles-Español", () => {
        const releaseName = "Spider-Man (2002) Blu-Ray [720p] Dual Ingles-Español";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "spanish"] });
    });

    it("Abuela (2015) 1080p BluRay x264 AC3 Dual Latino-Inglés", () => {
        const releaseName = "Abuela (2015) 1080p BluRay x264 AC3 Dual Latino-Inglés";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "latino"] });
    });

    it("Dumbo.2019.1080p.Dual.Lat", () => {
        const releaseName = "Dumbo.2019.1080p.Dual.Lat";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "latino"] });
    });

    it("A.Score.to.Settle.2019.lati.mp4", () => {
        const releaseName = " A.Score.to.Settle.2019.lati.mp4";

        expect(parse(releaseName)).to.deep.include({ languages: ["latino"] });
    });

    it("[S0.E04] Gambit królowej - Gra środkowa.Spanish Latin America.srt", () => {
        const releaseName = "[S0.E04] Gambit królowej - Gra środkowa.Spanish Latin America.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["latino"] });
    });

    it("Latin American.spa.srt", () => {
        const releaseName = "Latin American.spa.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["latino"] });
    });

    it("Men in Black International 2019 (inglês português)", () => {
        const releaseName = "Men in Black International 2019 (inglês português)";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "portuguese"] });
    });

    it("Assassins (1995) Sylvester Stallone.DVDrip.XviD - Italian Englis", () => {
        const releaseName = "Assassins (1995) Sylvester Stallone.DVDrip.XviD - Italian Englis";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "italian"] });
    });

    it("El Club de la Lucha[dvdrip][spanish]", () => {
        const releaseName = "El Club de la Lucha[dvdrip][spanish]";

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("The Curse Of The Weeping Woman 2019 BluRay 1080p Tel+Tam+hin+eng", () => {
        const releaseName = "The Curse Of The Weeping Woman 2019 BluRay 1080p Tel+Tam+hin+eng";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "hindi", "telugu", "tamil"] });
    });

    it("Inception 2010 1080p BRRIP[dual-audio][eng-hindi]", () => {
        const releaseName = "Inception 2010 1080p BRRIP[dual-audio][eng-hindi]";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "hindi"] });
    });

    it("Inception (2010) 720p BDRip Tamil+Telugu+Hindi+Eng", () => {
        const releaseName = "Inception (2010) 720p BDRip Tamil+Telugu+Hindi+Eng";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "hindi", "telugu", "tamil"] });
    });

    it("Subs/Dear.S01E02.WEBRip.x265-ION265/34_tam.srt", () => {
        const releaseName = "Subs/Dear.S01E02.WEBRip.x265-ION265/34_tam.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["tamil"] });
    });

    it("Subs/Dear.S01E02.WEBRip.x265-ION265/35_tel.srt", () => {
        const releaseName = "Subs/Dear.S01E02.WEBRip.x265-ION265/35_tel.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["telugu"] });
    });

    it("Dabangg 3 2019 AMZN WebRip Hindi 720p x264", () => {
        const releaseName = "Dabangg 3 2019 AMZN WebRip Hindi 720p x264";

        expect(parse(releaseName)).to.deep.include({ languages: ["hindi"] });
    });

    it("Quarantine [2008] [DVDRiP.XviD-M14CH0] [Lektor PL] [Arx]", () => {
        const releaseName = "Quarantine [2008] [DVDRiP.XviD-M14CH0] [Lektor PL] [Arx]";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("The Mandalorian S01E06 POLISH WEBRip x264-FLAME", () => {
        const releaseName = "The Mandalorian S01E06 POLISH WEBRip x264-FLAME";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Na Wspólnej (2024) [E3951-3954][1080p][WEB-DL][PL][x264-GhN]", () => {
        const releaseName = "Na Wspólnej (2024) [E3951-3954][1080p][WEB-DL][PL][x264-GhN]";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Kulej.2024.PL.1080p.AMZN.WEB-DL.x264-KiT", () => {
        const releaseName = "Kulej.2024.PL.1080p.AMZN.WEB-DL.x264-KiT";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Star.Wars.Skeleton.Crew.Sezon01.PLDUB.480p.DSNP.WEB-DL.H264.DDP5.1-K83", () => {
        const releaseName = "Star.Wars.Skeleton.Crew.Sezon01.PLDUB.480p.DSNP.WEB-DL.H264.DDP5.1-K83";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Bukmacher / Bookie [E02E07] [MULTi] [1080p] [AMZN] [WEB-DL] [H264] [DDP5.1.Atmos-K83] [Lektor PL]", () => {
        const releaseName = "Bukmacher / Bookie [E02E07] [MULTi] [1080p] [AMZN] [WEB-DL] [H264] [DDP5.1.Atmos-K83] [Lektor PL]";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi audio", "polish"] });
    });

    it("Gra 1968 [REKONSTRUKCJA] [1080p.WEB-DL.H264.AC3-FT] [Film Polski]", () => {
        const releaseName = "Gra 1968 [REKONSTRUKCJA] [1080p.WEB-DL.H264.AC3-FT] [Film Polski]";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Evil.Dead.Rise.2023.PLSUB.720p.MA.WEB-DL.H264.E-AC3-CMRG", () => {
        const releaseName = "Evil.Dead.Rise.2023.PLSUB.720p.MA.WEB-DL.H264.E-AC3-CMRG";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Wallace i Gromit: Zemsta pingwina / Wallace & Gromit: Vengeance Most Fowl (2024) [480p] [WEB-DL] [XviD] [DD5.1-K83] [Dubbing PL]", () => {
        const releaseName = "Wallace i Gromit: Zemsta pingwina / Wallace & Gromit: Vengeance Most Fowl (2024) [480p] [WEB-DL] [XviD] [DD5.1-K83] [Dubbing PL]";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Strażniczka smoków / Dragonkeeper (2024) [1080p] [H264] [Napisy PL]", () => {
        const releaseName = "Strażniczka smoków / Dragonkeeper (2024) [1080p] [H264] [Napisy PL]";

        expect(parse(releaseName)).to.deep.include({ languages: ["polish"] });
    });

    it("Carros 2 Dublado - Portugues BR (2011)", () => {
        const releaseName = "Carros 2 Dublado - Portugues BR (2011)";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("A.Simple.Plan.1998.720p.BDRIP.X264.dublado.portugues.BR.gmenezes", () => {
        const releaseName = "A.Simple.Plan.1998.720p.BDRIP.X264.dublado.portugues.BR.gmenezes";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("American.Horror.Story.S01E01.720p. PORTUGUÊS BR", () => {
        const releaseName = "American.Horror.Story.S01E01.720p. PORTUGUÊS BR";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Angel.S05E19.legendado.br.rmvb", () => {
        const releaseName = "Angel.S05E19.legendado.br.rmvb";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Grimm S01E11 Dublado BR [ kickUploader ]", () => {
        const releaseName = "Grimm S01E11 Dublado BR [ kickUploader ]";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("InuYasha.EP161.ptBR.subtitles.[inuplace.com.br].avi", () => {
        const releaseName = "InuYasha.EP161.ptBR.subtitles.[inuplace.com.br].avi";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Ghost.Rider.DivX_Gamonet(Ingles-Port.BR)-AC3.avi", () => {
        const releaseName = "Ghost.Rider.DivX_Gamonet(Ingles-Port.BR)-AC3.avi";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "portuguese"] });
    });

    it("I Am David  legendado pt/br", () => {
        const releaseName = "I Am David  legendado pt/br";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Lone Wolf and Cub 6 movies - legendas BR", () => {
        const releaseName = "Lone Wolf and Cub 6 movies - legendas BR";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Wonder Woman Season 3 (H.264 1080p; English/Portuguese-BR)", () => {
        const releaseName = "Wonder Woman Season 3 (H.264 1080p; English/Portuguese-BR)";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "portuguese"] });
    });

    it("MIB 3 - Homens de Preto 2012 ( Audio EN-BR - Leg BR  Mkv 1280x69", () => {
        const releaseName = "MIB 3 - Homens de Preto 2012 ( Audio EN-BR - Leg BR  Mkv 1280x69";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "portuguese"] });
    });

    it("my wife is a gangster 3 legendado em PT(BR)", () => {
        const releaseName = "my wife is a gangster 3 legendado em PT(BR)";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("A.Clockwork.Orange.1971.BRDRIP.1080p.DUAL.PORT-BR.ENG.gmenezes.m", () => {
        const releaseName = "A.Clockwork.Orange.1971.BRDRIP.1080p.DUAL.PORT-BR.ENG.gmenezes.m";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "portuguese"] });
    });

    it("Superman I - O Filme 1978 Leg. BR - Mkv 1280x528", () => {
        const releaseName = "Superman I - O Filme 1978 Leg. BR - Mkv 1280x528";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Subs/Brazilian.por.srt", () => {
        const releaseName = "Subs/Brazilian.por.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Brazilian Portuguese.por.srt", () => {
        const releaseName = "Brazilian Portuguese.por.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("[S0.E07] Gambit królowej - Gra końcowa.Portuguese Brazil.srt", () => {
        const releaseName = "[S0.E07] Gambit królowej - Gra końcowa.Portuguese Brazil.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("The Hit List (2011) DVD NTSC WS (eng-fre-pt-spa) [Sk]", () => {
        const releaseName = "The Hit List (2011) DVD NTSC WS (eng-fre-pt-spa) [Sk]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "french", "spanish"] }); // @TODO does not include pt
    });

    it("[POPAS] Neon Genesis Evangelion: The End of Evangelion [jp_PT-pt", () => {
        const releaseName = "[POPAS] Neon Genesis Evangelion: The End of Evangelion [jp_PT-pt";

        expect(parse(releaseName)).to.deep.include({ languages: ["japanese", "portuguese"] });
    });

    it("Zola Maseko - Drum (2004) PT subs", () => {
        const releaseName = "Zola Maseko - Drum (2004) PT subs";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Idrissa Ouedraogo - Yaaba (1989) EN ES FR PT", () => {
        const releaseName = "Idrissa Ouedraogo - Yaaba (1989) EN ES FR PT";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "french", "spanish"] }); // @TODO does not include pt
    });

    it("Metallica.Through.The.Never.2013 O Filme(leg.pt-pt)", () => {
        const releaseName = "Metallica.Through.The.Never.2013 O Filme(leg.pt-pt)";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Dinossauro (2000) --[ Ing / Pt / Esp ]", () => {
        const releaseName = "Dinossauro (2000) --[ Ing / Pt / Esp ]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish"] }); // @TODO does not include pt
    });

    it("Mulan 1 (1998) Versão Portuguesa", () => {
        const releaseName = "Mulan 1 (1998) Versão Portuguesa";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("The Guard 2011.DK.EN.ES.HR.NL.PT.RO.Subtitles", () => {
        const releaseName = "The Guard 2011.DK.EN.ES.HR.NL.PT.RO.Subtitles";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish", "romanian", "croatian", "dutch", "danish"] });
    });

    it("Titan.A.E.2000 720p  HDTV DTS Eng Fra Hun Rom Rus multisub", () => {
        const releaseName = "Titan.A.E.2000 720p  HDTV DTS Eng Fra Hun Rom Rus multisub";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "english", "french", "russian", "hungarian", "romanian"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Latin American es.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Latin American es.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["latino"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Brazilian pt.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Brazilian pt.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.pt.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.pt.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.es.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.es.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.de.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.de.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["german"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.it.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.it.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["italian"] });
    });

    it("Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.ar.ass", () => {
        const releaseName = "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.ar.ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["arabic"] });
    });

    it("Subs(ara,fre,ger).srt", () => {
        const releaseName = "Subs(ara,fre,ger).srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["french", "german", "arabic"] });
    });

    it("Subs(chi,eng,ind,kor,may,tha,vie).srt", () => {
        const releaseName = "Subs(chi,ind,kor,may,tha,vie).srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["korean", "chinese", "vietnamese", "indonesian", "thai", "malay"] });
    });

    it("Miami.Bici.2020.1080p.NETFLIX.WEB-DL.DDP5.1.H.264.EN-ROSub-ExtremlymTorrents", () => {
        const releaseName = "Miami.Bici.2020.1080p.NETFLIX.WEB-DL.DDP5.1.H.264.EN-ROSub-ExtremlymTorrents";
        expect(parse(releaseName)).to.deep.include({ languages: ["english", "romanian"] });
    });

    it("26_Romanian.srt", () => {
        const releaseName = "26_Romanian.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["romanian"] });
    });

    it("4_Bulgarian.srt", () => {
        const releaseName = "4_Bulgarian.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["bulgarian"] });
    });

    it("18_srp.srt", () => {
        const releaseName = "18_srp.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["serbian"] });
    });

    it("Aranyelet.S01.HUNGARIAN.1080p.WEBRip.DDP5.1.x264-SbR[rartv]", () => {
        const releaseName = "Aranyelet.S01.HUNGARIAN.1080p.WEBRip.DDP5.1.x264-SbR[rartv]";

        expect(parse(releaseName)).to.deep.include({ languages: ["hungarian"] });
    });

    it("Ponyo[2008]DvDrip-H264 Quad Audio[Eng Jap Fre Spa]AC3 5.1[DXO]", () => {
        const releaseName = "Ponyo[2008]DvDrip-H264 Quad Audio[Eng Jap Fre Spa]AC3 5.1[DXO]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "japanese", "french", "spanish"] });
    });

    it("The Mechanic [1972] Eng,Deu,Fra,Esp,Rus + multisub DVDrip", () => {
        const releaseName = "The Mechanic [1972] Eng,Deu,Fra,Esp,Rus + multisub DVDrip";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "english", "french", "spanish", "german", "russian"] });
    });

    it("Mommie Dearest [1981 PAL DVD][En.De.Fr.It.Es Multisubs[18]", () => {
        const releaseName = "Mommie Dearest [1981 PAL DVD][En.De.Fr.It.Es Multisubs[18]";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "english", "french", "spanish", "german"] }); // @TODO does not include it
    });

    it("Pasienio sargyba S01E03 (2016 WEBRip LT)", () => {
        const releaseName = "Pasienio sargyba S01E03 (2016 WEBRip LT)";

        expect(parse(releaseName)).to.deep.include({ languages: ["lithuanian"] });
    });

    it("24_Lithuanian.srt", () => {
        const releaseName = "24_Lithuanian.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["lithuanian"] });
    });

    it("25_Latvian.srt", () => {
        const releaseName = "25_Latvian.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["latvian"] });
    });
    it("13_Estonian.srt", () => {
        const releaseName = "13_Estonian.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["estonian"] });
    });

    it("Ip.Man.4.The.Finale.2019.CHINESE.1080p.BluRay.x264.TrueHD.7.1.Atmos-HDC", () => {
        const releaseName = "Ip.Man.4.The.Finale.2019.CHINESE.1080p.BluRay.x264.TrueHD.7.1.Atmos-HDC";

        expect(parse(releaseName)).to.deep.include({ languages: ["chinese"] });
    });

    it("should parse CHT language", () => {
        const releaseName = "[NC-Raws] 叫我對大哥 (WEB版) / Ore, Tsushima - 10 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4]";

        expect(parse(releaseName)).to.deep.include({ languages: ["chinese"] });
    });

    it("Inuyasha_TV+Finale+OVA+Film+CD+Manga+Other; dub jpn,chn,eng sub chs (2019-09-21)", () => {
        const releaseName = "Inuyasha_TV+Finale+OVA+Film+CD+Manga+Other; dub jpn,chn,eng sub chs (2019-09-21)";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "japanese", "chinese"] });
    });

    it("Initial D Live Action 2005 ENG/CHI", () => {
        const releaseName = "Initial D Live Action 2005 ENG/CHI";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "chinese"] });
    });

    it("Wolf.Warrior.2015.720p.BluRay.x264.Mandarin.AAC-ETRG", () => {
        const releaseName = "Wolf.Warrior.2015.720p.BluRay.x264.Mandarin.AAC-ETRG";

        expect(parse(releaseName)).to.deep.include({ languages: ["chinese"] });
    });


    it("9_zh-Hans.srt", () => {
        const releaseName = "9_zh-Hans.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["chinese"] });
    });

    it("Traditional Chinese.chi.srt", () => {
        const releaseName = "Traditional Chinese.chi.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["taiwanese"] });
    });

    it("Subs/Promare - Chinese (Traditional).ass", () => {
        const releaseName = "Subs/Promare - Chinese (Traditional).ass";

        expect(parse(releaseName)).to.deep.include({ languages: ["taiwanese"] });
    });

    it("10_zh-Hant.srt", () => {
        const releaseName = "10_zh-Hant.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["taiwanese"] });
    });

    it("Berserk 01-25 [dual audio JP,EN] MKV", () => {
        const releaseName = "Berserk 01-25 [dual audio JP,EN] MKV";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "japanese"] });
    });

    it("FLCL S05.1080p HMAX WEB-DL DD2.0 H 264-VARYG (FLCL: Shoegaze Dual-Audio Multi-Subs)", () => {
        const releaseName = "FLCL S05.1080p HMAX WEB-DL DD2.0 H 264-VARYG (FLCL: Shoegaze Dual-Audio Multi-Subs)";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "dual audio"] });
    });

    it("Shinjuku Swan 2015 JAP 1080p BluRay x264 DTS-JYK", () => {
        const releaseName = "Shinjuku Swan 2015 JAP 1080p BluRay x264 DTS-JYK";

        expect(parse(releaseName)).to.deep.include({ languages: ["japanese"] });
    });

    it("Wet.Woman.in.the.Wind.2016.JAPANESE.1080p.BluRay.x264-iKiW", () => {
        const releaseName = "Wet.Woman.in.the.Wind.2016.JAPANESE.1080p.BluRay.x264-iKiW";

        expect(parse(releaseName)).to.deep.include({ languages: ["japanese"] });
    });

    it("All.Love.E146.KOR.HDTV.XViD-DeBTV", () => {
        const releaseName = "All.Love.E146.KOR.HDTV.XViD-DeBTV";

        expect(parse(releaseName)).to.deep.include({ languages: ["korean"] });
    });

    it("The.Nun.2018.KORSUB.HDRip.XviD.MP3-STUTTERSHIT", () => {
        const releaseName = "The.Nun.2018.KORSUB.HDRip.XviD.MP3-STUTTERSHIT";

        expect(parse(releaseName)).to.deep.include({ languages: ["korean"] });
    });

    it("Burning.2018.KOREAN.720p.BluRay.H264.AAC-VXT", () => {
        const releaseName = "Burning.2018.KOREAN.720p.BluRay.H264.AAC-VXT";

        expect(parse(releaseName)).to.deep.include({ languages: ["korean"] });
    });

    it("Atonement.2017.KOREAN.ENSUBBED.1080p.WEBRip.x264-VXT", () => {
        const releaseName = "Atonement.2017.KOREAN.ENSUBBED.1080p.WEBRip.x264-VXTT";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "korean"] });
    });

    it("A Freira (2018) Dublado HD-TS 720p", () => {
        const releaseName = "A Freira (2018) Dublado HD-TS 720p";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Escobar El Patron Del Mal Capitulo 91 SD (2012-10-10) [SiRaDuDe]", () => {
        const releaseName = "Escobar El Patron Del Mal Capitulo 91 SD (2012-10-10) [SiRaDuDe]";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Bleach - 215 ao 220 - [DB-BR]", () => {
        const releaseName = "Bleach - 215 ao 220 - [DB-BR]";

        expect(parse(releaseName)).to.deep.include({ languages: ["portuguese"] });
    });

    it("Joker.2019.MULTi.Bluray.1080p.Atmos.7.1.En.Fr.Sp.Pt-DDR[EtHD]", () => {
        const releaseName = "Joker.2019.MULTi.Bluray.1080p.Atmos.7.1.En.Fr.Sp.Pt-DDR[EtHD]";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi audio", "english", "french"] }); // @TODO does not include sp,pt
    });

    it("Dilbert complete series + en subs", () => {
        const releaseName = "Dilbert complete series + en subs";

        expect(parse(releaseName)).to.deep.include({ languages: ["english"] });
    });

    it("The Next Karate Kid (1994) NTSC WS -Eng/Fre/Spa/Por- [ctang]", () => {
        const releaseName = "The Next Karate Kid (1994) NTSC WS -Eng/Fre/Spa/Por- [ctang]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "french", "spanish", "portuguese"] });
    });

    xit("arsenico por compasion 1944 Capra spanish castellano", () => {
        const releaseName = "arsenico por compasion 1944 Capra spanish castellano"; // @TODO should not detect

        expect(parse(releaseName)).to.deep.include({ languages: ["spanish"] });
    });

    it("Un.Homme.Et.Une.Femme.1966.DVDRip.XviD.AR [PT ENG ESP]", () => {
        const releaseName = "Un.Homme.Et.Une.Femme.1966.DVDRip.XviD.AR [PT ENG ESP]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish", "portuguese"] });
    });

    it("Geminis.2005.Argentina.ESP.ENG.PT.SUBS", () => {
        const releaseName = "Geminis.2005.Argentina.ESP.ENG.PT.SUBS";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish", "portuguese"] });
    });

    it("1917 2019 1080p Bluray x264-Sexmeup [Greek Subs] [Braveheart]", () => {
        const releaseName = "1917 2019 1080p Bluray x264-Sexmeup [Greek Subs] [Braveheart]";

        expect(parse(releaseName)).to.deep.include({ languages: ["greek"] });
    });

    it("The Lion King 1,2,3 Greek Language", () => {
        const releaseName = "The Lion King 1,2,3 Greek Language";

        expect(parse(releaseName)).to.deep.include({ languages: ["greek"] });
    });

    it("The Adams Family (1991) (Greek Subs integratet)", () => {
        const releaseName = "The Adams Family (1991) (Greek Subs integratet)";

        expect(parse(releaseName)).to.deep.include({ languages: ["greek"] });
    });

    it("6_Greek.srt", () => {
        const releaseName = "6_Greek.srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["greek"] });
    });

    it("The Insult (2017) [BluRay] [720p] Arabic", () => {
        const releaseName = "The Insult (2017) [BluRay] [720p] Arabic";

        expect(parse(releaseName)).to.deep.include({ languages: ["arabic"] });
    });

    it("The.Mexican.2001 - Arabic Subs Hardcoded", () => {
        const releaseName = "The.Mexican.2001 - Arabic Subs Hardcoded";

        expect(parse(releaseName)).to.deep.include({ languages: ["arabic"] });
    });

    it("Much Loved (2015) - DVDRip x265 HEVC - ARAB-ITA-FRE AUDIO (ENG S", () => {
        const releaseName = "Much Loved (2015) - DVDRip x265 HEVC - ARAB-ITA-FRE AUDIO (ENG S";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "french", "italian", "arabic"] });
    });

    it("42.2013.720p.BluRay.x264.HD4Ar Arab subtitle", () => {
        const releaseName = "42.2013.720p.BluRay.x264.HD4Ar Arab subtitle";

        expect(parse(releaseName)).to.deep.include({ languages: ["arabic"] });
    });

    it("Fauda.S01.HEBREW.1080p.NF.WEBRip.DD5.1.x264-TrollHD[rartv]", () => {
        const releaseName = "Fauda.S01.HEBREW.1080p.NF.WEBRip.DD5.1.x264-TrollHD[rartv]";

        expect(parse(releaseName)).to.deep.include({ languages: ["hebrew"] });
    });

    it("madagascar 720p hebrew dubbed.mkv", () => {
        const releaseName = "madagascar 720p hebrew dubbed.mkv";

        expect(parse(releaseName)).to.deep.include({ languages: ["hebrew"] });
    });

    it("Into.the.Night.S01E04.Ayaz.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track17_[heb].srt", () => {
        const releaseName = "Into.the.Night.S01E04.Ayaz.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track17_[heb].srt";

        expect(parse(releaseName)).to.deep.include({ languages: ["hebrew"] });
    });

    it("The.Protector.2018.S03.TURKISH.WEBRip.x264-ION10", () => {
        const releaseName = "The.Protector.2018.S03.TURKISH.WEBRip.x264-ION10";

        expect(parse(releaseName)).to.deep.include({ languages: ["turkish"] });
    });

    it("Recep Ivedik 6 (2020) NETFLIX 720p WEBDL (Turkish) - ExtremlymTorrents", () => {
        const releaseName = "Recep Ivedik 6 (2020) NETFLIX 720p WEBDL (Turkish) - ExtremlymTorrents";

        expect(parse(releaseName)).to.deep.include({ languages: ["turkish"] });
    });

    it("The Insider*1999*[DVD5][PAL][ENG, POL, sub. ROM, TUR]", () => {
        const releaseName = "The Insider*1999*[DVD5][PAL][ENG, POL, sub. ROM, TUR]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "polish", "romanian", "turkish"] });
    });

    it("Divorzio allitaliana [XviD - Ita Mp3 - Sub Eng Esp Tur]", () => {
        const releaseName = "Divorzio allitaliana [XviD - Ita Mp3 - Sub Eng Esp Tur]";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "spanish", "italian", "turkish"] });
    });

    it("2007 Saturno Contro [Saturn In Opposition] (ITA-FRA-TUR) [EngSub", () => {
        const releaseName = "2007 Saturno Contro [Saturn In Opposition] (ITA-FRA-TUR) [EngSub";

        expect(parse(releaseName)).to.deep.include({ languages: ["english", "french", "italian", "turkish"] });
    });

    it("Cowboy Bebop - 1080p BDrip Audio+sub MULTI (VF / VOSTFR)", () => {
        const releaseName = "Cowboy Bebop - 1080p BDrip Audio+sub MULTI (VF / VOSTFR)";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi audio", "french"] });
    });

    it("Casablanca 1942 BDRip 1080p [multi language,multi subs].mkv", () => {
        const releaseName = "Casablanca 1942 BDRip 1080p [multi language,multi subs].mkv";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "multi audio"] });
    });

    it("Avengers.Endgame.2019.4K.UHD.ITUNES.DL.H265.Dolby.ATMOS.MSUBS-Deflate.Telly", () => {
        const releaseName = "Avengers.Endgame.2019.4K.UHD.ITUNES.DL.H265.Dolby.ATMOS.MSUBS-Deflate.Telly";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs"] });
    });

    it("Dawn of the Planet of the Apes (2014) 720p BluRay x264 - MULTI S", () => {
        const releaseName = "Dawn of the Planet of the Apes (2014) 720p BluRay x264 - MULTI S";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs"] });
    });

    it("Pirates of the Caribbean On Stranger Tides (2011) DVD5 (Multi Su", () => {
        const releaseName = "Pirates of the Caribbean On Stranger Tides (2011) DVD5 (Multi Su";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs"] });
    });

    it("Jumanji The Next Level (2019) 720p HDCAM Ads Blurred x264 Dual A", () => {
        const releaseName = "Jumanji The Next Level (2019) 720p HDCAM Ads Blurred x264 Dual A";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio"] });
    });

    it("Men in Black International (2019) 720p Korsub HDRip x264 ESub [Dual Line Audio] [Hindi English]", () => {
        const releaseName = "Men in Black International (2019) 720p Korsub HDRip x264 ESub [Dual Line Audio] [Hindi English]";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "korean", "hindi"] });
    });

    it("Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]", () => {
        const releaseName = "Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio", "english", "spanish"] });
    });

    it("O Rei do Show 2018 Dual Áudio 4K UtraHD By.Luan.Harper", () => {
        const releaseName = "O Rei do Show 2018 Dual Áudio 4K UtraHD By.Luan.Harper.";

        expect(parse(releaseName)).to.deep.include({ languages: ["dual audio"] });
    });

    it("Cowboy Bebop The Movie (2001) BD 1080p.x265.Tri-Audio.Ita.Eng.Jap [Rady]", () => {
        const releaseName = "Cowboy Bebop The Movie (2001) BD 1080p.x265.Tri-Audio.Ita.Eng.Jap [Rady]";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi audio", "english", "japanese", "italian"] });
    });

    it("[IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][Dublado][HEVC 10Bits] 800p BD", () => {
        const releaseName = "[IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][HEVC 10Bits] 800p BD";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "multi audio"] });
    });

    it("Blue Seed - 01 (BDRip 720x480p x265 HEVC FLAC, AC3x2 2.0x3)(Triple Audio)[sxales].mkv", () => {
        const releaseName = "Blue Seed - 01 (BDRip 720x480p x265 HEVC FLAC, AC3x2 2.0x3)(Triple Audio)[sxales].mkv";

        expect(parse(releaseName)).to.deep.include({ languages: ["multi audio"] });
    });

    it("Ministri S01E02 SLOVAK 480p x264-mSD", () => {
        const releaseName = "Ministri S01E02 SLOVAK 480p x264-mSD";
        expect(parse(releaseName)).to.deep.include({ languages: ["slovakian"] });
    });

    it("Subs/35_slo.srt", () => {
        const releaseName = "Subs/35_slo.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["slovakian"] });
    });

    it("Seinfeld.COMPLETE.SLOSUBS.DVDRip.XviD", () => {
        const releaseName = "Seinfeld.COMPLETE.SLOSUBS.DVDRip.XviD";
        expect(parse(releaseName)).to.deep.include({ languages: ["slovakian"] });
    });

    it("Subs/36_Slovenian.srt", () => {
        const releaseName = "Subs/36_Slovenian.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["slovenian"] });
    });

    it("The House Bunny (2008) BDRemux 1080p MediaClub [RUS, UKR, ENG]", () => {
        const releaseName = "The House Bunny (2008) BDRemux 1080p MediaClub [RUS, UKR, ENG]";
        expect(parse(releaseName)).to.deep.include({ languages: ["english", "russian", "ukrainian"] });
    });

    it("L'immortel (2010) DVDRip AVC (Russian,Ukrainian)", () => {
        const releaseName = "L'immortel (2010) DVDRip AVC (Russian,Ukrainian)";
        expect(parse(releaseName)).to.deep.include({ languages: ["russian", "ukrainian"] });
    });

    it("Into.the.Night.S01E03.Mathieu.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track33_[vie].srt", () => {
        const releaseName = "Into.the.Night.S01E03.Mathieu.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track33_[vie].srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["vietnamese"] });
    });

    it("Subs/vie.srt", () => {
        const releaseName = "Subs/vie.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["vietnamese"] });
    });

    it("Subs/Vietnamese.srt", () => {
        const releaseName = "Subs/Vietnamese.vie.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["vietnamese"] });
    });

    it("Subs/Dear.S01E05.WEBRip.x265-ION265/25_may.srt", () => {
        const releaseName = "Subs/Dear.S01E05.WEBRip.x265-ION265/25_may.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["malay"] });
    });

    it("Midnight.Diner.Tokyo.Stories.S02E10.WEBRip.x264-ION10/14_Indonesian.srt", () => {
        const releaseName = "Midnight.Diner.Tokyo.Stories.S02E10.WEBRip.x264-ION10/14_Indonesian.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["indonesian"] });
    });

    it("should detect portuguese languages", () => {
        const releaseName = "Inglês,Português,Italiano,Francês,Polonês,Russo,Norueguês,Dinamarquês,Alemão,Espanhol,Chinês,Japonês,Coreano,Persa,Hebraico,Sueco,Árabe,Holandês,Tâmil,Tailandês";
        expect(parse(releaseName)).to.deep.include({ languages: ["english", "japanese", "korean", "chinese", "french", "spanish", "portuguese", "italian", "german", "russian", "tamil", "polish", "dutch", "danish", "swedish", "norwegian", "thai", "hebrew", "persian"] });
    });

    it("should detect multiple languages in language only title", () => {
        const releaseName = "russian,english,ukrainian";
        expect(parse(releaseName)).to.deep.include({ languages: ["english", "russian", "ukrainian"] });
    });

    it("Subs/Thai.srt", () => {
        const releaseName = "Subs/Thai.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["thai"] });
    });

    it("Food Choices (2016) WEB.1080p.H264_tha.srt", () => {
        const releaseName = "Food Choices (2016) WEB.1080p.H264_tha.srt";
        expect(parse(releaseName)).to.deep.include({ languages: ["thai"] });
    });

    it("Ekk Deewana Tha (2012) DVDRip 720p x264 AAC TaRa.mkv", () => {
        const releaseName = "Ekk Deewana Tha (2012) DVDRip 720p x264 AAC TaRa.mkv";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("My Big Fat Greek Wedding (2002) 720p BrRip x264 - YIFY", () => {
        const releaseName = "My Big Fat Greek Wedding (2002) 720p BrRip x264 - YIFY";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("Get Him to the Greek 2010 720p BluRay", () => {
        const releaseName = "Get Him to the Greek 2010 720p BluRay";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("[Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR", () => {
        const releaseName = "[Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not remove english from title", () => {
        const releaseName = "The English Patient (1996) 720p BrRip x264 - YIFY";
        expect(parse(releaseName)).to.deep.include({ title: "The English Patient" });
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect LT language from yts domain name", () => {
        const releaseName = "Do.Or.Die.1991.1080p.BluRay.x264-[YTS.LT].mp4";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect PT language from temporada season naming", () => {
        const releaseName = "Castlevania 2017 1º temporada completa 1080p";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect PT language with cap. episode title", () => {
        const releaseName = "City on a Hill - Temporada 1 [HDTV][Cap.110].avi";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect NL language from website", () => {
        const releaseName = "La inocencia [720p][wWw.EliteTorrent.NL].mkv";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect FI language from website", () => {
        const releaseName = "Reasonable Doubt - Temporada 1 [HDTV][Cap.101][www.AtomoHD.FI]";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect FI language from website v2", () => {
        const releaseName = "1883 - Temporada 1 [HDTV 720p][Cap.103][AC3 5.1][www.AtomoHD.fi]";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect TW language from website", () => {
        const releaseName = "Los Winchester - Temporada 1 [HDTV][Cap.103][www.atomoHD.tw]";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect CH language from website", () => {
        const releaseName = "El Inmortal- Temporada 1 [HDTV 720p][Cap.104][AC3 5.1][www.AtomoHD.ch]]";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect TEL language from website", () => {
        const releaseName = "Black Friday (2021) [BluRay Rip][AC3 5.1][www.atomixHQ.TEL]";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect SE language from website", () => {
        const releaseName = "Deep Blue Sea 3 [HDR][wWw.EliteTorrent.SE]";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year", () => {
        const releaseName = "The.Italian.Job.1969.1080p.BluRay.x265-RARBG.mp4";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v2", () => {
        const releaseName = "Chinese Zodiac (2012) 1080p BrRip x264 - YIFY";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v3", () => {
        const releaseName = "The German Doctor 2013 1080p WEBRip";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v4", () => {
        const releaseName = "Johnny English 2003 1080p BluRay";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v5", () => {
        const releaseName = "Polish Wedding (1998) 1080p (moviesbyrizzo upl).mp4";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v6", () => {
        const releaseName = "Russian.Doll.S02E02.2160p.NF.WEB-DL.DDP5.1.HDR.DV.HEVC-PEXA.mkv";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v7", () => {
        const releaseName = "The.Spanish.Prisoner.1997.1080p.BluRay.x265-RARBG";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v8", () => {
        const releaseName = "Japanese.Story.2003.1080p.WEBRip.x264-RARBG";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v9", () => {
        const releaseName = "[ Torrent9.cz ] The.InBetween.S01E10.FiNAL.HDTV.XviD-EXTREME.avi";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect language from title before year v10", () => {
        const releaseName = "Thai Massage (2022) 720p PDVDRip x264 AAC.mkv";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect dan language", () => {
        const releaseName = "Carson Daly 2017 09 13 Dan Harmon 720p HDTV x264-CROOKS";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect dan language v2", () => {
        const releaseName = "Dan Browns The Lost Symbol S01E03 1080p WEB H264-GLHF";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect ara language", () => {
        const releaseName = "Ben.Ara.2015.1080p.WEBRip.x265-RARBG.mp4";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should not detect ara language v2", () => {
        const releaseName = "Ara.(A.Break).2008.DVDRip";
        expect(parse(releaseName)).to.not.have.property("languages");
    });

    it("should detect multi subs with other languages", () => {
        const releaseName = "X-Men Complete 13 Movie Collection Sci-Fi 2000 - 2020 Eng Rus Multi-Subs 1080p [H264-mp4]";
        expect(parse(releaseName)).to.deep.include({ languages: ["multi subs", "english", "russian"] });
    });
});
