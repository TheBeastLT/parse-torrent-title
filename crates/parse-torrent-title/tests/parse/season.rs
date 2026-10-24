// Ported from test/season.js of the js library.

//! Parsing season

use crate::common::*;

/// should detect regular season correctly
#[test]
fn should_detect_regular_season_correctly() {
    assert_includes("The Simpsons S28E21 720p HDTV x264-AVS", json!({ "season": 28 }));
}

/// should detect regular season with lowercase correctly
#[test]
fn should_detect_regular_season_with_lowercase_correctly() {
    assert_includes("breaking.bad.s01e01.720p.bluray.x264-reward", json!({ "season": 1 }));
}

/// should detect regular season with high number
#[test]
fn should_detect_regular_season_with_high_number() {
    assert_includes("48 Hours S51E15 720p WEB x264-CookieMonster", json!({ "season": 51 }));
}

/// should detect regular season with high number v2
#[test]
fn should_detect_regular_season_with_high_number_v2() {
    assert_includes("Bargain.Hunt.S66E02.Builth Wells4.720p.WEB.H264-BeechyBoy.mp4", json!({ "season": 66 }));
}

/// should detect regular season with O instead of zero
#[test]
fn should_detect_regular_season_with_o_instead_of_zero() {
    assert_includes("Arrested Development SO2E04.avi", json!({ "season": 2 }));
}

/// should detect regular season with 3 digits
#[test]
fn should_detect_regular_season_with_3_digits() {
    assert_includes("S011E16.mkv", json!({ "season": 11 }));
}

/// should detect regular season with a space between
#[test]
fn should_detect_regular_season_with_a_space_between() {
    assert_includes("Dragon Ball Super S01 E23 French 1080p HDTV H264-Kesni", json!({ "season": 1 }));
}

/// should detect regular season with a letter a suffix
#[test]
fn should_detect_regular_season_with_a_letter_a_suffix() {
    assert_includes("The Twilight Zone 1985 S01E23a Shadow Play.mp4", json!({ "season": 1 }));
}

/// should detect regular season with a letter b suffix
#[test]
fn should_detect_regular_season_with_a_letter_b_suffix() {
    assert_includes(
        "Mash S10E01b Thats Show Biz Part 2 1080p H.264 (moviesbyrizzo upload).mp4",
        json!({ "season": 10 }),
    );
}

/// should detect regular season with a letter c suffix
#[test]
fn should_detect_regular_season_with_a_letter_c_suffix() {
    assert_includes("The Twilight Zone 1985 S01E22c The Library.mp4", json!({ "season": 1 }));
}

/// should detect regular episode without e separator
#[test]
fn should_detect_regular_episode_without_e_separator() {
    assert_includes("Desperate.Housewives.S0615.400p.WEB-DL.Rus.Eng.avi", json!({ "season": 6 }));
}

/// should detect season with SxEE format correctly
#[test]
fn should_detect_season_with_sx_ee_format_correctly() {
    assert_includes("Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV", json!({ "season": 8 }));
}

/// should detect season when written as such
#[test]
fn should_detect_season_when_written_as_such() {
    assert_includes("Orange Is The New Black Season 5 Episodes 1-10 INCOMPLETE (LEAKED)", json!({ "season": 5 }));
}

/// should detect season with parenthesis prefix and x separator
#[test]
fn should_detect_season_with_parenthesis_prefix_and_x_separator() {
    assert_includes("Smallville (1x02 Metamorphosis).avi", json!({ "season": 1 }));
}

/// should detect season with x separator and letter on left
#[test]
fn should_detect_season_with_x_separator_and_letter_on_left() {
    assert_includes("The.Man.In.The.High.Castle1x01.HDTV.XviD[www.DivxTotaL.com].avi", json!({ "season": 1 }));
}

/// should detect season with x separator and letter on right
#[test]
fn should_detect_season_with_x_separator_and_letter_on_right() {
    assert_includes("clny.3x11m720p.es[www.planetatorrent.com].mkv", json!({ "season": 3 }));
}

/// should detect multiple seasons separated with comma
#[test]
fn should_detect_multiple_seasons_separated_with_comma() {
    assert_includes(
        "Game Of Thrones Complete Season 1,2,3,4,5,6,7 406p mkv + Subs",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7] }),
    );
}

/// should detect multiple seasons separated with space with redundant digit suffix
#[test]
fn should_detect_multiple_seasons_separated_with_space_with_redundant_digit_suffix() {
    assert_includes(
        "Futurama Season 1 2 3 4 5 6 7 + 4 Movies - threesixtyp",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7] }),
    );
}

/// should detect multiple season separated with spaces and comma
#[test]
fn should_detect_multiple_season_separated_with_spaces_and_comma() {
    assert_includes("Breaking Bad Complete Season 1 , 2 , 3, 4 ,5 ,1080p HEVC", json!({ "seasons": [1, 2, 3, 4, 5] }));
}

/// should detect multiple season separated with comma, space and symbol at the end
#[test]
fn should_detect_multiple_season_separated_with_comma_space_and_symbol_at_the_end() {
    assert_includes("True Blood Season 1, 2, 3, 4, 5 & 6 + Extras BDRip TSV", json!({ "seasons": [1, 2, 3, 4, 5, 6] }));
}

/// How I Met Your Mother Season 1, 2, 3, 4, 5, & 6 + Extras DVDRip
#[test]
fn how_i_met_your_mother_season_1_2_3_4_5_6_extras_dvdrip() {
    assert_includes(
        "How I Met Your Mother Season 1, 2, 3, 4, 5, & 6 + Extras DVDRip",
        json!({ "seasons": [1, 2, 3, 4, 5, 6] }),
    );
}

/// should detect multiple seasons separated with space
#[test]
fn should_detect_multiple_seasons_separated_with_space() {
    assert_includes(
        "The Simpsons Season 20 21 22 23 24 25 26 27 - threesixtyp",
        json!({ "seasons": [20, 21, 22, 23, 24, 25, 26, 27] }),
    );
}

/// should detect multiple seasons separated with space an spanish season name
#[test]
fn should_detect_multiple_seasons_separated_with_space_an_spanish_season_name() {
    assert_includes(
        "Perdidos: Lost: Castellano: Temporadas 1 2 3 4 5 6 (Serie Com",
        json!({ "seasons": [1, 2, 3, 4, 5, 6] }),
    );
}

