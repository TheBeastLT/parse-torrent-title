// Ported from test/audio.js of the js library.

//! Parsing audio

use crate::common::*;

/// should detect the dts audio correctly
#[test]
fn should_detect_the_dts_audio_correctly() {
    assert_includes("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", json!({ "audio": "dts" }));
}

/// should detect the DTS-HD audio correctly
#[test]
fn should_detect_the_dts_hd_audio_correctly() {
    assert_includes("Gold 2016 1080p BluRay DTS-HD MA 5 1 x264-HDH", json!({ "audio": "dts-hd" }));
}

/// should detect the AAC audio correctly
#[test]
fn should_detect_the_aac_audio_correctly() {
    assert_includes("Rain Man 1988 REMASTERED 1080p BRRip x264 AAC-m2g", json!({ "audio": "aac" }));
}

/// should convert the AAC2.0 audio to AAC
#[test]
fn should_convert_the_aac2_0_audio_to_aac() {
    assert_includes("The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN", json!({ "audio": "aac" }));
}

/// should detect the dd5 audio correctly
#[test]
fn should_detect_the_dd5_audio_correctly() {
    assert_includes("Jimmy Kimmel 2017 05 03 720p HDTV DD5 1 MPEG2-CTL", json!({ "audio": "dd5.1" }));
}

/// should detect the AC3 audio correctly
#[test]
fn should_detect_the_ac3_audio_correctly() {
    assert_includes("A Dog's Purpose 2016 BDRip 720p X265 Ac3-GANJAMAN", json!({ "audio": "ac3" }));
}

/// should convert the AC-3 audio to AC3
#[test]
fn should_convert_the_ac_3_audio_to_ac3() {
    assert_includes("Retroactive 1997 BluRay 1080p AC-3 HEVC-d3g", json!({ "audio": "ac3" }));
}

/// should detect the mp3 audio correctly
#[test]
fn should_detect_the_mp3_audio_correctly() {
    assert_includes("Tempete 2016-TrueFRENCH-TVrip-H264-mp3", json!({ "audio": "mp3" }));
}

/// should detect the MD audio correctly
#[test]
#[ignore = "pending in the js suite"]
fn should_detect_the_md_audio_correctly() {
    assert_includes("Detroit.2017.BDRip.MD.GERMAN.x264-SPECTRE", json!({ "audio": "md" }));
}

/// should detect the eac3 5.1 audio correctly
#[test]
fn should_detect_the_eac3_5_1_audio_correctly() {
    assert_includes(
        "The Blacklist S07E04 (1080p AMZN WEB-DL x265 HEVC 10bit EAC-3 5.1)[Bandi]",
        json!({ "audio": "eac3" }),
    );
}

/// should detect the eac3 6.0 audio correctly
#[test]
fn should_detect_the_eac3_6_0_audio_correctly() {
    assert_includes("Condor.S01E03.1080p.WEB-DL.x265.10bit.EAC3.6.0-Qman[UTR].mkv", json!({ "audio": "eac3" }));
}

/// should detect the eac3 2.0 audio correctly 2
#[test]
fn should_detect_the_eac3_2_0_audio_correctly_2() {
    assert_includes(
        "The 13 Ghosts of Scooby-Doo (1985) S01 (1080p AMZN Webrip x265 10bit EAC-3 2.0 - Frys) [TAoE]",
        json!({ "audio": "eac3" }),
    );
}

/// should not detect mp3 audio inside a word
#[test]
fn should_not_detect_mp3_audio_inside_a_word() {
    assert_missing("[Thund3r3mp3ror] Attack on Titan - 23.mp4", &["audio"]);
}

/// should detect 2.0x2 audio
#[test]
fn should_detect_2_0x2_audio() {
    assert_includes(
        "Buttobi!! CPU - 02 (DVDRip 720x480p x265 HEVC AC3x2 2.0x2)(Dual Audio)[sxales].mkv",
        json!({ "audio": "2.0" }),
    );
}

