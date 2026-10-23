// Ported from test/volume.js of the js library.

//! Parsing volume

use crate::common::*;

/// [MTBB] Sword Art Onlineː Alicization - Volume 2 (BD 1080p)
#[test]
fn mtbb_sword_art_online_alicization_volume_2_bd_1080p() {
    assert_includes("[MTBB] Sword Art Onlineː Alicization - Volume 2 (BD 1080p)", json!({ "volumes": [2] }));
}

/// [Neutrinome] Sword Art Online Alicization Vol.2 - VOSTFR [1080p BDRemux] + DDL
#[test]
fn neutrinome_sword_art_online_alicization_vol_2_vostfr_1080p_bdremux_ddl() {
    assert_includes(
        "[Neutrinome] Sword Art Online Alicization Vol.2 - VOSTFR [1080p BDRemux] + DDL",
        json!({ "volumes": [2] }),
    );
}

/// [Mr. Kimiko] Oh My Goddess! - Vol. 7 [Kobo][2048px][CBZ]
#[test]
fn mr_kimiko_oh_my_goddess_vol_7_kobo_2048px_cbz() {
    assert_includes("[Mr. Kimiko] Oh My Goddess! - Vol. 7 [Kobo][2048px][CBZ]", json!({ "volumes": [7] }));
}

/// [MTBB] Cross Game - Volume 1-3 (WEB 720p)
#[test]
fn mtbb_cross_game_volume_1_3_web_720p() {
    assert_includes("[MTBB] Cross Game - Volume 1-3 (WEB 720p)", json!({ "volumes": [1, 2, 3] }));
}

/// PIXAR SHORT FILMS COLLECTION - VOLS. 1 & 2 + - BDrip 1080p
#[test]
fn pixar_short_films_collection_vols_1_2_bdrip_1080p() {
    assert_includes("PIXAR SHORT FILMS COLLECTION - VOLS. 1 & 2 + - BDrip 1080p", json!({ "volumes": [1, 2] }));
}

/// Altair - A Record of Battles Vol. 01-08 (Digital) (danke-Empire)
#[test]
fn altair_a_record_of_battles_vol_01_08_digital_danke_empire() {
    assert_includes(
        "Altair - A Record of Battles Vol. 01-08 (Digital) (danke-Empire)",
        json!({ "volumes": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
}

/// Guardians of the Galaxy Vol. 2 (2017) 720p HDTC x264 MKVTV
#[test]
fn guardians_of_the_galaxy_vol_2_2017_720p_hdtc_x264_mkvtv() {
    assert_includes(
        "Guardians of the Galaxy Vol. 2 (2017) 720p HDTC x264 MKVTV",
        json!({ "title": "Guardians of the Galaxy Vol. 2" }),
    );
    assert_missing("Guardians of the Galaxy Vol. 2 (2017) 720p HDTC x264 MKVTV", &["volumes"]);
}

/// Kill Bill: Vol. 1 (2003) BluRay 1080p 5.1CH x264 Ganool
#[test]
fn kill_bill_vol_1_2003_blu_ray_1080p_5_1_ch_x264_ganool() {
    assert_includes("Kill Bill: Vol. 1 (2003) BluRay 1080p 5.1CH x264 Ganool", json!({ "title": "Kill Bill: Vol. 1" }));
    assert_missing("Kill Bill: Vol. 1 (2003) BluRay 1080p 5.1CH x264 Ganool", &["volumes"]);
}

/// [Valenciano] Aquarion EVOL - 22 [1080p][AV1 10bit][FLAC][Eng sub].mkv
#[test]
fn valenciano_aquarion_evol_22_1080p_av1_10bit_flac_eng_sub_mkv() {
    assert_includes(
        "[Valenciano] Aquarion EVOL - 22 [1080p][AV1 10bit][FLAC][Eng sub].mkv",
        json!({ "title": "Aquarion EVOL" }),
    );
    assert_missing("[Valenciano] Aquarion EVOL - 22 [1080p][AV1 10bit][FLAC][Eng sub].mkv", &["volumes"]);
}
