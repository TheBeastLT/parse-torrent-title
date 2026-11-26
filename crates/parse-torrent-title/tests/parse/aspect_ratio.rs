//! Parsing aspect ratio

use crate::common::*;

/// should detect a 16x9 aspect ratio and end the title before it
#[test]
fn should_detect_a_16x9_aspect_ratio_and_end_the_title_before_it() {
    assert_includes(
        "Sleeping with the Enemy 16X9 (Altair-Starhome).m4v",
        json!({"aspectRatio": "16:9", "title": "Sleeping with the Enemy"}),
    );
}

/// should detect a parenthesized 16x9 aspect ratio
#[test]
fn should_detect_a_parenthesized_16x9_aspect_ratio() {
    assert_includes("01 Star Wars (1977) 1080p (16x9).mp4", json!({"aspectRatio": "16:9"}));
}

/// should detect a 16x9 aspect ratio glued to a letterbox tag
#[test]
fn should_detect_a_16x9_aspect_ratio_glued_to_a_letterbox_tag() {
    assert_includes("THE_MATRIX_16X9LB_N_AMERICA", json!({"aspectRatio": "16:9", "title": "THE MATRIX"}));
}

/// should detect a 16x9 aspect ratio with a cyrillic x
#[test]
fn should_detect_a_16x9_aspect_ratio_with_a_cyrillic_x() {
    assert_includes("ЦИРК ПРИЕХАЛ 1 16х9 (Kultura 53).mkv", json!({"aspectRatio": "16:9", "title": "ЦИРК ПРИЕХАЛ 1"}));
}

/// should detect a 4x3 aspect ratio after a dvdrip tag
#[test]
fn should_detect_a_4x3_aspect_ratio_after_a_dvdrip_tag() {
    assert_includes("La Caduta Degli Dei [1969,DVDRip,4X3].avi", json!({"aspectRatio": "4:3"}));
}

/// should detect a 4x3 aspect ratio after a resolution
#[test]
fn should_detect_a_4x3_aspect_ratio_after_a_resolution() {
    assert_includes("Smart House (1999) [720p 4x3 DD5.1 Eng CC] [Squee].ts", json!({"aspectRatio": "4:3"}));
}

/// should detect a 4x3 aspect ratio before a dvd remux tag
#[test]
fn should_detect_a_4x3_aspect_ratio_before_a_dvd_remux_tag() {
    assert_includes(
        "Sailor Moon R The Movie 4х3 DVDRemux.mkv",
        json!({"aspectRatio": "4:3", "title": "Sailor Moon R The Movie"}),
    );
}

/// should detect a parenthesized 4x3 aspect ratio
#[test]
fn should_detect_a_parenthesized_4x3_aspect_ratio() {
    assert_includes("Zeitgeist 1 (2007) The Movie (4x3).mp4", json!({"aspectRatio": "4:3"}));
}

/// should detect a 4x3 aspect ratio in a run of tags
#[test]
fn should_detect_a_4x3_aspect_ratio_in_a_run_of_tags() {
    assert_includes("[Venny] Dragon Ball Z - Movie 13 [R2FR] [x264] [4x3].mkv", json!({"aspectRatio": "4:3"}));
}

/// should not detect a bare 4x3 episode code as aspect ratio
#[test]
fn should_not_detect_a_bare_4x3_episode_code_as_aspect_ratio() {
    assert_missing("Heroes 4x3.avi", &["aspectRatio"]);
}

/// should not detect a bracketed 4x3 episode code before an episode title as aspect ratio
#[test]
fn should_not_detect_a_bracketed_4x3_episode_code_before_an_episode_title_as_aspect_ratio() {
    assert_missing("Czarna Żmija [4x3] Sztuka kamuflażu.avi", &["aspectRatio"]);
}

/// should not detect a 4x3 episode code before an hdtv source as aspect ratio
#[test]
fn should_not_detect_a_4x3_episode_code_before_an_hdtv_source_as_aspect_ratio() {
    assert_missing("Buffy cazavampiros 4x3 HDTV XviD Castellano.avi", &["aspectRatio"]);
}

/// should not detect a leading 16x9 show name as aspect ratio
#[test]
fn should_not_detect_a_leading_16x9_show_name_as_aspect_ratio() {
    assert_missing("16x9.2014.03.29.A.Fathers.Justice.HDTV.x264-CLDD", &["aspectRatio"]);
}
