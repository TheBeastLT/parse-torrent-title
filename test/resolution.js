const { expect } = require("chai");
const parse = require("../index").parse;

describe("Parsing resolution", () => {
    it("should detect the 1080p resolution correctly", () => {
        const releaseName = "Annabelle.2014.1080p.PROPER.HC.WEBRip.x264.AAC.2.0-RARBG";

        expect(parse(releaseName)).to.deep.include({ resolution: "1080p" });
    });

    it("should detect the 720p resolution correctly", () => {
        const releaseName = "doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov";

        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
    });

    it("should detect the 720p resolution with uppercase correctly", () => {
        const releaseName = "UFC 187 PPV 720P HDTV X264-KYR";

        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
    });

    it("should detect the 4k resolution correctly", () => {
        const releaseName = "The Smurfs 2 2013 COMPLETE FULL BLURAY UHD (4K) - IPT EXCLUSIVE";

        expect(parse(releaseName)).to.deep.include({ resolution: "4k" });
    });

    it("should detect the 2160p as 4k", () => {
        const releaseName = "Joker.2019.2160p.4K.BluRay.x265.10bit.HDR.AAC5.1";

        expect(parse(releaseName)).to.deep.include({ resolution: "4k" });
    });

    it("should detect custom aspect ratio for 4k resolution correctly", () => {
        const releaseName = "[Beatrice-Raws] Evangelion 3.333 You Can (Not) Redo [BDRip 3840x1632 HEVC TrueHD]";

        expect(parse(releaseName)).to.deep.include({ resolution: "4k" });
    });

    it("should detect custom aspect ratio for 1080p resolution correctly", () => {
        const releaseName = "[Erai-raws] Evangelion 3.0 You Can (Not) Redo - Movie [1920x960][Multiple Subtitle].mkv";

        expect(parse(releaseName)).to.deep.include({ resolution: "1080p" });
    });

    it("should detect custom aspect ratio for 720p resolution correctly", () => {
        const releaseName = "[JacobSwaggedUp] Kizumonogatari I: Tekketsu-hen (BD 1280x544) [MP4 Movie]";

        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
    });

    it("should detect 720i resolution and format as 720p", () => {
        const releaseName = "UFC 187 PPV 720i HDTV X264-KYR";

        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
    });

    it("should detect typo in 720p", () => {
        const releaseName = "IT Chapter Two.2019.7200p.AMZN WEB-DL.H264.[Eng Hin Tam Tel]DDP 5.1.MSubs.D0T.Telly";

        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
    });

    it("should detect typo in 1080p", () => {
        const releaseName = "Dumbo (1941) BRRip XvidHD 10800p-NPW";

        expect(parse(releaseName)).to.deep.include({ resolution: "1080p" });
    });

    it("should detect 1080 without spaces and M prefix", () => {
        const releaseName = "BluesBrothers2M1080.www.newpct.com.mkv";

        expect(parse(releaseName)).to.deep.include({ title: "BluesBrothers2", resolution: "1080p" });
    });

    it("should detect 1080 without spaces and BD prefix", () => {
        const releaseName = "BenHurParte2BD1080.www.newpct.com.mkv";

        expect(parse(releaseName)).to.deep.include({ title: "BenHurParte2", resolution: "1080p" });
    });

    it("should detect 720 without spaces title prefix", () => {
        const releaseName = "1993720p_101_WWW.NEWPCT1.COM.mkvv";

        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
    });

    it("should detect resolution in brackets without p suffix", () => {
        const releaseName = "[PALATA666] Resurrection 01 (720).mkv";
        expect(parse(releaseName)).to.deep.include({ resolution: "720p" });
        expect(parse(releaseName)).to.not.have.property("episodes");
    });

    it("should detect 1080p resolution when 4k remastered is present", () => {
        const releaseName = "Batman Returns 1992 4K Remastered BluRay 1080p DTS AC3 x264-MgB";
        expect(parse(releaseName)).to.deep.include({ resolution: "1080p" });
    });
});