/// should detect multiple seasons with with unequal separators
#[test]
fn should_detect_multiple_seasons_with_with_unequal_separators() {
    assert_includes("The Boondocks Season 1, 2 & 3", json!({ "seasons": [1, 2, 3] }));
}

/// should detect multiple seasons with with space and plus symbol
#[test]
fn should_detect_multiple_seasons_with_with_space_and_plus_symbol() {
    assert_includes("Boondocks, The - Seasons 1 + 2", json!({ "seasons": [1, 2] }));
}

/// should detect multiple seasons with implied range without s prefix
#[test]
fn should_detect_multiple_seasons_with_implied_range_without_s_prefix() {
    assert_includes("The Boondocks Seasons 1-4 MKV", json!({ "seasons": [1, 2, 3, 4] }));
}

/// should detect multiple seasons separated with space plus and symbol
#[test]
fn should_detect_multiple_seasons_separated_with_space_plus_and_symbol() {
    assert_includes("The Expanse Complete Seasons 01 & 02 1080p", json!({ "seasons": [1, 2] }));
}

/// should detect multiple seasons with s prefix and implied range
#[test]
fn should_detect_multiple_seasons_with_s_prefix_and_implied_range() {
    assert_includes(
        "Friends.Complete.Series.S01-S10.720p.BluRay.2CH.x265.HEVC-PSA",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }),
    );
}

/// should detect multiple seasons with s prefix separated with slash
#[test]
fn should_detect_multiple_seasons_with_s_prefix_separated_with_slash() {
    assert_includes(
        "Stargate Atlantis ALL Seasons - S01 / S02 / S03 / S04 / S05",
        json!({ "seasons": [1, 2, 3, 4, 5] }),
    );
}

/// should detect multiple seasons with season and parenthesis prefix
#[test]
fn should_detect_multiple_seasons_with_season_and_parenthesis_prefix() {
    assert_includes(
        "Stargate Atlantis Complete (Season 1 2 3 4 5) 720p HEVC x265",
        json!({ "seasons": [1, 2, 3, 4, 5] }),
    );
}

/// should detect multiple seasons with s prefix separated with hyphen
#[test]
fn should_detect_multiple_seasons_with_s_prefix_separated_with_hyphen() {
    assert_includes("Skam.S01-S02-S03.SweSub.720p.WEB-DL.H264", json!({ "seasons": [1, 2, 3] }));
}

/// should detect detect correct title with multiple season definitions
#[test]
fn should_detect_detect_correct_title_with_multiple_season_definitions() {
    assert_includes("Seinfeld S02 Season 2 720p WebRip ReEnc-DeeJayAhmed", json!({ "title": "Seinfeld", "season": 2 }));
}

/// should detect correct title with multiple season definitions
#[test]
fn should_detect_correct_title_with_multiple_season_definitions() {
    assert_includes(
        "Seinfeld Season 2 S02 720p AMZN WEBRip x265 HEVC Complete",
        json!({ "title": "Seinfeld", "season": 2 }),
    );
}

/// should detect multiple season when given implied range inside parenthesis without s prefix
#[test]
fn should_detect_multiple_season_when_given_implied_range_inside_parenthesis_without_s_prefix() {
    assert_includes("House MD All Seasons (1-8) 720p Ultra-Compressed", json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8] }));
}

/// should detect multiple season when given implied range with season prefix
#[test]
fn should_detect_multiple_season_when_given_implied_range_with_season_prefix() {
    assert_includes("Teen Titans Season 1-5", json!({ "seasons": [1, 2, 3, 4, 5] }));
}

/// should detect multiple season when given implied range y words with season prefix
#[test]
fn should_detect_multiple_season_when_given_implied_range_y_words_with_season_prefix() {
    assert_includes("Game Of Thrones - Season 1 to 6 (Eng Subs)", json!({ "seasons": [1, 2, 3, 4, 5, 6] }));
}

/// should detect multiple season with and separator
#[test]
fn should_detect_multiple_season_with_and_separator() {
    assert_includes("Travelers - Seasons 1 and 2 - Mp4 x264 AC3 1080p", json!({ "seasons": [1, 2] }));
}

/// should detect multiple season when given implied range separated with colon
#[test]
fn should_detect_multiple_season_when_given_implied_range_separated_with_colon() {
    assert_includes("Naruto Shippuden Season 1:11", json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] }));
}

/// should detect multiple season when given implied range separated with colon and space
#[test]
fn should_detect_multiple_season_when_given_implied_range_separated_with_colon_and_space() {
    assert_includes("South Park Complete Seasons 1: 11", json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] }));
}

/// should detect multiple season when title is with numbers
#[test]
fn should_detect_multiple_season_when_title_is_with_numbers() {
    assert_includes("24 Season 1-8 Complete with Subtitles", json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8] }));
}

/// should detect single season when contains non related number range
#[test]
fn should_detect_single_season_when_contains_non_related_number_range() {
    assert_includes("One Punch Man 01 - 12 Season 1 Complete [720p] [Eng Subs] [Xerxe:16", json!({ "season": 1 }));
}

/// should detect season withing brackets with dot separator
#[test]
fn should_detect_season_withing_brackets_with_dot_separator() {
    assert_includes("[5.01] Weight Loss.avi", json!({ "season": 5 }));
}

/// should detect season withing brackets with dot separator and episode in hunderds
#[test]
fn should_detect_season_withing_brackets_with_dot_separator_and_episode_in_hunderds() {
    assert_includes("Dragon Ball [5.134] Preliminary Peril.mp4", json!({ "season": 5 }));
}

/// should detect season with s prefix and single digit
#[test]
fn should_detect_season_with_s_prefix_and_single_digit() {
    assert_includes("Bron - S4 - 720P - SweSub.mp4", json!({ "season": 4 }));
}

/// Mad Men S02 Season 2 720p 5.1Ch BluRay ReEnc-DeeJayAhmed
#[test]
fn mad_men_s02_season_2_720p_5_1_ch_blu_ray_re_enc_dee_jay_ahmed() {
    assert_includes("Mad Men S02 Season 2 720p 5.1Ch BluRay ReEnc-DeeJayAhmed", json!({ "season": 2 }));
}

/// Friends S04 Season 4 1080p 5.1Ch BluRay ReEnc-DeeJayAhmed
#[test]
fn friends_s04_season_4_1080p_5_1_ch_blu_ray_re_enc_dee_jay_ahmed() {
    assert_includes("Friends S04 Season 4 1080p 5.1Ch BluRay ReEnc-DeeJayAhmed", json!({ "season": 4 }));
}

