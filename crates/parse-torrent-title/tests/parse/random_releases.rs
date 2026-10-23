// Ported from test/main.js of the js library.

//! Random releases

use crate::common::*;

/// sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR
#[test]
fn sons_of_anarchy_s05e10_480p_blu_ray_x264_gan_gste_r() {
    assert_equals(
        "sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR",
        json!({ "title": "sons of anarchy", "resolution": "480p", "seasons": [5], "season": 5, "episodes": [10], "episode": 10, "source": "BluRay", "codec": "x264", "group": "GAnGSteR" }),
    );
}

/// Color.Of.Night.Unrated.DC.VostFR.BRrip.x264
#[test]
fn color_of_night_unrated_dc_vost_fr_brrip_x264() {
    assert_equals(
        "Color.Of.Night.Unrated.DC.VostFR.BRrip.x264",
        json!({ "title": "Color Of Night", "unrated": true, "languages": ["french"], "source": "BRRip", "codec": "x264" }),
    );
}

/// Da Vinci Code DVDRip
#[test]
fn da_vinci_code_dvdrip() {
    assert_equals("Da Vinci Code DVDRip", json!({ "title": "Da Vinci Code", "source": "DVDRip" }));
}

/// Some.girls.1998.DVDRip
#[test]
fn some_girls_1998_dvdrip() {
    assert_equals("Some.girls.1998.DVDRip", json!({ "title": "Some girls", "source": "DVDRip", "year": 1998 }));
}

/// Ecrit.Dans.Le.Ciel.1954.MULTI.DVDRIP.x264.AC3-gismo65
#[test]
fn ecrit_dans_le_ciel_1954_multi_dvdrip_x264_ac3_gismo65() {
    assert_equals(
        "Ecrit.Dans.Le.Ciel.1954.MULTI.DVDRIP.x264.AC3-gismo65",
        json!({ "title": "Ecrit Dans Le Ciel", "source": "DVDRip", "year": 1954, "languages": ["multi audio"], "dubbed": true, "codec": "x264", "audio": "ac3", "group": "gismo65" }),
    );
}

/// 2019 After The Fall Of New York 1983 REMASTERED BDRip x264-GHOULS
#[test]
fn t_2019_after_the_fall_of_new_york_1983_remastered_bdrip_x264_ghouls() {
    assert_equals(
        "2019 After The Fall Of New York 1983 REMASTERED BDRip x264-GHOULS",
        json!({ "title": "2019 After The Fall Of New York", "source": "BDRip", "remastered": true, "year": 1983, "codec": "x264", "group": "GHOULS" }),
    );
}

/// Ghost In The Shell 2017 720p HC HDRip X264 AC3-EVO
#[test]
fn ghost_in_the_shell_2017_720p_hc_hdrip_x264_ac3_evo() {
    assert_equals(
        "Ghost In The Shell 2017 720p HC HDRip X264 AC3-EVO",
        json!({ "title": "Ghost In The Shell", "source": "HDRip", "hardcoded": true, "year": 2017, "resolution": "720p", "codec": "x264", "audio": "ac3", "group": "EVO" }),
    );
}

/// Rogue One 2016 1080p BluRay x264-SPARKS
#[test]
fn rogue_one_2016_1080p_blu_ray_x264_sparks() {
    assert_equals(
        "Rogue One 2016 1080p BluRay x264-SPARKS",
        json!({ "title": "Rogue One", "source": "BluRay", "year": 2016, "resolution": "1080p", "codec": "x264", "group": "SPARKS" }),
    );
}

/// Desperation 2006 Multi Pal DvdR9-TBW1973
#[test]
fn desperation_2006_multi_pal_dvd_r9_tbw1973() {
    assert_equals(
        "Desperation 2006 Multi Pal DvdR9-TBW1973",
        json!({ "title": "Desperation", "source": "DVD", "year": 2006, "languages": ["multi audio"], "dubbed": true, "region": "R9", "group": "TBW1973" }),
    );
}

/// Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG
#[test]
fn maman_j_ai_rate_l_avion_1990_vfi_1080p_blu_ray_dts_x265_htg() {
    assert_equals(
        "Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG",
        json!({ "title": "Maman, j'ai raté l'avion", "source": "BluRay", "year": 1990, "audio": "dts", "resolution": "1080p", "languages": ["french"], "codec": "x265", "group": "HTG" }),
    );
}

/// Game of Thrones - The Complete Season 3 [HDTV]
#[test]
fn game_of_thrones_the_complete_season_3_hdtv() {
    assert_equals(
        "Game of Thrones - The Complete Season 3 [HDTV]",
        json!({ "title": "Game of Thrones", "seasons": [3], "season": 3, "source": "HDTV" }),
    );
}

/// The Sopranos: The Complete Series (Season 1,2,3,4,5&6) + Extras
#[test]
fn the_sopranos_the_complete_series_season_1_2_3_4_5_6_extras() {
    assert_equals(
        "The Sopranos: The Complete Series (Season 1,2,3,4,5&6) + Extras",
        json!({ "title": "The Sopranos", "seasons": [1, 2, 3, 4, 5, 6], "complete": true, "extras": true }),
    );
}

/// Skins Season S01-S07 COMPLETE UK Soundtrack 720p WEB-DL
#[test]
fn skins_season_s01_s07_complete_uk_soundtrack_720p_web_dl() {
    assert_equals(
        "Skins Season S01-S07 COMPLETE UK Soundtrack 720p WEB-DL",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7], "title": "Skins", "resolution": "720p", "source": "WEB-DL" }),
    );
}

/// Futurama.COMPLETE.S01-S07.720p.BluRay.x265-HETeam
#[test]
fn futurama_complete_s01_s07_720p_blu_ray_x265_heteam() {
    assert_equals(
        "Futurama.COMPLETE.S01-S07.720p.BluRay.x265-HETeam",
        json!({ "title": "Futurama", "seasons": [1, 2, 3, 4, 5, 6, 7], "resolution": "720p", "source": "BluRay", "codec": "x265", "group": "HETeam" }),
    );
}

/// You.[Uncut].S01.SweSub.1080p.x264-Justiso
#[test]
fn you_uncut_s01_swe_sub_1080p_x264_justiso() {
    assert_equals(
        "You.[Uncut].S01.SweSub.1080p.x264-Justiso",
        json!({ "title": "You", "uncut": true, "seasons": [1], "season": 1, "languages": ["swedish"], "resolution": "1080p", "codec": "x264", "group": "Justiso" }),
    );
}

/// Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]
#[test]
fn stephen_colbert_2019_10_25_eddie_murphy_480p_x264_m_sd_eztv() {
    assert_equals(
        "Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]",
        json!({ "title": "Stephen Colbert", "date": "2019-10-25", "resolution": "480p", "codec": "x264" }),
    );
}

/// House MD Season 7 Complete MKV
#[test]
fn house_md_season_7_complete_mkv() {
    assert_equals(
        "House MD Season 7 Complete MKV",
        json!({ "title": "House MD", "season": 7, "seasons": [7], "container": "mkv" }),
    );
}

/// 2008 The Incredible Hulk Feature Film.mp4
#[test]
fn t_2008_the_incredible_hulk_feature_film_mp4() {
    assert_equals(
        "2008 The Incredible Hulk Feature Film.mp4",
        json!({ "title": "The Incredible Hulk Feature Film", "year": 2008, "container": "mp4", "extension": "mp4" }),
    );
}

