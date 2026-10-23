// Ported from test/threeD.js of the js library.

//! Parsing 3D

use crate::common::*;

/// should detect 3D HSBS together
#[test]
fn should_detect_3_d_hsbs_together() {
    assert_includes(
        "Incredibles 2 (2018) 3D HSBS 1080p BluRay H264 DolbyD 5.1 + nickarad",
        json!({ "threeD": "3D HSBS" }),
    );
}

/// should detect 3D H-SBS apart
#[test]
fn should_detect_3_d_h_sbs_apart() {
    assert_includes("Despicable.Me.2010.1080p.PROPER.3D.BluRay.H-SBS.x264-CULTHD [Pub", json!({ "threeD": "3D HSBS" }));
}

/// should detect 3D Half-SBS apart
#[test]
fn should_detect_3_d_half_sbs_apart() {
    assert_includes("Avengers.Infinity.War.2018.3D.BluRay.1080p.Half-SBS.DTS.x264-CHC", json!({ "threeD": "3D HSBS" }));
}

/// should detect 3D Half-SBS when 3D is in the name
#[test]
fn should_detect_3_d_half_sbs_when_3_d_is_in_the_name() {
    assert_includes("Gravity.3D.2013.1080p.BluRay.Half-SBS.DTS.x264-PublicHD", json!({ "threeD": "3D HSBS" }));
}

/// should detect 3D SBS apart
#[test]
fn should_detect_3_d_sbs_apart() {
    assert_includes("Guardians of the Galaxy Vol 3 2023 1080p 3D BluRay SBS x264", json!({ "threeD": "3D SBS" }));
}

/// should detect 3D SBS together
#[test]
fn should_detect_3_d_sbs_together() {
    assert_includes("3-D Zen Extreme Ecstasy 3D SBS (2011) [BDRip 1080p].avi", json!({ "threeD": "3D SBS" }));
}

/// should detect 3D SBS when 3D is small letter
#[test]
fn should_detect_3_d_sbs_when_3_d_is_small_letter() {
    assert_includes("Saw 3D (2010) 1080p 3d BrRip x264 SBS - 1.3GB - YIFY", json!({ "threeD": "3D SBS" }));
}

/// should detect 3D Full-SBS
#[test]
fn should_detect_3_d_full_sbs() {
    assert_includes(
        "Puss.In.Boots.The.Last.Wish.3D.(2022).Full-SBS.1080p.x264.ENG.AC3-JFC",
        json!({ "threeD": "3D SBS" }),
    );
}

/// should detect 3D HOU together
#[test]
fn should_detect_3_d_hou_together() {
    assert_includes("The Lego Ninjago Movie (2017) 3D HOU German DTS 1080p BluRay x264", json!({ "threeD": "3D HOU" }));
}

/// should detect 3D HOU apart
#[test]
fn should_detect_3_d_hou_apart() {
    assert_includes("47 Ronin 2013 3D BluRay HOU 1080p DTS x264-CHD3D", json!({ "threeD": "3D HOU" }));
}

/// should detect 3D H-OU
#[test]
fn should_detect_3_d_h_ou() {
    assert_includes("The Three Musketeers 3D 2011 1080p H-OU BDRip x264 ac3 vice", json!({ "threeD": "3D HOU" }));
}

/// should detect 3D H/OU
#[test]
fn should_detect_3_d_h_ou_2() {
    assert_includes(
        "Kiss Me, Kate 3D (1953) [BRRip.1080p.x264.3D H/OU-DTS/AC3] [Lektor PL] [Eng]",
        json!({ "threeD": "3D HOU" }),
    );
}

/// should detect 3D Half-OU
#[test]
fn should_detect_3_d_half_ou() {
    assert_includes("Pixels.2015.1080p.3D.BluRay.Half-OU.x264.DTS-HD.MA.7.1-RARBG", json!({ "threeD": "3D HOU" }));
}

