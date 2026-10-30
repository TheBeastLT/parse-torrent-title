// Ported from test/episode.js of the js library.

//! Parsing episode

use crate::common::*;

/// should detect regular episode correctly
#[test]
fn should_detect_regular_episode_correctly() {
    assert_includes("The Simpsons S28E21 720p HDTV x264-AVS", json!({ "episode": 21 }));
}

/// should detect regular episode with lowercase correctly
#[test]
fn should_detect_regular_episode_with_lowercase_correctly() {
    assert_includes("breaking.bad.s01e01.720p.bluray.x264-reward", json!({ "episode": 1 }));
}

/// should detect regular season with O instead of zero
#[test]
fn should_detect_regular_season_with_o_instead_of_zero() {
    assert_includes("Arrested Development SO2E04.avi", json!({ "episode": 4 }));
}

/// should detect regular episode with a space between
#[test]
fn should_detect_regular_episode_with_a_space_between() {
    assert_includes("Dragon Ball Super S01 E23 French 1080p HDTV H264-Kesni", json!({ "episode": 23 }));
}

/// should detect regular episode above 1000
#[test]
fn should_detect_regular_episode_above_1000() {
    assert_includes(
        "One.Piece.S01E1116.Lets.Go.Get.It!.Buggys.Big.Declaration.2160p.B-Global.WEB-DL.JPN.AAC2.0.H.264.MSubs-ToonsHub.mkv",
        json!({ "episode": 1116 }),
    );
}

/// should detect regular episode without e symbol after season
#[test]
fn should_detect_regular_episode_without_e_symbol_after_season() {
    assert_includes("The.Witcher.S01.07.2019.Dub.AVC.ExKinoRay.mkv", json!({ "episode": 7 }));
}

/// should detect regular episode with season symbol but wihout episode symbol
#[test]
fn should_detect_regular_episode_with_season_symbol_but_wihout_episode_symbol() {
    assert_includes("Vikings.s02.09.AVC.tahiy.mkv", json!({ "episode": 9 }));
}

/// should detect regular episode with a letter a suffix
#[test]
fn should_detect_regular_episode_with_a_letter_a_suffix() {
    assert_includes("The Twilight Zone 1985 S01E23a Shadow Play.mp4", json!({ "episode": 23 }));
}

/// should detect regular episode without break at the end
#[test]
fn should_detect_regular_episode_without_break_at_the_end() {
    assert_includes("Desperate_housewives_S03E02Le malheur aime la compagnie.mkv", json!({ "episode": 2 }));
}

/// should detect regular episode with a letter b suffix
#[test]
fn should_detect_regular_episode_with_a_letter_b_suffix() {
    assert_includes(
        "Mash S10E01b Thats Show Biz Part 2 1080p H.264 (moviesbyrizzo upload).mp4",
        json!({ "episode": 1 }),
    );
}

/// should detect regular episode with a letter c suffix
#[test]
fn should_detect_regular_episode_with_a_letter_c_suffix() {
    assert_includes("The Twilight Zone 1985 S01E22c The Library.mp4", json!({ "episode": 22 }));
}

/// should detect regular episode without e separator
#[test]
fn should_detect_regular_episode_without_e_separator() {
    assert_includes("Desperate.Housewives.S0615.400p.WEB-DL.Rus.Eng.avi", json!({ "episode": 15 }));
}

/// should detect episode with SxEE format correctly
#[test]
fn should_detect_episode_with_sx_ee_format_correctly() {
    assert_includes("Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV", json!({ "episode": 11 }));
}

/// should detect episode when written as such
#[test]
fn should_detect_episode_when_written_as_such() {
    assert_includes("Anubis saison 01 episode 38 tvrip FR", json!({ "episode": 38 }));
}

/// should detect episode when written as such shortened
#[test]
fn should_detect_episode_when_written_as_such_shortened() {
    assert_includes("Le Monde Incroyable de Gumball - Saison 5 Ep 14 - L'extérieur", json!({ "episode": 14 }));
}

/// should detect episode with parenthesis prefix and x separator
#[test]
fn should_detect_episode_with_parenthesis_prefix_and_x_separator() {
    assert_includes("Smallville (1x02 Metamorphosis).avi", json!({ "episode": 2 }));
}

/// should detect episode with x separator and letter on left
#[test]
fn should_detect_episode_with_x_separator_and_letter_on_left() {
    assert_includes("The.Man.In.The.High.Castle1x01.HDTV.XviD[www.DivxTotaL.com].avi", json!({ "episode": 1 }));
}

/// should detect episode with x separator and letter on right
#[test]
fn should_detect_episode_with_x_separator_and_letter_on_right() {
    assert_includes("clny.3x11m720p.es[www.planetatorrent.com].mkv", json!({ "episode": 11 }));
}

/// should detect episode when similar digits included
#[test]
fn should_detect_episode_when_similar_digits_included() {
    assert_includes(
        "Friends.S07E20.The.One.With.Rachel's.Big.Kiss.720p.BluRay.2CH.x265.HEVC-PSA.mkv",
        json!({ "season": 7, "episode": 20 }),
    );
}

/// should detect episode when separated with x and inside brackets
#[test]
fn should_detect_episode_when_separated_with_x_and_inside_brackets() {
    assert_includes("Friends - [8x18] - The One In Massapequa.mkv", json!({ "season": 8, "episode": 18 }));
}

/// should detect episode when separated with X
#[test]
fn should_detect_episode_when_separated_with_x() {
    assert_includes("Archivo 81 1X7 HDTV XviD Castellano.avi", json!({ "season": 1, "episode": 7 }));
}

/// should detect multiple episodes with x prefix and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_x_prefix_and_hyphen_separator() {
    assert_includes(
        "Friends - [7x23-24] - The One with Monica and Chandler's Wedding + Audio Commentary.mkv",
        json!({ "season": 7, "episodes": [23, 24] }),
    );
}

/// should detect episode when separated with x and has three digit episode
#[test]
fn should_detect_episode_when_separated_with_x_and_has_three_digit_episode() {
    assert_includes("Yu-Gi-Oh 3x089 - Awakening of Evil (Part 4).avi", json!({ "season": 3, "episodes": [89] }));
}

/// should detect multiple episodes with hyphen no spaces separator
#[test]
fn should_detect_multiple_episodes_with_hyphen_no_spaces_separator() {
    assert_includes("611-612 - Desperate Measures, Means & Ends.mp4", json!({ "episodes": [611, 612] }));
}

/// should detect multiple single episode with 10-bit notation in it
#[test]
fn should_detect_multiple_single_episode_with_10_bit_notation_in_it() {
    assert_includes(
        "[Final8]Suisei no Gargantia - 05 (BD 10-bit 1920x1080 x264 FLAC)[E0B15ACF].mkv",
        json!({ "episode": 5 }),
    );
}

/// should detect multiple episodes with episodes prefix and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_episodes_prefix_and_hyphen_separator() {
    assert_includes(
        "Orange Is The New Black Season 5 Episodes 1-10 INCOMPLETE (LEAKED)",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }),
    );
}

/// should detect multiple episodes with ep prefix and hyphen separator inside parentheses
#[test]
fn should_detect_multiple_episodes_with_ep_prefix_and_hyphen_separator_inside_parentheses() {
    assert_includes(
        "Vikings.Season.05.Ep(01-10).720p.WebRip.2Ch.x265.PSA",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }),
    );
}

/// should detect multiple episodes with hyphen separator and follower by open parenthesis
#[test]
fn should_detect_multiple_episodes_with_hyphen_separator_and_follower_by_open_parenthesis() {
    assert_includes(
        "[TBox] Dragon Ball Z Full 1-291(Subbed Jap Vers)",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291] }),
    );
}

/// should detect multiple episodes with e prefix and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_e_prefix_and_hyphen_separator() {
    assert_includes(
        "Marvel's.Agents.of.S.H.I.E.L.D.S02E01-03.Shadows.1080p.WEB-DL.DD5.1",
        json!({ "episodes": [1, 2, 3] }),
    );
}

/// should detect absolute episode with ep prefix
#[test]
fn should_detect_absolute_episode_with_ep_prefix() {
    assert_includes("Naruto Shippuden Ep 107 - Strange Bedfellows.mkv", json!({ "episode": 107 }));
}

/// should detect absolute episode in middle with hyphen dividers
#[test]
fn should_detect_absolute_episode_in_middle_with_hyphen_dividers() {
    assert_includes("Naruto Shippuden - 107 - Strange Bedfellows.mkv", json!({ "episode": 107 }));
}

/// should detect absolute episode in middle with similar resolution value
#[test]
fn should_detect_absolute_episode_in_middle_with_similar_resolution_value() {
    assert_includes("[AnimeRG] Naruto Shippuden - 107 [720p] [x265] [pseudo].mkv", json!({ "episode": 107 }));
}

/// should detect multiple absolute episodes separated by hyphen
#[test]
fn should_detect_multiple_absolute_episodes_separated_by_hyphen() {
    assert_includes("Naruto Shippuuden - 006-007.mkv", json!({ "episodes": [6, 7] }));
}

/// should detect absolute episode correctly not hindered by title digits with hashtag
#[test]
fn should_detect_absolute_episode_correctly_not_hindered_by_title_digits_with_hashtag() {
    assert_includes("321 - Family Guy Viewer Mail #1.avi", json!({ "episode": 321 }));
}

/// should detect hashtag episode
#[test]
fn should_detect_hashtag_episode() {
    assert_includes(
        "[AI-Raws] デュラララ!! #25 天下泰平 (BD HEVC 1920x1080 yuv444p10le FLAC 字幕)[2617AFAB].mkv",
        json!({ "title": "デュラララ!!", "episodes": [25] }),
    );
}

/// should detect hashtag episode followed by episode title
#[test]
fn should_detect_hashtag_episode_followed_by_episode_title() {
    assert_includes(
        "Mobile Suit Gundam the Witch from Mercury #10 Circling Thoughts.mkv",
        json!({ "title": "Mobile Suit Gundam the Witch from Mercury", "episodes": [10] }),
    );
}

/// should not detect hashtag audio track as episode
#[test]
fn should_not_detect_hashtag_audio_track_as_episode() {
    assert_missing(
        "Hannibal 2013-2015 [S01-S03] [10Bit] [1080p.BluRay.H265.EAC3.AS76-FT] [AUDIO #1 ENGLISH] [AUDIO #0 POLISH]",
        &["episodes"],
    );
}

/// should not detect hashtag movie number as episode
#[test]
fn should_not_detect_hashtag_movie_number_as_episode() {
    assert_missing("[LPF] Persona 3 the Movie: #1 Spring of Birth (2013) (BD 1080p x264 AAC)", &["episodes"]);
}

/// should not detect hashtag number after a numbered index as episode
#[test]
fn should_not_detect_hashtag_number_after_a_numbered_index_as_episode() {
    assert_missing("[AliQ] 16 - TV Shorts #1 [1080p].mp4", &["episodes"]);
}

/// should not detect hashtag number in parentheses as episode
#[test]
fn should_not_detect_hashtag_number_in_parentheses_as_episode() {
    assert_missing(
        "[AliQ] Steven Universe Short 03 We Are the Crystal Gems (Classroom Gems #2) [1080p;WEB-DL].mp4",
        &["episodes"],
    );
}

/// should not detect hashtag number after a dotted index as episode
#[test]
fn should_not_detect_hashtag_number_after_a_dotted_index_as_episode() {
    assert_missing("04. Panel Discussion #2 by  Various from Together for the Gospel 2006.avi", &["episodes"]);
}

/// should not detect hashtag number after a date as episode
#[test]
fn should_not_detect_hashtag_number_after_a_date_as_episode() {
    assert_missing("2023-04-14 Jays Catch #2.mkv", &["episodes"]);
}

/// should detect hashtag episode after a year
#[test]
fn should_detect_hashtag_episode_after_a_year() {
    assert_includes(
        "[AI-Raws] 鉄腕アトム(1980) #29 (BD HEVC 1528x1080 FLAC)[DA03CCC3].mkv",
        json!({ "title": "鉄腕アトム", "episodes": [29] }),
    );
}

/// should detect hashtag episode after a bare year
#[test]
fn should_detect_hashtag_episode_after_a_bare_year() {
    assert_includes(
        "Billy Connolly - World Tour Of Scotland 1994 #6.avi",
        json!({ "title": "Billy Connolly - World Tour Of Scotland", "episodes": [6] }),
    );
}

