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
});
