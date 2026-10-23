// Ported from test/date.js of the js library.

//! Parsing date

use crate::common::*;

/// Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]
#[test]
fn stephen_colbert_2019_10_25_eddie_murphy_480p_x264_m_sd_eztv() {
    assert_includes("Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]", json!({ "date": "2019-10-25" }));
}

/// Stephen Colbert 25/10/2019 Eddie Murphy 480p x264-mSD [eztv]
#[test]
fn stephen_colbert_25_10_2019_eddie_murphy_480p_x264_m_sd_eztv() {
    assert_includes("Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]", json!({ "date": "2019-10-25" }));
}

/// Jimmy.Fallon.2020.02.14.Steve.Buscemi.WEB.x264-XLF[TGx]
#[test]
fn jimmy_fallon_2020_02_14_steve_buscemi_web_x264_xlf_tgx() {
    assert_includes("Jimmy.Fallon.2020.02.14.Steve.Buscemi.WEB.x264-XLF[TGx]", json!({ "date": "2020-02-14" }));
}

/// The Young And The Restless - S43 E10986 - 2016-08-12
#[test]
fn the_young_and_the_restless_s43_e10986_2016_08_12() {
    assert_includes("The Young And The Restless - S43 E10986 - 2016-08-12", json!({ "date": "2016-08-12" }));
}

/// Indias Best Dramebaaz 2 Ep 19 (13 Feb 2016) HDTV x264-AquoTube
#[test]
fn indias_best_dramebaaz_2_ep_19_13_feb_2016_hdtv_x264_aquo_tube() {
    assert_includes("Indias Best Dramebaaz 2 Ep 19 (13 Feb 2016) HDTV x264-AquoTube", json!({ "date": "2016-02-13" }));
}

/// 07 2015 YR/YR 07-06-15.mp4
#[test]
fn t_07_2015_yr_yr_07_06_15_mp4() {
    assert_includes("07 2015 YR/YR 07-06-15.mp4", json!({ "date": "2015-07-06" }));
}

/// SIX.S01E05.400p.229mb.hdtv.x264-][ Collateral ][ 16-Feb-2017 mp4
#[test]
fn six_s01_e05_400p_229mb_hdtv_x264_collateral_16_feb_2017_mp4() {
    assert_includes(
        "SIX.S01E05.400p.229mb.hdtv.x264-][ Collateral ][ 16-Feb-2017 mp4",
        json!({ "date": "2017-02-16" }),
    );
}

/// SIX.S01E05.400p.229mb.hdtv.x264-][ Collateral ][ 16-Feb-17 mp4
#[test]
fn six_s01_e05_400p_229mb_hdtv_x264_collateral_16_feb_17_mp4() {
    assert_includes("SIX.S01E05.400p.229mb.hdtv.x264-][ Collateral ][ 16-Feb-17 mp4", json!({ "date": "2017-02-16" }));
}

/// WWE Smackdown - 11/21/17 - 21st November 2017 - Full Show
#[test]
fn wwe_smackdown_11_21_17_21st_november_2017_full_show() {
    assert_includes("WWE Smackdown - 11/21/17 - 21st November 2017 - Full Show", json!({ "date": "2017-11-21" }));
}

/// WWE Smackdown - 11/21/17 - 21st November 2017 - Full Show
#[test]
fn wwe_smackdown_11_21_17_21st_november_2017_full_show_2() {
    assert_includes("WWE Smackdown - 11/21/17 - 21st November 2017 - Full Show", json!({ "date": "2017-11-21" }));
}

/// WWE RAW 9th Dec 2019 WEBRip h264-TJ [TJET]
#[test]
fn wwe_raw_9th_dec_2019_webrip_h264_tj_tjet() {
    assert_includes("WWE RAW 9th Dec 2019 WEBRip h264-TJ [TJET]", json!({ "date": "2019-12-09" }));
}

/// WWE RAW 1st Dec 2019 WEBRip h264-TJ [TJET]
#[test]
fn wwe_raw_1st_dec_2019_webrip_h264_tj_tjet() {
    assert_includes("WWE RAW 1st Dec 2019 WEBRip h264-TJ [TJET]", json!({ "date": "2019-12-01" }));
}

/// WWE RAW 2nd Dec 2019 WEBRip h264-TJ [TJET]
#[test]
fn wwe_raw_2nd_dec_2019_webrip_h264_tj_tjet() {
    assert_includes("WWE RAW 2nd Dec 2019 WEBRip h264-TJ [TJET]", json!({ "date": "2019-12-02" }));
}

/// WWE RAW 3rd Dec 2019 WEBRip h264-TJ [TJET]
#[test]
fn wwe_raw_3rd_dec_2019_webrip_h264_tj_tjet() {
    assert_includes("WWE RAW 3rd Dec 2019 WEBRip h264-TJ [TJET]", json!({ "date": "2019-12-03" }));
}

/// EastEnders_20200116_19302000.mp4
#[test]
fn east_enders_20200116_19302000_mp4() {
    assert_includes("EastEnders_20200116_19302000.mp4", json!({ "date": "2020-01-16" }));
}

