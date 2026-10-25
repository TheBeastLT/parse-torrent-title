// Ported from test/title.js of the js library.

//! Parsing title

use crate::common::*;

/// should return the title
#[test]
fn should_return_the_title() {
    assert_includes("La famille bélier", json!({ "title": "La famille bélier" }));
}

/// should remove dots
#[test]
fn should_remove_dots() {
    assert_includes("La.famille.bélier", json!({ "title": "La famille bélier" }));
}

/// should not remove dots when they are part of the title
#[test]
fn should_not_remove_dots_when_they_are_part_of_the_title() {
    assert_includes("Mr. Nobody", json!({ "title": "Mr. Nobody" }));
}

/// should remove underscores
#[test]
fn should_remove_underscores() {
    assert_includes("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", json!({ "title": "doctor who" }));
}

/// should remove unnecessary translations
#[test]
fn should_remove_unnecessary_translations() {
    assert_includes(
        "[GM-Team][国漫][太乙仙魔录 灵飞纪 第3季][Magical Legend of Rise to immortality Ⅲ][01-26][AVC][GB][1080P]",
        json!({ "title": "Magical Legend of Rise to immortality Ⅲ" }),
    );
}

/// should remove unnecessary translations not included in brackets
#[test]
fn should_remove_unnecessary_translations_not_included_in_brackets() {
    assert_includes("【喵萌奶茶屋】★01月新番★[Rebirth][01][720p][简体][招募翻译]", json!({ "title": "Rebirth" }));
}

/// should remove japanese alt titles
#[test]
fn should_remove_japanese_alt_titles() {
    assert_includes(
        "【喵萌奶茶屋】★01月新番★[別對映像研出手！/映像研には手を出すな！/Eizouken ni wa Te wo Dasu na!][01][1080p][繁體]",
        json!({ "title": "Eizouken ni wa Te wo Dasu na!" }),
    );
}

/// should remove japanese alt titles when the main one is in the middle
#[test]
fn should_remove_japanese_alt_titles_when_the_main_one_is_in_the_middle() {
    assert_includes(
        "【喵萌奶茶屋】★01月新番★[別對映像研出手！/Eizouken ni wa Te wo Dasu na!/映像研には手を出すな！][01][1080p][繁體]",
        json!({ "title": "Eizouken ni wa Te wo Dasu na!" }),
    );
}

/// should keep a star inside the title after a group section
#[test]
fn should_keep_a_star_inside_the_title_after_a_group_section() {
    assert_includes(
        "[UCCUSS] BLACK★ROCK SHOOTER ブラック★ロックシューター 全8話 + OVA (BD 1920x1080p AVC FLAC SUPx2)",
        json!({ "title": "BLACK★ROCK SHOOTER" }),
    );
}

/// should keep a star inside the title before a trailing group section
#[test]
fn should_keep_a_star_inside_the_title_before_a_trailing_group_section() {
    assert_includes("Lucky☆Star ★Hi10★ [1080p]", json!({ "title": "Lucky☆Star" }));
}

/// should remove japanese alt titles without separators
#[test]
fn should_remove_japanese_alt_titles_without_separators() {
    assert_includes(
        "[Seed-Raws] 劇場版 ペンギン・ハイウェイ Penguin Highway The Movie (BD 1280x720 AVC AACx4 [5.1+2.0+2.0+2.0]).mp4",
        json!({ "title": "Penguin Highway The Movie" }),
    );
}

/// should not split slash separated title
#[test]
fn should_not_split_slash_separated_title() {
    assert_includes(
        "[SweetSub][Mutafukaz / MFKZ][Movie][BDRip][1080P][AVC 8bit][简体内嵌]",
        json!({ "title": "Mutafukaz / MFKZ" }),
    );
}

/// should clean release group tag title
#[test]
fn should_clean_release_group_tag_title() {
    assert_includes("[Erai-raws] Kingdom 3rd Season - 02 [1080p].mkv", json!({ "title": "Kingdom" }));
}

/// should detect remove russian alt title
#[test]
fn should_detect_remove_russian_alt_title() {
    assert_includes("Голубая волна / Blue Crush (2002) DVDRip", json!({ "title": "Blue Crush" }));
}

/// should not remove non english title if its the only thing left
#[test]
fn should_not_remove_non_english_title_if_its_the_only_thing_left() {
    assert_includes("Жихарка (2007) DVDRip", json!({ "title": "Жихарка" }));
}

/// should not remove non english title with digits in it
#[test]
fn should_not_remove_non_english_title_with_digits_in_it() {
    assert_includes("3 Миссия невыполнима 3 2006г. BDRip 1080p.mkv", json!({ "title": "3 Миссия невыполнима 3" }));
}

/// should not remove russian movie numbering with dot and space
#[test]
fn should_not_remove_russian_movie_numbering_with_dot_and_space() {
    assert_includes("1. Детские игры. 1988. 1080p. HEVC. 10bit..mkv", json!({ "title": "1. Детские игры." }));
}

/// should not remove russian movie numbering with number in title
#[test]
fn should_not_remove_russian_movie_numbering_with_number_in_title() {
    assert_includes(
        "01. 100 девчонок и одна в лифте 2000 WEBRip 1080p.mkv",
        json!({ "title": "01. 100 девчонок и одна в лифте" }),
    );
}

