// Ported from test/convert.js of the js library.

//! Parsing convert

use crate::common::*;

/// should detect if the release is convert
#[test]
fn should_detect_if_the_release_is_convert() {
    assert_includes("Better.Call.Saul.S03E04.CONVERT.720p.WEB.h264-TBS", json!({ "convert": true }));
}

/// should not detect convert when the release is not flagged as such
#[test]
fn should_not_detect_convert_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["convert"]);
}