/// should detect japanese episode marker
#[test]
fn should_detect_japanese_episode_marker() {
    assert_includes(
        "[ドラマ] ハケンの品格 第02話 「大論争！派遣VS正社員」 (1280x720 DivX640).avi",
        json!({ "title": "ハケンの品格", "episodes": [2] }),
    );
}

/// should detect a simplified chinese episode marker
#[test]
fn should_detect_a_simplified_chinese_episode_marker() {
    assert_includes(
        "[Doomdos] - Clevatess Ⅱ - 第13话 - [1080p BILIBILI COM WEB-DL]",
        json!({ "title": "Clevatess Ⅱ", "episodes": [13] }),
    );
}

/// should detect a simplified chinese episode marker after a chinese season
#[test]
fn should_detect_a_simplified_chinese_episode_marker_after_a_chinese_season() {
    assert_includes(
        "[Doomdos] - 小P优优 第五季 中文配音 - 第23话 - [1080p BILIBILI COM WEB-DL]",
        json!({ "seasons": [5], "episodes": [23] }),
    );
}

/// should detect a chinese ji episode marker in a bracketed episode title
#[test]
fn should_detect_a_chinese_ji_episode_marker_in_a_bracketed_episode_title() {
    assert_includes(
        "[银色子弹字幕组][名侦探柯南][第1209集 被盯上的是谁？][WEBRIP][简日双语MP4][1080P]",
        json!({ "title": "名侦探柯南", "episodes": [1209] }),
    );
}

/// should detect a chinese ji episode marker before a codec
#[test]
fn should_detect_a_chinese_ji_episode_marker_before_a_codec() {
    assert_includes(
        "【极影字幕·毁片党】完全相反的你和我 第17集 GB_CN AV1_opus 1080p",
        json!({ "title": "完全相反的你和我", "episodes": [17] }),
    );
}

/// should detect four digit episode range
#[test]
fn should_detect_four_digit_episode_range() {
    assert_includes(
        "[PrinceOtaku] One Piece - Egghead Island - Episodes 1123-1133 (English Dub) 720p",
        json!({ "title": "One Piece - Egghead Island" }),
    );
    assert_field_len(
        "[PrinceOtaku] One Piece - Egghead Island - Episodes 1123-1133 (English Dub) 720p",
        "episodes",
        11,
    );
    assert_field_at(
        "[PrinceOtaku] One Piece - Egghead Island - Episodes 1123-1133 (English Dub) 720p",
        "episodes",
        0,
        json!(1123),
    );
}

/// should detect four digit episode range with ep marker
#[test]
fn should_detect_four_digit_episode_range_with_ep_marker() {
    assert_field_len("One Piece Ep 1100-1110", "episodes", 11);
    assert_field_at("One Piece Ep 1100-1110", "episodes", 10, json!(1110));
}

/// should not detect year range after episodes word as episode range
#[test]
fn should_not_detect_year_range_after_episodes_word_as_episode_range() {
    assert_missing(
        "SUPERGIRL - Complete Season 5 S05 with CROSSOVER Episodes (2019-2020) - 720p AMZN Web-DL x264",
        &["episodes"],
    );
}

/// should detect phase episode
#[test]
fn should_detect_phase_episode() {
    assert_includes(
        "Mobile Suit Gundam SEED - Phase 01 - False Peace.mkv",
        json!({ "title": "Mobile Suit Gundam SEED", "episodes": [1] }),
    );
}

/// should not detect single digit phase as episode
#[test]
fn should_not_detect_single_digit_phase_as_episode() {
    assert_missing("Star Trek New Voyages Phase 2.mkv", &["episodes"]);
}

/// should detect absolute episode correctly not hindered by title digits with apostrophe
#[test]
fn should_detect_absolute_episode_correctly_not_hindered_by_title_digits_with_apostrophe() {
    assert_includes("512 - Airport '07.avi", json!({ "episode": 512 }));
}

/// should detect absolute episode at the begining even though its mixed with season
#[test]
fn should_detect_absolute_episode_at_the_begining_even_though_its_mixed_with_season() {
    assert_includes("102 - The Invitation.avi", json!({ "episode": 102 }));
}

/// should detect absolute episode double digit at the beginning
#[test]
fn should_detect_absolute_episode_double_digit_at_the_beginning() {
    assert_includes("02 The Invitation.mp4", json!({ "episode": 2 }));
}

/// should detect absolute episode triple digit at the beginning with zero padded
#[test]
fn should_detect_absolute_episode_triple_digit_at_the_beginning_with_zero_padded() {
    assert_includes("004 - Male Unbonding - [DVD].avi", json!({ "episode": 4 }));
}

/// should detect multiple absolute episodes separated by comma
#[test]
fn should_detect_multiple_absolute_episodes_separated_by_comma() {
    assert_includes(
        "The Amazing World of Gumball - 103, 104 - The Third - The Debt.mkv",
        json!({ "episodes": [103, 104] }),
    );
}

/// should detect absolute episode with a possible match at the end
#[test]
fn should_detect_absolute_episode_with_a_possible_match_at_the_end() {
    assert_includes(
        "The Amazing World of Gumball - 103 - The End - The Dress (720p.x264.ac3-5.1) [449].mkv",
        json!({ "episode": 103 }),
    );
}

/// should detect absolute episode with a divided episode into a part
#[test]
fn should_detect_absolute_episode_with_a_divided_episode_into_a_part() {
    assert_includes(
        "The Amazing World of Gumball - 107a - The Mystery (720p.x264.ac3-5.1) [449].mkv",
        json!({ "episode": 107 }),
    );
}

/// should detect absolute episode with a divided episode into b part
#[test]
fn should_detect_absolute_episode_with_a_divided_episode_into_b_part() {
    assert_includes(
        "The Amazing World of Gumball - 107b - The Mystery (720p.x264.ac3-5.1) [449].mkv",
        json!({ "episode": 107 }),
    );
}

/// should detect episode withing brackets with dot separator
#[test]
fn should_detect_episode_withing_brackets_with_dot_separator() {
    assert_includes("[5.01] Weight Loss.avi", json!({ "episode": 1 }));
}

/// should detect episode in hundreds withing brackets with dot separator
#[test]
fn should_detect_episode_in_hundreds_withing_brackets_with_dot_separator() {
    assert_includes("Dragon Ball [5.134] Preliminary Peril.mp4", json!({ "episode": 134 }));
}

/// should detect episode with spaces and hyphen separator
#[test]
fn should_detect_episode_with_spaces_and_hyphen_separator() {
    assert_includes("S01 - E03 - Fifty-Fifty.mkv", json!({ "episode": 3 }));
}

/// should detect multiple episodes separated with plus
#[test]
fn should_detect_multiple_episodes_separated_with_plus() {
    assert_includes("The Office S07E25+E26 Search Committee.mp4", json!({ "episodes": [25, 26] }));
}

/// [animeawake] Naruto Shippuden - 124 - Art_2.mkv
#[test]
fn animeawake_naruto_shippuden_124_art_2_mkv() {
    assert_includes("[animeawake] Naruto Shippuden - 124 - Art_2.mkv", json!({ "episode": 124 }));
}

/// [animeawake] Naruto Shippuden - 072 - The Quietly Approaching Threat_2.mkv
#[test]
fn animeawake_naruto_shippuden_072_the_quietly_approaching_threat_2_mkv() {
    assert_includes(
        "[animeawake] Naruto Shippuden - 072 - The Quietly Approaching Threat_2.mkv",
        json!({ "episode": 72 }),
    );
}

/// [animeawake] Naruto Shippuden - 120 - Kakashi Chronicles. Boys' Life on the Battlefield. Part 2.mkv
#[test]
fn animeawake_naruto_shippuden_120_kakashi_chronicles_boys_life_on_the_battlefield_part_2_mkv() {
    assert_includes(
        "[animeawake] Naruto Shippuden - 120 - Kakashi Chronicles. Boys' Life on the Battlefield. Part 2.mkv",
        json!({ "episode": 120 }),
    );
}

/// should detect single episode even when a possible range identifier hyphen is present
#[test]
fn should_detect_single_episode_even_when_a_possible_range_identifier_hyphen_is_present() {
    assert_includes(
        "Supernatural - S03E01 - 720p BluRay x264-Belex - Dual Audio + Legenda.mkv",
        json!({ "episode": 1 }),
    );
}

/// should not detect episodes when the range is for season
#[test]
fn should_not_detect_episodes_when_the_range_is_for_season() {
    assert_missing("[F-D] Fairy Tail Season 1 -6 + Extras [480P][Dual-Audio]", &["episodes"]);
}

/// should not detect episodes when the range is for seasons
#[test]
fn should_not_detect_episodes_when_the_range_is_for_seasons() {
    assert_missing("House MD All Seasons (1-8) 720p Ultra-Compressed", &["episodes"]);
}

/// should not detect episode when it indicates sequence of the movie in between hyhen separators
#[test]
fn should_not_detect_episode_when_it_indicates_sequence_of_the_movie_in_between_hyhen_separat() {
    assert_missing("Dragon Ball Z Movie - 09 - Bojack Unbound - 1080p", &["episodes"]);
}

/// should not detect episode when it indicates sequence of the movie at the start
#[test]
fn should_not_detect_episode_when_it_indicates_sequence_of_the_movie_at_the_start() {
    assert_missing("09 Movie - Dragon Ball Z - Bojack Unbound", &["episodes"]);
}

/// should detect detect episode with x separator and e prefix
#[test]
fn should_detect_detect_episode_with_x_separator_and_e_prefix() {
    assert_includes("24 - S01xE03.mp4", json!({ "season": 1, "episode": 3 }));
}

/// should detect detect episode correctly and not episode range
#[test]
fn should_detect_detect_episode_correctly_and_not_episode_range() {
    assert_includes("24 - S01E04 - x264 - dilpill.mkv", json!({ "episode": 4 }));
}

/// should detect detect episode correctly and not episode range with two codecs
#[test]
fn should_detect_detect_episode_correctly_and_not_episode_range_with_two_codecs() {
    assert_includes("24.Legacy.S01E05.720p.HEVC.x265-MeGusta", json!({ "episode": 5 }));
}

/// should detect detect absolute episode with a version
#[test]
fn should_detect_detect_absolute_episode_with_a_version() {
    assert_includes("[F-D] Fairy.Tail.-.004v2.-. [480P][Dual-Audio].mkv", json!({ "episode": 4 }));
}

/// should detect detect anime episode when title contains number range
#[test]
fn should_detect_detect_anime_episode_when_title_contains_number_range() {
    assert_includes(
        "[Erai-raws] 2-5 Jigen no Ririsa - 08 [480p][Multiple Subtitle][972D0669].mkv",
        json!({ "episode": 8 }),
    );
}

/// should detect detect absolute episode with a version and ep suffix
#[test]
fn should_detect_detect_absolute_episode_with_a_version_and_ep_suffix() {
    assert_includes("[Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv", json!({ "episode": 2 }));
}

/// should detect detect absolute episode and not detect any season modifier
#[test]
fn should_detect_detect_absolute_episode_and_not_detect_any_season_modifier() {
    assert_includes(
        "[a-s]_fairy_tail_-_003_-_infiltrate_the_everlue_mansion__rs2_[1080p_bd-rip][4CB16872].mkv",
        json!({ "episode": 3 }),
    );
    assert_missing(
        "[a-s]_fairy_tail_-_003_-_infiltrate_the_everlue_mansion__rs2_[1080p_bd-rip][4CB16872].mkv",
        &["seasons"],
    );
}

/// should detect episode after season with separator
#[test]
fn should_detect_episode_after_season_with_separator() {
    assert_includes("Food Wars! Shokugeki No Souma S4 - 11 (1080p)(HEVC x265 10bit)", json!({ "episode": 11 }));
}

/// should not detect episode range for mismatch episode marker e vs ep
#[test]
fn should_not_detect_episode_range_for_mismatch_episode_marker_e_vs_ep() {
    assert_includes("Dragon Ball Super S05E53 - Ep.129.mkv", json!({ "episode": 53 }));
}

