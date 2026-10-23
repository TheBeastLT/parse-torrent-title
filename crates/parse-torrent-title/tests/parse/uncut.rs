// Ported from test/uncut.js of the js library.

//! Parsing uncut

use crate::common::*;

/// should detect an uncut release before its year
#[test]
fn should_detect_an_uncut_release_before_its_year() {
    assert_includes(
        "The.Terminator.Uncut.1984.1080p.BluRay.x264",
        json!({ "title": "The Terminator", "uncut": true, "year": 1984 }),
    );
}

/// should detect an uncut release before its year in a spaced name
#[test]
fn should_detect_an_uncut_release_before_its_year_in_a_spaced_name() {
    assert_includes("Aliens Uncut 1986 1080p BluRay x264", json!({ "title": "Aliens", "uncut": true, "year": 1986 }));
}

/// should detect a bracketed uncut release
#[test]
fn should_detect_a_bracketed_uncut_release() {
    assert_includes(
        "Dawn of the Dead (Uncut) 1978 1080p BluRay",
        json!({ "title": "Dawn of the Dead", "uncut": true, "year": 1978 }),
    );
}

/// should detect an uncut release before its resolution
#[test]
fn should_detect_an_uncut_release_before_its_resolution() {
    assert_includes("Scarface Uncut 1080p BluRay x264", json!({ "title": "Scarface", "uncut": true }));
}

/// should keep uncut starting a title
#[test]
fn should_keep_uncut_starting_a_title() {
    assert_includes("Uncut Gems 2019 1080p WEB-DL x264", json!({ "title": "Uncut Gems", "year": 2019 }));
    assert_missing("Uncut Gems 2019 1080p WEB-DL x264", &["uncut"]);
}

/// should keep uncut starting a dotted title
#[test]
fn should_keep_uncut_starting_a_dotted_title() {
    assert_includes("Uncut.Gems.2019.1080p.BluRay", json!({ "title": "Uncut Gems", "year": 2019 }));
    assert_missing("Uncut.Gems.2019.1080p.BluRay", &["uncut"]);
}

/// should keep a title that is only uncut
#[test]
fn should_keep_a_title_that_is_only_uncut() {
    assert_includes("Uncut (1997) DVDRip", json!({ "title": "Uncut", "year": 1997 }));
    assert_missing("Uncut (1997) DVDRip", &["uncut"]);
}

/// should keep uncut followed by more title words
#[test]
fn should_keep_uncut_followed_by_more_title_words() {
    assert_includes(
        "The Uncut Story of Mewtwos Origin 2000 DVDRip",
        json!({ "title": "The Uncut Story of Mewtwos Origin", "year": 2000 }),
    );
    assert_missing("The Uncut Story of Mewtwos Origin 2000 DVDRip", &["uncut"]);
}