/// should not remove russian movie numbering with dot
#[test]
fn should_not_remove_russian_movie_numbering_with_dot() {
    assert_includes(
        "08.Планета.обезьян.Революция.2014.BDRip-HEVC.1080p.mkv",
        json!({ "title": "08 Планета обезьян Революция" }),
    );
}

/// should clear russian cast info from title
#[test]
fn should_clear_russian_cast_info_from_title() {
    assert_includes(
        "Американские животные / American Animals (Барт Лэйтон / Bart Layton) [2018, Великобритания, США, драма, криминал, BDRip] MVO (СВ Студия)",
        json!({ "title": "American Animals" }),
    );
}

/// should clear cast info from russian title
#[test]
fn should_clear_cast_info_from_russian_title() {
    assert_includes(
        "Греческая смоковница / Griechische Feigen / The Fruit Is Ripe (Зиги Ротемунд / Sigi Rothemund (as Siggi Götz)) [1976, Германия (ФРГ), эротика, комедия, приключения, DVDRip] 2 VO",
        json!({ "title": "Griechische Feigen / The Fruit Is Ripe" }),
    );
}

/// should clear cast info from russian title v2
#[test]
fn should_clear_cast_info_from_russian_title_v2() {
    assert_includes(
        "Греческая смоковница / The fruit is ripe / Griechische Feigen (Siggi Götz) [1976, Германия, Эротическая комедия, DVDRip]",
        json!({ "title": "The fruit is ripe / Griechische Feigen" }),
    );
}

/// should clear cast info from russian title v3
#[test]
fn should_clear_cast_info_from_russian_title_v3() {
    assert_includes(
        "Бастер / Buster (Дэвид Грин / David Green) [1988, Великобритания, Комедия, мелодрама, драма, приключения, криминал, биография, DVDRip]",
        json!({ "title": "Buster" }),
    );
}

/// should detect title even when year is in beginning with paranthesis
#[test]
fn should_detect_title_even_when_year_is_in_beginning_with_paranthesis() {
    assert_includes(
        "(2000) Le follie dell'imperatore - The Emperor's New Groove (DvdRip Ita Eng AC3 5.1).avi",
        json!({ "title": "Le follie dell'imperatore - The Emperor's New Groove" }),
    );
}

/// should remove chinese alt title
#[test]
fn should_remove_chinese_alt_title() {
    assert_includes(
        "[NC-Raws] 间谍过家家 / SPY×FAMILY - 04 (B-Global 1920x1080 HEVC AAC MKV)",
        json!({ "title": "SPY×FAMILY" }),
    );
}

/// should remove ep range in parenthesis
#[test]
fn should_remove_ep_range_in_parenthesis() {
    assert_includes(
        "GTO (Great Teacher Onizuka) (Ep. 1-43) Sub 480p lakshay",
        json!({ "title": "GTO (Great Teacher Onizuka)" }),
    );
}

/// should not fully remove partially russian title
#[test]
fn should_not_fully_remove_partially_russian_title() {
    assert_includes("Книгоноши / Кнiганошы (1987) TVRip от AND03AND | BLR", json!({ "title": "Кнiганошы" }));
}

/// should remove extension fully
#[test]
fn should_remove_extension_fully() {
    assert_includes("Yurusarezaru_mono2.srt", json!({ "title": "Yurusarezaru mono2" }));
}

/// should not detect season prefix in title
#[test]
fn should_not_detect_season_prefix_in_title() {
    assert_includes(
        "COMPASS2.0.ANIMATION.PROJECT.S01E02.Will.You.Be.My.Partner.1080p.CR.WEB-DL.JPN.AAC2.0.H.264.MSubs-ToonsHub.mkv",
        json!({ "title": "COMPASS2 0 ANIMATION PROJECT" }),
    );
}

/// should detect title with absolute episode at the beginning
#[test]
fn should_detect_title_with_absolute_episode_at_the_beginning() {
    assert_includes("1125 - One piece [Sub] 1080p", json!({ "title": "One piece" }));
}

/// should detect title before complete series in parentheses
#[test]
fn should_detect_title_before_complete_series_in_parentheses() {
    assert_includes(
        "Dragon Ball Z (Complete Series) [1080p] [MP4] [English Audio]",
        json!({ "title": "Dragon Ball Z" }),
    );
}

/// should detect title without sample flag
#[test]
fn should_detect_title_without_sample_flag() {
    assert_includes(
        "The Vanishing Of Sidney Hall 2017 Movies HDRip x264 5.1 +Sample",
        json!({ "title": "The Vanishing Of Sidney Hall" }),
    );
}

/// should detect title before season in parentheses
#[test]
fn should_detect_title_before_season_in_parentheses() {
    assert_includes(
        "[Judas] Vinland Saga (Season 2) [1080p][HEVC x265 10bit][Multi-Subs]",
        json!({ "title": "Vinland Saga" }),
    );
}

/// should detect title with dash in it
#[test]
fn should_detect_title_with_dash_in_it() {
    assert_includes(
        "Furiosa - A Mad Max Saga (2024) 2160p H265 HDR10 D V iTA EnG AC3 5 1 Sub iTA EnG NUiTA NUEnG AsPiDe-MIRCrew mkv",
        json!({ "title": "Furiosa - A Mad Max Saga" }),
    );
}

