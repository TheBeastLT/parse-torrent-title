// Ported from test/hardcoded.js of the js library.

//! Parsing hardcoded

use crate::common::*;

/// should detect if the release is hardcoded
#[test]
fn should_detect_if_the_release_is_hardcoded() {
    assert_includes("Ghost In The Shell 2017 1080p HC HDRip X264 AC3-EVO", json!({ "hardcoded": true }));
}

/// should not detect hardcoded when the release is not flagged as such
#[test]
fn should_not_detect_hardcoded_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["hardcoded"]);
}