/// should not detect episode range with other parameter ending
#[test]
fn should_not_detect_episode_range_with_other_parameter_ending() {
    assert_includes("DShaun.Micallefs.MAD.AS.HELL.S10E03.576p.x642-YADNUM.mkv", json!({ "episode": 3 }));
}

/// should not detect episode range with spaced hyphen separator
#[test]
fn should_not_detect_episode_range_with_spaced_hyphen_separator() {
    assert_includes("The Avengers (EMH) - S01 E15 - 459 (1080p - BluRay).mp4", json!({ "episode": 15 }));
}

/// should detect episode with a dot and hyphen separator
#[test]
fn should_detect_episode_with_a_dot_and_hyphen_separator() {
    assert_includes("My Little Pony FiM - 6.01 - No Second Prances.mkv", json!({ "episode": 1 }));
}

/// should detect season with a dot and episode prefix
#[test]
fn should_detect_season_with_a_dot_and_episode_prefix() {
    assert_includes("Desperate Housewives - Episode 1.22 - Goodbye for now.avi", json!({ "episode": 22 }));
}

/// should detect season with a dot and episode prefix v2
#[test]
fn should_detect_season_with_a_dot_and_episode_prefix_v2() {
    assert_includes("All of Us Are Dead . 2022 . S01 EP #1.2.mkv", json!({ "episode": 2 }));
}

/// should detect episode with number in a title
#[test]
fn should_detect_episode_with_number_in_a_title() {
    assert_includes("Mob Psycho 100 - 09 [1080p].mkv", json!({ "episode": 9 }));
}

/// should detect episode with number and a hyphen after it in a title
#[test]
fn should_detect_episode_with_number_and_a_hyphen_after_it_in_a_title() {
    assert_includes("3-Nen D-Gumi Glass no Kamen - 13 [480p]", json!({ "episode": 13 }));
}

/// should detect episode with of separator
#[test]
fn should_detect_episode_with_of_separator() {
    assert_includes("BBC Indian Ocean with Simon Reeve 5of6 Sri Lanka to Bangladesh.avi", json!({ "episode": 5 }));
}

/// should detect episode with of separator v1
#[test]
fn should_detect_episode_with_of_separator_v1() {
    assert_includes("Witches Of Salem - 2Of4 - Road To Hell - Gr.mkv", json!({ "episode": 2 }));
}

/// should detect episode with of separator v2
#[test]
fn should_detect_episode_with_of_separator_v2() {
    assert_includes("Das Boot Miniseries Original Uncut-Reevel Cd2 Of 3.avi", json!({ "episode": 2 }));
}

/// should detect multiple episodes with multiple E sign and no separator
#[test]
fn should_detect_multiple_episodes_with_multiple_e_sign_and_no_separator() {
    assert_includes("Stargate Universe S01E01E02E03.mp4", json!({ "episodes": [1, 2, 3] }));
}

/// should detect multiple episodes with multiple E sign and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_multiple_e_sign_and_hyphen_separator() {
    assert_includes("Stargate Universe S01E01-E02-E03.mp4", json!({ "episodes": [1, 2, 3] }));
}

/// should detect multiple episodes with eps prefix and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_eps_prefix_and_hyphen_separator() {
    assert_includes("MARATHON EPISODES/Orphan Black S3 Eps.05-08.mp4", json!({ "episodes": [5, 6, 7, 8] }));
}

/// should detect multiple episodes with E sign and hyphen spaced separator
#[test]
fn should_detect_multiple_episodes_with_e_sign_and_hyphen_spaced_separator() {
    assert_includes(
        "Pokemon Black & White E10 - E17 [CW] AVI",
        json!({ "episodes": [10, 11, 12, 13, 14, 15, 16, 17] }),
    );
}

/// should detect multiple episodes with E sign and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_e_sign_and_hyphen_separator() {
    assert_includes("Pokémon.S01E01-E04.SWEDISH.VHSRip.XviD-aka", json!({ "episodes": [1, 2, 3, 4] }));
}

/// should detect episode with single episode and not range
#[test]
fn should_detect_episode_with_single_episode_and_not_range() {
    assert_includes("[HorribleSubs] White Album 2 - 06 [1080p].mkv", json!({ "episode": 6 }));
}

/// should detect episode with E symbols without season
#[test]
fn should_detect_episode_with_e_symbols_without_season() {
    assert_includes("Mob.Psycho.100.II.E10.720p.WEB.x264-URANiME.mkv", json!({ "episode": 10 }));
}

/// should detect episode with E symbols without season v2
#[test]
fn should_detect_episode_with_e_symbols_without_season_v2() {
    assert_includes("E5.mkv", json!({ "episode": 5 }));
}

/// should detect episode without season
#[test]
fn should_detect_episode_without_season() {
    assert_includes("[OMDA] Bleach - 002 (480p x264 AAC) [rich_jc].mkv", json!({ "episode": 2 }));
}

/// should detect episode with a episode code including multiple numbers
#[test]
fn should_detect_episode_with_a_episode_code_including_multiple_numbers() {
    assert_includes(
        "[ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv",
        json!({ "episode": 19 }),
    );
}

/// should detect multiple episodes with x episode marker and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_x_episode_marker_and_hyphen_separator() {
    assert_includes(
        "BoJack Horseman [06x01-08 of 16] (2019-2020) WEB-DLRip 720p",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
}

/// should detect multiple episodes with russian episode marker and hyphen separator
#[test]
fn should_detect_multiple_episodes_with_russian_episode_marker_and_hyphen_separator() {
    assert_includes(
        "Мистер Робот / Mr. Robot / Сезон: 2 / Серии: 1-5 (12) [2016, США, WEBRip 1080p] MVO",
        json!({ "episodes": [1, 2, 3, 4, 5] }),
    );
}

/// should detect episode with russian episode marker and single episode
#[test]
fn should_detect_episode_with_russian_episode_marker_and_single_episode() {
    assert_includes("Викинги / Vikings / Сезон: 5 / Серии: 1 [2017, WEB-DL 1080p] MVO", json!({ "episode": 1 }));
}

/// should detect episode with russian episode marker and single episode and with total episodes value
#[test]
fn should_detect_episode_with_russian_episode_marker_and_single_episode_and_with_total_episod() {
    assert_includes("Викинги / Vikings / Сезон: 5 / Серии: 1 из 20 [2017, WEB-DL 1080p] MVO", json!({ "episode": 1 }));
}

/// should detect episode with russian arabic total episodes value separator
#[test]
fn should_detect_episode_with_russian_arabic_total_episodes_value_separator() {
    assert_includes("Prehistoric park.3iz6.Supercroc.DVDRip.Xvid.avi", json!({ "episode": 3 }));
}

/// should detect episode with shortened russian episode name
#[test]
fn should_detect_episode_with_shortened_russian_episode_name() {
    assert_includes("Меч (05 сер.) - webrip1080p.mkv", json!({ "episode": 5 }));
}

/// should detect episode with full russian episode name
#[test]
fn should_detect_episode_with_full_russian_episode_name() {
    assert_includes("Серия 11.mkv", json!({ "episode": 11 }));
}

/// should detect episode with full different russian episode name
#[test]
fn should_detect_episode_with_full_different_russian_episode_name() {
    assert_includes(
        "Разрушители легенд. MythBusters. Сезон 15. Эпизод 09. Скрытая угроза (2015).avi",
        json!({ "episode": 9 }),
    );
}

/// should detect episode with full different russian episode name v2
#[test]
fn should_detect_episode_with_full_different_russian_episode_name_v2() {
    assert_includes("Леди Баг и Супер-Кот – Сезон 3, Эпизод 21 – Кукловод 2 [1080p].mkv", json!({ "episode": 21 }));
}

/// should detect episode with full russian episode name with case suffix
#[test]
fn should_detect_episode_with_full_russian_episode_name_with_case_suffix() {
    assert_includes("Проклятие острова ОУК_ 5-й сезон 09-я серия_ Прорыв Дэна.avi", json!({ "episode": 9 }));
}

/// should detect episode with full russian episode name and no prefix
#[test]
fn should_detect_episode_with_full_russian_episode_name_and_no_prefix() {
    assert_includes("Интерны. Сезон №9. Серия №180.avi", json!({ "episode": 180 }));
}

/// should detect episode with russian episode name in non kirilica
#[test]
fn should_detect_episode_with_russian_episode_name_in_non_kirilica() {
    assert_includes("Tajny.sledstviya-20.01.serya.WEB-DL.(1080p).by.lunkin.mkv", json!({ "episode": 1 }));
}

/// should detect episode with russian episode name in non kirilica alternative 2
#[test]
fn should_detect_episode_with_russian_episode_name_in_non_kirilica_alternative_2() {
    assert_includes("Zvezdnie.Voiny.Voina.Klonov.3.sezon.22.seria.iz.22.XviD.HDRip.avi", json!({ "episode": 22 }));
}

/// should detect season with russian episode shortened word
#[test]
fn should_detect_season_with_russian_episode_shortened_word() {
    assert_includes("Otchayannie.domochozyaiki.(8.sez.21.ser.iz.23).2012.XviD.HDTVRip.avi", json!({ "episode": 21 }));
}

/// should detect episode with russian episode name in non kirilica alternative 3
#[test]
fn should_detect_episode_with_russian_episode_name_in_non_kirilica_alternative_3() {
    assert_includes("MosGaz.(08.seriya).2012.WEB-DLRip(AVC).ExKinoRay.mkv", json!({ "episode": 8 }));
}

/// should detect episode with russian episode name in non kirilica alternative 5
#[test]
fn should_detect_episode_with_russian_episode_name_in_non_kirilica_alternative_5() {
    assert_includes("Tajny.sledstvija.(2.sezon.12.serija.iz.12).2002.XviD.DVDRip.avi", json!({ "episode": 12 }));
}