/// should detect short dotted title
#[test]
fn should_detect_short_dotted_title() {
    assert_includes(
        "Court.Cam.S02E01.720p.WEB.h264-ROBOTS[rartv]",
        json!({ "title": "Court Cam", "season": 2, "episode": 1 }),
    );
}

/// should detect short title before episode name
#[test]
fn should_detect_short_title_before_episode_name() {
    assert_includes(
        "Body Cam S08E09 A Hail of Bullets 1080p MAX WEB-DL DD 2 0 H 264-playWEB",
        json!({ "title": "Body Cam", "source": "WEB-DL" }),
    );
}

/// should detect non english title with year
#[test]
fn should_detect_non_english_title_with_year() {
    assert_includes(
        "La jeune fille et les loup 2008 1080p BluRay DTS5.1 x264-SbR.mkv",
        json!({ "title": "La jeune fille et les loup", "year": 2008 }),
    );
}

/// should detect title with audio channels in parentheses
#[test]
fn should_detect_title_with_audio_channels_in_parentheses() {
    assert_includes("Deadpool (2016) (1080p) (7.1).mkv", json!({ "title": "Deadpool", "year": 2016 }));
}

/// should detect year title
#[test]
fn should_detect_year_title() {
    assert_includes(
        "1923 S02E01 The Killing Season 1080p AMZN WEB-DL DDP5 1 H 264-FLUX[TGx]",
        json!({ "title": "1923", "season": 2, "episode": 1 }),
    );
}

/// should detect title after leading episode brackets
#[test]
fn should_detect_title_after_leading_episode_brackets() {
    assert_includes(
        "[S8.Ep4] Trailer Park Boys - Orangie's Pretty F++kin' Tough.mp4",
        json!({ "title": "Trailer Park Boys - Orangie's Pretty F++kin' Tough" }),
    );
}

/// should detect title after leading hi10 tag
#[test]
fn should_detect_title_after_leading_hi10_tag() {
    assert_includes(
        "(Hi10)_Re_Zero_-_23_(BD_1080p)_(SCY)_(D62F164A).mkv",
        json!({ "title": "Re Zero", "episode": 23, "bitDepth": "10bit" }),
    );
}

/// should detect dotted title after site brackets
#[test]
fn should_detect_dotted_title_after_site_brackets() {
    assert_includes(
        "[BEST-TORRENTS.COM] The.Penguin.S01E07.MULTi.1080p.AMZN.WEB-DL.H264.DDP5.1.Atmos-K83",
        json!({ "title": "The Penguin" }),
    );
}

/// should detect english title in parentheses after russian title
#[test]
fn should_detect_english_title_in_parentheses_after_russian_title() {
    assert_includes("О мышах и людях (Of Mice and Men) 1992 BDRip 1080p.mkv", json!({ "title": "Of Mice and Men" }));
}

/// should detect title after site prefix
#[test]
fn should_detect_title_after_site_prefix() {
    assert_includes(
        "www.1TamilMV.phd - Oppenheimer (2023) English BluRay - 1080p - x264 - (DTS 5.1) - 7.3GB - ESub.mkv",
        json!({ "title": "Oppenheimer", "year": 2023 }),
    );
}

/// should detect title with year in it
#[test]
fn should_detect_title_with_year_in_it() {
    assert_includes(
        "Wonder Woman 1984 (2020) [UHDRemux 2160p DoVi P8 Es-DTSHD AC3 En-AC3].mkv",
        json!({ "title": "Wonder Woman 1984", "year": 2020 }),
    );
}

/// should not include all seasons prefix in title
#[test]
fn should_not_include_all_seasons_prefix_in_title() {
    assert_includes(
        "House MD All Seasons (1-8) 720p Ultra-Compressed",
        json!({ "title": "House MD", "seasons": [1, 2, 3, 4, 5, 6, 7, 8] }),
    );
}

/// should not include complete keyword before all seasons in title
#[test]
fn should_not_include_complete_keyword_before_all_seasons_in_title() {
    assert_includes(
        "Breaking Bad Complete (All Seasons 1 to 5) -nelly-",
        json!({ "title": "Breaking Bad", "seasons": [1, 2, 3, 4, 5] }),
    );
}

/// should not include trailing comma in title
#[test]
fn should_not_include_trailing_comma_in_title() {
    assert_includes(
        "The Golden Girls, Seasons 1 thru 7 Complete, X264",
        json!({ "title": "The Golden Girls", "seasons": [1, 2, 3, 4, 5, 6, 7] }),
    );
}

/// should use the latin title when cyrillic season info precedes it
#[test]
fn should_use_the_latin_title_when_cyrillic_season_info_precedes_it() {
    assert_includes(
        "Клиника / Сезон 8 / Серия 1-12 (HDTV 720p) / Scrubs / Season 8/ Episode 1-12 [2009, HDTV, Sub]",
        json!({ "title": "Scrubs", "seasons": [8] }),
    );
}

/// should use the latin title when cyrillic episode info precedes it
#[test]
fn should_use_the_latin_title_when_cyrillic_episode_info_precedes_it() {
    assert_includes("Балда / серии 01-25 (25) / The Baldy Man [1996, DVDRip]", json!({ "title": "The Baldy Man" }));
}