/// should detect multiple season with double seperators
#[test]
fn should_detect_multiple_season_with_double_seperators() {
    assert_includes("Doctor Who S01--S07--Complete with holiday episodes", json!({ "seasons": [1, 2, 3, 4, 5, 6, 7] }));
}

/// should detect season with a dot and hyphen separator
#[test]
fn should_detect_season_with_a_dot_and_hyphen_separator() {
    assert_includes("My Little Pony FiM - 6.01 - No Second Prances.mkv", json!({ "season": 6 }));
}

/// should detect season with a dot and epsiode prefix
#[test]
fn should_detect_season_with_a_dot_and_epsiode_prefix() {
    assert_includes("Desperate Housewives - Episode 1.22 - Goodbye for now.avi", json!({ "season": 1 }));
}

/// should detect season with a dot and episode prefix v2
#[test]
fn should_detect_season_with_a_dot_and_episode_prefix_v2() {
    assert_includes("All of Us Are Dead . 2022 . S01 EP #1.2.mkv", json!({ "season": 1 }));
}

/// should detect season with a year range afterwards
#[test]
fn should_detect_season_with_a_year_range_afterwards() {
    assert_includes("Empty Nest Season 1 (1988 - 89) fiveofseven", json!({ "season": 1 }));
}

/// should detect multiple seasons with russian season and hyphen separator
#[test]
fn should_detect_multiple_seasons_with_russian_season_and_hyphen_separator() {
    assert_includes(
        "Game of Thrones / Сезон: 1-8 / Серии: 1-73 из 73 [2011-2019, США, BDRip 1080p] MVO (LostFilm)",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
}

/// should detect multiple seasons with russian season and comma separator
#[test]
fn should_detect_multiple_seasons_with_russian_season_and_comma_separator() {
    assert_includes(
        "Друзья / Friends / Сезон: 1, 2 / Серии: 1-24 из 24 [1994-1999, США, BDRip 720p] MVO",
        json!({ "seasons": [1, 2] }),
    );
}

/// should detect season with russian season word
#[test]
fn should_detect_season_with_russian_season_word() {
    assert_includes(
        "Друзья / Friends / Сезон: 1 / Серии: 1-24 из 24 [1994-1995, США, BDRip 720p] MVO + Original + Sub (Rus, Eng)",
        json!({ "season": 1 }),
    );
}

/// should detect season with russian season word as folder name
#[test]
fn should_detect_season_with_russian_season_word_as_folder_name() {
    assert_includes("Сезон 5/Серия 11.mkv", json!({ "season": 5 }));
}

/// should detect season with russian season word followed by the number
#[test]
fn should_detect_season_with_russian_season_word_followed_by_the_number() {
    assert_includes(
        "Разрушители легенд. MythBusters. Сезон 15. Эпизод 09. Скрытая угроза (2015).avi",
        json!({ "season": 15 }),
    );
}

/// should detect season with russian season word followed by the number v2
#[test]
fn should_detect_season_with_russian_season_word_followed_by_the_number_v2() {
    assert_includes("Леди Баг и Супер-Кот – Сезон 3, Эпизод 21 – Кукловод 2 [1080p].mkv", json!({ "season": 3 }));
}

/// should detect episode with full russian season name with case suffix
#[test]
fn should_detect_episode_with_full_russian_season_name_with_case_suffix() {
    assert_includes("Проклятие острова ОУК_ 5-й сезон 09-я серия_ Прорыв Дэна.avi", json!({ "season": 5 }));
}

/// should detect season with russian season word with number at front
#[test]
fn should_detect_season_with_russian_season_word_with_number_at_front() {
    assert_includes("2 сезон 24 серия.avi", json!({ "season": 2 }));
}

/// should detect season with russian season word with number at front and nothing else
#[test]
fn should_detect_season_with_russian_season_word_with_number_at_front_and_nothing_else() {
    assert_includes("3 сезон", json!({ "season": 3 }));
}

/// should detect season with russian season word and underscore
#[test]
fn should_detect_season_with_russian_season_word_and_underscore() {
    assert_includes(
        "2. Discovery-Kak_ustroena_Vselennaya.(2.sezon_8.serii.iz.8).2012.XviD.HDTVRip.Krasnodarka",
        json!({ "season": 2 }),
    );
}

/// should detect season with russian season shortened word
#[test]
fn should_detect_season_with_russian_season_shortened_word() {
    assert_includes("Otchayannie.domochozyaiki.(8.sez.21.ser.iz.23).2012.XviD.HDTVRip.avi", json!({ "season": 8 }));
}

/// should detect season with russian season word and no prefix
#[test]
fn should_detect_season_with_russian_season_word_and_no_prefix() {
    assert_includes("Интерны. Сезон №9. Серия №180.avi", json!({ "season": 9 }));
}

/// should detect season with russian x separator
#[test]
fn should_detect_season_with_russian_x_separator() {
    assert_includes(
        "Discovery. Парни с Юкона / Yokon Men [06х01-08] (2017) HDTVRip от GeneralFilm | P1",
        json!({ "season": 6 }),
    );
}

/// should detect season before transliterated season word followed by episode range
#[test]
fn should_detect_season_before_transliterated_season_word_followed_by_episode_range() {
    assert_includes(
        "Prins.iz.Beverli.Hillz.(1.sezon.08-09.serii.iz.20).1990.XviD.TVRip.avi",
        json!({ "seasons": [1], "episodes": [8, 9] }),
    );
}

/// should detect season range with plus separator attached to seasons word
#[test]
fn should_detect_season_range_with_plus_separator_attached_to_seasons_word() {
    assert_includes("[UQW] Arslan Senki Seasons1+2 [BD 1080p AVC-YUV444P10 FLAC]", json!({ "seasons": [1, 2] }));
}

/// should detect season range with russian x separator
#[test]
fn should_detect_season_range_with_russian_x_separator() {
    assert_includes(
        "Пока-пока, Земля / Bye Bye, Earth [01-02х01-19 из 20] (2024) WEBRip 1080p by VLDeshka | L | DreamCast, JAMClub",
        json!({ "seasons": [1, 2] }),
    );
}

/// should detect season range with x separator
#[test]
fn should_detect_season_range_with_x_separator() {
    assert_includes("Show [01-03x01-30 из 30] (2020) WEBRip 1080p", json!({ "seasons": [1, 2, 3] }));
}

/// should not detect season range from a digit in the title before x separator
#[test]
fn should_not_detect_season_range_from_a_digit_in_the_title_before_x_separator() {
    assert_includes("stargate-sg1-10x01-maso-a-krev.avi", json!({ "seasons": [10], "title": "stargate-sg1" }));
}

/// should detect season with russian season word in araic letters
#[test]
fn should_detect_season_with_russian_season_word_in_araic_letters() {
    assert_includes("Zvezdnie.Voiny.Voina.Klonov.3.sezon.22.seria.iz.22.XviD.HDRip.avi", json!({ "season": 3 }));
}

/// should detect season with hyphen separator between episode
#[test]
fn should_detect_season_with_hyphen_separator_between_episode() {
    assert_includes("2-06. Девичья сила.mkv", json!({ "season": 2 }));
}

/// should detect season with hyphen separator between episode v2
#[test]
fn should_detect_season_with_hyphen_separator_between_episode_v2() {
    assert_includes("4-13 Cursed (HD).m4v", json!({ "season": 4 }));
}

/// should detect season with hyphen separator between episode v3
#[test]
fn should_detect_season_with_hyphen_separator_between_episode_v3() {
    assert_includes("Доктор Хаус 03-20.mkv", json!({ "season": 3 }));
}

/// should detect episodes with hyphen separator between episode v4
#[test]
fn should_detect_episodes_with_hyphen_separator_between_episode_v4() {
    assert_includes("Комиссар Рекс 11-13.avi", json!({ "season": 11 }));
}

/// should not detect season with hyphen separator when it's the title
#[test]
fn should_not_detect_season_with_hyphen_separator_when_it_s_the_title() {
    assert_missing("13-13-13 2013 DVDrip x264 AAC-MiLLENiUM", &["seasons"]);
    assert_includes("13-13-13 2013 DVDrip x264 AAC-MiLLENiUM", json!({ "title": "13-13-13" }));
}

/// should detect correct season with eps prefix and hyphen separator
#[test]
fn should_detect_correct_season_with_eps_prefix_and_hyphen_separator() {
    assert_includes("MARATHON EPISODES/Orphan Black S3 Eps.05-08.mp4", json!({ "season": 3 }));
}

/// should detect multiple seasons with end season without s symbol
#[test]
fn should_detect_multiple_seasons_with_end_season_without_s_symbol() {
    assert_includes(
        "Once Upon a Time [S01-07] (2011-2017) WEB-DLRip by Generalfilm",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7] }),
    );
}

