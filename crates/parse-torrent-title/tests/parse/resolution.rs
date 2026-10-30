// Ported from test/resolution.js of the js library.

//! Parsing resolution

use crate::common::*;

/// should detect the 1080p resolution correctly
#[test]
fn should_detect_the_1080p_resolution_correctly() {
    assert_includes("Annabelle.2014.1080p.PROPER.HC.WEBRip.x264.AAC.2.0-RARBG", json!({ "resolution": "1080p" }));
}

/// should detect the 720p resolution correctly
#[test]
fn should_detect_the_720p_resolution_correctly() {
    assert_includes("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", json!({ "resolution": "720p" }));
}

/// should detect the 720p resolution with uppercase correctly
#[test]
fn should_detect_the_720p_resolution_with_uppercase_correctly() {
    assert_includes("UFC 187 PPV 720P HDTV X264-KYR", json!({ "resolution": "720p" }));
}

/// should detect the 4k resolution correctly
#[test]
fn should_detect_the_4k_resolution_correctly() {
    assert_includes("The Smurfs 2 2013 COMPLETE FULL BLURAY UHD (4K) - IPT EXCLUSIVE", json!({ "resolution": "4k" }));
}

/// should detect the 2160p as 4k
#[test]
fn should_detect_the_2160p_as_4k() {
    assert_includes("Joker.2019.2160p.4K.BluRay.x265.10bit.HDR.AAC5.1", json!({ "resolution": "4k" }));
}

/// should detect custom aspect ratio for 4k resolution correctly
#[test]
fn should_detect_custom_aspect_ratio_for_4k_resolution_correctly() {
    assert_includes(
        "[Beatrice-Raws] Evangelion 3.333 You Can (Not) Redo [BDRip 3840x1632 HEVC TrueHD]",
        json!({ "resolution": "4k" }),
    );
}

/// should detect custom aspect ratio for 1080p resolution correctly
#[test]
fn should_detect_custom_aspect_ratio_for_1080p_resolution_correctly() {
    assert_includes(
        "[Erai-raws] Evangelion 3.0 You Can (Not) Redo - Movie [1920x960][Multiple Subtitle].mkv",
        json!({ "resolution": "1080p" }),
    );
}

/// should detect custom aspect ratio for 720p resolution correctly
#[test]
fn should_detect_custom_aspect_ratio_for_720p_resolution_correctly() {
    assert_includes(
        "[JacobSwaggedUp] Kizumonogatari I: Tekketsu-hen (BD 1280x544) [MP4 Movie]",
        json!({ "resolution": "720p" }),
    );
}

/// should detect 720i resolution and format as 720p
#[test]
fn should_detect_720i_resolution_and_format_as_720p() {
    assert_includes("UFC 187 PPV 720i HDTV X264-KYR", json!({ "resolution": "720p" }));
}

/// should detect typo in 720p
#[test]
fn should_detect_typo_in_720p() {
    assert_includes(
        "IT Chapter Two.2019.7200p.AMZN WEB-DL.H264.[Eng Hin Tam Tel]DDP 5.1.MSubs.D0T.Telly",
        json!({ "resolution": "720p" }),
    );
}

/// should detect typo in 1080p
#[test]
fn should_detect_typo_in_1080p() {
    assert_includes("Dumbo (1941) BRRip XvidHD 10800p-NPW", json!({ "resolution": "1080p" }));
}

/// should detect 1080 without spaces and M prefix
#[test]
fn should_detect_1080_without_spaces_and_m_prefix() {
    assert_includes(
        "BluesBrothers2M1080.www.newpct.com.mkv",
        json!({ "title": "BluesBrothers2", "resolution": "1080p" }),
    );
}

/// should detect 1080 without spaces and BD prefix
#[test]
fn should_detect_1080_without_spaces_and_bd_prefix() {
    assert_includes("BenHurParte2BD1080.www.newpct.com.mkv", json!({ "title": "BenHurParte2", "resolution": "1080p" }));
}

/// should detect 720 without spaces title prefix
#[test]
fn should_detect_720_without_spaces_title_prefix() {
    assert_includes("1993720p_101_WWW.NEWPCT1.COM.mkvv", json!({ "resolution": "720p" }));
}

/// should detect resolution in brackets without p suffix
#[test]
fn should_detect_resolution_in_brackets_without_p_suffix() {
    assert_includes("[PALATA666] Resurrection 01 (720).mkv", json!({ "resolution": "720p" }));
    assert_missing("[PALATA666] Resurrection 01 (720).mkv", &["episodes"]);
}

/// should detect 1080p resolution when 4k remastered is present
#[test]
fn should_detect_1080p_resolution_when_4k_remastered_is_present() {
    assert_includes(
        "Batman Returns 1992 4K Remastered BluRay 1080p DTS AC3 x264-MgB",
        json!({ "resolution": "1080p" }),
    );
}

