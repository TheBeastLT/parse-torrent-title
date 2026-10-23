// Ported from test/proper.js of the js library.

//! Parsing proper

use crate::common::*;

/// should detect if the release is proper
#[test]
fn should_detect_if_the_release_is_proper() {
    assert_includes("Into the Badlands S02E07 PROPER 720p HDTV x264-W4F", json!({ "proper": true }));
}

/// should detect if the release is real proper
#[test]
fn should_detect_if_the_release_is_real_proper() {
    assert_includes("Bossi-Reality-REAL PROPER-CDM-FLAC-1999-MAHOU", json!({ "proper": true }));
}

/// should not detect proper when the release is not flagged as such
#[test]
fn should_not_detect_proper_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["proper"]);
}
