// Ported from test/year.js of the js library.

//! Paring year

use crate::common::*;

/// should detect the year correctly
#[test]
fn should_detect_the_year_correctly() {
    assert_includes("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO", json!({ "year": 2014 }));
}

/// should detect the year within braces correctly
#[test]
fn should_detect_the_year_within_braces_correctly() {
    assert_includes("Hercules (2014) 1080p BrRip H264 - YIFY", json!({ "year": 2014 }));
}

/// should detect the year within brackets correctly
#[test]
fn should_detect_the_year_within_brackets_correctly() {
    assert_includes("One Shot [2014] DVDRip XViD-ViCKY", json!({ "year": 2014 }));
}

/// should detect the year but not the title if the title is a year
#[test]
fn should_detect_the_year_but_not_the_title_if_the_title_is_a_year() {
    assert_includes("2012 2009 1080p BluRay x264 REPACK-METiS", json!({ "year": 2009 }));
}

/// should detect the year at the beginning if there is none
#[test]
fn should_detect_the_year_at_the_beginning_if_there_is_none() {
    assert_includes("2008 The Incredible Hulk Feature Film.mp4'", json!({ "year": 2008 }));
}

/// should detect the year range
#[test]
fn should_detect_the_year_range() {
    assert_includes(
        "Harry Potter All Movies Collection 2001-2011 720p Dual KartiKing'",
        json!({ "year": "2001-2011" }),
    );
}

/// should detect year range with simplified end year
#[test]
fn should_detect_year_range_with_simplified_end_year() {
    assert_includes("Empty Nest Season 1 (1988 - 89) fiveofseven", json!({ "year": "1988-1989" }));
}

/// should not detect year from bitrate
#[test]
fn should_not_detect_year_from_bitrate() {
    assert_missing("04. Practice Two (1324mb 1916x1080 50fps 1970kbps x265 deef).mkv", &["year"]);
}

/// should not detect year from file size
#[test]
fn should_not_detect_year_from_file_size() {
    assert_missing("01. FP1 (2016mb 1920x1080 3961kbps x265 AAC deef).mkv", &["year"]);
}

/// should not detect year from spaced file size
#[test]
fn should_not_detect_year_from_spaced_file_size() {
    assert_missing("02. Race (1936 MB 1920x1080 x265).mkv", &["year"]);
}

/// should not detect year spanish episode
#[test]
fn should_not_detect_year_spanish_episode() {
    assert_missing("Anatomia De Grey - Temporada 19 [HDTV][Cap.1905][Castellano][www.AtomoHD.nu].avi", &["year"]);
}

/// should detect a bracketed list of years as a year range
#[test]
fn should_detect_a_bracketed_list_of_years_as_a_year_range() {
    assert_includes(
        "Багровые реки 1,2 / Les Rivières pourpres 1,2 [2000, 2004 г., триллер, криминал, детектив, DVDRip]",
        json!({ "year": "2000-2004" }),
    );
}

/// should detect a bracketed list of many years as a year range
#[test]
fn should_detect_a_bracketed_list_of_many_years_as_a_year_range() {
    assert_includes(
        "Бетховен 1,2,3,4,5,6 / Beethoven 1,2,3,4,5,6 [1992,1993,2000,2001,2003,2008 г., Комедия, DVDRip]",
        json!({ "year": "1992-2008" }),
    );
}

/// should detect an em dash year range
#[test]
fn should_detect_an_em_dash_year_range() {
    assert_includes(
        "Назад в будущее 1, 2, 3 / Back to the Future 1, 2, 3 [1985—1990, США, Комедии, Приключения]",
        json!({ "year": "1985-1990" }),
    );
}

/// should detect a single bracketed year before other tags
#[test]
fn should_detect_a_single_bracketed_year_before_other_tags() {
    assert_includes("4 дня в мае / 4 Tage im Mai [2011, BDRip]", json!({ "year": 2011 }));
}

/// should detect a space separated list of years as a year range
#[test]
fn should_detect_a_space_separated_list_of_years_as_a_year_range() {
    assert_includes(
        "Перевозчик 1,2,3 / The Transporter 1,2,3 [2002 2005 2008, BDRip] Dub",
        json!({ "year": "2002-2008" }),
    );
}