/// should not read a year numbered by a three digit session as a resolution
#[test]
fn should_not_read_a_year_numbered_by_a_three_digit_session_as_a_resolution() {
    assert_missing("Formula.1.2025x111.R21.BrazilianGP.Race.MULTi.SS", &["resolution"]);
}

/// should still read a cropped width and height resolution
#[test]
fn should_still_read_a_cropped_width_and_height_resolution() {
    assert_includes("[Arid] Mind Game [BDRip 1916x816 x264 FLAC]", json!({ "resolution": "816p" }));
}

/// should detect resolution with cyrillic p
#[test]
fn should_detect_resolution_with_cyrillic_p() {
    assert_includes("«Эрнест и Селестина» 1 сезон 05 серия 720р.mkv", json!({ "resolution": "720p" }));
    assert_includes("Show 1 сезон 05 серия 1080р.mkv", json!({ "resolution": "1080p" }));
}

/// should detect 1080p resolution from a FHD tag
#[test]
fn should_detect_1080p_resolution_from_a_fhd_tag() {
    assert_includes("[I.S.S.] Hunter Risvegliabestie Omegahorn 11 FHD", json!({ "resolution": "1080p" }));
}

/// should keep 1080p resolution when FHD precedes it
#[test]
fn should_keep_1080p_resolution_when_fhd_precedes_it() {
    assert_includes(
        "Metal Lords (2022)(FHD)(1080p)(x264)(WebDL)(MultiLang)(MultiSUB) PHDTeam",
        json!({ "resolution": "1080p", "title": "Metal Lords" }),
    );
}

/// should not detect resolution from a FHD group
#[test]
fn should_not_detect_resolution_from_a_fhd_group() {
    assert_includes("Meet.Joe.Black.1998.1080p.BluRay.x264-FHD", json!({ "resolution": "1080p", "group": "FHD" }));
}

/// should detect 480p resolution from a trailing SD tag
#[test]
fn should_detect_480p_resolution_from_a_trailing_sd_tag() {
    assert_includes("[I.S.S.] Hunter Risvegliabestie Omegahorn 11 SD", json!({ "resolution": "480p" }));
}

/// should detect 480p resolution from a SD tag before the extension
#[test]
fn should_detect_480p_resolution_from_a_sd_tag_before_the_extension() {
    assert_includes(
        "Formula.1.2025x04.Bahrain.Qualifying.SkyF1HD.SD",
        json!({ "resolution": "480p", "title": "Formula 1" }),
    );
}

/// should detect 480p resolution from a bracketed SD tag
#[test]
fn should_detect_480p_resolution_from_a_bracketed_sd_tag() {
    assert_includes(
        "I Cesaroni - Stagioni 01-03 (2006-2009 ITA) [SD] [WEB-DL] [direinde]",
        json!({ "resolution": "480p", "title": "I Cesaroni" }),
    );
}

/// should remove a leading bracketed SD tag from the title
#[test]
fn should_remove_a_leading_bracketed_sd_tag_from_the_title() {
    assert_includes(
        "(SD) Power Rangers Dino Charge S1 E02 (W4F).mp4",
        json!({ "resolution": "480p", "title": "Power Rangers Dino Charge" }),
    );
}

/// should not detect resolution from SD inside the title
#[test]
fn should_not_detect_resolution_from_sd_inside_the_title() {
    assert_includes("[Inka-Subs] Mobile Suit SD Gundam MK 3", json!({ "title": "Mobile Suit SD Gundam MK 3" }));
    assert_missing("[Inka-Subs] Mobile Suit SD Gundam MK 3", &["resolution"]);
}

/// should not detect resolution from a SD group
#[test]
fn should_not_detect_resolution_from_a_sd_group() {
    assert_includes("Falling Skies S01E07 SWESUB HDTV XviD-SD.avi", json!({ "group": "SD" }));
    assert_missing("Falling Skies S01E07 SWESUB HDTV XviD-SD.avi", &["resolution"]);
}

/// should not detect resolution from SD starting a title
#[test]
fn should_not_detect_resolution_from_sd_starting_a_title() {
    assert_includes("SD GUNDAM WORLD HEROES S01E03 XviD-AFG", json!({ "title": "SD GUNDAM WORLD HEROES" }));
    assert_missing("SD GUNDAM WORLD HEROES S01E03 XviD-AFG", &["resolution"]);
}

/// should detect 4k resolution when 1080p is added with a plus
#[test]
fn should_detect_4k_resolution_when_1080p_is_added_with_a_plus() {
    assert_includes(
        "[AI-Raws] Street Fighter II: The Animated Movie 4K+1080p MKV",
        json!({ "resolution": "4k", "title": "Street Fighter II: The Animated Movie" }),
    );
}