/// should detect multiple seasons with one space and hyphen separator
#[test]
fn should_detect_multiple_seasons_with_one_space_and_hyphen_separator() {
    assert_includes(
        "[F-D] Fairy Tail Season 1 -6 + Extras [480P][Dual-Audio]",
        json!({ "seasons": [1, 2, 3, 4, 5, 6] }),
    );
}

/// should detect multiple seasons with spaces and hyphen separator
#[test]
fn should_detect_multiple_seasons_with_spaces_and_hyphen_separator() {
    assert_includes(
        "Coupling Season 1 - 4 Complete DVDRip - x264 - MKV by RiddlerA",
        json!({ "seasons": [1, 2, 3, 4] }),
    );
}

/// should single season with spaces and hyphen separator
#[test]
fn should_single_season_with_spaces_and_hyphen_separator() {
    assert_includes("[HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ", json!({ "seasons": [4] }));
}

/// Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]
#[test]
fn tokyo_ghoul_root_a_07_s2_07_eng_sub_480p_email_protected() {
    assert_includes("Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]", json!({ "seasons": [2] }));
}

/// Ace of the Diamond: 1st Season
#[test]
fn ace_of_the_diamond_1st_season() {
    assert_includes("Ace of the Diamond: 1st Season", json!({ "seasons": [1] }));
}

/// Ace of the Diamond: 2nd Season
#[test]
fn ace_of_the_diamond_2nd_season() {
    assert_includes("Ace of the Diamond: 2nd Season", json!({ "seasons": [2] }));
}

/// Adventure Time 10 th season
#[test]
fn adventure_time_10_th_season() {
    assert_includes("Adventure Time 10 th season", json!({ "seasons": [10] }));
}

/// Kyoukai no Rinne (TV) 3rd Season - 23 [1080p]
#[test]
fn kyoukai_no_rinne_tv_3rd_season_23_1080p() {
    assert_includes("Kyoukai no Rinne (TV) 3rd Season - 23 [1080p]", json!({ "seasons": [3] }));
}

/// [Erai-raws] Granblue Fantasy The Animation Season 2 - 08 [1080p][Multiple Subtitle].mkv
#[test]
fn erai_raws_granblue_fantasy_the_animation_season_2_08_1080p_multiple_subtitle_mkv() {
    assert_includes(
        "[Erai-raws] Granblue Fantasy The Animation Season 2 - 08 [1080p][Multiple Subtitle].mkv",
        json!({ "seasons": [2] }),
    );
}

/// The Nile Egypts Great River with Bettany Hughes Series 1 4of4 10
#[test]
fn the_nile_egypts_great_river_with_bettany_hughes_series_1_4of4_10() {
    assert_includes("The Nile Egypts Great River with Bettany Hughes Series 1 4of4 10", json!({ "seasons": [1] }));
}

/// Teen Wolf - 04ª Temporada 720p
#[test]
fn teen_wolf_04a_temporada_720p() {
    assert_includes("Teen Wolf - 04ª Temporada 720p", json!({ "seasons": [4] }));
}

/// Vikings 3 Temporada 720p
#[test]
fn vikings_3_temporada_720p() {
    assert_includes("Vikings 3 Temporada 720p", json!({ "seasons": [3] }));
}

/// Eu, a Patroa e as Crianças  4° Temporada Completa - HDTV - Dublado
#[test]
fn eu_a_patroa_e_as_crianc_as_4_temporada_completa_hdtv_dublado() {
    assert_includes("Eu, a Patroa e as Crianças  4° Temporada Completa - HDTV - Dublado", json!({ "seasons": [4] }));
}

/// Merl - Temporada 1
#[test]
fn merl_temporada_1() {
    assert_includes("Merl - Temporada 1", json!({ "seasons": [1] }));
}

/// Elementar 3º Temporada Dublado
#[test]
fn elementar_3o_temporada_dublado() {
    assert_includes("Elementar 3º Temporada Dublado", json!({ "seasons": [3] }));
}

