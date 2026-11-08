// Ported from test/unrated.js of the js library.

//! Parsing unrated

use crate::common::*;

/// should detect if the release is unrated
#[test]
fn should_detect_if_the_release_is_unrated() {
    assert_includes("Identity.Thief.2013.Vostfr.UNRATED.BluRay.720p.DTS.x264-Nenuko", json!({ "unrated": true }));
}

/// should detect if the release is uncensored
#[test]
fn should_detect_if_the_release_is_uncensored() {
    assert_includes(
        "Charlie.les.filles.lui.disent.merci.2007.UNCENSORED.TRUEFRENCH.DVDRiP.AC3.Libe",
        json!({ "unrated": true }),
    );
}

/// should not detect unrated when the release is not flagged as such
#[test]
fn should_not_detect_unrated_when_the_release_is_not_flagged_as_such() {
    assert_missing("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", &["unrated"]);
}

/// should detect a bracketed chinese uncensored tag
#[test]
fn should_detect_a_bracketed_chinese_uncensored_tag() {
    assert_includes(
        "[极彩花雨] 异世界迷宫里的后宫生活 / Isekai Meikyuu de Harem wo [无修正][02][1080P][简体内嵌]",
        json!({ "unrated": true }),
    );
}