/// should use the latin title when cyrillic season brackets precede it
#[test]
fn should_use_the_latin_title_when_cyrillic_season_brackets_precede_it() {
    assert_includes(
        "Отчаянные дегустаторы отправляются... (Сезон 1 Серий 1-6 из 7) / The Supersizers go [2007, PDTVRip]",
        json!({ "title": "The Supersizers go", "seasons": [1] }),
    );
}

/// should use the latin title when cyrillic season and episode segments precede it
#[test]
fn should_use_the_latin_title_when_cyrillic_season_and_episode_segments_precede_it() {
    assert_includes(
        "Леди Баг и Супер-Кот / Сезон: 2 / Серии: 1-26 из 26 / Miraculous: Tales of Ladybug & Cat Noir [2018, WEB-DL]",
        json!({ "title": "Miraculous: Tales of Ladybug & Cat Noir", "seasons": [2] }),
    );
}

/// should use the latin title when cyrillic episode brackets precede it
#[test]
fn should_use_the_latin_title_when_cyrillic_episode_brackets_precede_it() {
    assert_includes(
        "Лето Нашей Тайны (1-20 серия (125)) / Corpo Dourado [1998, TVRip]",
        json!({ "title": "Corpo Dourado" }),
    );
}

/// should keep the cyrillic title when no latin title is present
#[test]
fn should_keep_the_cyrillic_title_when_no_latin_title_is_present() {
    assert_includes(
        "Мастершеф / Сезон: 2 / Серии: 1, 2 [2014, DVB-T2]",
        json!({ "title": "Мастершеф", "seasons": [2] }),
    );
}

/// should remove cast parenthesis before episode count of total
#[test]
fn should_remove_cast_parenthesis_before_episode_count_of_total() {
    assert_includes(
        "Хочу влюбиться / Koi ga Shitai x3 (Katayama Osamu, Yoshida Ken) 11/11 [Япония, 2001 г., романтика, драма, TVRip] [RAW] [JAP+Sub Rus]",
        json!({ "title": "Koi ga Shitai x3", "year": 2001 }),
    );
}

/// should remove cast parenthesis before episode count of total with details block
#[test]
fn should_remove_cast_parenthesis_before_episode_count_of_total_with_details_block() {
    assert_includes(
        "Наш учебник / Watashitachi no Kyokasho (Kawake Shunsaku, Hayama Hiroki) 12/12 [Япония, 2007 г., детектив, школа, TVRip]",
        json!({ "title": "Watashitachi no Kyokasho", "year": 2007 }),
    );
}

/// should remove single name cast parenthesis before episode count of total
#[test]
fn should_remove_single_name_cast_parenthesis_before_episode_count_of_total() {
    assert_includes(
        "Песня солнцу / Taiyo no uta (Yamamuro Daisuke) 10/10 [Япония, 2006 г., романтика, драма, TVRip]",
        json!({ "title": "Taiyo no uta", "year": 2006 }),
    );
}

/// should remove cast parenthesis with multiple alt titles before episode count of total
#[test]
fn should_remove_cast_parenthesis_with_multiple_alt_titles_before_episode_count_of_total() {
    assert_includes(
        "Якша / Яша / Yasha (Sato Shimako) 11/11 [Япония, 2000 г., Фантастика, драма, DVDRip][RAW][JAP+Sub Rus]",
        json!({ "title": "Yasha", "year": 2000 }),
    );
}

/// should remove cast parenthesis before episode count of total with short details block
#[test]
fn should_remove_cast_parenthesis_before_episode_count_of_total_with_short_details_block() {
    assert_includes(
        "Отец невесты / Hanayome to Papa (Sato Yuichi, Ishikawa Junichi) 12/12 [Япония, 2007 г., семейная]",
        json!({ "title": "Hanayome to Papa", "year": 2007 }),
    );
}

/// should remove cast parenthesis before details block
#[test]
fn should_remove_cast_parenthesis_before_details_block() {
    assert_includes(
        "Наш учебник / Watashitachi no Kyokasho (Kawake Shunsaku, Hayama Hiroki) [Япония, 2007 г.]",
        json!({ "title": "Watashitachi no Kyokasho", "year": 2007 }),
    );
}

/// should keep slash numbers in title
#[test]
fn should_keep_slash_numbers_in_title() {
    assert_includes("Fahrenheit 9/11 (2004) 1080p", json!({ "title": "Fahrenheit 9/11", "year": 2004 }));
}

/// should keep slash numbers title
#[test]
fn should_keep_slash_numbers_title() {
    assert_includes("24/7 2022 WEB-DL", json!({ "title": "24/7", "year": 2022 }));
}

/// should keep accented first letter
#[test]
fn should_keep_accented_first_letter() {
    assert_includes("Último Alvo 2024 WEB-DL 1080p x264 DUAL 5.1", json!({ "title": "Último Alvo", "year": 2024 }));
}

/// should keep accented first letter with brackets
#[test]
fn should_keep_accented_first_letter_with_brackets() {
    assert_includes("Área 15 2018 (720p) DUBLADO WWW.BLUDV.COM", json!({ "title": "Área 15", "year": 2018 }));
}