/// Beavis and Butt-Head - 1a. Temporada
#[test]
fn beavis_and_butt_head_1a_temporada() {
    assert_includes("Beavis and Butt-Head - 1a. Temporada", json!({ "seasons": [1] }));
}

/// 3Âº Temporada Bob esponja Pt-Br
#[test]
fn t_3_a_o_temporada_bob_esponja_pt_br() {
    assert_includes("3Âº Temporada Bob esponja Pt-Br", json!({ "seasons": [3] }));
}

/// Juego de Tronos - Temp.2 [ALTA DEFINICION 720p][Cap.209][Spanish].mkv
#[test]
fn juego_de_tronos_temp_2_alta_definicion_720p_cap_209_spanish_mkv() {
    assert_includes("Juego de Tronos - Temp.2 [ALTA DEFINICION 720p][Cap.209][Spanish].mkv", json!({ "seasons": [2] }));
}

/// Los Simpsons Temp 7 DVDrip Espanol De Espana
#[test]
fn los_simpsons_temp_7_dvdrip_espanol_de_espana() {
    assert_includes("Los Simpsons Temp 7 DVDrip Espanol De Espana", json!({ "seasons": [7] }));
}

/// should detect spanish season range with & separator
#[test]
fn should_detect_spanish_season_range_with_separator() {
    assert_includes("The Walking Dead [Temporadas 1 & 2 Completas Em HDTV E Legena", json!({ "seasons": [1, 2] }));
}

/// should detect spanish short season identifier
#[test]
fn should_detect_spanish_short_season_identifier() {
    assert_includes("My Little Pony - A Amizade é Mágica - T02E22.mp4", json!({ "seasons": [2] }));
}

/// should detect spanish short season identifier with xe separator
#[test]
fn should_detect_spanish_short_season_identifier_with_xe_separator() {
    assert_includes("30 M0N3D4S ESP T01XE08.mkv", json!({ "seasons": [1] }));
}

/// should detect sn naming scheme
#[test]
fn should_detect_sn_naming_scheme() {
    assert_includes("Sons of Anarchy Sn4 Ep14 HD-TV - To Be, Act 2, By Cool Release", json!({ "seasons": [4] }));
}

/// should detect single season and not range in filename
#[test]
fn should_detect_single_season_and_not_range_in_filename() {
    assert_includes("[FFA] Kiratto Pri☆chan Season 3 - 11 [1080p][HEVC].mkv", json!({ "seasons": [3] }));
}

/// should detect single season and not range in filename v2
#[test]
fn should_detect_single_season_and_not_range_in_filename_v2() {
    assert_includes(
        "[Erai-raws] Granblue Fantasy The Animation Season 2 - 10 [1080p][Multiple Subtitle].mkv",
        json!({ "seasons": [2] }),
    );
}

/// should detect single season and not range in filename v3
#[test]
fn should_detect_single_season_and_not_range_in_filename_v3() {
    assert_includes(
        "[SCY] Attack on Titan Season 3 - 11 (BD 1080p Hi10 FLAC) [1FA13150].mkv",
        json!({ "seasons": [3] }),
    );
}

/// should detect single zero season
#[test]
fn should_detect_single_zero_season() {
    assert_includes("DARKER THAN BLACK - S00E04 - Darker Than Black Gaiden OVA 3.mkv", json!({ "season": 0 }));
}

/// should detect nl season word
#[test]
fn should_detect_nl_season_word() {
    assert_includes(
        "Seizoen 22 - Zon & Maan Ultra Legendes/afl.18 Je ogen op de bal houden!.mp4",
        json!({ "season": 22 }),
    );
}

/// should detect italian season word
#[test]
fn should_detect_italian_season_word() {
    assert_includes(
        "Nobody Wants This - Stagione 1 (2024) [COMPLETA] 720p H264 ITA AAC 2.0-Zer0landia",
        json!({ "season": 1 }),
    );
}

/// should detect italian season range
#[test]
fn should_detect_italian_season_range() {
    assert_includes(
        "Red Oaks - Stagioni 01-03 (2014-2017) [COMPLETA] SD x264 AAC ITA SUB ITA - mkeagle3",
        json!({ "seasons": [1, 2, 3] }),
    );
}

/// should detect polish season
#[test]
fn should_detect_polish_season() {
    assert_includes(
        "Star.Wars.Skeleton.Crew.Sezon01.PLDUB.480p.DSNP.WEB-DL.H264.DDP5.1-K83",
        json!({ "seasons": [1] }),
    );
}

/// should detect polish season with S prefix
#[test]
fn should_detect_polish_season_with_s_prefix() {
    assert_includes("Bitten.SezonSO3.PL.480p.NF.WEB-DL.DD5.1.XviD-Ralf", json!({ "seasons": [3] }));
}

/// should detect regular season with year range before
#[test]
fn should_detect_regular_season_with_year_range_before() {
    assert_includes("'Lucky.Luke.1983-1992.S01E04.PL.720p.WEB-DL.H264-zyl.mkv'", json!({ "seasons": [1] }));
}

/// should detect polish season range
#[test]
fn should_detect_polish_season_range() {
    assert_includes(
        "Rizzoli & Isles 2010-2016 [Sezon 01-07] [1080p.WEB-DL.H265.EAC3-FT][Alusia]",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7] }),
    );
}

/// should detect season episode when not in boundary
#[test]
fn should_detect_season_episode_when_not_in_boundary() {
    assert_includes(
        "Those.About.to.DieS01E06.MULTi.720p.AMZN.WEB-DL.H264.DDP5.1-K83.mkv",
        json!({ "seasons": [1], "episodes": [6] }),
    );
}

/// should not detect season when it's part of the name
#[test]
fn should_not_detect_season_when_it_s_part_of_the_name() {
    assert_missing("Ranma-12-86.mp4", &["season"]);
}

/// should not detect season when it's part of group
#[test]
fn should_not_detect_season_when_it_s_part_of_group() {
    assert_missing("The Killer's Game 2024 PL 1080p WEB-DL H264 DD5.1-S56", &["season"]);
}

/// should not detect season when it's part of group v2
#[test]
fn should_not_detect_season_when_it_s_part_of_group_v2() {
    assert_missing("Apollo 13 (1995) [1080p] [WEB-DL] [x264] [E-AC3-S78] [Lektor PL]", &["season"]);
}