/// AEW DARK 4th December 2020 WEBRip h264-TJ
#[test]
fn aew_dark_4th_december_2020_webrip_h264_tj() {
    assert_includes("AEW DARK 4th December 2020 WEBRip h264-TJ", json!({ "date": "2020-12-04" }));
}

/// AEW DARK 4th November 2020 WEBRip h264-TJ
#[test]
fn aew_dark_4th_november_2020_webrip_h264_tj() {
    assert_includes("AEW DARK 4th November 2020 WEBRip h264-TJ", json!({ "date": "2020-11-04" }));
}

/// AEW DARK 4th October 2020 WEBRip h264-TJ
#[test]
fn aew_dark_4th_october_2020_webrip_h264_tj() {
    assert_includes("AEW DARK 4th October 2020 WEBRip h264-TJ", json!({ "date": "2020-10-04" }));
}

/// WWE NXT 30th Sept 2020 WEBRip h264-TJ
#[test]
fn wwe_nxt_30th_sept_2020_webrip_h264_tj() {
    assert_includes("WWE NXT 30th Sept 2020 WEBRip h264-TJ", json!({ "date": "2020-09-30" }));
}

/// AEW DARK 4th September 2020 WEBRip h264-TJ
#[test]
fn aew_dark_4th_september_2020_webrip_h264_tj() {
    assert_includes("AEW DARK 4th September 2020 WEBRip h264-TJ", json!({ "date": "2020-09-04" }));
}

/// WWE Main Event 6th August 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_6th_august_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 6th August 2020 WEBRip h264-TJ", json!({ "date": "2020-08-06" }));
}

/// WWE Main Event 4th July 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_4th_july_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 4th July 2020 WEBRip h264-TJ", json!({ "date": "2020-07-04" }));
}

/// WWE Main Event 4th June 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_4th_june_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 4th June 2020 WEBRip h264-TJ", json!({ "date": "2020-06-04" }));
}

/// WWE Main Event 4th May 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_4th_may_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 4th May 2020 WEBRip h264-TJ", json!({ "date": "2020-05-04" }));
}

/// WWE Main Event 4th April 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_4th_april_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 4th April 2020 WEBRip h264-TJ", json!({ "date": "2020-04-04" }));
}

/// WWE Main Event 3rd March 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_3rd_march_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 3rd March 2020 WEBRip h264-TJ", json!({ "date": "2020-03-03" }));
}

/// WWE Main Event 2nd February 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_2nd_february_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 2nd February 2020 WEBRip h264-TJ", json!({ "date": "2020-02-02" }));
}

/// WWE Main Event 1st January 2020 WEBRip h264-TJ
#[test]
fn wwe_main_event_1st_january_2020_webrip_h264_tj() {
    assert_includes("WWE Main Event 1st January 2020 WEBRip h264-TJ", json!({ "date": "2020-01-01" }));
}

/// wwf.raw.is.war.18.09.00.avi
#[test]
fn wwf_raw_is_war_18_09_00_avi() {
    assert_includes("wwf.raw.is.war.18.09.00.avi", json!({ "date": "2000-09-18" }));
}

/// The Colbert Report - 10-30-2010 - Rally to Restore Sanity and or Fear.avi
#[test]
fn the_colbert_report_10_30_2010_rally_to_restore_sanity_and_or_fear_avi() {
    assert_includes(
        "The Colbert Report - 10-30-2010 - Rally to Restore Sanity and or Fear.avi",
        json!({ "date": "2010-10-30" }),
    );
}

/// should not detect date from series title
#[test]
fn should_not_detect_date_from_series_title() {
    assert_missing("11 22 63 - Temporada 1 [HDTV][Cap.103][Español Castellano]", &["date"]);
}

/// should not detect date from movie title
#[test]
fn should_not_detect_date_from_movie_title() {
    assert_missing("September 30 1955 1977 1080p BluRay", &["date"]);
}

/// should not detect date from movie title v2
#[test]
fn should_not_detect_date_from_movie_title_v2() {
    assert_missing("11-11-11.2011.1080p.BluRay.x264.DTS-FGT", &["date"]);
}

/// should detect date with mixed separators
#[test]
fn should_detect_date_with_mixed_separators() {
    assert_includes("2015 07-20- Daily Show- Paul Rudd [EXTENDED].mp4", json!({ "date": "2015-07-20" }));
    assert_missing("2015 07-20- Daily Show- Paul Rudd [EXTENDED].mp4", &["episodes"]);
}

/// should not detect date with mixed separators when it's year and episode range
#[test]
fn should_not_detect_date_with_mixed_separators_when_it_s_year_and_episode_range() {
    assert_missing("[NoobSubs] Berserk 2016 01-12 (720p Blu-ray 8bit AAC MP4)", &["date"]);
    assert_includes("[NoobSubs] Berserk 2016 01-12 (720p Blu-ray 8bit AAC MP4)", json!({ "year": 2016 }));
}