/// should prefer latin only alt title over accented one
#[test]
fn should_prefer_latin_only_alt_title_over_accented_one() {
    assert_includes(
        "Полный рабочий день / À plein temps / Full Time (2021) BDRip 720p от msltel",
        json!({ "title": "Full Time", "year": 2021 }),
    );
}

/// should keep accented title when it is the only one
#[test]
fn should_keep_accented_title_when_it_is_the_only_one() {
    assert_includes(
        "Безумно грустная принцесса / Šíleně smutná princezna [1968, DVDRip]",
        json!({ "title": "Šíleně smutná princezna", "year": 1968 }),
    );
}

/// should keep ESP as part of the title before an episode dash
#[test]
fn should_keep_esp_as_part_of_the_title_before_an_episode_dash() {
    assert_includes("[HorribleSubs] Tokyo ESP - 08 [720p].mkv", json!({ "title": "Tokyo ESP", "episodes": [8] }));
}

/// should skip tag sections after the release group
#[test]
fn should_skip_tag_sections_after_the_release_group() {
    assert_includes(
        "[BonoboSubs][4k]Renegade Immortal - Xian Ni Episode 100",
        json!({ "title": "Renegade Immortal - Xian Ni", "episodes": [100] }),
    );
}

/// should skip a year section after the release group
#[test]
fn should_skip_a_year_section_after_the_release_group() {
    assert_includes(
        "[Fansubber] [1995] Hyper Doll - Dub[EN,ES,IT,JP] & Sub[EN,FR]",
        json!({ "title": "Hyper Doll", "year": 1995 }),
    );
}

/// should keep the longest section when the name is only bracketed sections
#[test]
fn should_keep_the_longest_section_when_the_name_is_only_bracketed_sections() {
    assert_includes(
        "[Wu Dong Qian Kun][2022][S3E06][1080p+] ENG SUB (Softsub)",
        json!({ "title": "Wu Dong Qian Kun" }),
    );
}

/// should take the longest section when only a short tag is left outside the sections
#[test]
fn should_take_the_longest_section_when_only_a_short_tag_is_left_outside_the_sections() {
    assert_includes(
        "[Kurina][Dou Luo Da Lu][Soul Land][120][1080P]EN",
        json!({ "title": "Dou Luo Da Lu", "episodes": [120] }),
    );
}

/// should not take the release group as title when other text remains
#[test]
fn should_not_take_the_release_group_as_title_when_other_text_remains() {
    assert_field_ne("[tvN] 미생.E20.END.141220.HDTV.H264.720p-WITH.mp4", "title", json!("tvN"));
    assert_field_ne(
        "[philosophy-raws][COWBOY BEBOP][TV 01-26+Movie+SP]][BDRIP][1080p 10bit][Three Audio]",
        "title",
        json!("philosophy-raws"),
    );
}

/// should not take a full width bracket section after the tags as title
#[test]
fn should_not_take_a_full_width_bracket_section_after_the_tags_as_title() {
    assert_includes(
        "[SweetSub][小書痴的下剋上 領主的養女][Honzuki no Gekokujou S04][16][WebRip][1080P][AVC 8bit][繁日雙語]（第四季）",
        json!({ "title": "Honzuki no Gekokujou", "seasons": [4], "episodes": [16] }),
    );
    assert_field("[Group][Show Name][05][1080P]【Season 2】", "title", json!("Show Name"));
}

/// should keep a short title after the release group
#[test]
fn should_keep_a_short_title_after_the_release_group() {
    assert_includes("[HorribleSubs] K - 11 [720p].mkv", json!({ "title": "K", "episodes": [11] }));
    assert_includes("[HDTV - Eng - SubITA] 24 S07E04 [CR-Bt]", json!({ "title": "24", "episodes": [4] }));
    assert_includes(
        "[NC-Raws] 谎颜 - 10 (B-Global Donghua 1920x1080 HEVC AAC MKV)",
        json!({ "title": "谎颜", "episodes": [10] }),
    );
}

/// should not keep a dangling sub marker before language codes
#[test]
fn should_not_keep_a_dangling_sub_marker_before_language_codes() {
    assert_includes("[Fansubber] [1998] Gekiganger 3 - Sub[EN,ES]", json!({ "title": "Gekiganger 3", "year": 1998 }));
}

/// should keep sub when it is part of the title
#[test]
fn should_keep_sub_when_it_is_part_of_the_title() {
    assert_includes("The.Sub.2019.1080p.WEBRip.x264", json!({ "title": "The Sub", "year": 2019 }));
}

/// should keep a hyphenated Ru as part of the title
#[test]
fn should_keep_a_hyphenated_ru_as_part_of_the_title() {
    assert_includes(
        "[PUNCH] To Love-Ru Darkness 2nd - 10 (1080p) [rich_jc].mkv",
        json!({ "title": "To Love-Ru Darkness 2nd", "episodes": [10] }),
    );
    assert_missing("[PUNCH] To Love-Ru Darkness 2nd - 10 (1080p) [rich_jc].mkv", &["languages"]);
}

/// should keep the Code Geass R2 season in the title
#[test]
fn should_keep_the_code_geass_r2_season_in_the_title() {
    assert_includes(
        "Code Geass R2 - 218 - Final Battle Tokyo II {C_P}.avi",
        json!({ "title": "Code Geass R2", "episodes": [218] }),
    );
}