/// should not detect season dot episode notation when it's a title
#[test]
fn should_not_detect_season_dot_episode_notation_when_it_s_a_title() {
    assert_missing("3.10 to Yuma (2007).mkv", &["season"]);
}

/// should not detect season dot episode notation when there is year in name
#[test]
fn should_not_detect_season_dot_episode_notation_when_there_is_year_in_name() {
    assert_missing("18.11 - A Code Of Secrecy (2014) x264 1080p-AAC-ESUB [Parth].mkv", &["season"]);
}

/// should detect norwegian season folder
#[test]
fn should_detect_norwegian_season_folder() {
    assert_includes("Sesong 2", json!({ "season": 2 }));
}

/// should detect german season folder
#[test]
fn should_detect_german_season_folder() {
    assert_includes("Staffel 1", json!({ "season": 1 }));
}

/// should detect swedish season folder
#[test]
fn should_detect_swedish_season_folder() {
    assert_includes("pepparkornen säsong 3", json!({ "season": 3 }));
}

/// should detect croatian season folder
#[test]
fn should_detect_croatian_season_folder() {
    assert_includes("Sezona 5 + Bonus", json!({ "season": 5 }));
}

/// should detect season number before season word
#[test]
fn should_detect_season_number_before_season_word() {
    assert_includes("2 Season", json!({ "season": 2 }));
}

/// should not detect year range end before season word as season
#[test]
fn should_not_detect_year_range_end_before_season_word_as_season() {
    assert_missing("Tottenham Hotspur 2012-13 Season Review.mp4", &["season"]);
}

/// should detect bare season folder
#[test]
fn should_detect_bare_season_folder() {
    assert_includes("S04", json!({ "season": 4 }));
}

/// should detect season at the end of the title
#[test]
fn should_detect_season_at_the_end_of_the_title() {
    assert_includes("Peaky Blinders S04", json!({ "title": "Peaky Blinders", "season": 4 }));
}

/// should not expand comma separated seasons into a range
#[test]
fn should_not_expand_comma_separated_seasons_into_a_range() {
    assert_includes("Горец (Сезон 1, 22 серии) / Highlander [1992, DVDRip]", json!({ "season": 1 }));
}

/// should detect swedish season range
#[test]
fn should_detect_swedish_season_range() {
    assert_includes("Bäst I Test Säsong 1-3 (1080p), Bast I Test", json!({ "seasons": [1, 2, 3] }));
}

/// should not detect season range across a removed year
#[test]
fn should_not_detect_season_range_across_a_removed_year() {
    assert_includes("Stargate SG-1. Season 9 (2005) 20.avi", json!({ "seasons": [9], "episode": 20 }));
}

/// should detect season with three digits padded
#[test]
fn should_detect_season_with_three_digits_padded() {
    assert_includes(
        "One.Piece.S004E111.Dash.For.a.Miracle!.Alabasta.Animal.Land!.1080p.NF.WEB-DL.DDP2.0.x264-KQRM.mkv",
        json!({ "season": 4, "episode": 111 }),
    );
}

/// should detect three digit season
#[test]
fn should_detect_three_digit_season() {
    assert_includes("house.hunters.s216e12.1080p.web.h264-spamneggs.mkv", json!({ "season": 216, "episode": 12 }));
}

/// should not detect zero season from zero padded complete series episode range
#[test]
fn should_not_detect_zero_season_from_zero_padded_complete_series_episode_range() {
    assert_missing("Naruto Shippuden (Complete Series 001-500) [720p][HEVC][x265]", &["seasons"]);
}

/// should detect single season with plural seasons keyword
#[test]
fn should_detect_single_season_with_plural_seasons_keyword() {
    assert_includes(
        "DareDevil - Seasons 2 - Episodes 10 thru 13",
        json!({ "title": "DareDevil", "seasons": [2], "season": 2 }),
    );
}

/// should detect season list with oxford comma and
#[test]
fn should_detect_season_list_with_oxford_comma_and() {
    assert_includes("Skam - Season 1, 2, and 3", json!({ "seasons": [1, 2, 3] }));
}

/// should detect seasons from first n seasons
#[test]
fn should_detect_seasons_from_first_n_seasons() {
    assert_includes(
        "The Boondocks 2005 Complete First 3 Seasons Burntodisc",
        json!({ "title": "The Boondocks", "seasons": [1, 2, 3] }),
    );
}

/// should not detect season from possessive s glued to a number
#[test]
fn should_not_detect_season_from_possessive_s_glued_to_a_number() {
    assert_missing("Ocean's11.avi", &["seasons"]);
    assert_missing("Ocean's.11.2001.1080p.BluRay.mkv", &["seasons"]);
}

/// should not detect season from an episode count after seasons
#[test]
fn should_not_detect_season_from_an_episode_count_after_seasons() {
    assert_missing("Transformers Armada - Complete series all Seasons 52 episodes [DVDRip]", &["seasons"]);
}

/// should detect season after seasons word followed by an episode
#[test]
fn should_detect_season_after_seasons_word_followed_by_an_episode() {
    assert_includes("The Walking Dead Seasons 5 episode 2.mkv", json!({ "seasons": [5], "episodes": [2] }));
}

/// should detect season before a parenthesized russian episode of total
#[test]
fn should_detect_season_before_a_parenthesized_russian_episode_of_total() {
    assert_includes(
        "Grazhdanin.Nachalnik.2.(1.seriy.iz.12).2005.DviX.DVDRip.avi",
        json!({ "seasons": [2], "episodes": [1] }),
    );
}

/// should detect season before a transliterated series word and episode
#[test]
fn should_detect_season_before_a_transliterated_series_word_and_episode() {
    assert_includes("Den.rozhdeniya.burzhuya.2.ser.01.avi", json!({ "seasons": [2], "episodes": [1] }));
}

/// should not detect season from the open season movie sequels
#[test]
fn should_not_detect_season_from_the_open_season_movie_sequels() {
    assert_includes(
        "Open Season 2 (2008) 720p BrRip x264 - 500MB - YIFY",
        json!({ "title": "Open Season 2", "year": 2008 }),
    );
    assert_missing("Open Season 2 (2008) 720p BrRip x264 - 500MB - YIFY", &["seasons"]);
}

/// should not detect season range from the open season movie pack
#[test]
fn should_not_detect_season_range_from_the_open_season_movie_pack() {
    assert_missing("Open Season 1-2 720p Dual Audio [eng-hin]", &["seasons"]);
}

