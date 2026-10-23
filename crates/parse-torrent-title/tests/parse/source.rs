// Ported from test/source.js of the js library.

//! Parsing source

use crate::common::*;

/// should detect the Bluray source correctly
#[test]
fn should_detect_the_bluray_source_correctly() {
    assert_includes("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", json!({ "source": "BluRay" }));
}

/// should detect the HDTV source correctly
#[test]
fn should_detect_the_hdtv_source_correctly() {
    assert_includes("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", json!({ "source": "HDTV" }));
}

/// should detect the HDTVRip source correctly
#[test]
fn should_detect_the_hdtvrip_source_correctly() {
    assert_includes("Rebecca.1940.720p.HDTVRip.HDCLUB", json!({ "source": "HDTV" }));
}

/// should detect the SATRip source correctly
#[test]
fn should_detect_the_satrip_source_correctly() {
    assert_includes("Gossip Girl - 1ª Temporada. (SAT-Rip)", json!({ "source": "SATRip" }));
}

/// should detect the DVDRip source correctly
#[test]
fn should_detect_the_dvdrip_source_correctly() {
    assert_includes("A Stable Life S01E01 DVDRip x264-Ltu", json!({ "source": "DVDRip" }));
}

/// should detect the Web-DL source correctly
#[test]
fn should_detect_the_web_dl_source_correctly() {
    assert_includes(
        "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN",
        json!({ "source": "WEB-DL" }),
    );
}

/// should detect the WebRip source correctly
#[test]
fn should_detect_the_web_rip_source_correctly() {
    assert_includes("Brown Nation S01E05 1080p WEBRip x264-JAWN", json!({ "source": "WEBRip" }));
}

/// should detect the TeleSync source correctly
#[test]
fn should_detect_the_tele_sync_source_correctly() {
    assert_includes("Star Wars The Last Jedi 2017 TeleSync AAC x264-MiniMe", json!({ "source": "TeleSync" }));
}

/// should detect the DVDScr source correctly
#[test]
fn should_detect_the_dvdscr_source_correctly() {
    assert_includes("The.Shape.of.Water.2017.DVDScr.XVID.AC3.HQ.Hive-CM8", json!({ "source": "SCR" }));
}

/// should detect the PPVRip source correctly
#[test]
fn should_detect_the_ppvrip_source_correctly() {
    assert_includes(
        "Cloudy With A Chance Of Meatballs 2 2013 720p PPVRip x264 AAC-FooKaS",
        json!({ "source": "PPVRip" }),
    );
}

/// should detect the WEBMux source correctly
#[test]
fn should_detect_the_webmux_source_correctly() {
    assert_includes("The.OA.1x08.L.Io.Invisibile.ITA.WEBMux.x264-UBi.mkv", json!({ "source": "WEBMux" }));
}

/// should detect the BD source correctly
#[test]
fn should_detect_the_bd_source_correctly() {
    assert_includes(
        "[UsifRenegade] Cardcaptor Sakura [BD][Remastered][1080p][HEVC_10Bit][Dual] + Movies",
        json!({ "source": "BDRip" }),
    );
}

/// should detect the BD-RM source correctly
#[test]
fn should_detect_the_bd_rm_source_correctly() {
    assert_includes(
        "[UsifRenegade] Cardcaptor Sakura - 54 [BD-RM][1080p][x265_10Bit][Dual_AAC].mkv",
        json!({ "source": "BDRip" }),
    );
}

/// should detect the MicroHD source correctly
#[test]
fn should_detect_the_micro_hd_source_correctly() {
    assert_includes("Elvis & Nixon (MicroHD-1080p)", json!({ "source": "HDRip" }));
}

/// should detect the UHDrip source correctly
#[test]
fn should_detect_the_uhdrip_source_correctly() {
    assert_includes("Bohemian Rhapsody 2018.2160p.UHDrip.x265.HDR.DD+.5.1-DTOne", json!({ "source": "UHDRip" }));
}

/// should detect the UltraHD source correctly
#[test]
fn should_detect_the_ultra_hd_source_correctly() {
    assert_includes("Blade.Runner.2049.2017.4K.UltraHD.BluRay.2160p.x264.TrueHD.Atmos", json!({ "source": "BluRay" }));
}