/// should detect a slash separated list of years as a year range
#[test]
fn should_detect_a_slash_separated_list_of_years_as_a_year_range() {
    assert_includes(
        "Миссия: Невыполнима [Трилогия] / Mission: Impossible [Trilogy] [1996 / 2000 / 2006 г., боевик]",
        json!({ "year": "1996-2006" }),
    );
}

/// should detect two bracketed years of films joined by an ampersand as a year range
#[test]
fn should_detect_two_bracketed_years_of_films_joined_by_an_ampersand_as_a_year_range() {
    assert_includes(
        "Wonder Woman(2017) & Wonder Woman 1984(2020) [1080p] [H264] [Dubbing,Lektor PL] SYRIX",
        json!({ "title": "Wonder Woman", "year": "2017-2020" }),
    );
}

/// should detect two bracketed years of numbered films joined by an ampersand as a year range
#[test]
fn should_detect_two_bracketed_years_of_numbered_films_joined_by_an_ampersand_as_a_year_range() {
    assert_includes(
        "Legion samobójców 1(2016) & Legion samobójców 2(2021) [1080p] [H264] [Lektor] SYRIX",
        json!({ "year": "2016-2021" }),
    );
}

/// should not detect a year range in a title with an ampersand and one year
#[test]
fn should_not_detect_a_year_range_in_a_title_with_an_ampersand_and_one_year() {
    assert_includes("Tom & Jerry (2021) 1080p WEBRip", json!({ "title": "Tom & Jerry", "year": 2021 }));
}

/// should detect three bracketed years of films joined by ampersands as a year range
#[test]
fn should_detect_three_bracketed_years_of_films_joined_by_ampersands_as_a_year_range() {
    assert_includes(
        "John Wick 1(2014)  &  John Wick 2(2017) &  John Wick 3(2019) [1080p] [H264] [Lektor PL] SYRIX",
        json!({ "year": "2014-2019" }),
    );
}

/// should keep seasons joined by an ampersand with their own years
#[test]
fn should_keep_seasons_joined_by_an_ampersand_with_their_own_years() {
    assert_includes(
        "[Ari-M] Berserk Memorial Edition + Berserk S1 (2016) & S2 (2017) [CR Web-dl 1080p]",
        json!({ "seasons": [1, 2] }),
    );
}

/// should detect the bracketed year after a title year and an alternative title
#[test]
fn should_detect_the_bracketed_year_after_a_title_year_and_an_alternative_title() {
    assert_includes(
        "Захват самолёта / Hijack 1971 / Haijaeking (2024) WEB-DL 1080p от Scar",
        json!({ "title": "Hijack 1971 / Haijaeking", "year": 2024 }),
    );
    assert_missing("Захват самолёта / Hijack 1971 / Haijaeking (2024) WEB-DL 1080p от Scar", &["episodes"]);
}

/// should detect the bracketed year after Wonder Woman 1984 and an alternative title
#[test]
fn should_detect_the_bracketed_year_after_wonder_woman_1984_and_an_alternative_title() {
    assert_includes(
        "Чудо-женщина 1984 / Wonder Woman 1984 / WW84 (2020) BDRip",
        json!({ "title": "Wonder Woman 1984 / WW84", "year": 2020 }),
    );
    assert_missing("Чудо-женщина 1984 / Wonder Woman 1984 / WW84 (2020) BDRip", &["episodes"]);
}

/// should detect the bracketed year after a year in the original title
#[test]
fn should_detect_the_bracketed_year_after_a_year_in_the_original_title() {
    assert_includes(
        "Забастовка / Sulis 1907 / The Riot (2023) BDRip от MegaPeer | P2 | ViruseProject",
        json!({ "title": "Sulis 1907 / The Riot", "year": 2023 }),
    );
    assert_missing("Забастовка / Sulis 1907 / The Riot (2023) BDRip от MegaPeer | P2 | ViruseProject", &["episodes"]);
}

/// should read the second of two years as the year and keep the first in the title
#[test]
fn should_read_the_second_of_two_years_as_the_year_and_keep_the_first_in_the_title() {
    assert_includes("Hijack.1971.2024.1080p.WEB-DL.mkv", json!({ "title": "Hijack 1971", "year": 2024 }));
}