/// should detect a chinese season marker
#[test]
fn should_detect_a_chinese_season_marker() {
    assert_includes(
        "[ANi] Re：從零開始的異世界生活 第四季 - 17 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4",
        json!({ "seasons": [4], "episodes": [17] }),
    );
}

/// should detect a chinese season marker with a two character numeral
#[test]
fn should_detect_a_chinese_season_marker_with_a_two_character_numeral() {
    assert_includes(
        "[Nekomoe kissaten][Kusuriya no Hitorigoto][第十二季][05][1080p].mp4",
        json!({ "title": "Kusuriya no Hitorigoto", "seasons": [12], "episodes": [5] }),
    );
}

/// should detect a chinese season marker with full width digits
#[test]
fn should_detect_a_chinese_season_marker_with_full_width_digits() {
    assert_includes("Show 第４季 - 05.mkv", json!({ "seasons": [4], "episodes": [5] }));
}

/// should detect a japanese season marker with its season name
#[test]
fn should_detect_a_japanese_season_marker_with_its_season_name() {
    assert_includes(
        "[アニメ BD] みなみけ 第2期(おかわり) 第01話「おかわり」(1920x1080 HEVC 10bit FLAC softSub(chi+eng) chap).mkv",
        json!({ "title": "みなみけ", "seasons": [2], "episodes": [1] }),
    );
}

/// should detect a japanese season marker followed by a kana name
#[test]
fn should_detect_a_japanese_season_marker_followed_by_a_kana_name() {
    assert_includes(
        "[アニメ BD] みなみけ 第4期(ただいま)「メニュー画面 4」(1920x1080 HEVC 10bit FLAC).mkv",
        json!({ "seasons": [4] }),
    );
}

/// should not detect a chinese issue number as season
#[test]
fn should_not_detect_a_chinese_issue_number_as_season() {
    assert_missing("奔跑吧 第5期 20231020.mp4", &["seasons"]);
}

/// should detect an ordinal season in a bracketed episode section
#[test]
fn should_detect_an_ordinal_season_in_a_bracketed_episode_section() {
    assert_includes(
        "[晚街与灯][Re Zero kara Hajimeru Isekai Seikatsu][4th - 16][总第82][WebRip][1080P_AVC_AAC][简日双语内嵌].mp4",
        json!({ "title": "Re Zero kara Hajimeru Isekai Seikatsu", "seasons": [4], "episodes": [16] }),
    );
}

/// should not detect a codec glued to an hdr tag as season
#[test]
fn should_not_detect_a_codec_glued_to_an_hdr_tag_as_season() {
    assert_missing(
        "The Tomorrow War 2021 REPACK UHD WebRip 1080p DD Atmos 5 1 DoVi HDR10x265-SM737",
        &["seasons", "episodes"],
    );
}

/// should not detect a release group of t and a number after a dash as season
#[test]
fn should_not_detect_a_release_group_of_t_and_a_number_after_a_dash_as_season() {
    assert_missing("Hell.Fest.2018.FRENCH.BDRip.XviD-T911", &["seasons"]);
}

/// should not detect an uploader tag of t and a number as season
#[test]
fn should_not_detect_an_uploader_tag_of_t_and_a_number_as_season() {
    assert_missing("Le nostre anime di notte 2017 1080p iTALiAN ENGLiSH Subbed WEBMux x265 AC3 by T7", &["seasons"]);
}

/// should not detect a release group ending in a digit, t and a number as season
#[test]
fn should_not_detect_a_release_group_ending_in_a_digit_t_and_a_number_as_season() {
    assert_missing("Movie.2019.1080p.AMZN.WEBRip.DDP5.1.x264-V3T0", &["seasons"]);
}

/// should detect a spanish season of t and a number ending a folder name
#[test]
fn should_detect_a_spanish_season_of_t_and_a_number_ending_a_folder_name() {
    assert_includes("Liga de la Justicia Ilimitada T1", json!({ "seasons": [1] }));
}

/// should not detect a release group of s and a number after a dash before the extension as season
#[test]
fn should_not_detect_a_release_group_of_s_and_a_number_after_a_dash_before_the_extension_as_s() {
    assert_missing("The.Naked.Gun.2025.PLSUB.Ai.1080p.WEB-DL.H264.DDP5.1.Atmos-S56.mkv", &["seasons"]);
}

/// should not detect a bracketed release group of s and a number after an audio tag as season
#[test]
fn should_not_detect_a_bracketed_release_group_of_s_and_a_number_after_an_audio_tag_as_season() {
    assert_missing("Armand (2024) [1080p] [WEB-DL] [H264] [DD5.1-S56] [Lektor PL] [ToAlien]", &["seasons"]);
}

/// should detect a bracketed season range ending in a two digit season
#[test]
fn should_detect_a_bracketed_season_range_ending_in_a_two_digit_season() {
    assert_includes(
        "Shameless (US) 2011 - 2021 [S01-S11] [720p.MAX.WEB-DL.HEVC.E-AC-3-FT][Lektor PL][Alusia]",
        json!({ "seasons": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] }),
    );
}

/// should detect a single digit season after a dash before the extension
#[test]
fn should_detect_a_single_digit_season_after_a_dash_before_the_extension() {
    assert_includes("04. Horrible Histories-S3.mp4", json!({ "seasons": [3] }));
}

/// should detect a season of s and a number ending the title
#[test]
fn should_detect_a_season_of_s_and_a_number_ending_the_title() {
    assert_includes("Have I Got A Bit More News For You S56", json!({ "seasons": [56] }));
}

/// should not detect a release group ending in t and a number as season
#[test]
fn should_not_detect_a_release_group_ending_in_t_and_a_number_as_season() {
    assert_missing("The Sisterhood of the Traveling Pants 2 2008 1080p BluRay x264-nikt0", &["seasons"]);
    assert_missing("Iron Man 2008.2160p.60fps.Ai-Remaster.RIFE4.18.DV.HDR10.Atmos-Acemt01", &["seasons"]);
}

/// should still detect a spanish temporada short marker
#[test]
fn should_still_detect_a_spanish_temporada_short_marker() {
    assert_includes("Serie T01E05 HDTV 720p", json!({ "seasons": [1], "episodes": [5] }));
}

