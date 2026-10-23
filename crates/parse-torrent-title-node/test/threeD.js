const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing 3D", () => {
    it("should detect 3D HSBS together", () => {
        const releaseName = "Incredibles 2 (2018) 3D HSBS 1080p BluRay H264 DolbyD 5.1 + nickarad";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HSBS" });
    });

    it("should detect 3D H-SBS apart", () => {
        const releaseName = "Despicable.Me.2010.1080p.PROPER.3D.BluRay.H-SBS.x264-CULTHD [Pub";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HSBS" });
    });

    it("should detect 3D Half-SBS apart", () => {
        const releaseName = "Avengers.Infinity.War.2018.3D.BluRay.1080p.Half-SBS.DTS.x264-CHC";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HSBS" });
    });

    it("should detect 3D Half-SBS when 3D is in the name", () => {
        const releaseName = "Gravity.3D.2013.1080p.BluRay.Half-SBS.DTS.x264-PublicHD";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HSBS" });
    });

    it("should detect 3D SBS apart", () => {
        const releaseName = "Guardians of the Galaxy Vol 3 2023 1080p 3D BluRay SBS x264";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D SBS" });
    });

    it("should detect 3D SBS together", () => {
        const releaseName = "3-D Zen Extreme Ecstasy 3D SBS (2011) [BDRip 1080p].avi";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D SBS" });
    });

    it("should detect 3D SBS when 3D is small letter", () => {
        const releaseName = "Saw 3D (2010) 1080p 3d BrRip x264 SBS - 1.3GB - YIFY";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D SBS" });
    });

    it("should detect 3D Full-SBS", () => {
        const releaseName = "Puss.In.Boots.The.Last.Wish.3D.(2022).Full-SBS.1080p.x264.ENG.AC3-JFC";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D SBS" });
    });

    it("should detect 3D HOU together", () => {
        const releaseName = "The Lego Ninjago Movie (2017) 3D HOU German DTS 1080p BluRay x264";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should detect 3D HOU apart", () => {
        const releaseName = "47 Ronin 2013 3D BluRay HOU 1080p DTS x264-CHD3D";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should detect 3D H-OU", () => {
        const releaseName = "The Three Musketeers 3D 2011 1080p H-OU BDRip x264 ac3 vice";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should detect 3D H/OU", () => {
        const releaseName = "Kiss Me, Kate 3D (1953) [BRRip.1080p.x264.3D H/OU-DTS/AC3] [Lektor PL] [Eng]";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should detect 3D Half-OU", () => {
        const releaseName = "Pixels.2015.1080p.3D.BluRay.Half-OU.x264.DTS-HD.MA.7.1-RARBG";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should detect 3D HalfOU", () => {
        const releaseName = "Солнце 3D / 3D Sun (2007) BDRip 1080p от Ash61 | 3D-Video | halfOU | L1";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should detect 3D OU together", () => {
        const releaseName = "Amazing Africa (2013) 3D OU 2160p Eng Rus";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D OU" });
    });

    it("should detect 3D Full OU together", () => {
        const releaseName = "For the Birds (2000) 3D Full OU 1080p";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D OU" });
    });

    it("should detect 3D", () => {
        const releaseName = "Incredibles 2 2018 3D BluRay";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D" });
    });

    it("should detect 3D with dots", () => {
        const releaseName = "Despicable.Me.3.2017.1080p.3D.BluRay.AVC.DTS-X.7.1-FGT";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D" });
    });

    it("should detect 3D with hyphen separator", () => {
        const releaseName = "Гемини / Gemini Man (2019) BDRemux 1080p от селезень | 3D-Video | Лицензия";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D" });
    });

    it("should detect 3D in brackets", () => {
        const releaseName = "Pokémon Detective Pikachu (2019) [BluRay] [3D]";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D" });
    });

    it("should detect 3D in brackets with something else", () => {
        const releaseName = "Doctor Strange in the Multiverse of Madness (2022) [1080p 3D] [B";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D" });
    });

    it("should detect HD3D", () => {
        const releaseName = "Бамблби / Bumblebee [2018, BDRemux, 1080p] BD3D";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D" });
    });

    it("should detect SideBySide", () => {
        const releaseName = "Дэдпул и Росомаха / Deadpool & Wolverine [2024, BDRip, 1080p] SideBySide";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D SBS" });
    });

    it("should detect Half SideBySide", () => {
        const releaseName = "Вий / Forbidden Kingdom [2014, WEB-DL] Half SideBySide";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HSBS" });
    });

    it("should detect OverUnder", () => {
        const releaseName = "Дэдпул и Росомаха / Deadpool & Wolverine [2024, BDRip, 1080p] OverUnder";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D OU" });
    });

    it("should detect Half OverUnder", () => {
        const releaseName = "Миссия «Луна» / Лунный / Mooned [2023, BDRip] Half OverUnder";
        expect(parse(releaseName)).to.deep.include({ threeD: "3D HOU" });
    });

    it("should not detect 3D in name", () => {
        const releaseName = "Texas.Chainsaw.3D.2013.PROPER.1080p.BluRay.x264-LiViDiTY";
        expect(parse(releaseName)).to.not.have.property("threeD");
    });

    it("should not detect 3D in name v2", () => {
        const releaseName = "Step Up 3D (2010) 720p BrRip x264 - 650MB - YIFY";
        expect(parse(releaseName)).to.not.have.property("threeD");
    });

    it("should not detect 3D in name v3", () => {
        const releaseName = "[YakuboEncodes] 3D Kanojo Real Girl - 01 ~ 24 [BD 1080p 10bit x265 HEVC][Dual-Audio Opus][Multi-Subs]";
        expect(parse(releaseName)).to.not.have.property("threeD");
    });
});
