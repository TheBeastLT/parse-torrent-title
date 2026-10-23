// Ported from test/remastered.js of the js library.

//! Parsing remastered

use crate::common::*;

/// should detect if the release is remastered
#[test]
fn should_detect_if_the_release_is_remastered() {
    assert_includes(
        "The Fifth Element 1997 REMASTERED MULTi 1080p BluRay HDLight AC3 x264 Zone80",
        json!({ "remastered": true }),
    );
}

/// should detect if the release is remaster (without 'ed')
#[test]
fn should_detect_if_the_release_is_remaster_without_ed() {
    assert_includes("Predator 1987 REMASTER MULTi 1080p BluRay x264 FiDELiO", json!({ "remastered": true }));
}

/// should detect polish rekonstrukcija
#[test]
fn should_detect_polish_rekonstrukcija() {
    assert_includes(
        "Gra 1968 [REKONSTRUKCJA] [1080p.WEB-DL.H264.AC3-FT] [Napisy PL] [Film Polski]",
        json!({ "remastered": true }),
    );
}

/// should not detect remastered when the release is not flagged as such
#[test]
fn should_not_detect_remastered_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["remastered"]);
}