/// should not keep the episode word before an episode dash
#[test]
fn should_not_keep_the_episode_word_before_an_episode_dash() {
    assert_includes(
        "[GoGoAnime.net] One Piece episode - 436 [English Sub][480p].avi",
        json!({ "title": "One Piece", "episodes": [436] }),
    );
}

/// should end the title before a dual numbered episode
#[test]
fn should_end_the_title_before_a_dual_numbered_episode() {
    assert_includes("Dragon Ball Z - 058 (072) (Toonami) [P][C][marktrade].mkv", json!({ "title": "Dragon Ball Z" }));
}

/// should keep a dashed part number in the title before a dual numbered episode
#[test]
fn should_keep_a_dashed_part_number_in_the_title_before_a_dual_numbered_episode() {
    assert_includes("Show - Part 2 - 05 (17).mkv", json!({ "title": "Show - Part 2" }));
}

/// should end the title before an unspaced dual numbered episode
#[test]
fn should_end_the_title_before_an_unspaced_dual_numbered_episode() {
    assert_includes("[Orphan] Smash Hit! - 03(1.5) (DVD) [632E4274].mkv", json!({ "title": "Smash Hit!" }));
}

/// should end the title before a dual numbered episode with a part letter
#[test]
fn should_end_the_title_before_a_dual_numbered_episode_with_a_part_letter() {
    assert_includes(
        "Dragon Ball Z - 117A (132A) (TV3) [P][DragoonClawNZ].mkv",
        json!({ "title": "Dragon Ball Z", "episodes": [132] }),
    );
}

/// should remove a chinese season marker from the title
#[test]
fn should_remove_a_chinese_season_marker_from_the_title() {
    assert_includes(
        "[ANi] 葬送的芙莉蓮 第二季 - 03 [1080P][Baha][WEB-DL][AAC AVC][CHT].mp4",
        json!({ "title": "葬送的芙莉蓮", "seasons": [2], "episodes": [3] }),
    );
}

/// should keep the romaji title after a chinese title with a season marker
#[test]
fn should_keep_the_romaji_title_after_a_chinese_title_with_a_season_marker() {
    assert_includes(
        "[晚街与灯][Re：从零开始的异世界生活 第四季 / Re:Zero kara Hajimeru Isekai Seikatsu 4th Season][15 / 总第81][简日双语][1080P]",
        json!({ "title": "Re:Zero kara Hajimeru Isekai Seikatsu", "seasons": [4], "episodes": [15] }),
    );
}

/// should drop a quoted episode title and a glued year suffix after a japanese title
#[test]
fn should_drop_a_quoted_episode_title_and_a_glued_year_suffix_after_a_japanese_title() {
    assert_includes("NNNドキュメント’26「縁組み家族 こどもの権利にオトナはどうこたえる」", json!({ "title": "NNN" }));
}

/// should keep a year suffix glued to a russian title
#[test]
fn should_keep_a_year_suffix_glued_to_a_russian_title() {
    assert_includes("Берлин'23 / Серии: 1-7 из 8 [2026, WEBRip-AVC]", json!({ "title": "Берлин'23" }));
}

/// should drop a trailing quoted japanese episode title
#[test]
fn should_drop_a_trailing_quoted_japanese_episode_title() {
    assert_includes(
        "ガリレオX「躍進するフィジカルAI開発 次世代ロボットは何ができるのか？」",
        json!({ "title": "ガリレオX" }),
    );
}

/// should drop a trailing quoted edition name
#[test]
fn should_drop_a_trailing_quoted_edition_name() {
    assert_includes(
        "[hydes] Patlabor: The Movie『Enthusiast's Collected Edition』JPBD Remux｜5xAudio/5xSub｜Extras",
        json!({ "title": "Patlabor: The Movie" }),
    );
}

/// should drop a trailing quoted subtitle with the dash before it
#[test]
fn should_drop_a_trailing_quoted_subtitle_with_the_dash_before_it() {
    assert_includes(
        "[JMAX] [2026.08.18] 学園アイドルマスター - 「ねえ、言っちゃうよ。」 [FLAC 96kHz/24bit]",
        json!({ "title": "学園アイドルマスター" }),
    );
}

/// should keep a quoted title after a movie prefix
#[test]
fn should_keep_a_quoted_title_after_a_movie_prefix() {
    assert_field_ne("Movie『 Your Name. 』2016 1080p.MP4", "title", json!("Movie"));
}

/// should keep removing chinese chars glued to a latin title
#[test]
fn should_keep_removing_chinese_chars_glued_to_a_latin_title() {
    assert_includes("[LoliHouse] MAO摩绪 - 20 [WebRip 1080p HEVC-10bit AAC][简繁内封字幕]", json!({ "title": "MAO" }));
}

/// should remove a channel number prefix from the title
#[test]
fn should_remove_a_channel_number_prefix_from_the_title() {
    assert_includes(
        "Ch5 The Rise and Fall of the King of Spain 2021 1080p HDTV x265-MVGroup",
        json!({ "title": "The Rise and Fall of the King of Spain", "year": 2021 }),
    );
}