/// should detect the UHD source correctly
#[test]
fn should_detect_the_uhd_source_correctly() {
    assert_includes("Terminator.Dark.Fate.2019.2160p.UHD.BluRay.X265.10bit.HDR.TrueHD", json!({ "source": "BluRay" }));
}

/// When We Were Boys 2013 BD Rip x264 titohmr
#[test]
fn when_we_were_boys_2013_bd_rip_x264_titohmr() {
    assert_includes("When We Were Boys 2013 BD Rip x264 titohmr", json!({ "source": "BDRip" }));
}

/// Key.and.Peele.s03e09.720p.web.dl.mrlss.sujaidr (pimprg)
#[test]
fn key_and_peele_s03e09_720p_web_dl_mrlss_sujaidr_pimprg() {
    assert_includes("Key.and.Peele.s03e09.720p.web.dl.mrlss.sujaidr (pimprg)", json!({ "source": "WEB-DL" }));
}

/// Godzilla 2014 HDTS HC XVID AC3 ACAB
#[test]
fn godzilla_2014_hdts_hc_xvid_ac3_acab() {
    assert_includes("Godzilla 2014 HDTS HC XVID AC3 ACAB", json!({ "source": "TeleSync" }));
}

/// Harry Potter And The Half Blood Prince 2009 telesync aac -- king
#[test]
fn harry_potter_and_the_half_blood_prince_2009_telesync_aac_king() {
    assert_includes(
        "Harry Potter And The Half Blood Prince 2009 telesync aac -- king",
        json!({ "source": "TeleSync" }),
    );
}

/// Capitao.America.2.TS.BrunoG
#[test]
fn capitao_america_2_ts_bruno_g() {
    assert_includes("Capitao.America.2.TS.BrunoG", json!({ "source": "TeleSync" }));
}

/// Star Trek TS-Screener Spanish Alta-Calidad 2da Version 2009 - Me
#[test]
fn star_trek_ts_screener_spanish_alta_calidad_2da_version_2009_me() {
    assert_includes(
        "Star Trek TS-Screener Spanish Alta-Calidad 2da Version 2009 - Me",
        json!({ "source": "TeleSync" }),
    );
}

/// Solo: A Star Wars Story (2018) English 720p TC x264 900MBTEAM TR
#[test]
fn solo_a_star_wars_story_2018_english_720p_tc_x264_900_mbteam_tr() {
    assert_includes(
        "Solo: A Star Wars Story (2018) English 720p TC x264 900MBTEAM TR",
        json!({ "source": "TeleCine" }),
    );
}

/// Alita Battle Angel 2019 720p HDTC-1XBET
#[test]
fn alita_battle_angel_2019_720p_hdtc_1_xbet() {
    assert_includes("Alita Battle Angel 2019 720p HDTC-1XBET", json!({ "source": "TeleCine" }));
}

/// My.Super.Ex.Girlfriend.FRENCH.TELECINE.XViD-VCDFRV
#[test]
fn my_super_ex_girlfriend_french_telecine_xvi_d_vcdfrv() {
    assert_includes("My.Super.Ex.Girlfriend.FRENCH.TELECINE.XViD-VCDFRV", json!({ "source": "TeleCine" }));
}

/// You're Next (2013) cam XVID
#[test]
fn you_re_next_2013_cam_xvid() {
    assert_includes("You're Next (2013) cam XVID", json!({ "source": "CAM" }));
}

/// Shes the one_2013(camrip)__TOPSIDER [email protected]
#[test]
fn shes_the_one_2013_camrip_topsider_email_protected() {
    assert_includes("Shes the one_2013(camrip)__TOPSIDER [email protected]", json!({ "source": "CAM" }));
}

/// Blair Witch 2016 HDCAM UnKnOwN
#[test]
fn blair_witch_2016_hdcam_un_kn_ow_n() {
    assert_includes("Blair Witch 2016 HDCAM UnKnOwN", json!({ "source": "CAM" }));
}

/// Thor : Love and Thunder (2022) Hindi HQCAM x264 AAC - QRips.mkv
#[test]
fn thor_love_and_thunder_2022_hindi_hqcam_x264_aac_qrips_mkv() {
    assert_includes("Thor : Love and Thunder (2022) Hindi HQCAM x264 AAC - QRips.mkv", json!({ "source": "CAM" }));
}