/// should detect episodes with russian x separator
#[test]
fn should_detect_episodes_with_russian_x_separator() {
    assert_includes(
        "Discovery. Парни с Юкона / Yokon Men [06х01-08] (2017) HDTVRip от GeneralFilm | P1",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
}

/// should detect episodes with hyphen separator between episode
#[test]
fn should_detect_episodes_with_hyphen_separator_between_episode() {
    assert_includes("2-06. Девичья сила.mkv", json!({ "episode": 6 }));
}

/// should detect episodes with hyphen separator between episode v2
#[test]
fn should_detect_episodes_with_hyphen_separator_between_episode_v2() {
    assert_includes("4-13 Cursed (HD).m4v", json!({ "episode": 13 }));
}

/// should detect not episodes with hyphen separator between episode when it's date
#[test]
fn should_detect_not_episodes_with_hyphen_separator_between_episode_when_it_s_date() {
    assert_missing("The Ed Show 10-19-12.mp4", &["episodes"]);
    assert_includes("The Ed Show 10-19-12.mp4", json!({ "date": "2012-10-19" }));
}

/// should detect not episodes with hyphen separator between episode when it's not supported date
#[test]
fn should_detect_not_episodes_with_hyphen_separator_between_episode_when_it_s_not_supported_d() {
    assert_includes("Hogan's Heroes - 516 - Get Fit or Go Flight - 1-09-70.divx", json!({ "episode": 516 }));
}

/// should detect episodes with hyphen separator between episode v3
#[test]
fn should_detect_episodes_with_hyphen_separator_between_episode_v3() {
    assert_includes("Доктор Хаус 03-20.mkv", json!({ "episode": 20 }));
}

/// should detect episodes with hyphen separator between episode v4
#[test]
fn should_detect_episodes_with_hyphen_separator_between_episode_v4() {
    assert_includes("Комиссар Рекс 11-13.avi", json!({ "episode": 13 }));
}

/// should detect episode after ordinal season and hyphen separator
#[test]
fn should_detect_episode_after_ordinal_season_and_hyphen_separator() {
    assert_includes("Kyoukai no Rinne (TV) 3rd Season - 23 [1080p]", json!({ "episode": 23 }));
}

/// should detect episode after ordinal season and hyphen separator and multiple spaces
#[test]
fn should_detect_episode_after_ordinal_season_and_hyphen_separator_and_multiple_spaces() {
    assert_includes(
        "[224] Shingeki no Kyojin - S03 - Part 1 -  13 [BDRip.1080p.x265.FLAC].mkv",
        json!({ "episode": 13 }),
    );
}

/// should detect spanish full episode identifier
#[test]
fn should_detect_spanish_full_episode_identifier() {
    assert_includes("El Chema Temporada 1 Capitulo 25", json!({ "episode": 25 }));
}

/// should detect spanish partial episode identifier
#[test]
fn should_detect_spanish_partial_episode_identifier() {
    assert_includes("Juego de Tronos - Temp.2 [ALTA DEFINICION 720p][Cap.209][Spanish].mkv", json!({ "episode": 209 }));
}

/// should detect spanish partial long episode identifier
#[test]
fn should_detect_spanish_partial_long_episode_identifier() {
    assert_includes(
        "Blue Bloods - Temporada 11 [HDTV 720p][Cap.1103][AC3 5.1 Castellano][www.PCTmix.com].mkv",
        json!({ "episode": 1103 }),
    );
}

/// should detect spanish partial episode identifier with common typo
#[test]
fn should_detect_spanish_partial_episode_identifier_with_common_typo() {
    assert_includes("Para toda la humanidad [4k 2160p][Caap.406](wolfmax4k.com).mkv", json!({ "episode": 406 }));
}

/// should detect spanish partial episode identifier v2
#[test]
fn should_detect_spanish_partial_episode_identifier_v2() {
    assert_includes("Mazinger-Z-Cap-52.avi", json!({ "episode": 52 }));
}

/// should detect latino full episode identifier
#[test]
fn should_detect_latino_full_episode_identifier() {
    assert_includes(
        "Yu-Gi-Oh! ZEXAL Temporada 1 Episodio 009 Dual Latino e Inglés [B3B4970E].mkv",
        json!({ "episode": 9 }),
    );
}

/// should detect spanish multiple episode identifier
#[test]
fn should_detect_spanish_multiple_episode_identifier() {
    assert_includes(
        "Bleach 10º Temporada - 215 ao 220 - [DB-BR]",
        json!({ "episodes": [215, 216, 217, 218, 219, 220] }),
    );
}

/// should detect spanish short season identifier
#[test]
fn should_detect_spanish_short_season_identifier() {
    assert_includes("My Little Pony - A Amizade é Mágica - T02E22.mp4", json!({ "episodes": [22] }));
}

/// should detect spanish short season identifier with xe separator
#[test]
fn should_detect_spanish_short_season_identifier_with_xe_separator() {
    assert_includes("30 M0N3D4S ESP T01XE08.mkv", json!({ "episodes": [8] }));
}

/// should not detect episode in episode checksum code
#[test]
fn should_not_detect_episode_in_episode_checksum_code() {
    assert_includes("[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90].mkv", json!({ "episodes": [11] }));
}

/// should not detect episode in episode checksum code without container
#[test]
fn should_not_detect_episode_in_episode_checksum_code_without_container() {
    assert_includes("[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]", json!({ "episodes": [11] }));
}

/// should not detect episode in episode checksum code with paranthesis
#[test]
fn should_not_detect_episode_in_episode_checksum_code_with_paranthesis() {
    assert_includes("(Hi10)_Re_Zero_Shin_Henshuu-ban_-_02v2_(720p)_(DDY)_(72006E34).mkv", json!({ "episodes": [2] }));
}

/// should not detect episode before season
#[test]
fn should_not_detect_episode_before_season() {
    assert_missing("22-7 (Season 1) (1080p)(HEVC x265 10bit)(Eng-Subs)-Judas[TGx] ⭐", &["episodes"]);
}

/// should detect multiple episode with tilde separator
#[test]
fn should_detect_multiple_episode_with_tilde_separator() {
    assert_includes(
        "[Erai-raws] Carole and Tuesday - 01 ~ 12 [1080p][Multiple Subtitle]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }),
    );
}

/// should detect multiple episode with tilde separator and season prefix
#[test]
fn should_detect_multiple_episode_with_tilde_separator_and_season_prefix() {
    assert_includes(
        "[Erai-raws] 3D Kanojo - Real Girl 2nd Season - 01 ~ 12 [720p]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }),
    );
}

/// should detect multiple episode with hyphen separator
#[test]
fn should_detect_multiple_episode_with_hyphen_separator() {
    assert_includes(
        "[FFA] Koi to Producer: EVOL×LOVE - 01 - 12 [1080p][HEVC][AAC]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }),
    );
}

/// should detect multiple episode with hyphen separator between parenthesis
#[test]
fn should_detect_multiple_episode_with_hyphen_separator_between_parenthesis() {
    assert_includes(
        "[BenjiD] Quan Zhi Gao Shou (The King’s Avatar) / Full-Time Master S01 (01 - 12) [1080p x265] [Soft sub] V2",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }),
    );
}

/// [HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ
#[test]
fn hr_boku_no_hero_academia_87_s4_24_1080p_hevc_multi_subs_hr_gz() {
    assert_includes("[HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ", json!({ "episodes": [24] }));
}

/// Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]
#[test]
fn tokyo_ghoul_root_a_07_s2_07_eng_sub_480p_email_protected() {
    assert_includes("Tokyo Ghoul Root A - 07 [S2-07] [Eng Sub] 480p [email protected]", json!({ "episodes": [7] }));
}

/// black-ish.S05E02.1080p..x265.10bit.EAC3.6.0-Qman[UTR].mkv
#[test]
fn black_ish_s05_e02_1080p_x265_10bit_eac3_6_0_qman_utr_mkv() {
    assert_includes("black-ish.S05E02.1080p..x265.10bit.EAC3.6.0-Qman[UTR].mkv", json!({ "episodes": [2] }));
}

/// [Eng Sub] Rebirth Ep #36 [8CF3ADFA].mkv
#[test]
fn eng_sub_rebirth_ep_36_8_cf3_adfa_mkv() {
    assert_includes("[Eng Sub] Rebirth Ep #36 [8CF3ADFA].mkv", json!({ "episodes": [36] }));
}

/// [92 Impatient Eilas & Miyafuji] Strike Witches - Road to Berlin - 01 [1080p][BCDFF6A2].mkv
#[test]
fn t_92_impatient_eilas_miyafuji_strike_witches_road_to_berlin_01_1080p_bcdff6_a2_mkv() {
    assert_includes(
        "[92 Impatient Eilas & Miyafuji] Strike Witches - Road to Berlin - 01 [1080p][BCDFF6A2].mkv",
        json!({ "episodes": [1] }),
    );
}

/// [224] Darling in the FranXX - 14 [BDRip.1080p.x265.FLAC].mkv
#[test]
fn t_224_darling_in_the_fran_xx_14_bdrip_1080p_x265_flac_mkv() {
    assert_includes("[224] Darling in the FranXX - 14 [BDRip.1080p.x265.FLAC].mkv", json!({ "episodes": [14] }));
}

/// [Erai-raws] Granblue Fantasy The Animation Season 2 - 10 [1080p][Multiple Subtitle].mkv
#[test]
fn erai_raws_granblue_fantasy_the_animation_season_2_10_1080p_multiple_subtitle_mkv() {
    assert_includes(
        "[Erai-raws] Granblue Fantasy The Animation Season 2 - 10 [1080p][Multiple Subtitle].mkv",
        json!({ "episodes": [10] }),
    );
}

/// [Erai-raws] Shingeki no Kyojin Season 3 - 11 [1080p][Multiple Subtitle].mkv
#[test]
fn erai_raws_shingeki_no_kyojin_season_3_11_1080p_multiple_subtitle_mkv() {
    assert_includes(
        "[Erai-raws] Shingeki no Kyojin Season 3 - 11 [1080p][Multiple Subtitle].mkv",
        json!({ "episodes": [11] }),
    );
}

/// should detect single zero episode
#[test]
fn should_detect_single_zero_episode() {
    assert_includes("DARKER THAN BLACK - S00E00.mkv", json!({ "episode": 0 }));
}

/// should detect anime episode when title contain similar pattern
#[test]
fn should_detect_anime_episode_when_title_contain_similar_pattern() {
    assert_includes("[Erai-raws] 22-7 - 11 .mkv", json!({ "episode": 11 }));
}

/// should detect anime episode after year in title
#[test]
fn should_detect_anime_episode_after_year_in_title() {
    assert_includes(
        "[Golumpa] Star Blazers 2202 - 22 (Uchuu Senkan Yamato 2022) [FuniDub 1080p x264 AAC] [A24B89C8].mkv",
        json!({ "episode": 22 }),
    );
}

/// should detect anime episode after year property
#[test]
fn should_detect_anime_episode_after_year_property() {
    assert_includes("[SubsPlease] Digimon Adventure (2020) - 35 (720p) [4E7BA28A].mkv", json!({ "episode": 35 }));
}

/// should detect anime absolute episode before episode name
#[test]
fn should_detect_anime_absolute_episode_before_episode_name() {
    assert_includes("[HnY] Beyblade X 136 - The 88th Prince.mkv", json!({ "title": "Beyblade X", "episode": 136 }));
}

/// should detect anime absolute episode before episode name with version number
#[test]
fn should_detect_anime_absolute_episode_before_episode_name_with_version_number() {
    assert_includes("[HnY] Beyblade X 135 - Version 2.0.mkv", json!({ "title": "Beyblade X", "episode": 135 }));
}

/// should detect anime zero padded episode before episode name
#[test]
fn should_detect_anime_zero_padded_episode_before_episode_name() {
    assert_includes(
        "[HnY] Beyblade Burst Sparking 09 - Phantom Dragon! Mirage Fafnir!! (1080p).mkv",
        json!({ "title": "Beyblade Burst Sparking", "episode": 9 }),
    );
}

/// should not detect anime movie number before movie name as episode
#[test]
fn should_not_detect_anime_movie_number_before_movie_name_as_episode() {
    assert_missing(
        "[Ah-le-le] Detective Conan Movie 13 - The Raven Chaser (BDRip 720p x264 AAC AC3) [D38C77B7].mkv",
        &["episodes"],
    );
}

/// should not detect title number before movie name as episode
#[test]
fn should_not_detect_title_number_before_movie_name_as_episode() {
    assert_missing(
        "[Hudie] Digimon Adventure 02 - Digimon Hurricane Touchdown!! - Transcendent Evolution!! The Golden Digimentals (Movie 3, BD) [2B6CA1B4].mkv",
        &["episodes"],
    );
}

/// should not detect sequel number before name with year as episode
#[test]
fn should_not_detect_sequel_number_before_name_with_year_as_episode() {
    assert_missing("[ATG] Resident evil 05 - Retribution [ATG 2012] 720p.mp4", &["episodes"]);
}

/// should not detect title number before movie part name as episode
#[test]
fn should_not_detect_title_number_before_movie_part_name_as_episode() {
    assert_missing(
        "[Koten_Gars] Adieu Galaxy Express 999 - Movie 2 [BD][Hi10][1080p][LPCM+TrueHD] [3C8686BC].mkv",
        &["episodes"],
    );
}

/// should not detect movie number after spaced movie word as episode
#[test]
fn should_not_detect_movie_number_after_spaced_movie_word_as_episode() {
    assert_missing("[Anime Time] Dragon Ball - Movie  001 - Curse Of The Blood Rubies.mkv", &["episodes"]);
}

/// should not detect spanish movie number as episode
#[test]
fn should_not_detect_spanish_movie_number_as_episode() {
    assert_missing(
        "[A&C] One Piece Película 02 - Jango's Dance Carnival [BDrip] [Jap] [Multi-Subs] [149FE7EE].mkv",
        &["episodes"],
    );
}

/// should not detect decimal movie number as episode
#[test]
fn should_not_detect_decimal_movie_number_as_episode() {
    assert_missing("[Baws] Evangelion 2.22 - You Can (Not) Advance (1080p BD HEVC FLAC) [56E7A5B8].mkv", &["episodes"]);
}

/// should not detect title number before season marker as episode
#[test]
fn should_not_detect_title_number_before_season_marker_as_episode() {
    assert_missing("[Salieri] Mob Psycho 100 - S2 - OVA (1080p) (HDR) [Dual Audio].mkv", &["episodes"]);
}

/// should keep marked episode over title number before episode name
#[test]
fn should_keep_marked_episode_over_title_number_before_episode_name() {
    assert_includes(
        "[INVERTI] Cyborg 009 - The Cyborg Soldier EP22 (480p) [DUAL AUDIO].mkv",
        json!({ "title": "Cyborg 009 - The Cyborg Soldier", "episode": 22 }),
    );
}