/// 【4月/悠哈璃羽字幕社】[UHA-WINGS][不要输！恶之军团][Makeruna!! Aku no Gundan!][04][1080p AVC_AAC][简繁外挂][sc_tc]
#[test]
fn t_4_uha_wings_makeruna_aku_no_gundan_04_1080p_avc_aac_sc_tc() {
    assert_equals(
        "【4月/悠哈璃羽字幕社】[UHA-WINGS][不要输！恶之军团][Makeruna!! Aku no Gundan!][04][1080p AVC_AAC][简繁外挂][sc_tc]",
        json!({ "title": "Makeruna!! Aku no Gundan!", "episodes": [4], "episode": 4, "resolution": "1080p", "codec": "avc", "audio": "aac" }),
    );
}

/// [GM-Team][国漫][西行纪之集结篇][The Westward Ⅱ][2019][17][AVC][GB][1080P]
#[test]
fn gm_team_the_westward_ii_2019_17_avc_gb_1080_p() {
    assert_equals(
        "[GM-Team][国漫][西行纪之集结篇][The Westward Ⅱ][2019][17][AVC][GB][1080P]",
        json!({ "title": "The Westward Ⅱ", "year": 2019, "episodes": [17], "episode": 17, "resolution": "1080p", "codec": "avc", "group": "GM-Team" }),
    );
}

/// Черное зеркало / Black Mirror / Сезон 4 / Серии 1-6 (6) [2017, США, WEBRip 1080p] MVO + Eng Sub
#[test]
fn black_mirror_4_1_6_6_2017_webrip_1080p_mvo_eng_sub() {
    assert_equals(
        "Черное зеркало / Black Mirror / Сезон 4 / Серии 1-6 (6) [2017, США, WEBRip 1080p] MVO + Eng Sub",
        json!({ "title": "Black Mirror", "year": 2017, "seasons": [4], "season": 4, "episodes": [1, 2, 3, 4, 5, 6], "languages": ["english"], "resolution": "1080p", "source": "WEBRip" }),
    );
}

/// [neoHEVC] Student Council's Discretion / Seitokai no Ichizon [Season 1] [BD 1080p x265 HEVC AAC]
#[test]
fn neo_hevc_student_council_s_discretion_seitokai_no_ichizon_season_1_bd_1080p_x265_hevc_aac() {
    assert_equals(
        "[neoHEVC] Student Council's Discretion / Seitokai no Ichizon [Season 1] [BD 1080p x265 HEVC AAC]",
        json!({ "title": "Student Council's Discretion / Seitokai no Ichizon", "seasons": [1], "season": 1, "resolution": "1080p", "source": "BDRip", "audio": "aac", "codec": "hevc", "group": "neoHEVC" }),
    );
}

/// [Commie] Chihayafuru 3 - 21 [BD 720p AAC] [5F1911ED].mkv
#[test]
fn commie_chihayafuru_3_21_bd_720p_aac_5_f1911_ed_mkv() {
    assert_equals(
        "[Commie] Chihayafuru 3 - 21 [BD 720p AAC] [5F1911ED].mkv",
        json!({ "title": "Chihayafuru 3", "episodes": [21], "episode": 21, "resolution": "720p", "source": "BDRip", "audio": "aac", "container": "mkv", "extension": "mkv", "episodeCode": "5F1911ED", "group": "Commie" }),
    );
}

/// [DVDRip-ITA]The Fast and the Furious: Tokyo Drift [CR-Bt]
#[test]
fn dvdrip_ita_the_fast_and_the_furious_tokyo_drift_cr_bt() {
    assert_equals(
        "[DVDRip-ITA]The Fast and the Furious: Tokyo Drift [CR-Bt]",
        json!({ "title": "The Fast and the Furious: Tokyo Drift", "source": "DVDRip", "languages": ["italian"] }),
    );
}

