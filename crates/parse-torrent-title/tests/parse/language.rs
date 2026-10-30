// Ported from test/language.js of the js library.

//! Parsing language

use crate::common::*;

/// should detect the russian language correctly
#[test]
fn should_detect_the_russian_language_correctly() {
    assert_includes(
        "Deadpool 2016 1080p BluRay DTS Rus Ukr 3xEng HDCL",
        json!({ "languages": ["russian", "ukrainian"] }),
    );
}

/// should detect the netherlands language correctly
#[test]
fn should_detect_the_netherlands_language_correctly() {
    assert_includes("VAIANA: MOANA (2017) NL-Retail [2D] EAGLE", json!({ "languages": ["dutch"] }));
}

/// should detect the flemish language correctly
#[test]
fn should_detect_the_flemish_language_correctly() {
    assert_includes("De Noodcentrale S02E05 FLEMISH 540p WEBRip AAC H 264", json!({ "languages": ["dutch"] }));
}

/// should detect the truefrench language correctly
#[test]
fn should_detect_the_truefrench_language_correctly() {
    assert_includes("The Intern 2015 TRUEFRENCH 720p BluRay x264-PiNKPANTERS", json!({ "languages": ["french"] }));
}

/// should detect the vff language correctly
#[test]
fn should_detect_the_vff_language_correctly() {
    assert_includes("After Earth 2013 VFF BDrip x264 YJ", json!({ "languages": ["french"] }));
}

/// should detect the french language correctly
#[test]
fn should_detect_the_french_language_correctly() {
    assert_includes("127.Heures.FRENCH.DVDRip.AC3.XViD-DVDFR", json!({ "languages": ["french"] }));
}

/// should detect the vostfr language with lowercase correctly
#[test]
fn should_detect_the_vostfr_language_with_lowercase_correctly() {
    assert_includes("Color.Of.Night.Unrated.DC.VostFR.BRrip.x264", json!({ "languages": ["french"] }));
}

/// should detect the multi language correctly
#[test]
fn should_detect_the_multi_language_correctly() {
    assert_includes("Le Labyrinthe 2014 Multi-VF2 1080p BluRay x264-PopHD", json!({ "languages": ["multi audio"] }));
}

/// should detect the VFI language correctly
#[test]
fn should_detect_the_vfi_language_correctly() {
    assert_includes("Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG", json!({ "languages": ["french"] }));
}

/// should detect the italian language correctly
#[test]
fn should_detect_the_italian_language_correctly() {
    assert_includes("South.Park.S21E10.iTALiAN.FiNAL.AHDTV.x264-NTROPiC", json!({ "languages": ["italian"] }));
}

/// 2- English- {SDH}.srt
#[test]
fn t_2_english_sdh_srt() {
    assert_includes("2- English- {SDH}.srt", json!({ "languages": ["english"] }));
}

/// 1_ English-Subs.srt
#[test]
fn t_1_english_subs_srt() {
    assert_includes("1_ English-Subs.srt", json!({ "languages": ["english"] }));
}

/// House S 1 CD 1-6 svensk, danska, norsk, finsk sub
#[test]
fn house_s_1_cd_1_6_svensk_danska_norsk_finsk_sub() {
    assert_includes(
        "House S 1 CD 1-6 svensk, danska, norsk, finsk sub",
        json!({ "languages": ["danish", "finnish", "swedish", "norwegian"] }),
    );
}

/// Svein.Og.Rotta.NORSK.Nordic.Subs.2006
#[test]
fn svein_og_rotta_norsk_nordic_subs_2006() {
    assert_includes(
        "Svein.Og.Rotta.NORSK.Nordic.Subs.2006",
        json!({ "languages": ["danish", "finnish", "swedish", "norwegian"] }),
    );
}

/// Borat med Norsk Undertekst
#[test]
fn borat_med_norsk_undertekst() {
    assert_includes("Borat med Norsk Undertekst", json!({ "languages": ["norwegian"] }));
}

/// Subs/21_Bokmal.srt
#[test]
fn subs_21_bokmal_srt() {
    assert_includes("Subs/21_Bokmal.srt", json!({ "languages": ["norwegian"] }));
}

/// Subs/nob.srt
#[test]
fn subs_nob_srt() {
    assert_includes("Subs/nob.srt", json!({ "languages": ["norwegian"] }));
}

/// Subs/5_nor.srt
#[test]
fn subs_5_nor_srt() {
    assert_includes("Subs/5_nor.srt", json!({ "languages": ["norwegian"] }));
}

/// Curious.George.2.Follow.That.Monkey.2009.DK.SWE.UK.PAL.DVDR-CATC
#[test]
fn curious_george_2_follow_that_monkey_2009_dk_swe_uk_pal_dvdr_catc() {
    assert_includes(
        "Curious.George.2.Follow.That.Monkey.2009.DK.SWE.UK.PAL.DVDR-CATC",
        json!({ "languages": ["danish", "swedish"] }),
    );
}

/// Yes.Man.Dk-Subs.2009.dingel
#[test]
fn yes_man_dk_subs_2009_dingel() {
    assert_includes("Yes.Man.Dk-Subs.2009.dingel", json!({ "languages": ["danish"] }));
}

/// Dan-SDH.srt
#[test]
fn dan_sdh_srt() {
    assert_includes("Dan-SDH.srt", json!({ "languages": ["danish"] }));
}

/// dan.srt
#[test]
fn dan_srt() {
    assert_includes("dan.srt", json!({ "languages": ["danish"] }));
}

/// Red Riding 1974 [2009 PAL DVD][En Subs[Sv.No.Fi]
#[test]
fn red_riding_1974_2009_pal_dvd_en_subs_sv_no_fi() {
    assert_includes(
        "Red Riding 1974 [2009 PAL DVD][En Subs[Sv.No.Fi]",
        json!({ "languages": ["english", "finnish", "swedish"] }),
    );
}

/// Comme une Image (Look at Me) [2004 PAL DVD][Fr Subs[Sv.Da.No]
#[test]
fn comme_une_image_look_at_me_2004_pal_dvd_fr_subs_sv_da_no() {
    assert_includes(
        "Comme une Image (Look at Me) [2004 PAL DVD][Fr Subs[Sv.Da.No]",
        json!({ "languages": ["french", "swedish"] }),
    );
}

/// A.Good.Day.To.Die.Hard.2013.SWESUB.DANSUB.FiNSUB.720p.WEB-DL.-Ro
#[test]
fn a_good_day_to_die_hard_2013_swesub_dansub_fi_nsub_720p_web_dl_ro() {
    assert_includes(
        "A.Good.Day.To.Die.Hard.2013.SWESUB.DANSUB.FiNSUB.720p.WEB-DL.-Ro",
        json!({ "languages": ["danish", "finnish", "swedish"] }),
    );
}

/// The.Prisoner.1967-1968.Complete.Series.Subs.English+Nordic
#[test]
fn the_prisoner_1967_1968_complete_series_subs_english_nordic() {
    assert_includes(
        "The.Prisoner.1967-1968.Complete.Series.Subs.English+Nordic",
        json!({ "languages": ["english", "danish", "finnish", "swedish", "norwegian"] }),
    );
}

/// Royal.Pains.S05E02.HDTV.subtitulado.esp.sc.avi
#[test]
fn royal_pains_s05_e02_hdtv_subtitulado_esp_sc_avi() {
    assert_includes("Royal.Pains.S05E02.HDTV.subtitulado.esp.sc.avi", json!({ "languages": ["spanish"] }));
}

/// Desmembrados (2006) [HDrip-XviD-AC3][Castellano]
#[test]
fn desmembrados_2006_hdrip_xvi_d_ac3_castellano() {
    assert_includes("Desmembrados (2006) [HDrip-XviD-AC3][Castellano]", json!({ "languages": ["spanish"] }));
}

/// 10_Spanish-Subs.srt
#[test]
fn t_10_spanish_subs_srt() {
    assert_includes("10_Spanish-Subs.srt", json!({ "languages": ["spanish"] }));
}

/// Patriot Games [1992] Eng, Ger, Cze, Hun, Pol + multisub  DVDrip
#[test]
fn patriot_games_1992_eng_ger_cze_hun_pol_multisub_dvdrip() {
    assert_includes(
        "Patriot Games [1992] Eng, Ger, Cze, Hun, Pol + multisub  DVDrip",
        json!({ "languages": ["multi subs", "english", "german", "polish", "czech", "hungarian"] }),
    );
}

/// Elvis Presley - La via del Male (King creole) - IT EN FR DE ES
#[test]
fn elvis_presley_la_via_del_male_king_creole_it_en_fr_de_es() {
    assert_includes(
        "Elvis Presley - La via del Male (King creole) - IT EN FR DE ES",
        json!({ "languages": ["english", "french", "spanish", "italian", "german"] }),
    );
}

