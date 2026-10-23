// Ported from test/extended.js of the js library.

//! Parsing extended

use crate::common::*;

/// should detect if the release is extended
#[test]
fn should_detect_if_the_release_is_extended() {
    assert_includes("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", json!({ "extended": true }));
}

/// should detect extended in the title when separated
#[test]
fn should_detect_extended_in_the_title_when_separated() {
    assert_includes(
        "Ghostbusters - Extended (2016) 1080p H265 BluRay Rip ita eng AC3 5.1",
        json!({ "extended": true }),
    );
}

/// should not detect extended when the release is not flagged as such
#[test]
fn should_not_detect_extended_when_the_release_is_not_flagged_as_such() {
    assert_missing("Better.Call.Saul.S03E04.CONVERT.720p.WEB.h264-TBS", &["extended"]);
}
