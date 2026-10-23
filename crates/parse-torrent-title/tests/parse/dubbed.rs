// Ported from test/dubbed.js of the js library.

//! Parsing dubbed

use crate::common::*;

/// should detect the english dubbed language correctly
#[test]
fn should_detect_the_english_dubbed_language_correctly() {
    assert_includes("Yo-Kai Watch S01E71 DUBBED 720p HDTV x264-W4F", json!({ "dubbed": true }));
}

/// should detect dub correctly
#[test]
fn should_detect_dub_correctly() {
    assert_includes(
        "[Golumpa] Kochoki - 11 (Kochoki - Wakaki Nobunaga) [English Dub] [FuniDub 720p x264 AAC] [MKV] [4FA0D898]",
        json!({ "dubbed": true }),
    );
}

/// should detect dub correctly
#[test]
fn should_detect_dub_correctly_2() {
    assert_includes(
        "[Golumpa] Kochoki - 11 (Kochoki - Wakaki Nobunaga) [English Dub] [FuniDub 720p x264 AAC] [MKV] [4FA0D898]",
        json!({ "dubbed": true }),
    );
}

/// should detect dubs correctly
#[test]
fn should_detect_dubs_correctly() {
    assert_includes("[Aomori-Raws] Juushinki Pandora (01-13) [Dubs & Subs]", json!({ "dubbed": true }));
}

/// should detect dual audio correctly
#[test]
fn should_detect_dual_audio_correctly() {
    assert_includes("[LostYears] Tsuredure Children (WEB 720p Hi10 AAC) [Dual-Audio]", json!({ "dubbed": true }));
}

/// should detect dual-audio correctly
#[test]
fn should_detect_dual_audio_correctly_2() {
    assert_includes("[DB] Gamers! [Dual Audio 10bit 720p][HEVC-x265]", json!({ "dubbed": true }));
}

/// should detect multi-audio correctly
#[test]
fn should_detect_multi_audio_correctly() {
    assert_includes("[DragsterPS] Yu-Gi-Oh! S02 [480p] [Multi-Audio] [Multi-Subs]", json!({ "dubbed": true }));
}

/// should detect dublado correctly
#[test]
fn should_detect_dublado_correctly() {
    assert_includes("A Freira (2018) Dublado HD-TS 720p", json!({ "dubbed": true }));
}

/// should detect dual correctly
#[test]
fn should_detect_dual_correctly() {
    assert_includes("Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]", json!({ "dubbed": true }));
}

/// should detect dubbing correctly
#[test]
fn should_detect_dubbing_correctly() {
    assert_includes("Vaiana Skarb oceanu / Moana (2016) [720p] [WEB-DL] [x264] [Dubbing]", json!({ "dubbed": true }));
}

/// should not detect dual subs
#[test]
fn should_not_detect_dual_subs() {
    assert_missing(
        "[Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR",
        &["dubbed"],
    );
}

/// should not detect multi-dub
#[test]
fn should_not_detect_multi_dub() {
    assert_includes(
        "[IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][HEVC 10Bits] 800p BD",
        json!({ "dubbed": true }),
    );
}