/// should keep trailing anime episode over title number
#[test]
fn should_keep_trailing_anime_episode_over_title_number() {
    assert_includes(
        "[EMBER] Zom 100 - Zombie ni Naru made ni Shitai 100 no Koto - 04.mkv",
        json!({ "title": "Zom 100 - Zombie ni Naru made ni Shitai 100 no Koto", "episode": 4 }),
    );
}

/// should detect anime episode with hyphen number in title
#[test]
fn should_detect_anime_episode_with_hyphen_number_in_title() {
    assert_includes("[SubsPlease] Fairy Tail - 100 Years Quest - 05 (1080p) [1107F3A9].mkv", json!({ "episode": 5 }));
}

/// should detect anime episode recap episode
#[test]
fn should_detect_anime_episode_recap_episode() {
    assert_includes("[KH] Sword Art Online II - 14.5 - Debriefing.mkv", json!({ "episode": 14 }));
}

/// should detect four digit anime episode
#[test]
fn should_detect_four_digit_anime_episode() {
    assert_includes("[SSA] Detective Conan - 1001 [720p].mkv", json!({ "episode": 1001 }));
}

/// should detect season_episode pattern
#[test]
fn should_detect_season_episode_pattern() {
    assert_includes("Pwer-04_05.avi", json!({ "season": 4, "episode": 5 }));
}

/// should detect season_episode pattern v2
#[test]
fn should_detect_season_episode_pattern_v2() {
    assert_includes("SupNat-11_06.avi", json!({ "season": 11, "episode": 6 }));
}

/// should detect season_episode pattern v3
#[test]
fn should_detect_season_episode_pattern_v3() {
    assert_includes("office_03_19.avi", json!({ "season": 3, "episode": 19 }));
}

/// should detect season_episode pattern with years in title
#[test]
fn should_detect_season_episode_pattern_with_years_in_title() {
    assert_includes("Spergrl-2016-02_04.avi", json!({ "season": 2, "episode": 4 }));
}

/// should detect final with dash season_episode pattern with years in title
#[test]
fn should_detect_final_with_dash_season_episode_pattern_with_years_in_title() {
    assert_includes("Iron-Fist-2017-01_13-F.avi", json!({ "season": 1, "episode": 13 }));
}

/// should detect final with dot season_episode pattern with years in title
#[test]
fn should_detect_final_with_dot_season_episode_pattern_with_years_in_title() {
    assert_includes("Lgds.of.Tmrow-02_17.F.avi", json!({ "season": 2, "episode": 17 }));
}

/// should detect season.episode pattern
#[test]
fn should_detect_season_episode_pattern_2() {
    assert_includes("Ozk.02.09.avi", json!({ "season": 2, "episode": 9 }));
}

/// should detect final season.episode pattern
#[test]
fn should_detect_final_season_episode_pattern() {
    assert_includes("Ozk.02.10.F.avi", json!({ "season": 2, "episode": 10 }));
}

/// should detect not detect season_episode pattern when other pattern present
#[test]
fn should_detect_not_detect_season_episode_pattern_when_other_pattern_present() {
    assert_includes("Cestovatelé_S02E04_11_27.mkv", json!({ "season": 2, "episode": 4 }));
}

/// should detect not detect season_episode pattern when it's additional info
#[test]
fn should_detect_not_detect_season_episode_pattern_when_it_s_additional_info() {
    assert_includes("S03E13_91.avi", json!({ "season": 3, "episode": 13 }));
}

/// should detect not detect season.episode pattern (not working yet)
#[test]
fn should_detect_not_detect_season_episode_pattern_not_working_yet() {
    assert_includes("wwe.nxt.uk.11.26.mkv", json!({ "season": 11, "episode": 26 }));
}

/// should detect not detect season.episode pattern when other pattern present
#[test]
fn should_detect_not_detect_season_episode_pattern_when_other_pattern_present_2() {
    assert_includes("Chernobyl.S01E01.1.23.45.mkv", json!({ "season": 1, "episode": 1 }));
}

/// should detect season.episode pattern with S identifier
#[test]
fn should_detect_season_episode_pattern_with_s_identifier() {
    assert_includes("The.Witcher.S01.07.mp4", json!({ "season": 1, "episode": 7 }));
}

/// should detect season episode pattern with S identifier
#[test]
fn should_detect_season_episode_pattern_with_s_identifier_2() {
    assert_includes("Breaking Bad S02 03.mkv", json!({ "season": 2, "episode": 3 }));
}

/// should detect season episode pattern with Season prefix
#[test]
fn should_detect_season_episode_pattern_with_season_prefix() {
    assert_includes("NCIS Season 11 01.mp4", json!({ "season": 11, "episode": 1 }));
}

/// should detect not detect season.episode pattern when it's a date
#[test]
fn should_detect_not_detect_season_episode_pattern_when_it_s_a_date() {
    assert_includes("Top Gear - 3x05 - 2003.11.23.avi", json!({ "season": 3, "episode": 5 }));
}

/// should detect episode in brackets
#[test]
fn should_detect_episode_in_brackets() {
    assert_includes("[KTKJ]_[BLEACH]_[DVDRIP]_[116]_[x264_640x480_aac].mkv", json!({ "episode": 116 }));
}

/// should detect episode in brackets but not years
#[test]
fn should_detect_episode_in_brackets_but_not_years() {
    assert_includes("[GM-Team][国漫][绝代双骄][Legendary Twins][2022][08][HEVC][GB][4K].mp4", json!({ "episode": 8 }));
}

/// should not detect season-episode pattern when with dot split
#[test]
fn should_not_detect_season_episode_pattern_when_with_dot_split() {
    assert_includes("SG-1. Season 4.16. (2010).avi", json!({ "season": 4, "episode": 16 }));
}

/// should not detect polish episode
#[test]
fn should_not_detect_polish_episode() {
    assert_includes("Reset- odc. 10.mp4", json!({ "episode": 10 }));
}

/// should not detect season-episode pattern when it's a date
#[test]
#[ignore = "pending in the js suite"]
fn should_not_detect_season_episode_pattern_when_it_s_a_date() {
    assert_includes("8-6 2006.07.16.avi", json!({ "season": 8, "episode": 6 }));
}

/// should not detect season episode pattern but absolute episode
#[test]
fn should_not_detect_season_episode_pattern_but_absolute_episode() {
    assert_missing("523 23.mp4", &["season"]);
    assert_includes("523 23.mp4", json!({ "episode": 523 }));
}

/// should detect only episode
#[test]
fn should_detect_only_episode() {
    assert_missing("Chernobyl E02 1 23 45.mp4", &["season"]);
    assert_includes("Chernobyl E02 1 23 45.mp4", json!({ "episode": 2 }));
}

/// should detect regular episode with year range before
#[test]
fn should_detect_regular_episode_with_year_range_before() {
    assert_includes("Lucky.Luke.1983-1992.S01E04.PL.720p.WEB-DL.H264-zyl.mkv", json!({ "episodes": [4] }));
}

/// should detect season dot episode format
#[test]
fn should_detect_season_dot_episode_format() {
    assert_includes("11.14 - The List.mp4", json!({ "seasons": [11], "episodes": [14] }));
}

/// should detect only episode v2
#[test]
fn should_detect_only_episode_v2() {
    assert_missing("Watch Gary And His Demons Episode 10 - 0.00.07-0.11.02.mp4", &["season"]);
    assert_includes("Watch Gary And His Demons Episode 10 - 0.00.07-0.11.02.mp4", json!({ "episode": 10 }));
}

/// should detect only episode v3
#[test]
fn should_detect_only_episode_v3() {
    assert_missing("523 23.mp4", &["season"]);
    assert_includes("523 23.mp4", json!({ "episode": 523 }));
}

/// should not detect season.episode pattern when it's a date without other pattern
#[test]
fn should_not_detect_season_episode_pattern_when_it_s_a_date_without_other_pattern() {
    assert_missing("wwf.raw.is.war.18.09.00.avi", &["seasons"]);
    assert_missing("wwf.raw.is.war.18.09.00.avi", &["episode"]);
}

/// should not detect episodes when it's 2.0 sound
#[test]
fn should_not_detect_episodes_when_it_s_2_0_sound() {
    assert_missing("The Rat Race (1960) [1080p] [WEB-DL] [x264] [DD] [2-0] [DReaM] [LEKTOR PL]", &["seasons"]);
    assert_missing("The Rat Race (1960) [1080p] [WEB-DL] [x264] [DD] [2-0] [DReaM] [LEKTOR PL]", &["episodes"]);
}

/// should not detect episodes when it's 2.0 sound v2
#[test]
fn should_not_detect_episodes_when_it_s_2_0_sound_v2() {
    assert_missing("Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [2.0] [Lektor PL]", &["seasons"]);
    assert_missing("Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [2.0] [Lektor PL]", &["episodes"]);
}

/// should not detect episodes when it's 5.1 sound
#[test]
fn should_not_detect_episodes_when_it_s_5_1_sound() {
    assert_missing("A Quiet Place: Day One (2024) [1080p] [WEB-DL] [x264] [AC3] [DD] [5-1] [LEKTOR PL]", &["seasons"]);
    assert_missing("A Quiet Place: Day One (2024) [1080p] [WEB-DL] [x264] [AC3] [DD] [5-1] [LEKTOR PL]", &["episodes"]);
}

/// should not detect episodes when it's 5.1 sound v2
#[test]
fn should_not_detect_episodes_when_it_s_5_1_sound_v2() {
    assert_missing("Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [5.1] [Lektor PL]", &["seasons"]);
    assert_missing("Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [5.1] [Lektor PL]", &["episodes"]);
}

/// should not detect episodes when it's 7.1 sound
#[test]
fn should_not_detect_episodes_when_it_s_7_1_sound() {
    assert_missing("Frequency (2000) [1080p] [BluRay] [REMUX] [AVC] [DTS] [HD] [MA] [7-1] [MR]", &["seasons"]);
    assert_missing("Frequency (2000) [1080p] [BluRay] [REMUX] [AVC] [DTS] [HD] [MA] [7-1] [MR]", &["episodes"]);
}

/// should not detect episodes when it's 7.1 sound v2
#[test]
fn should_not_detect_episodes_when_it_s_7_1_sound_v2() {
    assert_missing("Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [7.1] [Lektor PL]", &["seasons"]);
    assert_missing("Avatar 2009 [1080p.BDRip.x264.AC3-azjatycki] [7.1] [Lektor PL]", &["episodes"]);
}

/// should detect four digit episode with E prefix instead of year
#[test]
fn should_detect_four_digit_episode_with_e_prefix_instead_of_year() {
    assert_includes("M.jak.Milosc.E1944.1080p.WEB.DL.x264.PL.GhN.mp4", json!({ "episode": 1944 }));
    assert_missing("M.jak.Milosc.E1944.1080p.WEB.DL.x264.PL.GhN.mp4", &["year"]);
}

/// should detect four digit episode with ep prefix instead of year
#[test]
fn should_detect_four_digit_episode_with_ep_prefix_instead_of_year() {
    assert_includes("Santa_Barbara_ep_1900_vhsrip_rus_xvid.avi", json!({ "episode": 1900 }));
    assert_missing("Santa_Barbara_ep_1900_vhsrip_rus_xvid.avi", &["year"]);
}

/// should detect season and episode with dot separator before extension
#[test]
fn should_detect_season_and_episode_with_dot_separator_before_extension() {
    assert_includes("Mangust.2.01.avi", json!({ "season": 2, "episode": 1 }));
}

/// should detect season and three digit episode with dot separator before extension
#[test]
fn should_detect_season_and_three_digit_episode_with_dot_separator_before_extension() {
    assert_includes("Glukhar-3.036.avi", json!({ "season": 3, "episode": 36 }));
}

/// should not detect runtime suffix as season and episode
#[test]
fn should_not_detect_runtime_suffix_as_season_and_episode() {
    assert_missing("sokol'nichiy_tomas_dvdrip_1.46.avi", &["season"]);
    assert_missing("sokol'nichiy_tomas_dvdrip_1.46.avi", &["episodes"]);
}

/// should not detect season dot episode in the middle of a time
#[test]
fn should_not_detect_season_dot_episode_in_the_middle_of_a_time() {
    assert_includes("Elements of Chernobyl Episode 1 1.23.45.mkv", json!({ "episode": 1 }));
    assert_missing("Elements of Chernobyl Episode 1 1.23.45.mkv", &["season"]);
}

