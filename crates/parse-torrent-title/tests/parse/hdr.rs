// Ported from test/hdr.js of the js library.

//! Parsing hdr

use crate::common::*;

/// should detect HDR source correctly
#[test]
fn should_detect_hdr_source_correctly() {
    assert_includes("The.Mandalorian.S01E06.4K.HDR.2160p 4.42GB", json!({ "hdr": ["HDR"] }));
}

/// should detect HDR10 source correctly
#[test]
fn should_detect_hdr10_source_correctly() {
    assert_includes(
        "Spider-Man - Complete Movie Collection (2002-2022) 1080p.HEVC.HDR10.1920x800.x265. DTS-HD",
        json!({ "hdr": ["HDR"], "bitDepth": "10bit" }),
    );
}

/// should detect HDR10Plus source correctly
#[test]
fn should_detect_hdr10_plus_source_correctly() {
    assert_includes(
        "Bullet.Train.2022.2160p.AMZN.WEB-DL.x265.10bit.HDR10Plus.DDP5.1-SMURF",
        json!({ "hdr": ["HDR10+"], "bitDepth": "10bit" }),
    );
}

/// should detect DV source correctly v1
#[test]
fn should_detect_dv_source_correctly_v1() {
    assert_includes("Belle (2021) 2160p 10bit 4KLight DOLBY VISION BluRay DDP 7.1 x265-QTZ", json!({ "hdr": ["DV"] }));
}

/// should detect DV source correctly v2
#[test]
fn should_detect_dv_source_correctly_v2() {
    assert_includes(
        "Андор / Andor [01x01-03 из 12] (2022) WEB-DL-HEVC 2160p | 4K | Dolby Vision TV | NewComers, HDRezka Studio",
        json!({ "hdr": ["DV"] }),
    );
}

/// should detect DV source correctly v3
#[test]
fn should_detect_dv_source_correctly_v3() {
    assert_includes("АBullet.Train.2022.2160p.WEB-DL.DDP5.1.DV.MKV.x265-NOGRP", json!({ "hdr": ["DV"] }));
}

/// should detect DV source correctly v4
#[test]
fn should_detect_dv_source_correctly_v4() {
    assert_includes("Bullet.Train.2022.2160p.WEB-DL.DoVi.DD5.1.HEVC-EVO[TGx]", json!({ "hdr": ["DV"] }));
}

/// should detect HDR/DV source correctly v1
#[test]
fn should_detect_hdr_dv_source_correctly_v1() {
    assert_includes(
        "Спайдерхед / Spiderhead (2022) WEB-DL-HEVC 2160p | 4K | HDR | Dolby Vision Profile 8 | P | NewComers, Jaskier",
        json!({ "hdr": ["DV", "HDR"] }),
    );
}

/// should detect HDR/DV source correctly v2
#[test]
fn should_detect_hdr_dv_source_correctly_v2() {
    assert_includes(
        "House.of.the.Dragon.S01E07.2160p.10bit.HDR.DV.WEBRip.6CH.x265.HEVC-PSA",
        json!({ "hdr": ["DV", "HDR"] }),
    );
}

/// should detect HDR/HDR10+/DV source correctly
#[test]
fn should_detect_hdr_hdr10_dv_source_correctly() {
    assert_includes(
        "Флешбэк / Memory (2022) WEB-DL-HEVC 2160p | 4K | HDR | HDR10+ | Dolby Vision Profile 8 | Pazl Voice",
        json!({ "hdr": ["DV", "HDR10+", "HDR"] }),
    );
}