/// FernGully [H264 - Ita Dut Fre Ger Eng Spa Aac - MultiSub]
#[test]
fn fern_gully_h264_ita_dut_fre_ger_eng_spa_aac_multi_sub() {
    assert_includes(
        "FernGully [H264 - Ita Dut Fre Ger Eng Spa Aac - MultiSub]",
        json!({ "languages": ["multi subs", "english", "french", "spanish", "italian", "german", "dutch"] }),
    );
}

/// Jesus de Montreal / Jesus of Montreal - subtitulos espanol
#[test]
fn jesus_de_montreal_jesus_of_montreal_subtitulos_espanol() {
    assert_includes("Jesus de Montreal / Jesus of Montreal - subtitulos espanol", json!({ "languages": ["spanish"] }));
}

/// Los.Vengadores.DVDR español ingles clon
#[test]
fn los_vengadores_dvdr_espan_ol_ingles_clon() {
    assert_includes("Los.Vengadores.DVDR español ingles clon", json!({ "languages": ["english", "spanish"] }));
}

/// EMPIRE STATE 2013 DVDRip TRNC English and Española Latin
#[test]
fn empire_state_2013_dvdrip_trnc_english_and_espan_ola_latin() {
    assert_includes(
        "EMPIRE STATE 2013 DVDRip TRNC English and Española Latin",
        json!({ "languages": ["english", "spanish"] }),
    );
}

/// Mary Poppins Returns 2019 DVDRip LATINO-1XBET
#[test]
fn mary_poppins_returns_2019_dvdrip_latino_1_xbet() {
    assert_includes("Mary Poppins Returns 2019 DVDRip LATINO-1XBET", json!({ "languages": ["latino"] }));
}

/// Los Simpsons S18 E01 Latino
#[test]
fn los_simpsons_s18_e01_latino() {
    assert_includes("Los Simpsons S18 E01 Latino", json!({ "languages": ["latino"] }));
}

/// Spider-Man (2002) Blu-Ray [720p] Dual Ingles-Español
#[test]
fn spider_man_2002_blu_ray_720p_dual_ingles_espan_ol() {
    assert_includes(
        "Spider-Man (2002) Blu-Ray [720p] Dual Ingles-Español",
        json!({ "languages": ["dual audio", "english", "spanish"] }),
    );
}

/// Abuela (2015) 1080p BluRay x264 AC3 Dual Latino-Inglés
#[test]
fn abuela_2015_1080p_blu_ray_x264_ac3_dual_latino_ingle_s() {
    assert_includes(
        "Abuela (2015) 1080p BluRay x264 AC3 Dual Latino-Inglés",
        json!({ "languages": ["dual audio", "english", "latino"] }),
    );
}

/// Dumbo.2019.1080p.Dual.Lat
#[test]
fn dumbo_2019_1080p_dual_lat() {
    assert_includes("Dumbo.2019.1080p.Dual.Lat", json!({ "languages": ["dual audio", "latino"] }));
}

/// A.Score.to.Settle.2019.lati.mp4
#[test]
fn a_score_to_settle_2019_lati_mp4() {
    assert_includes(" A.Score.to.Settle.2019.lati.mp4", json!({ "languages": ["latino"] }));
}

/// [S0.E04] Gambit królowej - Gra środkowa.Spanish Latin America.srt
#[test]
fn s0_e04_gambit_kro_lowej_gra_s_rodkowa_spanish_latin_america_srt() {
    assert_includes(
        "[S0.E04] Gambit królowej - Gra środkowa.Spanish Latin America.srt",
        json!({ "languages": ["latino"] }),
    );
}

/// Latin American.spa.srt
#[test]
fn latin_american_spa_srt() {
    assert_includes("Latin American.spa.srt", json!({ "languages": ["latino"] }));
}

/// Men in Black International 2019 (inglês português)
#[test]
fn men_in_black_international_2019_ingle_s_portugue_s() {
    assert_includes(
        "Men in Black International 2019 (inglês português)",
        json!({ "languages": ["english", "portuguese"] }),
    );
}

/// Assassins (1995) Sylvester Stallone.DVDrip.XviD - Italian Englis
#[test]
fn assassins_1995_sylvester_stallone_dvdrip_xvi_d_italian_englis() {
    assert_includes(
        "Assassins (1995) Sylvester Stallone.DVDrip.XviD - Italian Englis",
        json!({ "languages": ["english", "italian"] }),
    );
}

/// El Club de la Lucha[dvdrip][spanish]
#[test]
fn el_club_de_la_lucha_dvdrip_spanish() {
    assert_includes("El Club de la Lucha[dvdrip][spanish]", json!({ "languages": ["spanish"] }));
}

/// The Curse Of The Weeping Woman 2019 BluRay 1080p Tel+Tam+hin+eng
#[test]
fn the_curse_of_the_weeping_woman_2019_blu_ray_1080p_tel_tam_hin_eng() {
    assert_includes(
        "The Curse Of The Weeping Woman 2019 BluRay 1080p Tel+Tam+hin+eng",
        json!({ "languages": ["english", "hindi", "telugu", "tamil"] }),
    );
}

/// Inception 2010 1080p BRRIP[dual-audio][eng-hindi]
#[test]
fn inception_2010_1080p_brrip_dual_audio_eng_hindi() {
    assert_includes(
        "Inception 2010 1080p BRRIP[dual-audio][eng-hindi]",
        json!({ "languages": ["dual audio", "english", "hindi"] }),
    );
}

/// Inception (2010) 720p BDRip Tamil+Telugu+Hindi+Eng
#[test]
fn inception_2010_720p_bdrip_tamil_telugu_hindi_eng() {
    assert_includes(
        "Inception (2010) 720p BDRip Tamil+Telugu+Hindi+Eng",
        json!({ "languages": ["english", "hindi", "telugu", "tamil"] }),
    );
}

/// Subs/Dear.S01E02.WEBRip.x265-ION265/34_tam.srt
#[test]
fn subs_dear_s01_e02_webrip_x265_ion265_34_tam_srt() {
    assert_includes("Subs/Dear.S01E02.WEBRip.x265-ION265/34_tam.srt", json!({ "languages": ["tamil"] }));
}

/// Subs/Dear.S01E02.WEBRip.x265-ION265/35_tel.srt
#[test]
fn subs_dear_s01_e02_webrip_x265_ion265_35_tel_srt() {
    assert_includes("Subs/Dear.S01E02.WEBRip.x265-ION265/35_tel.srt", json!({ "languages": ["telugu"] }));
}

/// Dabangg 3 2019 AMZN WebRip Hindi 720p x264
#[test]
fn dabangg_3_2019_amzn_web_rip_hindi_720p_x264() {
    assert_includes("Dabangg 3 2019 AMZN WebRip Hindi 720p x264", json!({ "languages": ["hindi"] }));
}

/// Quarantine [2008] [DVDRiP.XviD-M14CH0] [Lektor PL] [Arx]
#[test]
fn quarantine_2008_dvdri_p_xvi_d_m14_ch0_lektor_pl_arx() {
    assert_includes("Quarantine [2008] [DVDRiP.XviD-M14CH0] [Lektor PL] [Arx]", json!({ "languages": ["polish"] }));
}

/// The Mandalorian S01E06 POLISH WEBRip x264-FLAME
#[test]
fn the_mandalorian_s01_e06_polish_webrip_x264_flame() {
    assert_includes("The Mandalorian S01E06 POLISH WEBRip x264-FLAME", json!({ "languages": ["polish"] }));
}

/// Na Wspólnej (2024) [E3951-3954][1080p][WEB-DL][PL][x264-GhN]
#[test]
fn na_wspo_lnej_2024_e3951_3954_1080p_web_dl_pl_x264_gh_n() {
    assert_includes("Na Wspólnej (2024) [E3951-3954][1080p][WEB-DL][PL][x264-GhN]", json!({ "languages": ["polish"] }));
}

/// Kulej.2024.PL.1080p.AMZN.WEB-DL.x264-KiT
#[test]
fn kulej_2024_pl_1080p_amzn_web_dl_x264_ki_t() {
    assert_includes("Kulej.2024.PL.1080p.AMZN.WEB-DL.x264-KiT", json!({ "languages": ["polish"] }));
}

/// Star.Wars.Skeleton.Crew.Sezon01.PLDUB.480p.DSNP.WEB-DL.H264.DDP5.1-K83
#[test]
fn star_wars_skeleton_crew_sezon01_pldub_480p_dsnp_web_dl_h264_ddp5_1_k83() {
    assert_includes(
        "Star.Wars.Skeleton.Crew.Sezon01.PLDUB.480p.DSNP.WEB-DL.H264.DDP5.1-K83",
        json!({ "languages": ["polish"] }),
    );
}