/// should detect season and episode with dot separator at the start
#[test]
fn should_detect_season_and_episode_with_dot_separator_at_the_start() {
    assert_includes("3.001.Кизиловый щербет.WEBRip.1080p.RG.Russkie.serialy.mkv", json!({ "season": 3, "episode": 1 }));
}

/// should detect season and episode with dot separator at the start followed by title
#[test]
fn should_detect_season_and_episode_with_dot_separator_at_the_start_followed_by_title() {
    assert_includes("1.01 Shattered Vows.avi", json!({ "season": 1, "episode": 1 }));
}

/// should not detect number in parentheses with a suffix as episode
#[test]
fn should_not_detect_number_in_parentheses_with_a_suffix_as_episode() {
    assert_missing(
        "[Asakura] Kaiko Sareta Ankoku Heishi (30-dai) no Slow na Second Life 01 [BDRip 1920x1080 x265 10bit FLAC].mkv",
        &["episodes"],
    );
}

/// should still detect episode after title with number in parentheses with a dash
#[test]
fn should_still_detect_episode_after_title_with_number_in_parentheses_with_a_dash() {
    assert_includes(
        "[ASW] Kaiko sareta Ankoku Heishi (30-dai) no Slow na Second Life - 01 [1080p HEVC][8FAB46AC].mkv",
        json!({ "episode": 1 }),
    );
}

/// should detect transliterated russian episode
#[test]
fn should_detect_transliterated_russian_episode() {
    assert_includes(
        "Shejh.Badijar.Istorija.ljubvi.i.predatelstva.1978.(4-ja.serija.iz.4).XviD.TVRip.avi",
        json!({ "episode": 4 }),
    );
}

/// should detect serbian episode
#[test]
fn should_detect_serbian_episode() {
    assert_includes("Lud, Zbunjen, Normalan (2014) Epizoda 170.flv", json!({ "episode": 170 }));
}

/// should not expand comma separated decimal into a range
#[test]
fn should_not_expand_comma_separated_decimal_into_a_range() {
    assert_missing("Yuragi-Sou No Yuuna-San - 03,5 OVA1 uncen  [1080p].mkv", &["episodes"]);
}

/// should not expand comma separated duration into a range
#[test]
fn should_not_expand_comma_separated_duration_into_a_range() {
    assert_missing("Следствие ведут знатоки фильм 1 Черный маклер 1,46.avi", &["episodes"]);
}

/// should still detect comma separated consecutive episodes
#[test]
fn should_still_detect_comma_separated_consecutive_episodes() {
    assert_includes("LVDB 1,2.avi", json!({ "episodes": [1, 2] }));
}

/// should not detect episode range when end is a decimal number
#[test]
fn should_not_detect_episode_range_when_end_is_a_decimal_number() {
    assert_includes("Astrid.et.Raphaelle-S04E03-10.000.metres.mkv", json!({ "season": 4, "episodes": [3] }));
}

/// should not detect episode range from time range
#[test]
fn should_not_detect_episode_range_from_time_range() {
    assert_includes(
        "The.Pitt.1x15.Quindicesima.Ora.21.00-22.00.ITA.ENG.1080p.DLMux.AAC.x265-Pir8.mkv",
        json!({ "season": 1, "episodes": [15] }),
    );
}

/// should detect episode with year numbered season
#[test]
fn should_detect_episode_with_year_numbered_season() {
    assert_includes(
        "Looney Tunes - S1940E24 - A Wild Hare.mkv",
        json!({ "title": "Looney Tunes", "season": 1940, "episode": 24 }),
    );
    assert_missing("Looney Tunes - S1940E24 - A Wild Hare.mkv", &["year"]);
}

/// should detect episode with year numbered season and separate year
#[test]
fn should_detect_episode_with_year_numbered_season_and_separate_year() {
    assert_includes(
        "Tom and Jerry S1947E27 - Cat Fishin (1947) [1080p BluRay REMUX AVC FLAC 1.0].mkv",
        json!({ "season": 1947, "episode": 27, "year": 1947 }),
    );
}

/// should detect episode with s.N ep.N notation
#[test]
fn should_detect_episode_with_s_n_ep_n_notation() {
    assert_includes(
        "[apreder]Les_Pays_d'en_Haut_s.4_ep.01(2019)DVB.mkv",
        json!({ "title": "Les Pays d'en Haut", "season": 4, "episode": 1 }),
    );
}

/// should detect episode with three digit s.N ep.N notation
#[test]
fn should_detect_episode_with_three_digit_s_n_ep_n_notation() {
    assert_includes("[apreder]Un_si_grand_soleil_s.8_ep.001(2025)DVB.mkv", json!({ "season": 8, "episode": 1 }));
}

/// should detect episode before bracketed resolution
#[test]
fn should_detect_episode_before_bracketed_resolution() {
    assert_includes(
        "Kamen Rider Black - 24 [GSG.Persona99][480].rus.jpn.mp4",
        json!({ "episode": 24, "resolution": "480p" }),
    );
}

/// should detect absolute episode before resolution in parentheses
#[test]
fn should_detect_absolute_episode_before_resolution_in_parentheses() {
    assert_includes("37 Yali Capkini  (1080).mp4", json!({ "episode": 37, "resolution": "1080p" }));
}

/// should still detect year glued to episode
#[test]
fn should_still_detect_year_glued_to_episode() {
    assert_includes("Insiders.2023E43.Sarah.Hanson-Young.Greens.Senator.720p", json!({ "year": 2023, "episode": 43 }));
}

/// should not detect episode from year numbered season without episode
#[test]
fn should_not_detect_episode_from_year_numbered_season_without_episode() {
    assert_missing("MythBusters S2009 Complete 720p WEBRips x264 [i c]", &["episodes"]);
}

/// should detect absolute episode at the beginning followed by title
#[test]
fn should_detect_absolute_episode_at_the_beginning_followed_by_title() {
    assert_includes("1125 - One piece [Sub] 1080p", json!({ "episode": 1125 }));
}

/// should detect episode right after the year
#[test]
fn should_detect_episode_right_after_the_year() {
    assert_includes("[NoobSubs] Berserk 2016 12 (720p Blu-ray 8bit AAC).mp4", json!({ "year": 2016, "episode": 12 }));
}

/// should detect concatenated episode right after the year
#[test]
fn should_detect_concatenated_episode_right_after_the_year() {
    assert_includes("the.flash.2014.113.hdtv-lol.mp4", json!({ "year": 2014, "episode": 113 }));
}

/// should detect episode right after the year in parentheses
#[test]
fn should_detect_episode_right_after_the_year_in_parentheses() {
    assert_includes(
        "[Some-Stuffs]_Pocket_Monsters_(2019)_088_(1080p)_[0AA4D76E].mkv",
        json!({ "year": 2019, "episode": 88 }),
    );
}

/// should detect episode right after the year followed by episode title
#[test]
fn should_detect_episode_right_after_the_year_followed_by_episode_title() {
    assert_includes("Sherlock.Holmes.(1954).11.The Red Headed League.mp4", json!({ "year": 1954, "episode": 11 }));
}

/// should detect dash wrapped episode right after the year
#[test]
fn should_detect_dash_wrapped_episode_right_after_the_year() {
    assert_includes(
        "Flash Gordon 1996 -24- The Wrath of Ming [Sub+Eng+Fre].mkv",
        json!({ "year": 1996, "episode": 24 }),
    );
}

/// should detect concatenated episode right after the resolution
#[test]
fn should_detect_concatenated_episode_right_after_the_resolution() {
    assert_includes("WhenCallsTheHeart720p_204_WWW.NEWPCT1.COM.mkv", json!({ "resolution": "720p", "episode": 204 }));
}

/// should detect series word episode
#[test]
fn should_detect_series_word_episode() {
    assert_includes(
        "My Family, Series 1, 08 Much Ado About Ben 720p WEB-DL HEVC x265 BONE.mkv",
        json!({ "season": 1, "episode": 8 }),
    );
}

/// should detect series word episode with dot separator
#[test]
fn should_detect_series_word_episode_with_dot_separator() {
    assert_includes("Timmy Time Series 2.01 Timmy Learns Magic.mkv", json!({ "season": 2, "episode": 1 }));
}

/// should detect polish episode word
#[test]
fn should_detect_polish_episode_word() {
    assert_includes("Simpsonowie - The Simpsons - Sezon 35 Odcinek 11 .mp4", json!({ "season": 35, "episode": 11 }));
}

/// should detect swedish episode word
#[test]
fn should_detect_swedish_episode_word() {
    assert_includes("Avsnitt 02 - Dishonor.mkv", json!({ "episode": 2 }));
}

/// should detect dutch episode word
#[test]
fn should_detect_dutch_episode_word() {
    assert_includes("'t Schaep met de 5 pooten aflevering 4.mp4", json!({ "episode": 4 }));
}

/// should detect german episode word
#[test]
fn should_detect_german_episode_word() {
    assert_includes(
        "Super Dragonball Heroes (Deutsch) [Folge 21] (720p_30fps_H264-192kbit_AAC).mp4",
        json!({ "episode": 21 }),
    );
}

/// should detect finnish episode word
#[test]
fn should_detect_finnish_episode_word() {
    assert_includes("Silkkitie.30.päivässä.-.Jakso-04.flv", json!({ "episode": 4 }));
}

/// should detect italian episode word
#[test]
fn should_detect_italian_episode_word() {
    assert_includes("Catch 22 - Puntata 4 (2019).HDTV.720p.H264.italian.Ac3-2.0-BaMax71.mkv", json!({ "episode": 4 }));
}

/// should not detect channels after the year as episode
#[test]
fn should_not_detect_channels_after_the_year_as_episode() {
    assert_missing("Godzilla.vs.Kong.2021.1080p.10bit.WEBRip.6CH.x265.HEVC-PSA.mkv", &["episodes"]);
}

/// should not detect audio after the year as episode
#[test]
fn should_not_detect_audio_after_the_year_as_episode() {
    assert_missing("Nobody (2021) 1080p 5.1 - 2.0 x264 Phun Psyz.mp4", &["episodes"]);
}

/// should not detect cd number after the year as episode
#[test]
fn should_not_detect_cd_number_after_the_year_as_episode() {
    assert_missing("Jism 2 (2012) 1 CD DVD Rip XivD MP3 E-SUB Team DST", &["episodes"]);
}

/// should not detect size after the year as episode
#[test]
fn should_not_detect_size_after_the_year_as_episode() {
    assert_missing("Mission Impossible 5 Rogue Nation 2015 700 MB", &["episodes"]);
}

/// should not detect date after the year as episode
#[test]
fn should_not_detect_date_after_the_year_as_episode() {
    assert_missing("The Bold and the Beautiful 2015 2 27", &["episodes"]);
}

/// should detect episode range with tilde starting from zero
#[test]
fn should_detect_episode_range_with_tilde_starting_from_zero() {
    assert_includes(
        "[Erai-raws] Boku no Hero Academia S2 - 00~25 [1080p][Multiple Subtitle]",
        json!({ "episodes": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25] }),
    );
}

/// should detect episode range in brackets with ep prefix
#[test]
fn should_detect_episode_range_in_brackets_with_ep_prefix() {
    assert_includes(
        "Naruto Complete [Ep 01 - 220][English][480p]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220] }),
    );
}

/// should detect season and episode without s prefix
#[test]
fn should_detect_season_and_episode_without_s_prefix() {
    assert_includes("Vikkatakavi 01E06.mkv", json!({ "season": 1, "episode": 6 }));
}

/// should detect season and episode with series and ep prefixes
#[test]
fn should_detect_season_and_episode_with_series_and_ep_prefixes() {
    assert_includes("Steptoe And Son Series 05EP01 A Death In The Family.mp4", json!({ "season": 5, "episode": 1 }));
}

/// should detect ova episode with underscores
#[test]
fn should_detect_ova_episode_with_underscores() {
    assert_includes(
        "[Cleo]Monster_Musume_no_Iru_Nichijou_-_OVA_01_(Dual Audio_10bit_BD720p_x265).mkv",
        json!({ "episode": 1 }),
    );
}

/// should detect russian episode count of total as range
#[test]
fn should_detect_russian_episode_count_of_total_as_range() {
    assert_includes(
        "Викинги / Vikings / Сезон: 5 / Серии: 5 из 20 [2017, WEB-DL 1080p] MVO",
        json!({ "episodes": [1, 2, 3, 4, 5] }),
    );
}

