// Ported from test/extras.js of the js library.

//! Parsing extras

use crate::common::*;

/// should detect added ova and keep it out of the title
#[test]
fn should_detect_added_ova_and_keep_it_out_of_the_title() {
    assert_includes("[Judas] Elfen Lied + OVA (BD 1080p)", json!({ "title": "Elfen Lied", "extras": true }));
}

/// should detect added specials without spaces
#[test]
fn should_detect_added_specials_without_spaces() {
    assert_includes(
        "[Mysteria] Ao Haru Ride+OVA [BD 1080p] (HEVC x265 10-bit AAC)",
        json!({ "title": "Ao Haru Ride", "extras": true }),
    );
}

/// should detect a chain of added extras
#[test]
fn should_detect_a_chain_of_added_extras() {
    assert_includes(
        "[Pog42] Shirobako + Specials + OVAs (BD 1080p HEVC x265 10-bit)",
        json!({ "title": "Shirobako", "extras": true }),
    );
}

/// should detect added specials and extras joined by and
#[test]
fn should_detect_added_specials_and_extras_joined_by_and() {
    assert_includes("Shiki + Specials and Extras (2010-2011)", json!({ "title": "Shiki", "extras": true }));
}

/// should detect added extras in parentheses
#[test]
fn should_detect_added_extras_in_parentheses() {
    assert_includes("Hyouka (+ OVA) [BD]", json!({ "title": "Hyouka", "extras": true }));
}

/// should detect added counted movies
#[test]
fn should_detect_added_counted_movies() {
    assert_includes("Naruto Shippuuden 1-15+ 3 Movies", json!({ "extras": true }));
}

/// should detect added extras for a non anime series
#[test]
fn should_detect_added_extras_for_a_non_anime_series() {
    assert_includes(
        "Star Trek Original Series TOS +extras 720p EN-SUB",
        json!({ "title": "Star Trek Original Series TOS", "extras": true }),
    );
}

/// should not detect extras from an ova title
#[test]
fn should_not_detect_extras_from_an_ova_title() {
    assert_missing("Hyouka OVA.mkv", &["extras"]);
}

/// should not detect extras from a dash separated ova
#[test]
fn should_not_detect_extras_from_a_dash_separated_ova() {
    assert_missing("Kyochuu Rettou - OVA (Uncut)", &["extras"]);
}

/// should not detect extras from an ampersand in the title
#[test]
fn should_not_detect_extras_from_an_ampersand_in_the_title() {
    assert_missing("Panty & Stocking with Garterbelt [BD]", &["extras"]);
}

/// should not detect extras from a movie title with an ampersand
#[test]
fn should_not_detect_extras_from_a_movie_title_with_an_ampersand() {
    assert_missing("Tom & Jerry Movie", &["extras"]);
}

/// should detect added soundtracks and books
#[test]
fn should_detect_added_soundtracks_and_books() {
    assert_includes(
        "Hyouka_TV+Novel+Manga+Music+Other; dub jpn sub chs (2019-10-06)",
        json!({ "title": "Hyouka TV", "extras": true }),
    );
}

/// should end the title before a numbered special section
#[test]
fn should_end_the_title_before_a_numbered_special_section() {
    assert_includes(
        "[Moozzi2] Aria The Animation [SP01] TV-Spot - 01 (BD 1440x1080 x.264 Flac).mkv",
        json!({ "title": "Aria The Animation", "extras": true }),
    );
}

/// should end the title before a numbered special section with a named special
#[test]
fn should_end_the_title_before_a_numbered_special_section_with_a_named_special() {
    assert_includes(
        "[BananaBoat] Yuyushiki [SP03] Nyanyashiki - 01 (BD 1920x1080 x.264 Flac).mkv",
        json!({ "title": "Yuyushiki", "extras": true }),
    );
}

/// should not detect extras from a leading special section
#[test]
fn should_not_detect_extras_from_a_leading_special_section() {
    assert_includes("[SP01] Show - 01.mkv", json!({ "title": "Show" }));
    assert_missing("[SP01] Show - 01.mkv", &["extras"]);
}

/// should not detect extras from a special word without a number
#[test]
fn should_not_detect_extras_from_a_special_word_without_a_number() {
    assert_missing("[Group] Show [SP] - 01.mkv", &["extras"]);
}

/// should not detect extras from other in an ampersand title
#[test]
fn should_not_detect_extras_from_other_in_an_ampersand_title() {
    assert_includes(
        "[uba] Love, Chunibyo & Other Delusions! (2012) - S02 v3 (BD Remux)",
        json!({ "title": "Love, Chunibyo & Other Delusions!" }),
    );
}

/// should detect extras added with plus and keep them out of the title
#[test]
fn should_detect_extras_added_with_plus_and_keep_them_out_of_the_title() {
    assert_includes(
        "Magic Boy plus Extra Princess Kuro | 1959 | DVDRip | 720p | x265 | AC3 | German",
        json!({ "title": "Magic Boy", "year": 1959, "extras": true }),
    );
}

/// should keep plus in a title when followed by a movie
#[test]
fn should_keep_plus_in_a_title_when_followed_by_a_movie() {
    assert_includes(
        "Macross Plus Movie Edition 1995 720p BluRay x264-GeneMige",
        json!({ "title": "Macross Plus Movie Edition", "year": 1995 }),
    );
}

/// should keep plus in a title when followed by a special
#[test]
fn should_keep_plus_in_a_title_when_followed_by_a_special() {
    assert_field("Amagami SS Plus Special", "title", json!("Amagami SS Plus Special"));
}

/// should keep plus in a title when followed by an ova
#[test]
fn should_keep_plus_in_a_title_when_followed_by_an_ova() {
    assert_field("[SpurdoMux] Macross Plus OVA 1-2 BD 720p (Dual audio)", "title", json!("Macross Plus OVA"));
}
