// Ported from test/repack.js of the js library.

//! Parsing repack

use crate::common::*;

/// should detect if the release is repack
#[test]
fn should_detect_if_the_release_is_repack() {
    assert_includes("Silicon Valley S04E03 REPACK HDTV x264-SVA", json!({ "repack": true }));
}

/// should detect if the release is rerip
#[test]
fn should_detect_if_the_release_is_rerip() {
    assert_includes(
        "Expedition Unknown S03E14 Corsicas Nazi Treasure RERIP 720p HDTV x264-W4F",
        json!({ "repack": true }),
    );
}

/// should not detect repack when the release is not flagged as such
#[test]
fn should_not_detect_repack_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["repack"]);
}
