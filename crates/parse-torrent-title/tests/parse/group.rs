// Ported from test/group.js of the js library.

//! Parsing group

use crate::common::*;

/// should detect the HD2 group correctly
#[test]
fn should_detect_the_hd2_group_correctly() {
    assert_includes("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", json!({ "group": "HD2" }));
}

/// should detect the HDH group correctly
#[test]
fn should_detect_the_hdh_group_correctly() {
    assert_includes("Gold 2016 1080p BluRay DTS-HD MA 5 1 x264-HDH", json!({ "group": "HDH" }));
}

/// should detect the YIFY group correctly
#[test]
fn should_detect_the_yify_group_correctly() {
    assert_includes("Hercules (2014) 1080p BrRip H264 - YIFY", json!({ "group": "YIFY" }));
}

/// should detect group before container file type
#[test]
fn should_detect_group_before_container_file_type() {
    assert_includes("The.Expanse.S05E02.720p.WEB.x264-Worldmkv.mkv", json!({ "group": "Worldmkv" }));
}

/// should detect group with site source tag
#[test]
fn should_detect_group_with_site_source_tag() {
    assert_includes("The.Expanse.S05E02.PROPER.720p.WEB.h264-KOGi[rartv]", json!({ "group": "KOGi" }));
}

/// should detect group with site source tag before container file type
#[test]
fn should_detect_group_with_site_source_tag_before_container_file_type() {
    assert_includes("The.Expanse.S05E02.1080p.AMZN.WEB.DDP5.1.x264-NTb[eztv.re].mp4", json!({ "group": "NTb" }));
}

/// should detect when there is no group
#[test]
fn should_detect_when_there_is_no_group() {
    assert_missing("Western - L'homme qui n'a pas d'étoile-1955.Multi.DVD9", &["group"]);
}

/// should detect when there is no group with hyphen separator
#[test]
fn should_detect_when_there_is_no_group_with_hyphen_separator() {
    assert_missing("Power (2014) - S02E03.mp4", &["group"]);
}

/// should detect when there is no group with hyphen separator and no container
#[test]
fn should_detect_when_there_is_no_group_with_hyphen_separator_and_no_container() {
    assert_missing("Power (2014) - S02E03", &["group"]);
}

/// should detect when there is no group when it is episode
#[test]
fn should_detect_when_there_is_no_group_when_it_is_episode() {
    assert_missing("3-Nen D-Gumi Glass no Kamen - 13", &["group"]);
}

/// should detect when there is no group when it is ep symbol
#[test]
fn should_detect_when_there_is_no_group_when_it_is_ep_symbol() {
    assert_missing("3-Nen D-Gumi Glass no Kamen - Ep13", &["group"]);
}

/// should detect anime group in brackets
#[test]
fn should_detect_anime_group_in_brackets() {
    assert_includes("[AnimeRG] One Punch Man - 09 [720p].mkv", json!({ "group": "AnimeRG" }));
}

/// should detect anime group in brackets with underscores
#[test]
fn should_detect_anime_group_in_brackets_with_underscores() {
    assert_includes("[Mazui]_Hyouka_-_03_[DF5E813A].mkv", json!({ "group": "Mazui" }));
}

/// should detect anime group in brackets with numbers
#[test]
fn should_detect_anime_group_in_brackets_with_numbers() {
    assert_includes("[H3] Hunter x Hunter - 38 [1280x720] [x264]", json!({ "group": "H3" }));
}

/// should detect anime group in brackets with spaces
#[test]
fn should_detect_anime_group_in_brackets_with_spaces() {
    assert_includes("[KNK E MMS Fansubs] Nisekoi - 20 Final [PT-BR].mkv", json!({ "group": "KNK E MMS Fansubs" }));
}

/// should detect anime group in brackets when bracket part exist at the end
#[test]
fn should_detect_anime_group_in_brackets_when_bracket_part_exist_at_the_end() {
    assert_includes(
        "[ToonsHub] JUJUTSU KAISEN - S02E01 (Japanese 2160p x264 AAC) [Multi-Subs].mkv",
        json!({ "group": "ToonsHub" }),
    );
}

