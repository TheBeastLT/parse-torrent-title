// Ported from test/container.js of the js library.

//! Parsing container

use crate::common::*;

/// should detect if the mkv release has the correct container
#[test]
fn should_detect_if_the_mkv_release_has_the_correct_container() {
    assert_includes("Kevin Hart What Now (2016) 1080p BluRay x265 6ch -Dtech mkv", json!({ "container": "mkv" }));
}

/// should detect if the mp4 release has the correct container
#[test]
fn should_detect_if_the_mp4_release_has_the_correct_container() {
    assert_includes("The Gorburger Show S01E05 AAC MP4-Mobile", json!({ "container": "mp4" }));
}

/// should detect if the avi release has the correct container
#[test]
fn should_detect_if_the_avi_release_has_the_correct_container() {
    assert_includes("[req]Night of the Lepus (1972) DVDRip XviD avi", json!({ "container": "avi" }));
}
