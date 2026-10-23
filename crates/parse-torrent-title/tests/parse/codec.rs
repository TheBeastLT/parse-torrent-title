// Ported from test/codec.js of the js library.

//! Parsing codec

use crate::common::*;

/// should detect the hevc codec correctly
#[test]
fn should_detect_the_hevc_codec_correctly() {
    assert_includes("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", json!({ "codec": "hevc" }));
}

/// should detect the x264 codec correctly
#[test]
fn should_detect_the_x264_codec_correctly() {
    assert_includes("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", json!({ "codec": "x264" }));
}

/// should detect the Web-DL codec correctly
#[test]
fn should_detect_the_web_dl_codec_correctly() {
    assert_includes("The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN", json!({ "codec": "h264" }));
}

/// should detect the XviD codec correctly
#[test]
fn should_detect_the_xvi_d_codec_correctly() {
    assert_includes("Gotham S03E17 XviD-AFG", json!({ "codec": "xvid" }));
}

/// should detect the Mpeg2 codec correctly
#[test]
fn should_detect_the_mpeg2_codec_correctly() {
    assert_includes("Jimmy Kimmel 2017 05 03 720p HDTV DD5 1 MPEG2-CTL", json!({ "codec": "mpeg2" }));
}

/// should detect the HEVC10bit codec correctly
#[test]
fn should_detect_the_hevc10bit_codec_correctly() {
    assert_includes(
        "[Anime Time] Re Zero kara Hajimeru Isekai Seikatsu (Season 2 Part 1) [1080p][HEVC10bit x265][Multi Sub]",
        json!({ "codec": "hevc", "bitDepth": "10bit" }),
    );
}

/// should detect the HEVC10 codec correctly
#[test]
fn should_detect_the_hevc10_codec_correctly() {
    assert_includes(
        "[naiyas] Fate Stay Night - Unlimited Blade Works Movie [BD 1080P HEVC10 QAACx2 Dual Audio]",
        json!({ "codec": "hevc", "bitDepth": "10bit" }),
    );
}

/// should not detect 264 codec from episode number
#[test]
fn should_not_detect_264_codec_from_episode_number() {
    assert_missing("[DB]_Bleach_264_[012073FE].avi", &["codec"]);
}

/// should not detect 265 codec from episode number
#[test]
fn should_not_detect_265_codec_from_episode_number() {
    assert_missing("[DB]_Bleach_265_[B4A04EC9].avi", &["codec"]);
}

/// should detect av1 codec
#[test]
fn should_detect_av1_codec() {
    assert_includes("Love.Island.S03E32.1080p.AV1.10bit-MeGusta.mkv", json!({ "codec": "av1" }));
}