/// Bukmacher / Bookie [E02E07] [MULTi] [1080p] [AMZN] [WEB-DL] [H264] [DDP5.1.Atmos-K83] [Lektor PL]
#[test]
fn bukmacher_bookie_e02_e07_multi_1080p_amzn_web_dl_h264_ddp5_1_atmos_k83_lektor_pl() {
    assert_includes(
        "Bukmacher / Bookie [E02E07] [MULTi] [1080p] [AMZN] [WEB-DL] [H264] [DDP5.1.Atmos-K83] [Lektor PL]",
        json!({ "languages": ["multi audio", "polish"] }),
    );
}

/// Gra 1968 [REKONSTRUKCJA] [1080p.WEB-DL.H264.AC3-FT] [Film Polski]
#[test]
fn gra_1968_rekonstrukcja_1080p_web_dl_h264_ac3_ft_film_polski() {
    assert_includes(
        "Gra 1968 [REKONSTRUKCJA] [1080p.WEB-DL.H264.AC3-FT] [Film Polski]",
        json!({ "languages": ["polish"] }),
    );
}

/// Evil.Dead.Rise.2023.PLSUB.720p.MA.WEB-DL.H264.E-AC3-CMRG
#[test]
fn evil_dead_rise_2023_plsub_720p_ma_web_dl_h264_e_ac3_cmrg() {
    assert_includes("Evil.Dead.Rise.2023.PLSUB.720p.MA.WEB-DL.H264.E-AC3-CMRG", json!({ "languages": ["polish"] }));
}

/// Wallace i Gromit: Zemsta pingwina / Wallace & Gromit: Vengeance Most Fowl (2024) [480p] [WEB-DL] [XviD] [DD5.1-K83] [Dubbing PL]
#[test]
fn wallace_i_gromit_zemsta_pingwina_wallace_gromit_vengeance_most_fowl_2024_480p_web_dl_xvi_d() {
    assert_includes(
        "Wallace i Gromit: Zemsta pingwina / Wallace & Gromit: Vengeance Most Fowl (2024) [480p] [WEB-DL] [XviD] [DD5.1-K83] [Dubbing PL]",
        json!({ "languages": ["polish"] }),
    );
}

/// Strażniczka smoków / Dragonkeeper (2024) [1080p] [H264] [Napisy PL]
#[test]
fn straz_niczka_smoko_w_dragonkeeper_2024_1080p_h264_napisy_pl() {
    assert_includes(
        "Strażniczka smoków / Dragonkeeper (2024) [1080p] [H264] [Napisy PL]",
        json!({ "languages": ["polish"] }),
    );
}

/// Carros 2 Dublado - Portugues BR (2011)
#[test]
fn carros_2_dublado_portugues_br_2011() {
    assert_includes("Carros 2 Dublado - Portugues BR (2011)", json!({ "languages": ["portuguese"] }));
}

/// A.Simple.Plan.1998.720p.BDRIP.X264.dublado.portugues.BR.gmenezes
#[test]
fn a_simple_plan_1998_720p_bdrip_x264_dublado_portugues_br_gmenezes() {
    assert_includes(
        "A.Simple.Plan.1998.720p.BDRIP.X264.dublado.portugues.BR.gmenezes",
        json!({ "languages": ["portuguese"] }),
    );
}

/// American.Horror.Story.S01E01.720p. PORTUGUÊS BR
#[test]
fn american_horror_story_s01_e01_720p_portugue_s_br() {
    assert_includes("American.Horror.Story.S01E01.720p. PORTUGUÊS BR", json!({ "languages": ["portuguese"] }));
}

/// Angel.S05E19.legendado.br.rmvb
#[test]
fn angel_s05_e19_legendado_br_rmvb() {
    assert_includes("Angel.S05E19.legendado.br.rmvb", json!({ "languages": ["portuguese"] }));
}

/// Grimm S01E11 Dublado BR [ kickUploader ]
#[test]
fn grimm_s01_e11_dublado_br_kick_uploader() {
    assert_includes("Grimm S01E11 Dublado BR [ kickUploader ]", json!({ "languages": ["portuguese"] }));
}

/// InuYasha.EP161.ptBR.subtitles.[inuplace.com.br].avi
#[test]
fn inu_yasha_ep161_pt_br_subtitles_inuplace_com_br_avi() {
    assert_includes("InuYasha.EP161.ptBR.subtitles.[inuplace.com.br].avi", json!({ "languages": ["portuguese"] }));
}

/// Ghost.Rider.DivX_Gamonet(Ingles-Port.BR)-AC3.avi
#[test]
fn ghost_rider_div_x_gamonet_ingles_port_br_ac3_avi() {
    assert_includes(
        "Ghost.Rider.DivX_Gamonet(Ingles-Port.BR)-AC3.avi",
        json!({ "languages": ["english", "portuguese"] }),
    );
}

/// I Am David  legendado pt/br
#[test]
fn i_am_david_legendado_pt_br() {
    assert_includes("I Am David  legendado pt/br", json!({ "languages": ["portuguese"] }));
}

/// Lone Wolf and Cub 6 movies - legendas BR
#[test]
fn lone_wolf_and_cub_6_movies_legendas_br() {
    assert_includes("Lone Wolf and Cub 6 movies - legendas BR", json!({ "languages": ["portuguese"] }));
}

/// Wonder Woman Season 3 (H.264 1080p; English/Portuguese-BR)
#[test]
fn wonder_woman_season_3_h_264_1080p_english_portuguese_br() {
    assert_includes(
        "Wonder Woman Season 3 (H.264 1080p; English/Portuguese-BR)",
        json!({ "languages": ["english", "portuguese"] }),
    );
}

/// MIB 3 - Homens de Preto 2012 ( Audio EN-BR - Leg BR  Mkv 1280x69
#[test]
fn mib_3_homens_de_preto_2012_audio_en_br_leg_br_mkv_1280x69() {
    assert_includes(
        "MIB 3 - Homens de Preto 2012 ( Audio EN-BR - Leg BR  Mkv 1280x69",
        json!({ "languages": ["english", "portuguese"] }),
    );
}

/// my wife is a gangster 3 legendado em PT(BR)
#[test]
fn my_wife_is_a_gangster_3_legendado_em_pt_br() {
    assert_includes("my wife is a gangster 3 legendado em PT(BR)", json!({ "languages": ["portuguese"] }));
}

/// A.Clockwork.Orange.1971.BRDRIP.1080p.DUAL.PORT-BR.ENG.gmenezes.m
#[test]
fn a_clockwork_orange_1971_brdrip_1080p_dual_port_br_eng_gmenezes_m() {
    assert_includes(
        "A.Clockwork.Orange.1971.BRDRIP.1080p.DUAL.PORT-BR.ENG.gmenezes.m",
        json!({ "languages": ["dual audio", "english", "portuguese"] }),
    );
}

/// Superman I - O Filme 1978 Leg. BR - Mkv 1280x528
#[test]
fn superman_i_o_filme_1978_leg_br_mkv_1280x528() {
    assert_includes("Superman I - O Filme 1978 Leg. BR - Mkv 1280x528", json!({ "languages": ["portuguese"] }));
}

/// Subs/Brazilian.por.srt
#[test]
fn subs_brazilian_por_srt() {
    assert_includes("Subs/Brazilian.por.srt", json!({ "languages": ["portuguese"] }));
}

/// Brazilian Portuguese.por.srt
#[test]
fn brazilian_portuguese_por_srt() {
    assert_includes("Brazilian Portuguese.por.srt", json!({ "languages": ["portuguese"] }));
}

/// [S0.E07] Gambit królowej - Gra końcowa.Portuguese Brazil.srt
#[test]
fn s0_e07_gambit_kro_lowej_gra_kon_cowa_portuguese_brazil_srt() {
    assert_includes(
        "[S0.E07] Gambit królowej - Gra końcowa.Portuguese Brazil.srt",
        json!({ "languages": ["portuguese"] }),
    );
}

/// The Hit List (2011) DVD NTSC WS (eng-fre-pt-spa) [Sk]
#[test]
fn the_hit_list_2011_dvd_ntsc_ws_eng_fre_pt_spa_sk() {
    assert_includes(
        "The Hit List (2011) DVD NTSC WS (eng-fre-pt-spa) [Sk]",
        json!({ "languages": ["english", "french", "spanish"] }),
    );
}

/// [POPAS] Neon Genesis Evangelion: The End of Evangelion [jp_PT-pt
#[test]
fn popas_neon_genesis_evangelion_the_end_of_evangelion_jp_pt_pt() {
    assert_includes(
        "[POPAS] Neon Genesis Evangelion: The End of Evangelion [jp_PT-pt",
        json!({ "languages": ["japanese", "portuguese"] }),
    );
}

/// Zola Maseko - Drum (2004) PT subs
#[test]
fn zola_maseko_drum_2004_pt_subs() {
    assert_includes("Zola Maseko - Drum (2004) PT subs", json!({ "languages": ["portuguese"] }));
}