/// should detect 3D HalfOU
#[test]
fn should_detect_3_d_half_ou_2() {
    assert_includes(
        "Солнце 3D / 3D Sun (2007) BDRip 1080p от Ash61 | 3D-Video | halfOU | L1",
        json!({ "threeD": "3D HOU" }),
    );
}

/// should detect 3D OU together
#[test]
fn should_detect_3_d_ou_together() {
    assert_includes("Amazing Africa (2013) 3D OU 2160p Eng Rus", json!({ "threeD": "3D OU" }));
}

/// should detect 3D Full OU together
#[test]
fn should_detect_3_d_full_ou_together() {
    assert_includes("For the Birds (2000) 3D Full OU 1080p", json!({ "threeD": "3D OU" }));
}

/// should detect 3D
#[test]
fn should_detect_3_d() {
    assert_includes("Incredibles 2 2018 3D BluRay", json!({ "threeD": "3D" }));
}

/// should detect 3D with dots
#[test]
fn should_detect_3_d_with_dots() {
    assert_includes("Despicable.Me.3.2017.1080p.3D.BluRay.AVC.DTS-X.7.1-FGT", json!({ "threeD": "3D" }));
}

/// should detect 3D with hyphen separator
#[test]
fn should_detect_3_d_with_hyphen_separator() {
    assert_includes(
        "Гемини / Gemini Man (2019) BDRemux 1080p от селезень | 3D-Video | Лицензия",
        json!({ "threeD": "3D" }),
    );
}

/// should detect 3D in brackets
#[test]
fn should_detect_3_d_in_brackets() {
    assert_includes("Pokémon Detective Pikachu (2019) [BluRay] [3D]", json!({ "threeD": "3D" }));
}

/// should detect 3D in brackets with something else
#[test]
fn should_detect_3_d_in_brackets_with_something_else() {
    assert_includes("Doctor Strange in the Multiverse of Madness (2022) [1080p 3D] [B", json!({ "threeD": "3D" }));
}

/// should detect HD3D
#[test]
fn should_detect_hd3_d() {
    assert_includes("Бамблби / Bumblebee [2018, BDRemux, 1080p] BD3D", json!({ "threeD": "3D" }));
}

/// should detect SideBySide
#[test]
fn should_detect_side_by_side() {
    assert_includes(
        "Дэдпул и Росомаха / Deadpool & Wolverine [2024, BDRip, 1080p] SideBySide",
        json!({ "threeD": "3D SBS" }),
    );
}

/// should detect Half SideBySide
#[test]
fn should_detect_half_side_by_side() {
    assert_includes("Вий / Forbidden Kingdom [2014, WEB-DL] Half SideBySide", json!({ "threeD": "3D HSBS" }));
}

/// should detect OverUnder
#[test]
fn should_detect_over_under() {
    assert_includes(
        "Дэдпул и Росомаха / Deadpool & Wolverine [2024, BDRip, 1080p] OverUnder",
        json!({ "threeD": "3D OU" }),
    );
}

/// should detect Half OverUnder
#[test]
fn should_detect_half_over_under() {
    assert_includes("Миссия «Луна» / Лунный / Mooned [2023, BDRip] Half OverUnder", json!({ "threeD": "3D HOU" }));
}

/// should not detect 3D in name
#[test]
fn should_not_detect_3_d_in_name() {
    assert_missing("Texas.Chainsaw.3D.2013.PROPER.1080p.BluRay.x264-LiViDiTY", &["threeD"]);
}

/// should not detect 3D in name v2
#[test]
fn should_not_detect_3_d_in_name_v2() {
    assert_missing("Step Up 3D (2010) 720p BrRip x264 - 650MB - YIFY", &["threeD"]);
}

/// should not detect 3D in name v3
#[test]
fn should_not_detect_3_d_in_name_v3() {
    assert_missing(
        "[YakuboEncodes] 3D Kanojo Real Girl - 01 ~ 24 [BD 1080p 10bit x265 HEVC][Dual-Audio Opus][Multi-Subs]",
        &["threeD"],
    );
}
