// Ported from test/retail.js of the js library.

//! Parsing retail

use crate::common::*;

/// should detect if the release is retail
#[test]
fn should_detect_if_the_release_is_retail() {
    assert_includes("MONSTER HIGH: ELECTRIFIED (2017) Retail PAL DVD9 [EAGLE]", json!({ "retail": true }));
}

/// should not detect retail when the release is not flagged as such
#[test]
fn should_not_detect_retail_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["retail"]);
}