/// Idrissa Ouedraogo - Yaaba (1989) EN ES FR PT
#[test]
fn idrissa_ouedraogo_yaaba_1989_en_es_fr_pt() {
    assert_includes(
        "Idrissa Ouedraogo - Yaaba (1989) EN ES FR PT",
        json!({ "languages": ["english", "french", "spanish"] }),
    );
}

/// Metallica.Through.The.Never.2013 O Filme(leg.pt-pt)
#[test]
fn metallica_through_the_never_2013_o_filme_leg_pt_pt() {
    assert_includes("Metallica.Through.The.Never.2013 O Filme(leg.pt-pt)", json!({ "languages": ["portuguese"] }));
}

/// Dinossauro (2000) --[ Ing / Pt / Esp ]
#[test]
fn dinossauro_2000_ing_pt_esp() {
    assert_includes("Dinossauro (2000) --[ Ing / Pt / Esp ]", json!({ "languages": ["english", "spanish"] }));
}

/// Mulan 1 (1998) Versão Portuguesa
#[test]
fn mulan_1_1998_versa_o_portuguesa() {
    assert_includes("Mulan 1 (1998) Versão Portuguesa", json!({ "languages": ["portuguese"] }));
}

/// The Guard 2011.DK.EN.ES.HR.NL.PT.RO.Subtitles
#[test]
fn the_guard_2011_dk_en_es_hr_nl_pt_ro_subtitles() {
    assert_includes(
        "The Guard 2011.DK.EN.ES.HR.NL.PT.RO.Subtitles",
        json!({ "languages": ["english", "spanish", "romanian", "croatian", "dutch", "danish"] }),
    );
}

