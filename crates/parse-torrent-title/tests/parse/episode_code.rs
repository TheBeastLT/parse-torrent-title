// Ported from test/episodeCode.js of the js library.

//! Parsing episode code

use crate::common::*;

/// [Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39].mkv
#[test]
fn golumpa_fairy_tail_214_funi_dub_720p_x264_aac_5_e46_ac39_mkv() {
    assert_includes(
        "[Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39].mkv",
        json!({ "episodeCode": "5E46AC39" }),
    );
}

/// [Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv
#[test]
fn exiled_destiny_tokyo_underground_ep02v2_41858470_mkv() {
    assert_includes("[Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv", json!({ "episodeCode": "41858470" }));
}

/// [ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv
#[test]
fn acx_el_cazador_de_la_bruja_19_a_man_who_protects_ssj_saiyan_elite_9_e199846_mkv() {
    assert_includes(
        "[ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv",
        json!({ "episodeCode": "9E199846" }),
    );
}

/// [CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]
#[test]
fn cbm_medaka_box_11_this_is_the_end_720p_436_e0_e90() {
    assert_includes(
        "[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]",
        json!({ "episodeCode": "436E0E90" }),
    );
}

/// Gankutsuou.-.The.Count.Of.Monte.Cristo[2005].-.04.-.[720p.BD.HEVC.x265].[FLAC].[Jd].[DHD].[b6e6e648].mkv
#[test]
fn gankutsuou_the_count_of_monte_cristo_2005_04_720p_bd_hevc_x265_flac_jd_dhd_b6e6e648_mkv() {
    assert_includes(
        "Gankutsuou.-.The.Count.Of.Monte.Cristo[2005].-.04.-.[720p.BD.HEVC.x265].[FLAC].[Jd].[DHD].[b6e6e648].mkv",
        json!({ "episodeCode": "B6E6E648" }),
    );
}

/// [D0ugyB0y] Nanatsu no Taizai Fundo no Shinpan - 01 (1080p WEB NF x264 AAC[9CC04E06]).mkv
#[test]
fn d0ugy_b0y_nanatsu_no_taizai_fundo_no_shinpan_01_1080p_web_nf_x264_aac_9_cc04_e06_mkv() {
    assert_includes(
        "[D0ugyB0y] Nanatsu no Taizai Fundo no Shinpan - 01 (1080p WEB NF x264 AAC[9CC04E06]).mkv",
        json!({ "episodeCode": "9CC04E06" }),
    );
}

/// should not detect episode code not at the end
#[test]
fn should_not_detect_episode_code_not_at_the_end() {
    assert_missing("Lost.[Perdidos].6x05.HDTV.XviD.[www.DivxTotaL.com].avi", &["episodeCode"]);
}

/// should not detect episode code when it's a word
#[test]
fn should_not_detect_episode_code_when_it_s_a_word() {
    assert_missing("Lost - Stagioni 01-06 (2004-2010) [COMPLETA] SD x264 AAC ITA SUB ITA", &["episodeCode"]);
}

/// should not detect episode code when it's only numbers
#[test]
fn should_not_detect_episode_code_when_it_s_only_numbers() {
    assert_missing("The voice of Holland S05E08 [20141017]  NL Battles 1.mp4", &["episodeCode"]);
}