/// should remove a dotted channel 4 prefix from the title
#[test]
fn should_remove_a_dotted_channel_4_prefix_from_the_title() {
    assert_includes(
        "Ch4.Graysons.Art.Club.Series.2.1080p.HDTV.x265.AAC.MVGroup.org.mkv",
        json!({ "title": "Graysons Art Club" }),
    );
}

/// should keep a language name followed by title words before the year in the title
#[test]
fn should_keep_a_language_name_followed_by_title_words_before_the_year_in_the_title() {
    assert_includes("A Serbian Film 2010 1080p BluRay", json!({ "title": "A Serbian Film", "year": 2010 }));
    assert_missing("A Serbian Film 2010 1080p BluRay", &["languages"]);
}

/// should keep a dot separated language name followed by title words before the year in the title
#[test]
fn should_keep_a_dot_separated_language_name_followed_by_title_words_before_the_year_in_the_t() {
    assert_includes(
        "The.French.Connection.1971.x264.DTS-WAF",
        json!({ "title": "The French Connection", "year": 1971 }),
    );
    assert_missing("The.French.Connection.1971.x264.DTS-WAF", &["languages"]);
}

/// should keep a language name followed by title words before any tag in the title without a year
#[test]
fn should_keep_a_language_name_followed_by_title_words_before_any_tag_in_the_title_without_a() {
    assert_includes(
        "A.Serbian.Film.1080p.Multi.Subs.AAC.x264-DLKING.mp4",
        json!({ "title": "A Serbian Film", "languages": ["multi subs"] }),
    );
}

/// should keep a language name before the season in the title while reading the caps language tag
#[test]
fn should_keep_a_language_name_before_the_season_in_the_title_while_reading_the_caps_language() {
    assert_includes(
        "A.French.Village.S02.FRENCH.1080p.AMZN.WEBRip.DDP2.0.x264-SbR[rartv]",
        json!({ "title": "A French Village", "seasons": [2], "languages": ["french"] }),
    );
}

/// should keep a leading language name in the title of a series
#[test]
fn should_keep_a_leading_language_name_in_the_title_of_a_series() {
    assert_includes("Swedish.Dicks.S01E01.WEB-DL.x264-RARBG", json!({ "title": "Swedish Dicks", "seasons": [1] }));
    assert_missing("Swedish.Dicks.S01E01.WEB-DL.x264-RARBG", &["languages"]);
}

/// should keep a language name in the middle of a title before the year
#[test]
fn should_keep_a_language_name_in_the_middle_of_a_title_before_the_year() {
    assert_includes(
        "The French Dispatch 2021 WEB-DL Sonata Premiere",
        json!({ "title": "The French Dispatch", "year": 2021 }),
    );
    assert_missing("The French Dispatch 2021 WEB-DL Sonata Premiere", &["languages"]);
}

/// should keep a language surname followed by title words before the year
#[test]
fn should_keep_a_language_surname_followed_by_title_words_before_the_year() {
    assert_includes(
        "Dawn.French.Live-Thirty.Million.Minutes.2016.1080p.WEBRip.x265-RARBG",
        json!({ "title": "Dawn French Live-Thirty Million Minutes", "year": 2016 }),
    );
    assert_missing("Dawn.French.Live-Thirty.Million.Minutes.2016.1080p.WEBRip.x265-RARBG", &["languages"]);
}

/// should end the title at a scene caps language tag after a mixed case title
#[test]
fn should_end_the_title_at_a_scene_caps_language_tag_after_a_mixed_case_title() {
    assert_includes(
        "Vuoi.Sapere.Un.Segreto.iTALiAN.DVDRip.XviD-Republic",
        json!({ "title": "Vuoi Sapere Un Segreto", "languages": ["italian"] }),
    );
}

/// should end the title at a caps language tag before the year
#[test]
fn should_end_the_title_at_a_caps_language_tag_before_the_year() {
    assert_includes(
        "The Reader ITALIAN 2009-BY NERO69.avi",
        json!({ "title": "The Reader", "year": 2009, "languages": ["italian"] }),
    );
}

/// should keep a caps language word inside an all caps title
#[test]
fn should_keep_a_caps_language_word_inside_an_all_caps_title() {
    assert_includes("THE ITALIAN JOB 2003 1080p BluRay x264", json!({ "title": "THE ITALIAN JOB", "year": 2003 }));
    assert_missing("THE ITALIAN JOB 2003 1080p BluRay x264", &["languages"]);
}

/// should not detect a language code inside an accented word
#[test]
fn should_not_detect_a_language_code_inside_an_accented_word() {
    assert_includes(
        "Mistrovský plán / Masterplan (2026)(CZ/EN)[2160p][HDR10+/DV][HEVC]",
        json!({ "title": "Masterplan", "year": 2026, "languages": ["english", "czech"] }),
    );
}

/// should not detect an episode marker inside an accented word
#[test]
fn should_not_detect_an_episode_marker_inside_an_accented_word() {
    assert_includes("Sonríe 2 (2024).1080p.dual-lat.cinecalidad.day.mp4", json!({ "title": "Sonríe 2", "year": 2024 }));
    assert_missing("Sonríe 2 (2024).1080p.dual-lat.cinecalidad.day.mp4", &["episodes"]);
}