/// should keep a title year before a dotted release year without a trailing dot
#[test]
fn should_keep_a_title_year_before_a_dotted_release_year_without_a_trailing_dot() {
    assert_includes("Chernobyl 1986.2021.1080p.WEBRip.x264-EVO", json!({ "title": "Chernobyl 1986", "year": 2021 }));
}

/// should read a release year older than the year in the title
#[test]
fn should_read_a_release_year_older_than_the_year_in_the_title() {
    assert_includes("Godzilla.2000.1999.720p.BluRay.x264-WaLMaRT", json!({ "title": "Godzilla 2000", "year": 1999 }));
}

/// should not keep a repeated year in the title
#[test]
fn should_not_keep_a_repeated_year_in_the_title() {
    assert_includes(
        "Silent House 2011 2011 480p BRRiP XViD AC3-LEGi0N",
        json!({ "title": "Silent House", "year": 2011 }),
    );
}

/// should read the first of three years of a trilogy as the year
#[test]
fn should_read_the_first_of_three_years_of_a_trilogy_as_the_year() {
    assert_includes("Poltergeist Trilogy 1982 1986 1988 720p BRRip x264 -MgB", json!({ "year": 1982 }));
}

/// should read the first year of a collection with two years
#[test]
fn should_read_the_first_year_of_a_collection_with_two_years() {
    assert_includes(
        "House On Haunted Hill Complete Collection - Horror 1959 2007 Eng Subs 720p [H264-mp4]",
        json!({ "year": 1959 }),
    );
}

/// should read the first year of two films joined by and
#[test]
fn should_read_the_first_year_of_two_films_joined_by_and() {
    assert_includes(
        "Doom And Doom Annihilation - Unrated Extended 2005 2019 Eng Rus Multi Subs 720p [H264-mp4]",
        json!({ "year": 2005 }),
    );
}

/// should not read a resolution width after a year as the year
#[test]
fn should_not_read_a_resolution_width_after_a_year_as_the_year() {
    assert_includes("Arrival 2016 1920 x 808 x264 Phun Psyz", json!({ "title": "Arrival", "year": 2016 }));
}

/// should read a year numbered by sessions as the year
#[test]
fn should_read_a_year_numbered_by_sessions_as_the_year() {
    assert_includes("MotoGP.2021x08.Germany.Race.BTSportHD.1080p", json!({ "title": "MotoGP", "year": 2021 }));
}

/// should read a year numbered by a three digit session as the year
#[test]
fn should_read_a_year_numbered_by_a_three_digit_session_as_the_year() {
    assert_includes(
        "Formula.1.2024x123.Round.24.AbuDhabiGP.Race.International.MULTi.720p.SS",
        json!({ "title": "Formula 1", "year": 2024, "resolution": "720p" }),
    );
}

/// should not read a truncated resolution width as a year numbered by sessions
#[test]
fn should_not_read_a_truncated_resolution_width_as_a_year_numbered_by_sessions() {
    assert_includes("Conan II - O Destruidor 1984 (Audio EN-ES - Leg BR - Mkv 1920x81", json!({ "year": 1984 }));
}

/// should detect the year before an alternative title in a bracketed title
#[test]
fn should_detect_the_year_before_an_alternative_title_in_a_bracketed_title() {
    assert_includes(
        "[DBD-Raws][4K_SDR+Remastered][摩斯拉 1961/Mothra][2160P][UHDBDRip][HEVC-10bit][简繁外挂][FLAC][MKV]",
        json!({ "year": 1961, "title": "Mothra" }),
    );
}

/// should not read a year before a two digit year as the year of an alternative title
#[test]
fn should_not_read_a_year_before_a_two_digit_year_as_the_year_of_an_alternative_title() {
    assert_includes(
        "Кабуто, наездник в маске / Kamen Rider Kabuto (15/49) [Япония, 2006/07 г., токусацу, TVRip] [JAP+Sub Rus] [RAW]",
        json!({ "year": 2006, "title": "Kamen Rider Kabuto" }),
    );
}

/// should keep the first year when a later bracket has a year before a slash
#[test]
fn should_keep_the_first_year_when_a_later_bracket_has_a_year_before_a_slash() {
    assert_includes(
        "Исцеление / Снадобье / Cure / Kyua [1997, BDRip 1080p] [4K REMASTERED, 2024 / FRA Transfer] VO (liosaa)",
        json!({ "year": 1997 }),
    );
}