/// should detect russian full episode count in brackets as range
#[test]
fn should_detect_russian_full_episode_count_in_brackets_as_range() {
    assert_includes(
        "Клинок, рассекающий демонов (ТВ-1) / Kimetsu no Yaiba / Demon Slayer [TV] [26 из 26] [RUS(ext), ENG, JAP+Sub]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26] }),
    );
}

/// should detect full episode count with slash before details block as range
#[test]
fn should_detect_full_episode_count_with_slash_before_details_block_as_range() {
    assert_includes(
        "Наш учебник / Watashitachi no Kyokasho (Kawake Shunsaku, Hayama Hiroki) 12/12 [Япония, 2007 г., детектив, школа, TVRip]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }),
    );
}

/// should detect full episode count with slash before multiple bracket blocks as range
#[test]
fn should_detect_full_episode_count_with_slash_before_multiple_bracket_blocks_as_range() {
    assert_includes(
        "Якша / Яша / Yasha (Sato Shimako) 11/11 [Япония, 2000 г., Фантастика, драма, DVDRip][RAW][JAP+Sub Rus]",
        json!({ "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] }),
    );
}

/// should not detect episodes from different slash numbers
#[test]
fn should_not_detect_episodes_from_different_slash_numbers() {
    assert_missing("Fahrenheit 9/11 [2004, BDRip]", &["episodes"]);
}

/// should not detect episodes from same slash numbers without details block
#[test]
fn should_not_detect_episodes_from_same_slash_numbers_without_details_block() {
    assert_missing("Some Show 7/7 2021 1080p WEB-DL", &["episodes"]);
}

/// should detect turkish bolum episode
#[test]
fn should_detect_turkish_bolum_episode() {
    assert_includes("Kaderimin Yazıldığı Gün 26. Bölüm.mkv", json!({ "episode": 26 }));
}

/// should not detect season number before russian season word as episode
#[test]
fn should_not_detect_season_number_before_russian_season_word_as_episode() {
    assert_includes(
        "01 - Дело о золотом таланте - Приключения Пети и Волка - 4 Сезон - D2T.mkv",
        json!({ "season": 4 }),
    );
    assert_missing("01 - Дело о золотом таланте - Приключения Пети и Волка - 4 Сезон - D2T.mkv", &["episodes"]);
}

/// should detect transliterated russian serija episode
#[test]
fn should_detect_transliterated_russian_serija_episode() {
    assert_includes("01. Griffiny - Sezon 21 - Serija 1 - Oskary.mkv", json!({ "season": 21, "episode": 1 }));
}

/// should not detect series number as part of episode list
#[test]
fn should_not_detect_series_number_as_part_of_episode_list() {
    assert_includes(
        "My Family, Series 1, 02 Pain In The Class 720p WEB-DL HEVC x265 BONE.mkv",
        json!({ "season": 1, "episodes": [2] }),
    );
}

/// should not detect episode from bracketed audio channels
#[test]
fn should_not_detect_episode_from_bracketed_audio_channels() {
    assert_missing("Deadpool (2016) [2160p] [7.1 AAC ENG] [5.1 AAC ENG FRE GER ITA SPA] [COMMENTARY]", &["episodes"]);
}

/// should detect episode range with thru
#[test]
fn should_detect_episode_range_with_thru() {
    assert_includes("DareDevil - Seasons 2 - Episodes 10 thru 13", json!({ "episodes": [10, 11, 12, 13] }));
}

/// should detect episode from bracketed x of y
#[test]
fn should_detect_episode_from_bracketed_x_of_y() {
    assert_includes("BBC Planet Earth [3 of 11] Caves.avi", json!({ "title": "BBC Planet Earth", "episodes": [3] }));
}

/// should not detect episode from disc numbering in multi season pack
#[test]
fn should_not_detect_episode_from_disc_numbering_in_multi_season_pack() {
    assert_missing("Forensic Files Complete Series Seasons 01-06 [1 of 4]", &["episodes"]);
}

/// should not detect episode from disc numbering after season range
#[test]
fn should_not_detect_episode_from_disc_numbering_after_season_range() {
    assert_missing("Forensic Files Seasons 01-06 [1 of 4]", &["episodes"]);
    assert_missing("Forensic Files S01-S06 [1 of 4]", &["episodes"]);
}

/// should detect season and three digit episode at the start of a file name
#[test]
fn should_detect_season_and_three_digit_episode_at_the_start_of_a_file_name() {
    assert_includes(
        "3-104 Ралли в Клубе Микки Мауса часть 2 (Road Rally Part 2).avi",
        json!({ "seasons": [3], "episodes": [104] }),
    );
}

/// should not detect season and episode from a movie title starting with numbers
#[test]
fn should_not_detect_season_and_episode_from_a_movie_title_starting_with_numbers() {
    assert_includes("3-10 to Yuma (2007) 1080p BluRay x264.mkv", json!({ "title": "3-10 to Yuma", "year": 2007 }));
    assert_missing("3-10 to Yuma (2007) 1080p BluRay x264.mkv", &["episodes"]);
    assert_missing("3-10.to.Yuma.2007.1080p.BluRay.x264.mkv", &["episodes"]);
}

/// should detect episode before cyrillic resolution marker
#[test]
fn should_detect_episode_before_cyrillic_resolution_marker() {
    assert_includes(
        "«Эрнест и Селестина» 1 сезон 05 серия 720р.mkv",
        json!({ "seasons": [1], "episodes": [5], "resolution": "720p" }),
    );
}

/// should detect both episodes of a descending double episode code
#[test]
fn should_detect_both_episodes_of_a_descending_double_episode_code() {
    assert_includes(
        "Its.Pony.S01E31E22.Annie-Versary.Teachers.Pet.1080p.AMZN.WEB-DL.DDP5.1.H.264.mkv",
        json!({ "seasons": [1], "episodes": [22, 31] }),
    );
    assert_includes("Show.S01E01E02.1080p.mkv", json!({ "episodes": [1, 2] }));
}

/// should detect zero padded episode glued to the name of a file
#[test]
fn should_detect_zero_padded_episode_glued_to_the_name_of_a_file() {
    assert_includes("esperanca001.avi", json!({ "title": "esperanca", "episodes": [1] }));
    assert_includes("GodHandTeru05.avi", json!({ "title": "GodHandTeru", "episodes": [5] }));
}

/// should not detect episode from a number glued to a movie title
#[test]
fn should_not_detect_episode_from_a_number_glued_to_a_movie_title() {
    assert_missing("Apollo13.mkv", &["episodes"]);
}

/// should not detect episode from a number glued to a movie title v2
#[test]
fn should_not_detect_episode_from_a_number_glued_to_a_movie_title_v2() {
    assert_missing("Terminator2.avi", &["episodes"]);
}

/// should not detect episode from a number glued to a movie title v3
#[test]
fn should_not_detect_episode_from_a_number_glued_to_a_movie_title_v3() {
    assert_missing("Agent47.mkv", &["episodes"]);
}

/// should detect episode with a number in a glued title
#[test]
fn should_detect_episode_with_a_number_in_a_glued_title() {
    assert_includes("Numb3rs.S01E01.mkv", json!({ "title": "Numb3rs", "seasons": [1], "episodes": [1] }));
}

/// should detect episode glued to ep suffix
#[test]
fn should_detect_episode_glued_to_ep_suffix() {
    assert_includes("AniPlay_Kumiho_6ep_dub.mp4", json!({ "episodes": [6] }));
    assert_includes("AniPlay_Kumiho_10ep_dub.mp4", json!({ "episodes": [10] }));
}

/// should detect season and episode from volume episode naming
#[test]
fn should_detect_season_and_episode_from_volume_episode_naming() {
    assert_includes("v2e09. Morning, Noon and Night.rus.eng.avi", json!({ "seasons": [2], "episodes": [9] }));
    assert_includes("V01E14 - Heart Of Ice.mkv", json!({ "seasons": [1], "episodes": [14] }));
}

/// should detect season and episode before transliterated series word
#[test]
fn should_detect_season_and_episode_before_transliterated_series_word() {
    assert_includes(
        "Morskje.Dyvolj.3.01.serij.iz.16.2009.DivX.DVDRip.Kinozal.TV.avi",
        json!({ "seasons": [3], "episodes": [1] }),
    );
    assert_includes("Vsegda.govori.vsegda.6.1-3.ser.iz.8.avi", json!({ "seasons": [6], "episodes": [1, 2, 3] }));
    assert_includes("Vsegda.govori.vsegda.5.02.ser.iz.10.avi", json!({ "seasons": [5], "episodes": [2] }));
}

/// should not detect season from a single number before transliterated series word
#[test]
fn should_not_detect_season_from_a_single_number_before_transliterated_series_word() {
    assert_missing("Brigada.01.seriya.avi", &["seasons"]);
    assert_missing("Ulica.razbityh.fonarej.2.serija.avi", &["seasons"]);
}

/// should detect season and episode from x prefixed code at the start
#[test]
fn should_detect_season_and_episode_from_x_prefixed_code_at_the_start() {
    assert_includes("x1001 - The Return Of Chef.avi", json!({ "seasons": [10], "episodes": [1] }));
    assert_includes("x302 - Some Title.avi", json!({ "seasons": [3], "episodes": [2] }));
}

/// should not detect season and episode from codec or resolution with x prefix
#[test]
fn should_not_detect_season_and_episode_from_codec_or_resolution_with_x_prefix() {
    assert_missing("x264 - Some Movie 2010.mkv", &["episodes"]);
    assert_missing("x1080 - Some Movie 2010.mkv", &["episodes"]);
}

/// should not detect episode range from the middle of a hyphenated number chain
#[test]
fn should_not_detect_episode_range_from_the_middle_of_a_hyphenated_number_chain() {
    assert_missing("90-60-90.modelos_001.avi", &["episodes"]);
}

/// should detect season prefixed three digit episode code at the start of a file name
#[test]
fn should_detect_season_prefixed_three_digit_episode_code_at_the_start_of_a_file_name() {
    assert_includes("3-307 The Last Shout (1).mp4", json!({ "seasons": [3], "episodes": [7] }));
    assert_includes(
        "2-208 Documentary - How To Be Absolutely Fabulous.mp4",
        json!({ "seasons": [2], "episodes": [8] }),
    );
}

/// should detect episode range instead of season from two digit start with three digit end
#[test]
fn should_detect_episode_range_instead_of_season_from_two_digit_start_with_three_digit_end() {
    assert_includes("96-100.mp4", json!({ "episodes": [96, 97, 98, 99, 100] }));
    assert_missing("96-100.mp4", &["seasons"]);
    assert_includes("99-100 Douluo Dalu.mkv", json!({ "episodes": [99, 100] }));
    assert_missing("99-100 Douluo Dalu.mkv", &["seasons"]);
}

/// should not detect season and episode from a phone number at the start
#[test]
fn should_not_detect_season_and_episode_from_a_phone_number_at_the_start() {
    assert_missing("Extras/1-900 Hotline", &["seasons"]);
    assert_missing("1-800 Hotline.mkv", &["seasons"]);
}

/// should not detect episode from a part number glued to the name
#[test]
fn should_not_detect_episode_from_a_part_number_glued_to_the_name() {
    assert_missing("SupercutALTverWithRecaps720pPart01.mkv", &["episodes"]);
    assert_missing("dvd01.avi", &["episodes"]);
}

/// should not detect episode from a season number glued to ep before the episode number
#[test]
fn should_not_detect_episode_from_a_season_number_glued_to_ep_before_the_episode_number() {
    assert_field_ne("Istoriya.Zatoichi.s.1ep.03.1974.DVDRip.avi", "episodes", json!([1]));
}

/// should not detect episode range from a hyphenated name at the start
#[test]
fn should_not_detect_episode_range_from_a_hyphenated_name_at_the_start() {
    assert_missing("1-900-Beavis.avi", &["episodes"]);
}

/// should detect dot separated episode pair after season before transliterated season word
#[test]
fn should_detect_dot_separated_episode_pair_after_season_before_transliterated_season_word() {
    assert_includes(
        "Doktor.Kuinn.Zhenshina.vrach.(2.sezon.26.27.serija.iz.27).1994.DivX.DVDRip.avi",
        json!({ "seasons": [2], "episodes": [26, 27] }),
    );
    assert_includes("Sekretnye.materialy.sezon.1.24.seriya.iz.24.avi", json!({ "seasons": [1], "episodes": [24] }));
}

/// should detect episode range before russian series word
#[test]
fn should_detect_episode_range_before_russian_series_word() {
    assert_includes(
        "Черепашки ниндзя 3 сезон 12-13 серия[Saint Sound].mkv",
        json!({ "seasons": [3], "episodes": [12, 13] }),
    );
}

/// should detect episode after year followed by dot and space
#[test]
fn should_detect_episode_after_year_followed_by_dot_and_space() {
    assert_includes("STEPonee Хороший доктор Good Doctor 2013. 5 [озвучка ]_xvid.avi", json!({ "episodes": [5] }));
    assert_includes("STEPonee Хороший доктор Good Doctor 2013. 19 [озвучка ].avi", json!({ "episodes": [19] }));
    assert_missing("Istoriya.Zatoichi.s.1ep.03.1974.DVDRip_4_prob4 .mkv", &["episodes"]);
}

/// should detect zero padded episode glued to the name after a release group
#[test]
fn should_detect_zero_padded_episode_glued_to_the_name_after_a_release_group() {
    assert_includes("[alliance]Queens01.avi", json!({ "episodes": [1] }));
    assert_includes("[clubfate]sandglass02.avi", json!({ "episodes": [2] }));
    assert_missing("[group]Apollo13.mkv", &["episodes"]);
}

/// should detect zero padded season and episode at the start before the year
#[test]
fn should_detect_zero_padded_season_and_episode_at_the_start_before_the_year() {
    assert_includes("2-07.Быть человеком.2010.WEB-DL1080p.GeneralFilm.mkv", json!({ "season": 2, "episodes": [7] }));
}

/// should detect zero padded episode range at the start before the year
#[test]
fn should_detect_zero_padded_episode_range_at_the_start_before_the_year() {
    assert_includes("03-04.Slomannyje.sudby.2013.WEB-DLRip.Files-x.avi", json!({ "episodes": [3, 4] }));
}

/// should not detect an episode in a dotted single letter abbreviation
#[test]
fn should_not_detect_an_episode_in_a_dotted_single_letter_abbreviation() {
    assert_includes("[Fansubber] [2006] C.E. 73 Stargazer --- sub[EN,ES,IT]", json!({ "title": "C.E. 73 Stargazer" }));
    assert_missing("[Fansubber] [2006] C.E. 73 Stargazer --- sub[EN,ES,IT]", &["episodes"]);
}

/// should not detect an episode in an undotted single letter abbreviation
#[test]
fn should_not_detect_an_episode_in_an_undotted_single_letter_abbreviation() {
    assert_missing("[SnOF + Nks] SEED C.E.73 STARGAZER  - SEED ASTRAY", &["episodes"]);
}

/// should detect a zero padded episode after a dotted single letter
#[test]
fn should_detect_a_zero_padded_episode_after_a_dotted_single_letter() {
    assert_includes("P.E.01", json!({ "episodes": [1] }));
    assert_includes("Zoo.2.T.E.01.02.Dual.WWW.AZTORRENTS.ORG", json!({ "episodes": [1] }));
}

/// should detect an E episode marker after a dotted single letter word
#[test]
fn should_detect_an_e_episode_marker_after_a_dotted_single_letter_word() {
    assert_includes(
        "Street.Fighter.II.V.E23.iNTERNAL.DVDRip.x264-WaLMaRT",
        json!({ "title": "Street Fighter II V", "episodes": [23] }),
    );
    assert_includes(
        "To.Be.Hero.X.E22.MULTi.1080p.WEB.x264-AMB3R",
        json!({ "title": "To Be Hero X", "episodes": [22] }),
    );
}

/// should detect a spaced E episode marker after a word
#[test]
fn should_detect_a_spaced_e_episode_marker_after_a_word() {
    assert_includes("Show Name E. 05 720p", json!({ "title": "Show Name", "episodes": [5] }));
}

/// should detect the bracketed episode of a dual numbered episode
#[test]
fn should_detect_the_bracketed_episode_of_a_dual_numbered_episode() {
    assert_includes(
        "Dragon Ball Z - 155 (170) (Toonami Midnight Run) [P][C][No NEP][RetroCCN].mkv",
        json!({ "title": "Dragon Ball Z", "episodes": [170] }),
    );
}

/// should detect a glued ova episode and keep the ova word in the title
#[test]
fn should_detect_a_glued_ova_episode_and_keep_the_ova_word_in_the_title() {
    assert_includes(
        "[D3] Shingeki no Kyojin OVA3 (576p Hi10 FLAC).mkv",
        json!({ "title": "Shingeki no Kyojin OVA", "episodes": [3] }),
    );
}

/// should detect a glued zero padded ova episode
#[test]
fn should_detect_a_glued_zero_padded_ova_episode() {
    assert_includes("Girls und Panzer OVA01.mkv", json!({ "title": "Girls und Panzer OVA", "episodes": [1] }));
}

/// should detect a dash separated oad episode with an episode letter
#[test]
fn should_detect_a_dash_separated_oad_episode_with_an_episode_letter() {
    assert_includes(
        "Attack on Titan - OADE01 - Ilse's Notebook.mkv",
        json!({ "title": "Attack on Titan - OAD", "episodes": [1] }),
    );
}

/// should detect a dash separated ova episode and end the title before its number
#[test]
fn should_detect_a_dash_separated_ova_episode_and_end_the_title_before_its_number() {
    assert_includes(
        "[Doki] To LOVE-Ru Darkness - OVA 01 (1280x720 Hi10P AAC) [12345678].mkv",
        json!({ "title": "To LOVE-Ru Darkness - OVA", "episodes": [1] }),
    );
}

/// should detect a numbered special after a season
#[test]
fn should_detect_a_numbered_special_after_a_season() {
    assert_includes(
        "[Judas] High School DxD S1 - SP01.mkv",
        json!({ "title": "High School DxD", "seasons": [1], "episodes": [1] }),
    );
}

/// should detect a dash separated numbered special
#[test]
fn should_detect_a_dash_separated_numbered_special() {
    assert_includes("[FFF] Highschool DxD - SP05 [BD][1080p-FLAC][AF78DBF4].mkv", json!({ "episodes": [5] }));
}

/// should detect an episode range after a russian season glued to its number
#[test]
fn should_detect_an_episode_range_after_a_russian_season_glued_to_its_number() {
    assert_includes("БК_1сезон 261-263.mp4", json!({ "seasons": [1], "episodes": [261, 262, 263] }));
}

/// should detect the absolute episode of a season numbered past its first part
#[test]
fn should_detect_the_absolute_episode_of_a_season_numbered_past_its_first_part() {
    assert_includes("S02_56-082.avi", json!({ "seasons": [2], "episodes": [82] }));
}

/// should detect the absolute episode of a first season episode
#[test]
fn should_detect_the_absolute_episode_of_a_first_season_episode() {
    assert_includes("S01_01-001.avi", json!({ "seasons": [1], "episodes": [1] }));
}

/// should detect an episode range after a season without an extension
#[test]
fn should_detect_an_episode_range_after_a_season_without_an_extension() {
    assert_includes(
        "Show S02 01-026",
        json!({ "seasons": [2], "episodes": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26] }),
    );
}

/// should detect a japanese double episode range marker
#[test]
fn should_detect_a_japanese_double_episode_range_marker() {
    assert_includes(
        "[アニメ BD] ギャラクシーエンジェル 第2期(Z) 第09-10話「GA解析懐石／お好み♥復讐ランチ」(1440x1072 HEVC 10bit FLAC softSub(chi+eng) chap).mkv",
        json!({ "title": "ギャラクシーエンジェル", "seasons": [2], "episodes": [9, 10] }),
    );
}

/// should not detect episode from E-AC3 5.1 audio channels
#[test]
fn should_not_detect_episode_from_e_ac3_5_1_audio_channels() {
    assert_missing("Tu corazón será destrozado Lat-Rus Subt Esp H264 E-AC3 5.1 BDRip Hd 1080p", &["episode"]);
}

/// should not detect episode from E-AC3 7.1 audio channels
#[test]
fn should_not_detect_episode_from_e_ac3_7_1_audio_channels() {
    assert_missing(
        "Black Panther (2018) [IMAX] 2160p H265 HDR10 DV ITA E-AC3 7.1 ITA ENG AC3 5.1 SUB ITA ENG",
        &["episode"],
    );
}

/// should read a cyrillic E after the season as the episode marker
#[test]
fn should_read_a_cyrillic_e_after_the_season_as_the_episode_marker() {
    assert_includes("Skins(UK).S03Е06.Naomi.720p.WEB-DL.1001cinema.mkv", json!({ "seasons": [3], "episodes": [6] }));
}

/// should read a cyrillic E in a dotted LostFilm release
#[test]
fn should_read_a_cyrillic_e_in_a_dotted_lost_film_release() {
    assert_includes("Carnival.Row.S01Е06.AMZN.WEB-DL.1080p.LostFilm.mkv", json!({ "seasons": [1], "episodes": [6] }));
}

/// should read a cyrillic E in a comma separated name
#[test]
fn should_read_a_cyrillic_e_in_a_comma_separated_name() {
    assert_includes("Malory Towers, S06Е04, The Band.mkv", json!({ "seasons": [6], "episodes": [4] }));
}

/// should read a lowercase cyrillic e in an underscore separated name
#[test]
fn should_read_a_lowercase_cyrillic_e_in_an_underscore_separated_name() {
    assert_includes(
        "S05/Leverage_2012_S05е12_rus[LostFilm_TV]_DD5.1_x264_Nikolai.Baranov_WEB-DLRip.AVC_(720p).mp4",
        json!({ "seasons": [5], "episodes": [12] }),
    );
}

/// should keep a cyrillic word starting with E next to a season untouched
#[test]
fn should_keep_a_cyrillic_word_starting_with_e_next_to_a_season_untouched() {
    assert_includes("Show S1 Епизод 4 [2020, WEB-DL]", json!({ "seasons": [1] }));
}

/// should detect an absolute episode after a resolution glued to a group
#[test]
fn should_detect_an_absolute_episode_after_a_resolution_glued_to_a_group() {
    assert_includes("[BonoboSubs][4k]Renegade Immortal - 仙逆 Xian Ni - 164 v2", json!({ "episodes": [164] }));
}

/// should detect an absolute episode after a resolution group
#[test]
fn should_detect_an_absolute_episode_after_a_resolution_group() {
    assert_includes("[Shridhuu][4K] Renegade Immortal / 仙逆 / Xian Ni - 163", json!({ "episodes": [163] }));
}

/// should not detect an episode after a leading source group
#[test]
fn should_not_detect_an_episode_after_a_leading_source_group() {
    assert_missing("[DVD-Rip] Siddu +2 1st Attempt (2010) Lotus 350 MB X264 - TamilC", &["episodes"]);
}

/// should not detect a running time in minutes as episode
#[test]
fn should_not_detect_a_running_time_in_minutes_as_episode() {
    assert_includes("42nd St. Forever Vol. 1 (127 min).avi", json!({ "title": "42nd St. Forever", "volumes": [1] }));
    assert_missing("42nd St. Forever Vol. 1 (127 min).avi", &["episodes"]);
}

/// should not detect a two digit running time as episode
#[test]
fn should_not_detect_a_two_digit_running_time_as_episode() {
    assert_missing("42nd St. Forever Vol. 2 (95 min).avi", &["episodes"]);
}

/// should not detect a running time before a language as episode
#[test]
fn should_not_detect_a_running_time_before_a_language_as_episode() {
    assert_includes("Les Demons  (141 min) English  Jess Franco", json!({ "title": "Les Demons" }));
    assert_missing("Les Demons  (141 min) English  Jess Franco", &["episodes"]);
}

/// should not detect a running time in hours and minutes as episode
#[test]
fn should_not_detect_a_running_time_in_hours_and_minutes_as_episode() {
    assert_includes("Movie Title (1h 32min) 1080p.mkv", json!({ "title": "Movie Title", "resolution": "1080p" }));
    assert_missing("Movie Title (1h 32min) 1080p.mkv", &["episodes"]);
}