/// should not detect a container inside an accented word
#[test]
fn should_not_detect_a_container_inside_an_accented_word() {
    assert_includes("Aviões 2 (2014) BluRay 1080p 5.1 Dublado - S4F", json!({ "title": "Aviões 2", "year": 2014 }));
    assert_missing("Aviões 2 (2014) BluRay 1080p 5.1 Dublado - S4F", &["container"]);
}

/// should keep a dashed subtitle of a bare file name in the title
#[test]
fn should_keep_a_dashed_subtitle_of_a_bare_file_name_in_the_title() {
    assert_includes("Thor - Ragnarok.mp4", json!({ "title": "Thor - Ragnarok" }));
}

/// should keep a hyphenated bare file name whole in the title
#[test]
fn should_keep_a_hyphenated_bare_file_name_whole_in_the_title() {
    assert_includes("Ant-Man.mp4", json!({ "title": "Ant-Man" }));
}

/// should remove a spanish anime series flag from the title
#[test]
fn should_remove_a_spanish_anime_series_flag_from_the_title() {
    assert_includes("Cyberpunk Edgerunners 2 (Serie Anime) Temp 02 Sub", json!({ "title": "Cyberpunk Edgerunners 2" }));
}

/// should remove a spanish tv series flag from the title
#[test]
fn should_remove_a_spanish_tv_series_flag_from_the_title() {
    assert_includes(
        "Halt and Catch Fire (Serie de TV) [HDTv][AC3][Cas] (1T) [E02]",
        json!({ "title": "Halt and Catch Fire" }),
    );
}

/// should remove a series tv flag from the title
#[test]
fn should_remove_a_series_tv_flag_from_the_title() {
    assert_includes("Crisis (Series TV) [HDiTunes][AC3][Esp] [1x11] (1T)", json!({ "title": "Crisis" }));
}

/// should remove a bare serie flag from the title
#[test]
fn should_remove_a_bare_serie_flag_from_the_title() {
    assert_includes("Grimm (Serie) [T03][HDiTunes][DUAL][E02/16]", json!({ "title": "Grimm" }));
}

/// should remove a spanish cartoon series flag from the title
#[test]
fn should_remove_a_spanish_cartoon_series_flag_from_the_title() {
    assert_includes("Primal (Serie Dibujos) Temp 3 - 03x04 Lat", json!({ "title": "Primal" }));
}

/// should remove an anime flag from the title
#[test]
fn should_remove_an_anime_flag_from_the_title() {
    assert_includes("Akira Kurosawa's - Samurai 7 (Anime) [2004]", json!({ "title": "Akira Kurosawa's - Samurai 7" }));
}

/// should parse a russian title with many slash separated names without backtracking
#[test]
fn should_parse_a_russian_title_with_many_slash_separated_names_without_backtracking() {
    assert_parses_within(
        "Ратные подвиги Александра Невского/ Битва на поле Куликовом/ Освобождение Москвы/ Полтавская битва/ Битва при Гангуте/ Сражение за Петербург/ Битва за Измаил/ Адмирал Ушаков/ Бородино и его герои/ Оборона. Севастополь/ Освобождение. Болгария/ (Л. Фишель) [2004-2012.г., Исторический, DVDRip]",
        2000,
    );
    assert_includes(
        "Ратные подвиги Александра Невского/ Битва на поле Куликовом/ Освобождение Москвы/ Полтавская битва/ Битва при Гангуте/ Сражение за Петербург/ Битва за Измаил/ Адмирал Ушаков/ Бородино и его герои/ Оборона. Севастополь/ Освобождение. Болгария/ (Л. Фишель) [2004-2012.г., Исторический, DVDRip]",
        json!({ "year": "2004-2012", "source": "DVDRip" }),
    );
}

/// should still drop russian names before a latin title
#[test]
fn should_still_drop_russian_names_before_a_latin_title() {
    assert_includes(
        "Ратные подвиги/ Битва на поле/ Освобождение Москвы/ Battle of the Field [2004, DVDRip]",
        json!({ "title": "Battle of the Field", "year": 2004 }),
    );
}

/// should keep a title number before a year numbered by sessions
#[test]
fn should_keep_a_title_number_before_a_year_numbered_by_sessions() {
    assert_includes("Formula.1.2021x21.Saudi.Arabia.Race.SkyF1HD.1080p", json!({ "title": "Formula 1", "year": 2021 }));
}

/// should keep a spaced title number before a year numbered by sessions
#[test]
fn should_keep_a_spaced_title_number_before_a_year_numbered_by_sessions() {
    assert_includes(
        "Formula 1 2026x16 R02 ChineseGP Sprint Qualifying F1TV 720p SS",
        json!({ "title": "Formula 1", "year": 2026 }),
    );
}

/// should keep a glued title number before a year numbered by sessions
#[test]
fn should_keep_a_glued_title_number_before_a_year_numbered_by_sessions() {
    assert_includes("GP2 2015x05 Great Britain Feature Race SkyF1HD 720p50", json!({ "title": "GP2", "year": 2015 }));
}

/// should keep 3D starting a name after a group
#[test]
fn should_keep_3_d_starting_a_name_after_a_group() {
    assert_includes("[GM] 3D Kanojo Real Girl S01E01-S02E04", json!({ "title": "3D Kanojo Real Girl", "group": "GM" }));
}