/// Avatar The Way of Water (2022) 1080p HQ S-Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv
#[test]
fn avatar_the_way_of_water_2022_1080p_hq_s_print_dual_audio_hindi_english_x264_aac_hc_esub_ci() {
    assert_includes(
        "Avatar The Way of Water (2022) 1080p HQ S-Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv",
        json!({ "source": "CAM" }),
    );
}

/// Avatar The Way of Water (2022) 1080p S Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv
#[test]
fn avatar_the_way_of_water_2022_1080p_s_print_dual_audio_hindi_english_x264_aac_hc_esub_cine() {
    assert_includes(
        "Avatar The Way of Water (2022) 1080p S Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv",
        json!({ "source": "CAM" }),
    );
}

/// Good Deeds 2012 SCR XViD-KiNGDOM
#[test]
fn good_deeds_2012_scr_xvi_d_ki_ngdom() {
    assert_includes("Good Deeds 2012 SCR XViD-KiNGDOM", json!({ "source": "SCR" }));
}

/// Genova DVD-Screener Spanish 2008
#[test]
fn genova_dvd_screener_spanish_2008() {
    assert_includes("Genova DVD-Screener Spanish 2008", json!({ "source": "SCR" }));
}

/// El Albergue Rojo BR-Screener Spanish 2007
#[test]
fn el_albergue_rojo_br_screener_spanish_2007() {
    assert_includes("El Albergue Rojo BR-Screener Spanish 2007", json!({ "source": "SCR" }));
}

/// The.Mysteries.of.Pittsburgh.LIMITED.SCREENER.XviD-COALiTiON [NOR
#[test]
fn the_mysteries_of_pittsburgh_limited_screener_xvi_d_coali_ti_on_nor() {
    assert_includes("The.Mysteries.of.Pittsburgh.LIMITED.SCREENER.XviD-COALiTiON [NOR", json!({ "source": "SCR" }));
}

/// El.curioso.caso.de.benjamin.button-BRScreener-[EspaDivx.com].rar
#[test]
fn el_curioso_caso_de_benjamin_button_brscreener_espa_divx_com_rar() {
    assert_includes("El.curioso.caso.de.benjamin.button-BRScreener-[EspaDivx.com].rar", json!({ "source": "SCR" }));
}

/// Thor- Love and Thunder (2022) Original Hindi Dubbed 1080p HQ PreDVD Rip x264 AAC [1.7 GB]- CineVood.mkv
#[test]
fn thor_love_and_thunder_2022_original_hindi_dubbed_1080p_hq_pre_dvd_rip_x264_aac_1_7_gb_cine() {
    assert_includes(
        "Thor- Love and Thunder (2022) Original Hindi Dubbed 1080p HQ PreDVD Rip x264 AAC [1.7 GB]- CineVood.mkv",
        json!({ "source": "SCR" }),
    );
}

/// Black Panther Wakanda Forever 2022 Hindi 1080p PDVDRip x264 AAC CineVood.mkv
#[test]
fn black_panther_wakanda_forever_2022_hindi_1080p_pdvdrip_x264_aac_cine_vood_mkv() {
    assert_includes(
        "Black Panther Wakanda Forever 2022 Hindi 1080p PDVDRip x264 AAC CineVood.mkv",
        json!({ "source": "SCR" }),
    );
}

/// Vampire in Vegas (2009) NL Subs DVDR DivXNL-Team
#[test]
fn vampire_in_vegas_2009_nl_subs_dvdr_div_xnl_team() {
    assert_includes("Vampire in Vegas (2009) NL Subs DVDR DivXNL-Team", json!({ "source": "DVD" }));
}

/// should parse WEB-DLRip source
#[test]
fn should_parse_web_dlrip_source() {
    assert_includes(
        "Звонок из прошлого / Kol / The Call (2020) WEB-DLRip | ViruseProject",
        json!({ "source": "WEB-DL" }),
    );
}

/// should parse BluRay Rip source
#[test]
fn should_parse_blu_ray_rip_source() {
    assert_includes(
        "La nube (2020) [BluRay Rip][AC3 5.1 Castellano][www.maxitorrent.com]",
        json!({ "source": "BRRip" }),
    );
}

/// should parse BluRay remux together source
#[test]
fn should_parse_blu_ray_remux_together_source() {
    assert_includes(
        "Joker.2019.2160p.BluRay.REMUX.HEVC.DTS-HD.MA.TrueHD.7.1.Atmos-FGT",
        json!({ "source": "BluRay REMUX" }),
    );
}