/// should detect anime group in brackets with a link
#[test]
fn should_detect_anime_group_in_brackets_with_a_link() {
    assert_includes(
        "[HD-ELITE.NET] -  The.Art.Of.The.Steal.2014.DVDRip.XviD.Dual.Aud",
        json!({ "group": "HD-ELITE.NET" }),
    );
}

/// should not detect brackets group when group is detected at the end of title
#[test]
fn should_not_detect_brackets_group_when_group_is_detected_at_the_end_of_title() {
    assert_includes("[Russ]Lords.Of.London.2014.XviD.H264.AC3-BladeBDP", json!({ "group": "BladeBDP" }));
}

/// should detect group in parenthesis
#[test]
fn should_detect_group_in_parenthesis() {
    assert_includes(
        "Jujutsu Kaisen S02E01 2160p WEB H.265 AAC -Tsundere-Raws (B-Global).mkv",
        json!({ "group": "B-Global" }),
    );
}

/// should not detect brackets group when it contains other parsed parameters
#[test]
fn should_not_detect_brackets_group_when_it_contains_other_parsed_parameters() {
    assert_missing("[DVD-RIP] Kaavalan (2011) Sruthi XVID [700Mb] [TCHellRaiser]", &["group"]);
}

/// should not detect brackets group when it contains other parsed parameters
#[test]
fn should_not_detect_brackets_group_when_it_contains_other_parsed_parameters_2() {
    assert_missing("[DVD-RIP] Kaavalan (2011) Sruthi XVID [700Mb] [TCHellRaiser]", &["group"]);
}

/// should not detect brackets group when it contains other parsed parameters for series
#[test]
fn should_not_detect_brackets_group_when_it_contains_other_parsed_parameters_for_series() {
    assert_missing("[DvdMux - XviD - Ita Mp3 Eng Ac3 - Sub Ita Eng] Sanctuary S01e01", &["group"]);
}

/// should not detect group from episode
#[test]
fn should_not_detect_group_from_episode() {
    assert_missing("the-x-files-502.mkv", &["group"]);
}

/// should not detect a dash separated numbered special as group
#[test]
fn should_not_detect_a_dash_separated_numbered_special_as_group() {
    assert_includes("[Judas] High School DxD S1 - SP01.mkv", json!({ "group": "Judas" }));
}

/// should not detect a dash separated ova as group after a bracket group
#[test]
fn should_not_detect_a_dash_separated_ova_as_group_after_a_bracket_group() {
    assert_includes("[Judas] High School DxD S3 - OVA.mkv", json!({ "group": "Judas" }));
}

/// should not detect a dash separated ova as group
#[test]
fn should_not_detect_a_dash_separated_ova_as_group() {
    assert_missing("Girls Und Panzer - OVA.mkv", &["group"]);
}

/// should not detect a chinese episode marker as group
#[test]
fn should_not_detect_a_chinese_episode_marker_as_group() {
    assert_missing("Soul Land - S14E04 - 第225集.mkv", &["group"]);
}

/// should detect group from a bracket closed with an opening bracket
#[test]
fn should_detect_group_from_a_bracket_closed_with_an_opening_bracket() {
    assert_includes(
        "[Anime Time[ Attack On Titan OAD - Lost Girls 01.mkv",
        json!({ "group": "Anime Time", "title": "Attack On Titan OAD - Lost Girls", "episodes": [1] }),
    );
}

/// should not detect a group in a bare file name with a dashed subtitle
#[test]
fn should_not_detect_a_group_in_a_bare_file_name_with_a_dashed_subtitle() {
    assert_missing("Spider-Man - Homecoming.mkv", &["group"]);
}

/// should detect a group right after a removed codec
#[test]
fn should_detect_a_group_right_after_a_removed_codec() {
    assert_includes("BKFC.33.WEB.x264-HEADKICK.mp4", json!({ "group": "HEADKICK" }));
}

/// should detect a group after a bracketed 4k
#[test]
fn should_detect_a_group_after_a_bracketed_4k() {
    assert_includes(
        "[4K][DBD-Raws][魔法少女奈叶剧场版 2nd A's/Mahou Shoujo Lyrical Nanoha The Movie 2nd A's][2160P][BDRip][HEVC-10bit][简繁外挂][FLAC+DTS][MKV](魔法少女リリカルなのはThe MOVIE 2nd A's)",
        json!({ "group": "DBD-Raws" }),
    );
}