/// [BluRay Rip 720p ITA AC3 - ENG AC3 SUB] Hostel[2005]-LIFE[ultimafrontiera]
#[test]
fn blu_ray_rip_720p_ita_ac3_eng_ac3_sub_hostel_2005_life_ultimafrontiera() {
    assert_equals(
        "[BluRay Rip 720p ITA AC3 - ENG AC3 SUB] Hostel[2005]-LIFE[ultimafrontiera]",
        json!({ "title": "Hostel", "year": 2005, "resolution": "720p", "source": "BRRip", "audio": "ac3", "languages": ["english", "italian"], "group": "LIFE" }),
    );
}

/// [OFFICIAL ENG SUB] Soul Land Episode 121-125 [1080p][Soft Sub][Web-DL][Douluo Dalu][斗罗大陆]
#[test]
fn official_eng_sub_soul_land_episode_121_125_1080p_soft_sub_web_dl_douluo_dalu() {
    assert_equals(
        "[OFFICIAL ENG SUB] Soul Land Episode 121-125 [1080p][Soft Sub][Web-DL][Douluo Dalu][斗罗大陆]",
        json!({ "title": "Soul Land", "episodes": [121, 122, 123, 124, 125], "resolution": "1080p", "source": "WEB-DL", "languages": ["english"] }),
    );
}

/// [720p] The God of Highschool Season 1
#[test]
fn t_720p_the_god_of_highschool_season_1() {
    assert_equals(
        "[720p] The God of Highschool Season 1",
        json!({ "title": "The God of Highschool", "seasons": [1], "season": 1, "resolution": "720p" }),
    );
}

/// Heidi Audio Latino DVDRip [cap. 3 Al 18]
#[test]
fn heidi_audio_latino_dvdrip_cap_3_al_18() {
    assert_equals(
        "Heidi Audio Latino DVDRip [cap. 3 Al 18]",
        json!({ "title": "Heidi", "episodes": [3], "episode": 3, "source": "DVDRip", "languages": ["latino"] }),
    );
}

/// Sprint.2024.S01.COMPLETE.1080p.WEB.h264-EDITH[TGx]
#[test]
fn sprint_2024_s01_complete_1080p_web_h264_edith_tgx() {
    assert_equals(
        "Sprint.2024.S01.COMPLETE.1080p.WEB.h264-EDITH[TGx]",
        json!({ "title": "Sprint", "year": 2024, "seasons": [1], "season": 1, "resolution": "1080p", "source": "WEB", "codec": "h264", "group": "EDITH" }),
    );
}

/// High Heat *2022* [BRRip.XviD] Lektor PL
#[test]
fn high_heat_2022_brrip_xvi_d_lektor_pl() {
    assert_equals(
        "High Heat *2022* [BRRip.XviD] Lektor PL",
        json!({ "title": "High Heat", "year": 2022, "codec": "xvid", "source": "BRRip", "languages": ["polish"] }),
    );
}

/// Ghost Busters *1984-2021* [720p] [BDRip] [AC-3] [XviD] [Lektor + Dubbing PL] [DYZIO]
#[test]
fn ghost_busters_1984_2021_720p_bdrip_ac_3_xvi_d_lektor_dubbing_pl_dyzio() {
    assert_equals(
        "Ghost Busters *1984-2021* [720p] [BDRip] [AC-3] [XviD] [Lektor + Dubbing PL] [DYZIO]",
        json!({ "title": "Ghost Busters", "year": "1984-2021", "resolution": "720p", "audio": "ac3", "codec": "xvid", "source": "BDRip", "languages": ["polish"] }),
    );
}

/// 20-20.2024.11.15.Fatal.Disguise.XviD-AFG[EZTVx.to].avi
#[test]
fn t_20_20_2024_11_15_fatal_disguise_xvi_d_afg_eztvx_to_avi() {
    assert_equals(
        "20-20.2024.11.15.Fatal.Disguise.XviD-AFG[EZTVx.to].avi",
        json!({ "title": "20-20", "date": "2024-11-15", "codec": "xvid", "source": "XviD", "group": "AFG", "container": "avi", "extension": "avi" }),
    );
}
