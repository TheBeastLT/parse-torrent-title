const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing half episode", () => {
    it("should detect half episode after anime dash episode", () => {
        const releaseName = "[Doki] Hai to Gensou no Grimgar - 02.5 (1920x1080 HEVC BD FLAC) [FCDA1B13].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [2], halfEpisode: true });
    });

    it("should detect half episode with version suffix", () => {
        const releaseName = "[SubsPlease] Baraou no Souretsu - 12.5v2 (720p) [7A329328].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [12], halfEpisode: true });
    });

    it("should detect half episode after season episode code", () => {
        const releaseName = "[Judas] Koisuru Asteroid - S01E06.5.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [6], halfEpisode: true });
    });

    it("should detect half episode after short season episode code", () => {
        const releaseName = "Kyōsōgiga.2013.S1E05.5.VOSTFR.1080p.WEBRip.Opus.x265.X5-452.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [5], halfEpisode: true });
    });

    it("should detect half episode after ep prefix", () => {
        const releaseName = "[Exiled-Destiny]_009-1_Ep09.5_(4a940426).mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [9], halfEpisode: true });
    });

    it("should detect half episode after episode word", () => {
        const releaseName = "NanoCore S3 Episode 10.5 English Subbed.mp4";

        expect(parse(releaseName)).to.deep.include({ seasons: [3], episodes: [10], halfEpisode: true });
    });

    it("should detect half episode at the start", () => {
        const releaseName = "11.5 - To Be Needed.mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [11], halfEpisode: true });
    });

    it("should detect half episode after the name", () => {
        const releaseName = "[E-F]_Hikaru_no_Go_12.5_New_Year_Special.avi";

        expect(parse(releaseName)).to.deep.include({ episodes: [12], halfEpisode: true });
    });

    it("should detect half episode in brackets", () => {
        const releaseName = "[DBD-Raws][Gangsta.][09.5][1080P][BDRip][HEVC-10bit][FLAC].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [9], halfEpisode: true });
    });

    it("should detect half episode after season word with season episode", () => {
        const releaseName = "Red Dwarf Season 08 Episode 01.5 - Back In The Red - Extended-1.mp4";

        expect(parse(releaseName)).to.deep.include({ seasons: [8], episodes: [1], halfEpisode: true });
    });

    it("should not detect half episode from audio channels", () => {
        const releaseName = "Show.S01E05.1080p.WEB-DL.DDP5.1.H.264-GRP.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from codec", () => {
        const releaseName = "Show.S01E06.1080p.AAC2.0.H.265.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from size", () => {
        const releaseName = "Show S01E07 1080p 5.5 GB";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from version", () => {
        const releaseName = "Show - 07 [1080p] [v1.5].mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from season", () => {
        const releaseName = "Show Season 1.5 Complete";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from frame rate", () => {
        const releaseName = "[Group] Show - 12 (BD 1080p 23.976fps).mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from imdb rating", () => {
        const releaseName = "Community S01E12 Comparative Religion IMDB 8.5.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a number in the episode title", () => {
        const releaseName = "Warrior.Nun.S01E01.Psalm.46.5.1080p.10bit.WEBRip.6CH.x265.HEVC-PSA.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a number in the show name", () => {
        const releaseName = "Special Ops 1.5 S01E02 720p DSNP WEBRip AAC 5.1 MSubs x264 - LOKiHD.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from secondary numbering in parentheses", () => {
        const releaseName = "[PM]Pocket_Monsters_-_065_(038.5)_-_Rougela's_Christmas[H264_SUB][8866E393].mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a number glued to the episode title", () => {
        const releaseName = "La.Casa.Di.Carta.4xe05.5.Minuti.Prima.ITA.WEBRIP.x264-mkeagle3.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from imdb episode title", () => {
        const releaseName = "S01E05 Episode #1.5 IMDB 9.0.mp4";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a number after a cross episode code", () => {
        const releaseName = "Warrior.Nun.1x01.Salmi.46.5.ITA.ENG.1080p.NFRip.AAC.x265-Pir8.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a season before cross episode", () => {
        const releaseName = "Doctor Who 2005 - 4.5 x01 - Planet of the Dead (Special 04.11.09).avi";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a season before episode range", () => {
        const releaseName = "Vsegda.govori.vsegda.3.5-6.ser.iz.8.avi";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a number starting the show name", () => {
        const releaseName = "[RUBaDUB][1080p] 2.5 Dimensional Seduction - 01 [BD x265 10bit Dual Audio AC3][CF1401CD].mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should detect half episode before secondary numbering in parentheses", () => {
        const releaseName = "[Anime Time] Kuroko No Basket Season 02 - 16.5 (41.5).mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [2], episodes: [16], halfEpisode: true });
    });

    it("should detect half episode before of total numbering", () => {
        const releaseName = "Nana_TV_[ru_jp]_[11.5_of_47]_[AnimeReactor_Ru].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [11], halfEpisode: true });
    });

    it("should not detect half episode from a number in the episode title after a dash episode", () => {
        const releaseName = "14 - Duck Dodgers in the 24.5 Century.mp4";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should detect season and episode from imdb episode title instead of half episode", () => {
        const releaseName = "Episode #1.5.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [5] }).and.to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an episode title starting with a time", () => {
        const releaseName = "The.Pitt.S01E11.5.00.P.M.1080p.AMZN.WEB-DL.DDP5.1.H.264-RGzsRutracker.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an episode title starting with a number", () => {
        const releaseName = "AwesomenessTV.S02E05.5.Ways.to.Reject.a.Guy.480p.HDTV.x264-mSD.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a translated episode title starting with a number", () => {
        const releaseName = "La.casa.de.papel.A.K.A.Money.Heist.S04E05.5.minutos.antes.DUAL-AUDIO.SPA-ENG.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a hyphenated episode title starting with a number", () => {
        const releaseName = "Deadliest.Catch.S11E13.5-Year.Storm.Part.2.XviD-AFG.avi";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an episode title with a time after the number", () => {
        const releaseName = "That.70s.Show.S06E21.5.15.720p.BluRay.FLAC2.0.x264-NTb.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a show name with a hyphenated word", () => {
        const releaseName = "[Serious Chat] 2.5-jigen no Ririsa 01 [BDRip 1080p HEVC FLAC].mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should detect half episode before release tags in a dotted name", () => {
        const releaseName = "Tengen.Toppa.Gurren.Lagann.E05.5.BDRip.720p.x264 (by OCG).mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [5], halfEpisode: true });
    });

    it("should detect half episode before a subtitle language tag", () => {
        const releaseName = "Nura.Rise.Of.The.Yokai.Clan.S02E12.5.SUBFRENCH.1080p.BluRay.x264-KAZETV.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [2], episodes: [12], halfEpisode: true });
    });

    it("should detect half episode before a special word", () => {
        const releaseName = "S12E11.5.Special.The.Journey.So.Far.720p-YoshQ.mp4";

        expect(parse(releaseName)).to.deep.include({ seasons: [12], episodes: [11], halfEpisode: true });
    });

    it("should detect half episode before the extension in a dotted name", () => {
        const releaseName = "Full.Metal.Panic.Invisible.Victory.S01E08.5.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [8], halfEpisode: true });
    });

    it("should detect half episode before a dotted episode title", () => {
        const releaseName = "Solo.Leveling.S01E07.5.How.to.Get.Stronger.1080p.CR.WEB-DL.DUAL.AAC2.0.H.264-VARYG.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [7], halfEpisode: true });
    });

    it("should detect half episode before a hyphenated episode title", () => {
        const releaseName = "Violet Evergarden S01E04.5-Surely, Someday, You Will Understand Love.mkv";

        expect(parse(releaseName)).to.deep.include({ seasons: [1], episodes: [4], halfEpisode: true });
    });

    it("should detect half episode before a release language tag", () => {
        const releaseName = "Nana.Ep11.5.MULTi.DVDRip.x264-NEGI3737.mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [11], halfEpisode: true });
    });

    it("should not detect half episode from an episode title starting with a time unit", () => {
        const releaseName = "Wahlburgers.S01E02.5.OClock.is.Dinnertime.HDTV.x264-CLDD.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an episode title starting with a number range", () => {
        const releaseName = "House.MD.S06E14.5.to.9.720p.10bit.BluRay.x265-budgetbits.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should detect zero padded half episode before a dash title", () => {
        const releaseName = "[Abystoma] Deca-Dence 05.5 - Install (recap) (WEB 720p) [628FF0F5].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [5], halfEpisode: true });
    });

    it("should detect single digit half episode before release info", () => {
        const releaseName = "[HorribleSubs] Cop Craft - 9.5 [720p].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [9], halfEpisode: true });
    });

    it("should detect single digit half episode after episode word before a dash title", () => {
        const releaseName = "SPECIAL-Episode 6.5 - Lucy In London.mp4";

        expect(parse(releaseName)).to.deep.include({ episodes: [6], halfEpisode: true });
    });

    it("should detect half episode after a dotted episode word", () => {
        const releaseName = "[Deadmau- RAWS] Shiki.2010.S01.BDRip.Rus.Jap.Deadmauvlad.Ep.20.5.avi";

        expect(parse(releaseName)).to.deep.include({ episodes: [20], halfEpisode: true });
    });

    it("should detect half episode with a four digit episode", () => {
        const releaseName = "One Piece - 1015.5 - A Special Episode to Admire Zoro-senpai and Sanji-senpai! [1080p].mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [1015], halfEpisode: true });
    });

    it("should not detect half episode from a number in the title next to a different episode", () => {
        const releaseName = "S1953E18 Duck Dodgers in the 24.5 Century MM.avi";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an imdb rating equal to the episode", () => {
        const releaseName = "S01E08 Heroes IMDB 8.5.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from season and episode notation at the start", () => {
        const releaseName = "1.5 The Third Floor Flat.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an episode title starting with a volume unit", () => {
        const releaseName = "Life.2x19.5.Litri.ITA-ENG.1080p.DLMux.h264-NovaRip.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should detect zero padded half episode at the start", () => {
        const releaseName = "07.5 Tenchi Muyo! Special Mihoshi's Space Adventure.mkv";

        expect(parse(releaseName)).to.deep.include({ episodes: [7], halfEpisode: true });
    });

    it("should not detect half episode from imdb season and episode title", () => {
        const releaseName = "S05E05 Episode #5.5.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an episode title with a time after an underscore", () => {
        const releaseName = "That.'70s.Show.S06E21.5_15.HDTV.720p.x264.RUS.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a bare single digit season and episode", () => {
        const releaseName = "Yes Minister 1.5 - The Writing On The Wall.avi";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a mid season number before a part", () => {
        const releaseName = "TPB 8.5 - Part 2 - Shit Shank.mp4";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a date after the year", () => {
        const releaseName = "WWF.Monday.Night.Raw.1996.8.5.PDTV.XviD-W4F";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from an ordinal number", () => {
        const releaseName = "06. Duck Dodgers in the 24.5 th Century (1953) DVDRip.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a dimensional title", () => {
        const releaseName = "[RUBaDUB][1080p] 2.5 Dimensional Seduction - Clean Closing 1 [BD x265 10bit].mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a software version", () => {
        const releaseName = "Face ID - Apple's iOS 14.5 Fix Fully Explained! [Full].mp4";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a number in the episode title after the episode code", () => {
        const releaseName = "Downton.Abbey.S05E05.Episode.5.5.1080p.BluRay.REMUX.AVC.FLAC.2.0-NOGRP.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a measure in the episode title", () => {
        const releaseName = "S01E05 - Attack of the 5.5 ft. geek.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a distance", () => {
        const releaseName = "2.Nordic.Combined.WC.2015.03.07.Lahti.Team.Sprint.Gundersen.2x7.5.km.mkv";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });

    it("should not detect half episode from a day and month before the year", () => {
        const releaseName = "WWE.Friday.Night.Smackdown.16.5.2014.HDTV.x264-jkkk.mp4";

        expect(parse(releaseName)).to.not.have.property("halfEpisode");
    });
});