/// should parse Blu-Ray remux source
#[test]
fn should_parse_blu_ray_remux_source() {
    assert_includes(
        "Warcraft 2016 1080p Blu-ray Remux AVC TrueHD Atmos-KRaLiMaRKo",
        json!({ "source": "BluRay REMUX" }),
    );
}

/// should parse BluRay remux ahead source
#[test]
fn should_parse_blu_ray_remux_ahead_source() {
    assert_includes("Joker.2019.UHD.BluRay.2160p.TrueHD.Atmos.7.1.HEVC.REMUX-JAT", json!({ "source": "BluRay REMUX" }));
}

/// should parse BluRay remux before source
#[test]
fn should_parse_blu_ray_remux_before_source() {
    assert_includes(
        "Spider-Man No Way Home.2022.REMUX.1080p.Bluray.DTS-HD.MA.5.1.AVC-EVO[TGx]",
        json!({ "source": "BluRay REMUX" }),
    );
}

/// should parse BDRemux before source
#[test]
fn should_parse_bdremux_before_source() {
    assert_includes("Son of God 2014 HDR BDRemux 1080p.mkv", json!({ "source": "BluRay REMUX" }));
}

/// should parse UHDRemux before source
#[test]
fn should_parse_uhdremux_before_source() {
    assert_includes(
        "Peter Rabbit 2 [4K UHDremux][2160p][HDR10][DTS-HD 5.1 Castellano-TrueHD 7.1-Ingles+Subs][ES-EN]",
        json!({ "source": "BluRay REMUX" }),
    );
}

/// should parse 4kUHDRemux before source
#[test]
fn should_parse_4k_uhdremux_before_source() {
    assert_includes(
        "Snatch cerdos y diamantes [4KUHDremux 2160p][Castellano AC3 5.1-Ingles TrueHD 7.1+Subs]",
        json!({ "source": "BluRay REMUX" }),
    );
}

/// should parse HDDVDRip source
#[test]
fn should_parse_hddvdrip_source() {
    assert_includes(" Троя / Troy [2004 HDDVDRip-AVC] Dub + Original + Sub]", json!({ "source": "DVDRip" }));
}

/// should parse VHSRip source
#[test]
fn should_parse_vhsrip_source() {
    assert_includes("Структура момента (Расим Исмайлов) [1980, Драма, VHSRip]", json!({ "source": "DVDRip" }));
}

/// should parse VHS source
#[test]
fn should_parse_vhs_source() {
    assert_includes("Мужчины без женщин (Альгимантас Видугирис) [1981, Драма, VHS]", json!({ "source": "DVD" }));
}

/// should parse DVB source
#[test]
fn should_parse_dvb_source() {
    assert_includes("Преферанс по пятницам (Игорь Шешуков) [1984, Детектив, DVB]", json!({ "source": "HDTV" }));
}

/// should parse WEB-DLRip source
#[test]
fn should_parse_web_dlrip_source_2() {
    assert_includes("Соперницы (Алексей Дмитриев) [1929, драма, WEB-DLRip]", json!({ "source": "WEB-DL" }));
}

/// should parse HDTSRip source
#[test]
fn should_parse_hdtsrip_source() {
    assert_includes("Dragon Blade (2015) HDTSRip Exclusive", json!({ "source": "TeleSync" }));
}

/// should parse HDTCRip source
#[test]
fn should_parse_hdtcrip_source() {
    assert_includes("Criminal (2016) Hindi Dubbed HDTCRip", json!({ "source": "TeleCine" }));
}

/// should parse CAMHD source
#[test]
fn should_parse_camhd_source() {
    assert_includes("Avatar La Voie de l'eau.FRENCH.CAMHD.H264.AAC", json!({ "source": "CAM" }));
}

/// should not parse VHS in title
#[test]
fn should_not_parse_vhs_in_title() {
    assert_missing("VHS 3 Viral (2014)PL.mp4", &["source"]);
}

/// should detect CAM source attached to year
#[test]
fn should_detect_cam_source_attached_to_year() {
    assert_includes("A Very Harold & Kumar 3D Christmas (2011)CAM(900mb)Nl subs N", json!({ "source": "CAM" }));
}

/// should detect WEB source
#[test]
fn should_detect_web_source() {
    assert_includes("The.Last.of.Us.S01E08.1080p.WEB.H264-CAKES[TGx]", json!({ "source": "WEB" }));
}