/// should detect qaac2 audio
#[test]
fn should_detect_qaac2_audio() {
    assert_includes(
        "[naiyas] Fate Stay Night - Unlimited Blade Works Movie [BD 1080P HEVC10 QAACx2 Dual Audio]",
        json!({ "audio": "aac" }),
    );
}

/// should detect 2.0x5.1 audio
#[test]
fn should_detect_2_0x5_1_audio() {
    assert_includes(
        "Sakura Wars the Movie (2001) (BDRip 1920x1036p x265 HEVC FLACx2, AC3 2.0+5.1x2)(Dual Audio)[sxales].mkv",
        json!({ "audio": "2.0" }),
    );
}

/// should detect 5.1x2.0 audio
#[test]
fn should_detect_5_1x2_0_audio() {
    assert_includes(
        "Macross ~ Do You Remember Love (1984) (BDRip 1920x1036p x265 HEVC DTS-HD MA, FLAC, AC3x2 5.1+2.0x3)(Dual Audio)[sxales].mkv",
        json!({ "audio": "2.0" }),
    );
    assert_missing(
        "Macross ~ Do You Remember Love (1984) (BDRip 1920x1036p x265 HEVC DTS-HD MA, FLAC, AC3x2 5.1+2.0x3)(Dual Audio)[sxales].mkv",
        &["episodes"],
    );
}

/// should detect 5.1x2+2.0x3 audio
#[test]
fn should_detect_5_1x2_2_0x3_audio() {
    assert_includes(
        "Escaflowne (2000) (BDRip 1896x1048p x265 HEVC TrueHD, FLACx3, AC3 5.1x2+2.0x3)(Triple Audio)[sxales].mkv",
        json!({ "audio": "2.0" }),
    );
    assert_missing(
        "Escaflowne (2000) (BDRip 1896x1048p x265 HEVC TrueHD, FLACx3, AC3 5.1x2+2.0x3)(Triple Audio)[sxales].mkv",
        &["episodes"],
    );
}

/// should detect FLAC2.0x2 audio
#[test]
fn should_detect_flac2_0x2_audio() {
    assert_includes(
        "[SAD] Inuyasha - The Movie 4 - Fire on the Mystic Island [BD 1920x1036 HEVC10 FLAC2.0x2] [84E9A4A1].mkv",
        json!({ "audio": "flac" }),
    );
    assert_missing(
        "[SAD] Inuyasha - The Movie 4 - Fire on the Mystic Island [BD 1920x1036 HEVC10 FLAC2.0x2] [84E9A4A1].mkv",
        &["episodes"],
    );
}

/// should detect FLACx2 2.0x3 audio
#[test]
fn should_detect_flacx2_2_0x3_audio() {
    assert_includes(
        "Outlaw Star - 23 (BDRip 1440x1080p x265 HEVC AC3, FLACx2 2.0x3)(Dual Audio)[sxales].mkv",
        json!({ "audio": "2.0", "episode": 23 }),
    );
}

/// should detect 7.1 Atmos audio
#[test]
fn should_detect_7_1_atmos_audio() {
    assert_includes(
        "Spider-Man.No.Way.Home.2021.2160p.BluRay.REMUX.HEVC.TrueHD.7.1.Atmos-FraMeSToR",
        json!({ "audio": "7.1 Atmos" }),
    );
}

/// should detect hyphenated E-AC3 audio
#[test]
fn should_detect_hyphenated_e_ac3_audio() {
    assert_includes(
        "Tu corazón será destrozado Lat-Rus Subt Esp H264 E-AC3 5.1 BDRip Hd 1080p",
        json!({ "audio": "eac3" }),
    );
}

/// should detect hyphenated E-AC-3 audio
#[test]
fn should_detect_hyphenated_e_ac_3_audio() {
    assert_includes(
        "The Devil's Climb (2024) (1080p DS4K BluRay x265 10-bit SDR E-AC-3 5.1) [WeSLeY]",
        json!({ "audio": "eac3" }),
    );
}