/// should detect a dashed season with a dashed episode
#[test]
fn should_detect_a_dashed_season_with_a_dashed_episode() {
    assert_includes(
        "South Park (1997) S-17 E-08 a Song of Ass and Fire 1080p Blu Ray x265 Rcvr",
        json!({ "title": "South Park", "seasons": [17], "episodes": [8] }),
    );
}

/// should detect a dashed season and episode in a dotted name
#[test]
fn should_detect_a_dashed_season_and_episode_in_a_dotted_name() {
    assert_includes(
        "el.juego.del.calamar.-s-02-e-03.2024.-web-dl.-1080p-dual-lat",
        json!({ "title": "el juego del calamar", "seasons": [2], "episodes": [3] }),
    );
}

/// should detect a dashed season before a spaced episode
#[test]
fn should_detect_a_dashed_season_before_a_spaced_episode() {
    assert_includes(
        "Prehistoric Planet (2022) S-02 E 01.720p Atvp Webrip x264 Galaxy TV",
        json!({ "title": "Prehistoric Planet", "seasons": [2], "episodes": [1] }),
    );
}

/// should detect a dashed season at the end
#[test]
fn should_detect_a_dashed_season_at_the_end() {
    assert_includes("Peaky Blinders S-06", json!({ "title": "Peaky Blinders", "seasons": [6] }));
}

/// should detect a dashed season before a disc
#[test]
fn should_detect_a_dashed_season_before_a_disc() {
    assert_includes("SANCTUARY S-3 DISC 5 dvdrip", json!({ "title": "SANCTUARY", "seasons": [3] }));
}

/// should detect a dashed season before a dvd source
#[test]
fn should_detect_a_dashed_season_before_a_dvd_source() {
    assert_includes("Greatest American Hero s-3 dvd rip", json!({ "title": "Greatest American Hero", "seasons": [3] }));
}

/// should keep an explicit season over a later dashed model number
#[test]
fn should_keep_an_explicit_season_over_a_later_dashed_model_number() {
    assert_includes("Pimp My RIde S3 E02 Chevrolet S-10 Pickup", json!({ "seasons": [3], "episodes": [2] }));
}

/// should not detect a dashed model number as season
#[test]
fn should_not_detect_a_dashed_model_number_as_season() {
    assert_missing("Chevrolet S-10 Pickup Restoration 1080p", &["seasons"]);
}

/// should not keep a spelled season range in the title before the same S range
#[test]
fn should_not_keep_a_spelled_season_range_in_the_title_before_the_same_s_range() {
    assert_includes(
        "Game of Thrones Season 1-8 S01-S08 2160 (4K HDR10 AV1) (Opus 5.1 384bit)",
        json!({ "title": "Game of Thrones", "seasons": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
    assert_missing("Game of Thrones Season 1-8 S01-S08 2160 (4K HDR10 AV1) (Opus 5.1 384bit)", &["episodes"]);
}

/// should not keep a spaced spelled seasons range in the title before the same S range
#[test]
fn should_not_keep_a_spaced_spelled_seasons_range_in_the_title_before_the_same_s_range() {
    assert_includes(
        "Game of Thrones Seasons 1 - 8 S01-S08 1080p",
        json!({ "title": "Game of Thrones", "seasons": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
}

/// should detect season 10 followed by a resolution
#[test]
fn should_detect_season_10_followed_by_a_resolution() {
    assert_includes("Friends S10 1080p", json!({ "title": "Friends", "seasons": [10] }));
}

/// should detect season 10 followed by a source
#[test]
fn should_detect_season_10_followed_by_a_source() {
    assert_includes("MASH S10 DVDRip", json!({ "title": "MASH", "seasons": [10] }));
}

/// should detect a dotted season 10 followed by a source
#[test]
fn should_detect_a_dotted_season_10_followed_by_a_source() {
    assert_includes("Friends.S10.DVDRip.XviD", json!({ "title": "Friends", "seasons": [10] }));
}

/// should detect season 20 followed by a resolution
#[test]
fn should_detect_season_20_followed_by_a_resolution() {
    assert_includes(
        "Midsomer.Murders.S20.720p.AMZN.WEBRip.DDP2.0.x264-NTb[rartv]",
        json!({ "title": "Midsomer Murders", "seasons": [20] }),
    );
}

/// should detect season 30 followed by a resolution
#[test]
fn should_detect_season_30_followed_by_a_resolution() {
    assert_includes(
        "Mister.Rogers.Neighborhood.S30.720p.WEBRip.AAC2.0.x264-NOGRP[rartv]",
        json!({ "title": "Mister Rogers Neighborhood", "seasons": [30] }),
    );
}

/// should detect a bracketed season 10
#[test]
fn should_detect_a_bracketed_season_10() {
    assert_includes(
        "The Walking Dead [S10] [720p] [DSNP] [WEB-DL] [x264] [AC3-r4u] [Lektor PL]",
        json!({ "title": "The Walking Dead", "seasons": [10] }),
    );
}

/// should not detect a release group of s10 after an audio tag as season
#[test]
fn should_not_detect_a_release_group_of_s10_after_an_audio_tag_as_season() {
    assert_missing("The Killer's Game 2024 PL 1080p WEB-DL H264 DD5.1-S10", &["seasons"]);
}

/// should not detect a bracketed release group of s10 after an audio tag as season
#[test]
fn should_not_detect_a_bracketed_release_group_of_s10_after_an_audio_tag_as_season() {
    assert_missing("Apollo 13 (1995) [1080p] [WEB-DL] [x264] [E-AC3-S10] [Lektor PL]", &["seasons"]);
}

/// should not detect a release group of s20 before the extension as season
#[test]
fn should_not_detect_a_release_group_of_s20_before_the_extension_as_season() {
    assert_missing("Movie 2024 1080p WEB-DL x264-S20.mkv", &["seasons"]);
}

/// should not detect atmos audio channels as season
#[test]
fn should_not_detect_atmos_audio_channels_as_season() {
    assert_missing(
        "The.Mandalorian.and.Grogu.2026.UHD.2160p.Bluray.IMAX.DoVi.TrueHD.Atmos7.1.HDR.HEVC.X265-FZHD",
        &["seasons"],
    );
}

/// should not detect dotted atmos audio channels as season
#[test]
fn should_not_detect_dotted_atmos_audio_channels_as_season() {
    assert_missing(
        "Spider-Man.No.Way.Home.2021.MULTi.REMUX.2160p.UHD.Blu-ray.DV.HDR.HEVC.ATMOS7.1-DENDA",
        &["seasons"],
    );
}