/// Titan.A.E.2000 720p  HDTV DTS Eng Fra Hun Rom Rus multisub
#[test]
fn titan_a_e_2000_720p_hdtv_dts_eng_fra_hun_rom_rus_multisub() {
    assert_includes(
        "Titan.A.E.2000 720p  HDTV DTS Eng Fra Hun Rom Rus multisub",
        json!({ "languages": ["multi subs", "english", "french", "russian", "hungarian", "romanian"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Latin American es.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_latin_american_es_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Latin American es.ass",
        json!({ "languages": ["latino"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Brazilian pt.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_brazilian_pt_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.Brazilian pt.ass",
        json!({ "languages": ["portuguese"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.pt.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_pt_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.pt.ass",
        json!({ "languages": ["portuguese"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.es.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_es_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.es.ass",
        json!({ "languages": ["spanish"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.de.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_de_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.de.ass",
        json!({ "languages": ["german"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.it.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_it_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.it.ass",
        json!({ "languages": ["italian"] }),
    );
}

/// Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.ar.ass
#[test]
fn frieren_beyond_journey_s_end_s01_e01_tba_webdl_1080p_ar_ass() {
    assert_includes(
        "Frieren - Beyond Journey's End - S01E01 - TBA WEBDL-1080p.ar.ass",
        json!({ "languages": ["arabic"] }),
    );
}

/// Subs(ara,fre,ger).srt
#[test]
fn subs_ara_fre_ger_srt() {
    assert_includes("Subs(ara,fre,ger).srt", json!({ "languages": ["french", "german", "arabic"] }));
}

/// Subs(chi,eng,ind,kor,may,tha,vie).srt
#[test]
fn subs_chi_eng_ind_kor_may_tha_vie_srt() {
    assert_includes(
        "Subs(chi,ind,kor,may,tha,vie).srt",
        json!({ "languages": ["korean", "chinese", "vietnamese", "indonesian", "thai", "malay"] }),
    );
}

/// Miami.Bici.2020.1080p.NETFLIX.WEB-DL.DDP5.1.H.264.EN-ROSub-ExtremlymTorrents
#[test]
fn miami_bici_2020_1080p_netflix_web_dl_ddp5_1_h_264_en_rosub_extremlym_torrents() {
    assert_includes(
        "Miami.Bici.2020.1080p.NETFLIX.WEB-DL.DDP5.1.H.264.EN-ROSub-ExtremlymTorrents",
        json!({ "languages": ["english", "romanian"] }),
    );
}

/// 26_Romanian.srt
#[test]
fn t_26_romanian_srt() {
    assert_includes("26_Romanian.srt", json!({ "languages": ["romanian"] }));
}

/// 4_Bulgarian.srt
#[test]
fn t_4_bulgarian_srt() {
    assert_includes("4_Bulgarian.srt", json!({ "languages": ["bulgarian"] }));
}

/// 18_srp.srt
#[test]
fn t_18_srp_srt() {
    assert_includes("18_srp.srt", json!({ "languages": ["serbian"] }));
}

/// Aranyelet.S01.HUNGARIAN.1080p.WEBRip.DDP5.1.x264-SbR[rartv]
#[test]
fn aranyelet_s01_hungarian_1080p_webrip_ddp5_1_x264_sb_r_rartv() {
    assert_includes(
        "Aranyelet.S01.HUNGARIAN.1080p.WEBRip.DDP5.1.x264-SbR[rartv]",
        json!({ "languages": ["hungarian"] }),
    );
}

/// Ponyo[2008]DvDrip-H264 Quad Audio[Eng Jap Fre Spa]AC3 5.1[DXO]
#[test]
fn ponyo_2008_dv_drip_h264_quad_audio_eng_jap_fre_spa_ac3_5_1_dxo() {
    assert_includes(
        "Ponyo[2008]DvDrip-H264 Quad Audio[Eng Jap Fre Spa]AC3 5.1[DXO]",
        json!({ "languages": ["english", "japanese", "french", "spanish"] }),
    );
}

/// The Mechanic [1972] Eng,Deu,Fra,Esp,Rus + multisub DVDrip
#[test]
fn the_mechanic_1972_eng_deu_fra_esp_rus_multisub_dvdrip() {
    assert_includes(
        "The Mechanic [1972] Eng,Deu,Fra,Esp,Rus + multisub DVDrip",
        json!({ "languages": ["multi subs", "english", "french", "spanish", "german", "russian"] }),
    );
}

/// Mommie Dearest [1981 PAL DVD][En.De.Fr.It.Es Multisubs[18]
#[test]
fn mommie_dearest_1981_pal_dvd_en_de_fr_it_es_multisubs_18() {
    assert_includes(
        "Mommie Dearest [1981 PAL DVD][En.De.Fr.It.Es Multisubs[18]",
        json!({ "languages": ["multi subs", "english", "french", "spanish", "german"] }),
    );
}

/// Pasienio sargyba S01E03 (2016 WEBRip LT)
#[test]
fn pasienio_sargyba_s01_e03_2016_webrip_lt() {
    assert_includes("Pasienio sargyba S01E03 (2016 WEBRip LT)", json!({ "languages": ["lithuanian"] }));
}

/// 24_Lithuanian.srt
#[test]
fn t_24_lithuanian_srt() {
    assert_includes("24_Lithuanian.srt", json!({ "languages": ["lithuanian"] }));
}

/// 25_Latvian.srt
#[test]
fn t_25_latvian_srt() {
    assert_includes("25_Latvian.srt", json!({ "languages": ["latvian"] }));
}

/// 13_Estonian.srt
#[test]
fn t_13_estonian_srt() {
    assert_includes("13_Estonian.srt", json!({ "languages": ["estonian"] }));
}

/// Ip.Man.4.The.Finale.2019.CHINESE.1080p.BluRay.x264.TrueHD.7.1.Atmos-HDC
#[test]
fn ip_man_4_the_finale_2019_chinese_1080p_blu_ray_x264_true_hd_7_1_atmos_hdc() {
    assert_includes(
        "Ip.Man.4.The.Finale.2019.CHINESE.1080p.BluRay.x264.TrueHD.7.1.Atmos-HDC",
        json!({ "languages": ["chinese"] }),
    );
}

/// should parse CHT language
#[test]
fn should_parse_cht_language() {
    assert_includes(
        "[NC-Raws] 叫我對大哥 (WEB版) / Ore, Tsushima - 10 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4]",
        json!({ "languages": ["chinese"] }),
    );
}

/// Inuyasha_TV+Finale+OVA+Film+CD+Manga+Other; dub jpn,chn,eng sub chs (2019-09-21)
#[test]
fn inuyasha_tv_finale_ova_film_cd_manga_other_dub_jpn_chn_eng_sub_chs_2019_09_21() {
    assert_includes(
        "Inuyasha_TV+Finale+OVA+Film+CD+Manga+Other; dub jpn,chn,eng sub chs (2019-09-21)",
        json!({ "languages": ["english", "japanese", "chinese"] }),
    );
}

/// Initial D Live Action 2005 ENG/CHI
#[test]
fn initial_d_live_action_2005_eng_chi() {
    assert_includes("Initial D Live Action 2005 ENG/CHI", json!({ "languages": ["english", "chinese"] }));
}

/// Wolf.Warrior.2015.720p.BluRay.x264.Mandarin.AAC-ETRG
#[test]
fn wolf_warrior_2015_720p_blu_ray_x264_mandarin_aac_etrg() {
    assert_includes("Wolf.Warrior.2015.720p.BluRay.x264.Mandarin.AAC-ETRG", json!({ "languages": ["chinese"] }));
}

/// 9_zh-Hans.srt
#[test]
fn t_9_zh_hans_srt() {
    assert_includes("9_zh-Hans.srt", json!({ "languages": ["chinese"] }));
}

/// Traditional Chinese.chi.srt
#[test]
fn traditional_chinese_chi_srt() {
    assert_includes("Traditional Chinese.chi.srt", json!({ "languages": ["taiwanese"] }));
}

/// Subs/Promare - Chinese (Traditional).ass
#[test]
fn subs_promare_chinese_traditional_ass() {
    assert_includes("Subs/Promare - Chinese (Traditional).ass", json!({ "languages": ["taiwanese"] }));
}

/// 10_zh-Hant.srt
#[test]
fn t_10_zh_hant_srt() {
    assert_includes("10_zh-Hant.srt", json!({ "languages": ["taiwanese"] }));
}

/// Berserk 01-25 [dual audio JP,EN] MKV
#[test]
fn berserk_01_25_dual_audio_jp_en_mkv() {
    assert_includes(
        "Berserk 01-25 [dual audio JP,EN] MKV",
        json!({ "languages": ["dual audio", "english", "japanese"] }),
    );
}

/// FLCL S05.1080p HMAX WEB-DL DD2.0 H 264-VARYG (FLCL: Shoegaze Dual-Audio Multi-Subs)
#[test]
fn flcl_s05_1080p_hmax_web_dl_dd2_0_h_264_varyg_flcl_shoegaze_dual_audio_multi_subs() {
    assert_includes(
        "FLCL S05.1080p HMAX WEB-DL DD2.0 H 264-VARYG (FLCL: Shoegaze Dual-Audio Multi-Subs)",
        json!({ "languages": ["multi subs", "dual audio"] }),
    );
}

/// Shinjuku Swan 2015 JAP 1080p BluRay x264 DTS-JYK
#[test]
fn shinjuku_swan_2015_jap_1080p_blu_ray_x264_dts_jyk() {
    assert_includes("Shinjuku Swan 2015 JAP 1080p BluRay x264 DTS-JYK", json!({ "languages": ["japanese"] }));
}

/// Wet.Woman.in.the.Wind.2016.JAPANESE.1080p.BluRay.x264-iKiW
#[test]
fn wet_woman_in_the_wind_2016_japanese_1080p_blu_ray_x264_i_ki_w() {
    assert_includes("Wet.Woman.in.the.Wind.2016.JAPANESE.1080p.BluRay.x264-iKiW", json!({ "languages": ["japanese"] }));
}

/// All.Love.E146.KOR.HDTV.XViD-DeBTV
#[test]
fn all_love_e146_kor_hdtv_xvi_d_de_btv() {
    assert_includes("All.Love.E146.KOR.HDTV.XViD-DeBTV", json!({ "languages": ["korean"] }));
}

/// The.Nun.2018.KORSUB.HDRip.XviD.MP3-STUTTERSHIT
#[test]
fn the_nun_2018_korsub_hdrip_xvi_d_mp3_stuttershit() {
    assert_includes("The.Nun.2018.KORSUB.HDRip.XviD.MP3-STUTTERSHIT", json!({ "languages": ["korean"] }));
}

/// Burning.2018.KOREAN.720p.BluRay.H264.AAC-VXT
#[test]
fn burning_2018_korean_720p_blu_ray_h264_aac_vxt() {
    assert_includes("Burning.2018.KOREAN.720p.BluRay.H264.AAC-VXT", json!({ "languages": ["korean"] }));
}

/// Atonement.2017.KOREAN.ENSUBBED.1080p.WEBRip.x264-VXT
#[test]
fn atonement_2017_korean_ensubbed_1080p_webrip_x264_vxt() {
    assert_includes(
        "Atonement.2017.KOREAN.ENSUBBED.1080p.WEBRip.x264-VXTT",
        json!({ "languages": ["english", "korean"] }),
    );
}

/// A Freira (2018) Dublado HD-TS 720p
#[test]
fn a_freira_2018_dublado_hd_ts_720p() {
    assert_includes("A Freira (2018) Dublado HD-TS 720p", json!({ "languages": ["portuguese"] }));
}

/// Escobar El Patron Del Mal Capitulo 91 SD (2012-10-10) [SiRaDuDe]
#[test]
fn escobar_el_patron_del_mal_capitulo_91_sd_2012_10_10_si_ra_du_de() {
    assert_includes(
        "Escobar El Patron Del Mal Capitulo 91 SD (2012-10-10) [SiRaDuDe]",
        json!({ "languages": ["portuguese"] }),
    );
}

/// Bleach - 215 ao 220 - [DB-BR]
#[test]
fn bleach_215_ao_220_db_br() {
    assert_includes("Bleach - 215 ao 220 - [DB-BR]", json!({ "languages": ["portuguese"] }));
}

/// Joker.2019.MULTi.Bluray.1080p.Atmos.7.1.En.Fr.Sp.Pt-DDR[EtHD]
#[test]
fn joker_2019_multi_bluray_1080p_atmos_7_1_en_fr_sp_pt_ddr_et_hd() {
    assert_includes(
        "Joker.2019.MULTi.Bluray.1080p.Atmos.7.1.En.Fr.Sp.Pt-DDR[EtHD]",
        json!({ "languages": ["multi audio", "english", "french"] }),
    );
}

/// Dilbert complete series + en subs
#[test]
fn dilbert_complete_series_en_subs() {
    assert_includes("Dilbert complete series + en subs", json!({ "languages": ["english"] }));
}

/// The Next Karate Kid (1994) NTSC WS -Eng/Fre/Spa/Por- [ctang]
#[test]
fn the_next_karate_kid_1994_ntsc_ws_eng_fre_spa_por_ctang() {
    assert_includes(
        "The Next Karate Kid (1994) NTSC WS -Eng/Fre/Spa/Por- [ctang]",
        json!({ "languages": ["english", "french", "spanish", "portuguese"] }),
    );
}

/// arsenico por compasion 1944 Capra spanish castellano
#[test]
#[ignore = "pending in the js suite"]
fn arsenico_por_compasion_1944_capra_spanish_castellano() {
    assert_includes("arsenico por compasion 1944 Capra spanish castellano", json!({ "languages": ["spanish"] }));
}

/// Un.Homme.Et.Une.Femme.1966.DVDRip.XviD.AR [PT ENG ESP]
#[test]
fn un_homme_et_une_femme_1966_dvdrip_xvi_d_ar_pt_eng_esp() {
    assert_includes(
        "Un.Homme.Et.Une.Femme.1966.DVDRip.XviD.AR [PT ENG ESP]",
        json!({ "languages": ["english", "spanish", "portuguese"] }),
    );
}

/// Geminis.2005.Argentina.ESP.ENG.PT.SUBS
#[test]
fn geminis_2005_argentina_esp_eng_pt_subs() {
    assert_includes(
        "Geminis.2005.Argentina.ESP.ENG.PT.SUBS",
        json!({ "languages": ["english", "spanish", "portuguese"] }),
    );
}

/// 1917 2019 1080p Bluray x264-Sexmeup [Greek Subs] [Braveheart]
#[test]
fn t_1917_2019_1080p_bluray_x264_sexmeup_greek_subs_braveheart() {
    assert_includes("1917 2019 1080p Bluray x264-Sexmeup [Greek Subs] [Braveheart]", json!({ "languages": ["greek"] }));
}

/// The Lion King 1,2,3 Greek Language
#[test]
fn the_lion_king_1_2_3_greek_language() {
    assert_includes("The Lion King 1,2,3 Greek Language", json!({ "languages": ["greek"] }));
}

/// The Adams Family (1991) (Greek Subs integratet)
#[test]
fn the_adams_family_1991_greek_subs_integratet() {
    assert_includes("The Adams Family (1991) (Greek Subs integratet)", json!({ "languages": ["greek"] }));
}

/// 6_Greek.srt
#[test]
fn t_6_greek_srt() {
    assert_includes("6_Greek.srt", json!({ "languages": ["greek"] }));
}

/// The Insult (2017) [BluRay] [720p] Arabic
#[test]
fn the_insult_2017_blu_ray_720p_arabic() {
    assert_includes("The Insult (2017) [BluRay] [720p] Arabic", json!({ "languages": ["arabic"] }));
}

/// The.Mexican.2001 - Arabic Subs Hardcoded
#[test]
fn the_mexican_2001_arabic_subs_hardcoded() {
    assert_includes("The.Mexican.2001 - Arabic Subs Hardcoded", json!({ "languages": ["arabic"] }));
}

/// Much Loved (2015) - DVDRip x265 HEVC - ARAB-ITA-FRE AUDIO (ENG S
#[test]
fn much_loved_2015_dvdrip_x265_hevc_arab_ita_fre_audio_eng_s() {
    assert_includes(
        "Much Loved (2015) - DVDRip x265 HEVC - ARAB-ITA-FRE AUDIO (ENG S",
        json!({ "languages": ["english", "french", "italian", "arabic"] }),
    );
}

/// 42.2013.720p.BluRay.x264.HD4Ar Arab subtitle
#[test]
fn t_42_2013_720p_blu_ray_x264_hd4_ar_arab_subtitle() {
    assert_includes("42.2013.720p.BluRay.x264.HD4Ar Arab subtitle", json!({ "languages": ["arabic"] }));
}

/// Fauda.S01.HEBREW.1080p.NF.WEBRip.DD5.1.x264-TrollHD[rartv]
#[test]
fn fauda_s01_hebrew_1080p_nf_webrip_dd5_1_x264_troll_hd_rartv() {
    assert_includes("Fauda.S01.HEBREW.1080p.NF.WEBRip.DD5.1.x264-TrollHD[rartv]", json!({ "languages": ["hebrew"] }));
}

/// madagascar 720p hebrew dubbed.mkv
#[test]
fn madagascar_720p_hebrew_dubbed_mkv() {
    assert_includes("madagascar 720p hebrew dubbed.mkv", json!({ "languages": ["hebrew"] }));
}

/// Into.the.Night.S01E04.Ayaz.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track17_[heb].srt
#[test]
fn into_the_night_s01_e04_ayaz_1080p_nf_web_dl_ddp5_1_x264_ntg_track17_heb_srt() {
    assert_includes(
        "Into.the.Night.S01E04.Ayaz.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track17_[heb].srt",
        json!({ "languages": ["hebrew"] }),
    );
}

/// The.Protector.2018.S03.TURKISH.WEBRip.x264-ION10
#[test]
fn the_protector_2018_s03_turkish_webrip_x264_ion10() {
    assert_includes("The.Protector.2018.S03.TURKISH.WEBRip.x264-ION10", json!({ "languages": ["turkish"] }));
}

/// Recep Ivedik 6 (2020) NETFLIX 720p WEBDL (Turkish) - ExtremlymTorrents
#[test]
fn recep_ivedik_6_2020_netflix_720p_webdl_turkish_extremlym_torrents() {
    assert_includes(
        "Recep Ivedik 6 (2020) NETFLIX 720p WEBDL (Turkish) - ExtremlymTorrents",
        json!({ "languages": ["turkish"] }),
    );
}

/// The Insider*1999*[DVD5][PAL][ENG, POL, sub. ROM, TUR]
#[test]
fn the_insider_1999_dvd5_pal_eng_pol_sub_rom_tur() {
    assert_includes(
        "The Insider*1999*[DVD5][PAL][ENG, POL, sub. ROM, TUR]",
        json!({ "languages": ["english", "polish", "romanian", "turkish"] }),
    );
}

/// Divorzio allitaliana [XviD - Ita Mp3 - Sub Eng Esp Tur]
#[test]
fn divorzio_allitaliana_xvi_d_ita_mp3_sub_eng_esp_tur() {
    assert_includes(
        "Divorzio allitaliana [XviD - Ita Mp3 - Sub Eng Esp Tur]",
        json!({ "languages": ["english", "spanish", "italian", "turkish"] }),
    );
}

/// 2007 Saturno Contro [Saturn In Opposition] (ITA-FRA-TUR) [EngSub
#[test]
fn t_2007_saturno_contro_saturn_in_opposition_ita_fra_tur_eng_sub() {
    assert_includes(
        "2007 Saturno Contro [Saturn In Opposition] (ITA-FRA-TUR) [EngSub",
        json!({ "languages": ["english", "french", "italian", "turkish"] }),
    );
}

/// Cowboy Bebop - 1080p BDrip Audio+sub MULTI (VF / VOSTFR)
#[test]
fn cowboy_bebop_1080p_bdrip_audio_sub_multi_vf_vostfr() {
    assert_includes(
        "Cowboy Bebop - 1080p BDrip Audio+sub MULTI (VF / VOSTFR)",
        json!({ "languages": ["multi audio", "french"] }),
    );
}

/// Casablanca 1942 BDRip 1080p [multi language,multi subs].mkv
#[test]
fn casablanca_1942_bdrip_1080p_multi_language_multi_subs_mkv() {
    assert_includes(
        "Casablanca 1942 BDRip 1080p [multi language,multi subs].mkv",
        json!({ "languages": ["multi subs", "multi audio"] }),
    );
}

/// Avengers.Endgame.2019.4K.UHD.ITUNES.DL.H265.Dolby.ATMOS.MSUBS-Deflate.Telly
#[test]
fn avengers_endgame_2019_4_k_uhd_itunes_dl_h265_dolby_atmos_msubs_deflate_telly() {
    assert_includes(
        "Avengers.Endgame.2019.4K.UHD.ITUNES.DL.H265.Dolby.ATMOS.MSUBS-Deflate.Telly",
        json!({ "languages": ["multi subs"] }),
    );
}

/// Dawn of the Planet of the Apes (2014) 720p BluRay x264 - MULTI S
#[test]
fn dawn_of_the_planet_of_the_apes_2014_720p_blu_ray_x264_multi_s() {
    assert_includes(
        "Dawn of the Planet of the Apes (2014) 720p BluRay x264 - MULTI S",
        json!({ "languages": ["multi subs"] }),
    );
}

/// Pirates of the Caribbean On Stranger Tides (2011) DVD5 (Multi Su
#[test]
fn pirates_of_the_caribbean_on_stranger_tides_2011_dvd5_multi_su() {
    assert_includes(
        "Pirates of the Caribbean On Stranger Tides (2011) DVD5 (Multi Su",
        json!({ "languages": ["multi subs"] }),
    );
}

/// Jumanji The Next Level (2019) 720p HDCAM Ads Blurred x264 Dual A
#[test]
fn jumanji_the_next_level_2019_720p_hdcam_ads_blurred_x264_dual_a() {
    assert_includes(
        "Jumanji The Next Level (2019) 720p HDCAM Ads Blurred x264 Dual A",
        json!({ "languages": ["dual audio"] }),
    );
}

/// Men in Black International (2019) 720p Korsub HDRip x264 ESub [Dual Line Audio] [Hindi English]
#[test]
fn men_in_black_international_2019_720p_korsub_hdrip_x264_esub_dual_line_audio_hindi_english() {
    assert_includes(
        "Men in Black International (2019) 720p Korsub HDRip x264 ESub [Dual Line Audio] [Hindi English]",
        json!({ "languages": ["dual audio", "english", "korean", "hindi"] }),
    );
}

/// Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]
#[test]
fn fame_1980_dvdrip_dual_ac3_eng_spa() {
    assert_includes(
        "Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]",
        json!({ "languages": ["dual audio", "english", "spanish"] }),
    );
}

/// O Rei do Show 2018 Dual Áudio 4K UtraHD By.Luan.Harper
#[test]
fn o_rei_do_show_2018_dual_a_udio_4_k_utra_hd_by_luan_harper() {
    assert_includes("O Rei do Show 2018 Dual Áudio 4K UtraHD By.Luan.Harper.", json!({ "languages": ["dual audio"] }));
}

/// Cowboy Bebop The Movie (2001) BD 1080p.x265.Tri-Audio.Ita.Eng.Jap [Rady]
#[test]
fn cowboy_bebop_the_movie_2001_bd_1080p_x265_tri_audio_ita_eng_jap_rady() {
    assert_includes(
        "Cowboy Bebop The Movie (2001) BD 1080p.x265.Tri-Audio.Ita.Eng.Jap [Rady]",
        json!({ "languages": ["multi audio", "english", "japanese", "italian"] }),
    );
}

/// [IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][Dublado][HEVC 10Bits] 800p BD
#[test]
fn ice_blue_naruto_season_01_multi_dub_multi_sub_dublado_hevc_10_bits_800p_bd() {
    assert_includes(
        "[IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][HEVC 10Bits] 800p BD",
        json!({ "languages": ["multi subs", "multi audio"] }),
    );
}

/// Blue Seed - 01 (BDRip 720x480p x265 HEVC FLAC, AC3x2 2.0x3)(Triple Audio)[sxales].mkv
#[test]
fn blue_seed_01_bdrip_720x480p_x265_hevc_flac_ac3x2_2_0x3_triple_audio_sxales_mkv() {
    assert_includes(
        "Blue Seed - 01 (BDRip 720x480p x265 HEVC FLAC, AC3x2 2.0x3)(Triple Audio)[sxales].mkv",
        json!({ "languages": ["multi audio"] }),
    );
}

/// Ministri S01E02 SLOVAK 480p x264-mSD
#[test]
fn ministri_s01_e02_slovak_480p_x264_m_sd() {
    assert_includes("Ministri S01E02 SLOVAK 480p x264-mSD", json!({ "languages": ["slovakian"] }));
}

/// Subs/35_slo.srt
#[test]
fn subs_35_slo_srt() {
    assert_includes("Subs/35_slo.srt", json!({ "languages": ["slovakian"] }));
}

/// Seinfeld.COMPLETE.SLOSUBS.DVDRip.XviD
#[test]
fn seinfeld_complete_slosubs_dvdrip_xvi_d() {
    assert_includes("Seinfeld.COMPLETE.SLOSUBS.DVDRip.XviD", json!({ "languages": ["slovakian"] }));
}

/// Subs/36_Slovenian.srt
#[test]
fn subs_36_slovenian_srt() {
    assert_includes("Subs/36_Slovenian.srt", json!({ "languages": ["slovenian"] }));
}

/// The House Bunny (2008) BDRemux 1080p MediaClub [RUS, UKR, ENG]
#[test]
fn the_house_bunny_2008_bdremux_1080p_media_club_rus_ukr_eng() {
    assert_includes(
        "The House Bunny (2008) BDRemux 1080p MediaClub [RUS, UKR, ENG]",
        json!({ "languages": ["english", "russian", "ukrainian"] }),
    );
}

/// L'immortel (2010) DVDRip AVC (Russian,Ukrainian)
#[test]
fn l_immortel_2010_dvdrip_avc_russian_ukrainian() {
    assert_includes(
        "L'immortel (2010) DVDRip AVC (Russian,Ukrainian)",
        json!({ "languages": ["russian", "ukrainian"] }),
    );
}

/// Into.the.Night.S01E03.Mathieu.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track33_[vie].srt
#[test]
fn into_the_night_s01_e03_mathieu_1080p_nf_web_dl_ddp5_1_x264_ntg_track33_vie_srt() {
    assert_includes(
        "Into.the.Night.S01E03.Mathieu.1080p.NF.WEB-DL.DDP5.1.x264-NTG_track33_[vie].srt",
        json!({ "languages": ["vietnamese"] }),
    );
}

/// Subs/vie.srt
#[test]
fn subs_vie_srt() {
    assert_includes("Subs/vie.srt", json!({ "languages": ["vietnamese"] }));
}

/// Subs/Vietnamese.srt
#[test]
fn subs_vietnamese_srt() {
    assert_includes("Subs/Vietnamese.vie.srt", json!({ "languages": ["vietnamese"] }));
}

/// Subs/Dear.S01E05.WEBRip.x265-ION265/25_may.srt
#[test]
fn subs_dear_s01_e05_webrip_x265_ion265_25_may_srt() {
    assert_includes("Subs/Dear.S01E05.WEBRip.x265-ION265/25_may.srt", json!({ "languages": ["malay"] }));
}

/// Midnight.Diner.Tokyo.Stories.S02E10.WEBRip.x264-ION10/14_Indonesian.srt
#[test]
fn midnight_diner_tokyo_stories_s02_e10_webrip_x264_ion10_14_indonesian_srt() {
    assert_includes(
        "Midnight.Diner.Tokyo.Stories.S02E10.WEBRip.x264-ION10/14_Indonesian.srt",
        json!({ "languages": ["indonesian"] }),
    );
}

/// should detect portuguese languages
#[test]
fn should_detect_portuguese_languages() {
    assert_includes(
        "Inglês,Português,Italiano,Francês,Polonês,Russo,Norueguês,Dinamarquês,Alemão,Espanhol,Chinês,Japonês,Coreano,Persa,Hebraico,Sueco,Árabe,Holandês,Tâmil,Tailandês",
        json!({ "languages": ["english", "japanese", "korean", "chinese", "french", "spanish", "portuguese", "italian", "german", "russian", "tamil", "polish", "dutch", "danish", "swedish", "norwegian", "thai", "hebrew", "persian"] }),
    );
}

/// should detect multiple languages in language only title
#[test]
fn should_detect_multiple_languages_in_language_only_title() {
    assert_includes("russian,english,ukrainian", json!({ "languages": ["english", "russian", "ukrainian"] }));
}

/// Subs/Thai.srt
#[test]
fn subs_thai_srt() {
    assert_includes("Subs/Thai.srt", json!({ "languages": ["thai"] }));
}

/// Food Choices (2016) WEB.1080p.H264_tha.srt
#[test]
fn food_choices_2016_web_1080p_h264_tha_srt() {
    assert_includes("Food Choices (2016) WEB.1080p.H264_tha.srt", json!({ "languages": ["thai"] }));
}

/// Ekk Deewana Tha (2012) DVDRip 720p x264 AAC TaRa.mkv
#[test]
fn ekk_deewana_tha_2012_dvdrip_720p_x264_aac_ta_ra_mkv() {
    assert_missing("Ekk Deewana Tha (2012) DVDRip 720p x264 AAC TaRa.mkv", &["languages"]);
}

/// My Big Fat Greek Wedding (2002) 720p BrRip x264 - YIFY
#[test]
fn my_big_fat_greek_wedding_2002_720p_br_rip_x264_yify() {
    assert_missing("My Big Fat Greek Wedding (2002) 720p BrRip x264 - YIFY", &["languages"]);
}

/// Get Him to the Greek 2010 720p BluRay
#[test]
fn get_him_to_the_greek_2010_720p_blu_ray() {
    assert_missing("Get Him to the Greek 2010 720p BluRay", &["languages"]);
}

/// [Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR
#[test]
fn hakata_ramen_hoshiai_no_sora_stars_align_01_1080p_hevc_x265_10bit_dual_subs_hr_dr() {
    assert_missing(
        "[Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR",
        &["languages"],
    );
}

/// should not remove english from title
#[test]
fn should_not_remove_english_from_title() {
    assert_includes("The English Patient (1996) 720p BrRip x264 - YIFY", json!({ "title": "The English Patient" }));
    assert_missing("The English Patient (1996) 720p BrRip x264 - YIFY", &["languages"]);
}

/// should not detect LT language from yts domain name
#[test]
fn should_not_detect_lt_language_from_yts_domain_name() {
    assert_missing("Do.Or.Die.1991.1080p.BluRay.x264-[YTS.LT].mp4", &["languages"]);
}

/// should not detect PT language from temporada season naming
#[test]
fn should_not_detect_pt_language_from_temporada_season_naming() {
    assert_missing("Castlevania 2017 1º temporada completa 1080p", &["languages"]);
}

/// should not detect PT language with cap. episode title
#[test]
fn should_not_detect_pt_language_with_cap_episode_title() {
    assert_missing("City on a Hill - Temporada 1 [HDTV][Cap.110].avi", &["languages"]);
}

/// should not detect NL language from website
#[test]
fn should_not_detect_nl_language_from_website() {
    assert_missing("La inocencia [720p][wWw.EliteTorrent.NL].mkv", &["languages"]);
}

/// should not detect FI language from website
#[test]
fn should_not_detect_fi_language_from_website() {
    assert_missing("Reasonable Doubt - Temporada 1 [HDTV][Cap.101][www.AtomoHD.FI]", &["languages"]);
}

/// should not detect FI language from website v2
#[test]
fn should_not_detect_fi_language_from_website_v2() {
    assert_missing("1883 - Temporada 1 [HDTV 720p][Cap.103][AC3 5.1][www.AtomoHD.fi]", &["languages"]);
}

/// should not detect TW language from website
#[test]
fn should_not_detect_tw_language_from_website() {
    assert_missing("Los Winchester - Temporada 1 [HDTV][Cap.103][www.atomoHD.tw]", &["languages"]);
}

/// should not detect CH language from website
#[test]
fn should_not_detect_ch_language_from_website() {
    assert_missing("El Inmortal- Temporada 1 [HDTV 720p][Cap.104][AC3 5.1][www.AtomoHD.ch]]", &["languages"]);
}

/// should not detect TEL language from website
#[test]
fn should_not_detect_tel_language_from_website() {
    assert_missing("Black Friday (2021) [BluRay Rip][AC3 5.1][www.atomixHQ.TEL]", &["languages"]);
}

/// should not detect SE language from website
#[test]
fn should_not_detect_se_language_from_website() {
    assert_missing("Deep Blue Sea 3 [HDR][wWw.EliteTorrent.SE]", &["languages"]);
}

/// should not detect language from title before year
#[test]
fn should_not_detect_language_from_title_before_year() {
    assert_missing("The.Italian.Job.1969.1080p.BluRay.x265-RARBG.mp4", &["languages"]);
}

/// should not detect language from title before year v2
#[test]
fn should_not_detect_language_from_title_before_year_v2() {
    assert_missing("Chinese Zodiac (2012) 1080p BrRip x264 - YIFY", &["languages"]);
}

/// should not detect language from title before year v3
#[test]
fn should_not_detect_language_from_title_before_year_v3() {
    assert_missing("The German Doctor 2013 1080p WEBRip", &["languages"]);
}

/// should not detect language from title before year v4
#[test]
fn should_not_detect_language_from_title_before_year_v4() {
    assert_missing("Johnny English 2003 1080p BluRay", &["languages"]);
}

/// should not detect language from title before year v5
#[test]
fn should_not_detect_language_from_title_before_year_v5() {
    assert_missing("Polish Wedding (1998) 1080p (moviesbyrizzo upl).mp4", &["languages"]);
}

/// should not detect language from title before year v6
#[test]
fn should_not_detect_language_from_title_before_year_v6() {
    assert_missing("Russian.Doll.S02E02.2160p.NF.WEB-DL.DDP5.1.HDR.DV.HEVC-PEXA.mkv", &["languages"]);
}

/// should not detect language from title before year v7
#[test]
fn should_not_detect_language_from_title_before_year_v7() {
    assert_missing("The.Spanish.Prisoner.1997.1080p.BluRay.x265-RARBG", &["languages"]);
}

/// should not detect language from title before year v8
#[test]
fn should_not_detect_language_from_title_before_year_v8() {
    assert_missing("Japanese.Story.2003.1080p.WEBRip.x264-RARBG", &["languages"]);
}

/// should not detect language from title before year v9
#[test]
fn should_not_detect_language_from_title_before_year_v9() {
    assert_missing("[ Torrent9.cz ] The.InBetween.S01E10.FiNAL.HDTV.XviD-EXTREME.avi", &["languages"]);
}

/// should not detect language from title before year v10
#[test]
fn should_not_detect_language_from_title_before_year_v10() {
    assert_missing("Thai Massage (2022) 720p PDVDRip x264 AAC.mkv", &["languages"]);
}

/// should not detect dan language
#[test]
fn should_not_detect_dan_language() {
    assert_missing("Carson Daly 2017 09 13 Dan Harmon 720p HDTV x264-CROOKS", &["languages"]);
}

/// should not detect dan language v2
#[test]
fn should_not_detect_dan_language_v2() {
    assert_missing("Dan Browns The Lost Symbol S01E03 1080p WEB H264-GLHF", &["languages"]);
}

/// should not detect ara language
#[test]
fn should_not_detect_ara_language() {
    assert_missing("Ben.Ara.2015.1080p.WEBRip.x265-RARBG.mp4", &["languages"]);
}

/// should not detect ara language v2
#[test]
fn should_not_detect_ara_language_v2() {
    assert_missing("Ara.(A.Break).2008.DVDRip", &["languages"]);
}

/// should detect multi subs with other languages
#[test]
fn should_detect_multi_subs_with_other_languages() {
    assert_includes(
        "X-Men Complete 13 Movie Collection Sci-Fi 2000 - 2020 Eng Rus Multi-Subs 1080p [H264-mp4]",
        json!({ "languages": ["multi subs", "english", "russian"] }),
    );
}

/// should not detect dual audio from an exclaimed show name
#[test]
fn should_not_detect_dual_audio_from_an_exclaimed_show_name() {
    assert_includes(
        "[km&sxales] Dual! Parallel Trouble Adventures 01 [DVD 576p x264 AC3][80EE2516].mkv",
        json!({ "title": "Dual! Parallel Trouble Adventures 01" }),
    );
    assert_missing(
        "[km&sxales] Dual! Parallel Trouble Adventures 01 [DVD 576p x264 AC3][80EE2516].mkv",
        &["languages", "dubbed"],
    );
}

/// should detect a caps language tag after a mixed case title
#[test]
fn should_detect_a_caps_language_tag_after_a_mixed_case_title() {
    assert_includes("Movie.Name.SERBIAN.1080p", json!({ "title": "Movie Name", "languages": ["serbian"] }));
}

/// should detect a lowercase language tag directly before the year
#[test]
fn should_detect_a_lowercase_language_tag_directly_before_the_year() {
    assert_includes("Tanu Weds Manu hindi 2011 DVDRip", json!({ "title": "Tanu Weds Manu", "languages": ["hindi"] }));
}

/// should detect a bracketed language tag before the year
#[test]
fn should_detect_a_bracketed_language_tag_before_the_year() {
    assert_includes(
        "Celda 211 [DVDRIP][Spanish AC3 5.1][2010]",
        json!({ "title": "Celda 211", "languages": ["spanish"] }),
    );
}

/// should detect a language tag followed by movie before the year
#[test]
fn should_detect_a_language_tag_followed_by_movie_before_the_year() {
    assert_includes(
        "ram teri ganga maili hindi movie 1985",
        json!({ "title": "ram teri ganga maili", "languages": ["hindi"] }),
    );
}

/// should detect a spanish line tag before the year
#[test]
fn should_detect_a_spanish_line_tag_before_the_year() {
    assert_includes(
        "Millenium 2 DVD-Screener Spanish Line 2009",
        json!({ "title": "Millenium 2", "languages": ["spanish"] }),
    );
}

/// should detect a caps french tag before the year
#[test]
fn should_detect_a_caps_french_tag_before_the_year() {
    assert_includes("Taken 3 FRENCH BDRip 2014", json!({ "title": "Taken 3", "languages": ["french"] }));
}

/// should detect a caps french tag before the episode and year
#[test]
fn should_detect_a_caps_french_tag_before_the_episode_and_year() {
    assert_includes(
        "Mayor Of Kingstown FRENCH S03E03 HDTV 2024",
        json!({ "title": "Mayor Of Kingstown", "languages": ["french"] }),
    );
}

/// should detect a caps korean tag after a title that starts with the language
#[test]
fn should_detect_a_caps_korean_tag_after_a_title_that_starts_with_the_language() {
    assert_includes(
        "Korean.Pork.Belly.Rhapsody.S01E01.KOREAN.WEBRip.x264-ION10",
        json!({ "title": "Korean Pork Belly Rhapsody", "languages": ["korean"] }),
    );
}

/// should detect english dub after the title
#[test]
fn should_detect_english_dub_after_the_title() {
    assert_includes(
        "Yu Yu Hakusho English Dub 001-112 BD (1280x720 AAC)",
        json!({ "title": "Yu Yu Hakusho", "languages": ["english"] }),
    );
}

/// should detect english dubbed before the year
#[test]
fn should_detect_english_dubbed_before_the_year() {
    assert_includes(
        "Duel of the IRON FIST (english dubbed) 1971 release.avi",
        json!({ "title": "Duel of the IRON FIST", "languages": ["english"] }),
    );
}

/// should not detect a language from a title word before the year
#[test]
fn should_not_detect_a_language_from_a_title_word_before_the_year() {
    assert_includes("The Danish Girl 2015 1080p BluRay x264 DTS-JYK", json!({ "title": "The Danish Girl" }));
    assert_missing("The Danish Girl 2015 1080p BluRay x264 DTS-JYK", &["languages"]);
}

/// should not detect a language code inside an accented title word
#[test]
fn should_not_detect_a_language_code_inside_an_accented_title_word() {
    assert_includes("Fränk 2025 1080p WEBRip", json!({ "title": "Fränk" }));
    assert_missing("Fränk 2025 1080p WEBRip", &["languages"]);
}

/// should not detect swedish from an accented spanish word
#[test]
fn should_not_detect_swedish_from_an_accented_spanish_word() {
    assert_includes(
        "Perdoname Señor - Temporada 1 [HDTV 720p][Cap.101][AC3 5.1 Español Castellano]",
        json!({ "title": "Perdoname Señor", "languages": ["spanish"] }),
    );
}

/// should not detect polish from an accented spanish word
#[test]
fn should_not_detect_polish_from_an_accented_spanish_word() {
    assert_includes(
        "Políticamente incorrectos (2024) [Bluray 720p][Esp]",
        json!({ "title": "Políticamente incorrectos", "languages": ["spanish"] }),
    );
}

/// should detect a bracketed language followed by subtitles without a year
#[test]
fn should_detect_a_bracketed_language_followed_by_subtitles_without_a_year() {
    assert_includes(
        "The Obama Deception [French subtitles].avi",
        json!({ "title": "The Obama Deception", "languages": ["french"] }),
    );
}

/// should detect a language followed by subtitles without a year
#[test]
fn should_detect_a_language_followed_by_subtitles_without_a_year() {
    assert_includes("28 weeks later Dutch Subtitles Lilmjkl DVDRip", json!({ "languages": ["dutch"] }));
}

/// should detect a leading language in a plain comma separated language list
#[test]
fn should_detect_a_leading_language_in_a_plain_comma_separated_language_list() {
    assert_includes("french,english,spanish.srt", json!({ "languages": ["english", "french", "spanish"] }));
}
