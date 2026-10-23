// Ported from test/region.js of the js library.

//! Parsing region

use crate::common::*;

/// should detect the R5 region correctly
#[test]
fn should_detect_the_r5_region_correctly() {
    assert_includes("Welcome to New York 2014 R5 XviD AC3-SUPERFAST", json!({ "region": "R5" }));
}

/// should not detect region in the title
#[test]
fn should_not_detect_region_in_the_title() {
    assert_missing("[Coalgirls]_Code_Geass_R2_06_(1920x1080_Blu-ray_FLAC)_[F8C7FE25].mkv", &["region"]);
}
