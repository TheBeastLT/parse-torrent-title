// Ported from test/halfEpisode.js of the js library.

//! Parsing half episode

use crate::common::*;

/// should detect half episode after anime dash episode
#[test]
fn should_detect_half_episode_after_anime_dash_episode() {
    assert_includes(
        "[Doki] Hai to Gensou no Grimgar - 02.5 (1920x1080 HEVC BD FLAC) [FCDA1B13].mkv",
        json!({ "episodes": [2], "halfEpisode": true }),
    );
}

/// should detect half episode with version suffix
#[test]
fn should_detect_half_episode_with_version_suffix() {
    assert_includes(
        "[SubsPlease] Baraou no Souretsu - 12.5v2 (720p) [7A329328].mkv",
        json!({ "episodes": [12], "halfEpisode": true }),
    );
}

/// should detect half episode after season episode code
#[test]
fn should_detect_half_episode_after_season_episode_code() {
    assert_includes(
        "[Judas] Koisuru Asteroid - S01E06.5.mkv",
        json!({ "seasons": [1], "episodes": [6], "halfEpisode": true }),
    );
}

/// should detect half episode after short season episode code
#[test]
fn should_detect_half_episode_after_short_season_episode_code() {
    assert_includes(
        "Kyōsōgiga.2013.S1E05.5.VOSTFR.1080p.WEBRip.Opus.x265.X5-452.mkv",
        json!({ "seasons": [1], "episodes": [5], "halfEpisode": true }),
    );
}

/// should detect half episode after ep prefix
#[test]
fn should_detect_half_episode_after_ep_prefix() {
    assert_includes("[Exiled-Destiny]_009-1_Ep09.5_(4a940426).mkv", json!({ "episodes": [9], "halfEpisode": true }));
}

/// should detect half episode after episode word
#[test]
fn should_detect_half_episode_after_episode_word() {
    assert_includes(
        "NanoCore S3 Episode 10.5 English Subbed.mp4",
        json!({ "seasons": [3], "episodes": [10], "halfEpisode": true }),
    );
}

/// should detect half episode at the start
#[test]
fn should_detect_half_episode_at_the_start() {
    assert_includes("11.5 - To Be Needed.mkv", json!({ "episodes": [11], "halfEpisode": true }));
}

/// should detect half episode after the name
#[test]
fn should_detect_half_episode_after_the_name() {
    assert_includes("[E-F]_Hikaru_no_Go_12.5_New_Year_Special.avi", json!({ "episodes": [12], "halfEpisode": true }));
}

/// should detect half episode in brackets
#[test]
fn should_detect_half_episode_in_brackets() {
    assert_includes(
        "[DBD-Raws][Gangsta.][09.5][1080P][BDRip][HEVC-10bit][FLAC].mkv",
        json!({ "episodes": [9], "halfEpisode": true }),
    );
}

/// should detect half episode after season word with season episode
#[test]
fn should_detect_half_episode_after_season_word_with_season_episode() {
    assert_includes(
        "Red Dwarf Season 08 Episode 01.5 - Back In The Red - Extended-1.mp4",
        json!({ "seasons": [8], "episodes": [1], "halfEpisode": true }),
    );
}

/// should not detect half episode from audio channels
#[test]
fn should_not_detect_half_episode_from_audio_channels() {
    assert_missing("Show.S01E05.1080p.WEB-DL.DDP5.1.H.264-GRP.mkv", &["halfEpisode"]);
}

/// should not detect half episode from codec
#[test]
fn should_not_detect_half_episode_from_codec() {
    assert_missing("Show.S01E06.1080p.AAC2.0.H.265.mkv", &["halfEpisode"]);
}

/// should not detect half episode from size
#[test]
fn should_not_detect_half_episode_from_size() {
    assert_missing("Show S01E07 1080p 5.5 GB", &["halfEpisode"]);
}

/// should not detect half episode from version
#[test]
fn should_not_detect_half_episode_from_version() {
    assert_missing("Show - 07 [1080p] [v1.5].mkv", &["halfEpisode"]);
}

/// should not detect half episode from season
#[test]
fn should_not_detect_half_episode_from_season() {
    assert_missing("Show Season 1.5 Complete", &["halfEpisode"]);
}

/// should not detect half episode from frame rate
#[test]
fn should_not_detect_half_episode_from_frame_rate() {
    assert_missing("[Group] Show - 12 (BD 1080p 23.976fps).mkv", &["halfEpisode"]);
}

/// should not detect half episode from imdb rating
#[test]
fn should_not_detect_half_episode_from_imdb_rating() {
    assert_missing("Community S01E12 Comparative Religion IMDB 8.5.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a number in the episode title
#[test]
fn should_not_detect_half_episode_from_a_number_in_the_episode_title() {
    assert_missing("Warrior.Nun.S01E01.Psalm.46.5.1080p.10bit.WEBRip.6CH.x265.HEVC-PSA.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a number in the show name
#[test]
fn should_not_detect_half_episode_from_a_number_in_the_show_name() {
    assert_missing("Special Ops 1.5 S01E02 720p DSNP WEBRip AAC 5.1 MSubs x264 - LOKiHD.mkv", &["halfEpisode"]);
}

/// should not detect half episode from secondary numbering in parentheses
#[test]
fn should_not_detect_half_episode_from_secondary_numbering_in_parentheses() {
    assert_missing("[PM]Pocket_Monsters_-_065_(038.5)_-_Rougela's_Christmas[H264_SUB][8866E393].mkv", &["halfEpisode"]);
}

/// should not detect half episode from a number glued to the episode title
#[test]
fn should_not_detect_half_episode_from_a_number_glued_to_the_episode_title() {
    assert_missing("La.Casa.Di.Carta.4xe05.5.Minuti.Prima.ITA.WEBRIP.x264-mkeagle3.mkv", &["halfEpisode"]);
}

/// should not detect half episode from imdb episode title
#[test]
fn should_not_detect_half_episode_from_imdb_episode_title() {
    assert_missing("S01E05 Episode #1.5 IMDB 9.0.mp4", &["halfEpisode"]);
}

/// should not detect half episode from a number after a cross episode code
#[test]
fn should_not_detect_half_episode_from_a_number_after_a_cross_episode_code() {
    assert_missing("Warrior.Nun.1x01.Salmi.46.5.ITA.ENG.1080p.NFRip.AAC.x265-Pir8.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a season before cross episode
#[test]
fn should_not_detect_half_episode_from_a_season_before_cross_episode() {
    assert_missing("Doctor Who 2005 - 4.5 x01 - Planet of the Dead (Special 04.11.09).avi", &["halfEpisode"]);
}

/// should not detect half episode from a season before episode range
#[test]
fn should_not_detect_half_episode_from_a_season_before_episode_range() {
    assert_missing("Vsegda.govori.vsegda.3.5-6.ser.iz.8.avi", &["halfEpisode"]);
}

/// should not detect half episode from a number starting the show name
#[test]
fn should_not_detect_half_episode_from_a_number_starting_the_show_name() {
    assert_missing(
        "[RUBaDUB][1080p] 2.5 Dimensional Seduction - 01 [BD x265 10bit Dual Audio AC3][CF1401CD].mkv",
        &["halfEpisode"],
    );
}

/// should detect half episode before secondary numbering in parentheses
#[test]
fn should_detect_half_episode_before_secondary_numbering_in_parentheses() {
    assert_includes(
        "[Anime Time] Kuroko No Basket Season 02 - 16.5 (41.5).mkv",
        json!({ "seasons": [2], "episodes": [16], "halfEpisode": true }),
    );
}

/// should detect half episode before of total numbering
#[test]
fn should_detect_half_episode_before_of_total_numbering() {
    assert_includes(
        "Nana_TV_[ru_jp]_[11.5_of_47]_[AnimeReactor_Ru].mkv",
        json!({ "episodes": [11], "halfEpisode": true }),
    );
}

/// should not detect half episode from a number in the episode title after a dash episode
#[test]
fn should_not_detect_half_episode_from_a_number_in_the_episode_title_after_a_dash_episode() {
    assert_missing("14 - Duck Dodgers in the 24.5 Century.mp4", &["halfEpisode"]);
}

/// should detect season and episode from imdb episode title instead of half episode
#[test]
fn should_detect_season_and_episode_from_imdb_episode_title_instead_of_half_episode() {
    assert_includes("Episode #1.5.mkv", json!({ "seasons": [1], "episodes": [5] }));
    assert_missing("Episode #1.5.mkv", &["halfEpisode"]);
}

/// should not detect half episode from an episode title starting with a time
#[test]
fn should_not_detect_half_episode_from_an_episode_title_starting_with_a_time() {
    assert_missing("The.Pitt.S01E11.5.00.P.M.1080p.AMZN.WEB-DL.DDP5.1.H.264-RGzsRutracker.mkv", &["halfEpisode"]);
}

/// should not detect half episode from an episode title starting with a number
#[test]
fn should_not_detect_half_episode_from_an_episode_title_starting_with_a_number() {
    assert_missing("AwesomenessTV.S02E05.5.Ways.to.Reject.a.Guy.480p.HDTV.x264-mSD.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a translated episode title starting with a number
#[test]
fn should_not_detect_half_episode_from_a_translated_episode_title_starting_with_a_number() {
    assert_missing(
        "La.casa.de.papel.A.K.A.Money.Heist.S04E05.5.minutos.antes.DUAL-AUDIO.SPA-ENG.mkv",
        &["halfEpisode"],
    );
}

/// should not detect half episode from a hyphenated episode title starting with a number
#[test]
fn should_not_detect_half_episode_from_a_hyphenated_episode_title_starting_with_a_number() {
    assert_missing("Deadliest.Catch.S11E13.5-Year.Storm.Part.2.XviD-AFG.avi", &["halfEpisode"]);
}

/// should not detect half episode from an episode title with a time after the number
#[test]
fn should_not_detect_half_episode_from_an_episode_title_with_a_time_after_the_number() {
    assert_missing("That.70s.Show.S06E21.5.15.720p.BluRay.FLAC2.0.x264-NTb.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a show name with a hyphenated word
#[test]
fn should_not_detect_half_episode_from_a_show_name_with_a_hyphenated_word() {
    assert_missing("[Serious Chat] 2.5-jigen no Ririsa 01 [BDRip 1080p HEVC FLAC].mkv", &["halfEpisode"]);
}

/// should detect half episode before release tags in a dotted name
#[test]
fn should_detect_half_episode_before_release_tags_in_a_dotted_name() {
    assert_includes(
        "Tengen.Toppa.Gurren.Lagann.E05.5.BDRip.720p.x264 (by OCG).mkv",
        json!({ "episodes": [5], "halfEpisode": true }),
    );
}

/// should detect half episode before a subtitle language tag
#[test]
fn should_detect_half_episode_before_a_subtitle_language_tag() {
    assert_includes(
        "Nura.Rise.Of.The.Yokai.Clan.S02E12.5.SUBFRENCH.1080p.BluRay.x264-KAZETV.mkv",
        json!({ "seasons": [2], "episodes": [12], "halfEpisode": true }),
    );
}

/// should detect half episode before a special word
#[test]
fn should_detect_half_episode_before_a_special_word() {
    assert_includes(
        "S12E11.5.Special.The.Journey.So.Far.720p-YoshQ.mp4",
        json!({ "seasons": [12], "episodes": [11], "halfEpisode": true }),
    );
}

/// should detect half episode before the extension in a dotted name
#[test]
fn should_detect_half_episode_before_the_extension_in_a_dotted_name() {
    assert_includes(
        "Full.Metal.Panic.Invisible.Victory.S01E08.5.mkv",
        json!({ "seasons": [1], "episodes": [8], "halfEpisode": true }),
    );
}

/// should detect half episode before a dotted episode title
#[test]
fn should_detect_half_episode_before_a_dotted_episode_title() {
    assert_includes(
        "Solo.Leveling.S01E07.5.How.to.Get.Stronger.1080p.CR.WEB-DL.DUAL.AAC2.0.H.264-VARYG.mkv",
        json!({ "seasons": [1], "episodes": [7], "halfEpisode": true }),
    );
}

/// should detect half episode before a hyphenated episode title
#[test]
fn should_detect_half_episode_before_a_hyphenated_episode_title() {
    assert_includes(
        "Violet Evergarden S01E04.5-Surely, Someday, You Will Understand Love.mkv",
        json!({ "seasons": [1], "episodes": [4], "halfEpisode": true }),
    );
}

/// should detect half episode before a release language tag
#[test]
fn should_detect_half_episode_before_a_release_language_tag() {
    assert_includes("Nana.Ep11.5.MULTi.DVDRip.x264-NEGI3737.mkv", json!({ "episodes": [11], "halfEpisode": true }));
}

/// should not detect half episode from an episode title starting with a time unit
#[test]
fn should_not_detect_half_episode_from_an_episode_title_starting_with_a_time_unit() {
    assert_missing("Wahlburgers.S01E02.5.OClock.is.Dinnertime.HDTV.x264-CLDD.mkv", &["halfEpisode"]);
}

/// should not detect half episode from an episode title starting with a number range
#[test]
fn should_not_detect_half_episode_from_an_episode_title_starting_with_a_number_range() {
    assert_missing("House.MD.S06E14.5.to.9.720p.10bit.BluRay.x265-budgetbits.mkv", &["halfEpisode"]);
}

/// should detect zero padded half episode before a dash title
#[test]
fn should_detect_zero_padded_half_episode_before_a_dash_title() {
    assert_includes(
        "[Abystoma] Deca-Dence 05.5 - Install (recap) (WEB 720p) [628FF0F5].mkv",
        json!({ "episodes": [5], "halfEpisode": true }),
    );
}

/// should detect single digit half episode before release info
#[test]
fn should_detect_single_digit_half_episode_before_release_info() {
    assert_includes("[HorribleSubs] Cop Craft - 9.5 [720p].mkv", json!({ "episodes": [9], "halfEpisode": true }));
}

/// should detect single digit half episode after episode word before a dash title
#[test]
fn should_detect_single_digit_half_episode_after_episode_word_before_a_dash_title() {
    assert_includes("SPECIAL-Episode 6.5 - Lucy In London.mp4", json!({ "episodes": [6], "halfEpisode": true }));
}

/// should detect half episode after a dotted episode word
#[test]
fn should_detect_half_episode_after_a_dotted_episode_word() {
    assert_includes(
        "[Deadmau- RAWS] Shiki.2010.S01.BDRip.Rus.Jap.Deadmauvlad.Ep.20.5.avi",
        json!({ "episodes": [20], "halfEpisode": true }),
    );
}

/// should detect half episode with a four digit episode
#[test]
fn should_detect_half_episode_with_a_four_digit_episode() {
    assert_includes(
        "One Piece - 1015.5 - A Special Episode to Admire Zoro-senpai and Sanji-senpai! [1080p].mkv",
        json!({ "episodes": [1015], "halfEpisode": true }),
    );
}

/// should not detect half episode from a number in the title next to a different episode
#[test]
fn should_not_detect_half_episode_from_a_number_in_the_title_next_to_a_different_episode() {
    assert_missing("S1953E18 Duck Dodgers in the 24.5 Century MM.avi", &["halfEpisode"]);
}

/// should not detect half episode from an imdb rating equal to the episode
#[test]
fn should_not_detect_half_episode_from_an_imdb_rating_equal_to_the_episode() {
    assert_missing("S01E08 Heroes IMDB 8.5.mkv", &["halfEpisode"]);
}

/// should not detect half episode from season and episode notation at the start
#[test]
fn should_not_detect_half_episode_from_season_and_episode_notation_at_the_start() {
    assert_missing("1.5 The Third Floor Flat.mkv", &["halfEpisode"]);
}

/// should not detect half episode from an episode title starting with a volume unit
#[test]
fn should_not_detect_half_episode_from_an_episode_title_starting_with_a_volume_unit() {
    assert_missing("Life.2x19.5.Litri.ITA-ENG.1080p.DLMux.h264-NovaRip.mkv", &["halfEpisode"]);
}

/// should detect zero padded half episode at the start
#[test]
fn should_detect_zero_padded_half_episode_at_the_start() {
    assert_includes(
        "07.5 Tenchi Muyo! Special Mihoshi's Space Adventure.mkv",
        json!({ "episodes": [7], "halfEpisode": true }),
    );
}

/// should not detect half episode from imdb season and episode title
#[test]
fn should_not_detect_half_episode_from_imdb_season_and_episode_title() {
    assert_missing("S05E05 Episode #5.5.mkv", &["halfEpisode"]);
}

/// should not detect half episode from an episode title with a time after an underscore
#[test]
fn should_not_detect_half_episode_from_an_episode_title_with_a_time_after_an_underscore() {
    assert_missing("That.'70s.Show.S06E21.5_15.HDTV.720p.x264.RUS.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a bare single digit season and episode
#[test]
fn should_not_detect_half_episode_from_a_bare_single_digit_season_and_episode() {
    assert_missing("Yes Minister 1.5 - The Writing On The Wall.avi", &["halfEpisode"]);
}

/// should not detect half episode from a mid season number before a part
#[test]
fn should_not_detect_half_episode_from_a_mid_season_number_before_a_part() {
    assert_missing("TPB 8.5 - Part 2 - Shit Shank.mp4", &["halfEpisode"]);
}

/// should not detect half episode from a date after the year
#[test]
fn should_not_detect_half_episode_from_a_date_after_the_year() {
    assert_missing("WWF.Monday.Night.Raw.1996.8.5.PDTV.XviD-W4F", &["halfEpisode"]);
}

/// should not detect half episode from an ordinal number
#[test]
fn should_not_detect_half_episode_from_an_ordinal_number() {
    assert_missing("06. Duck Dodgers in the 24.5 th Century (1953) DVDRip.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a dimensional title
#[test]
fn should_not_detect_half_episode_from_a_dimensional_title() {
    assert_missing(
        "[RUBaDUB][1080p] 2.5 Dimensional Seduction - Clean Closing 1 [BD x265 10bit].mkv",
        &["halfEpisode"],
    );
}

/// should not detect half episode from a software version
#[test]
fn should_not_detect_half_episode_from_a_software_version() {
    assert_missing("Face ID - Apple's iOS 14.5 Fix Fully Explained! [Full].mp4", &["halfEpisode"]);
}

/// should not detect half episode from a number in the episode title after the episode code
#[test]
fn should_not_detect_half_episode_from_a_number_in_the_episode_title_after_the_episode_code() {
    assert_missing("Downton.Abbey.S05E05.Episode.5.5.1080p.BluRay.REMUX.AVC.FLAC.2.0-NOGRP.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a measure in the episode title
#[test]
fn should_not_detect_half_episode_from_a_measure_in_the_episode_title() {
    assert_missing("S01E05 - Attack of the 5.5 ft. geek.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a distance
#[test]
fn should_not_detect_half_episode_from_a_distance() {
    assert_missing("2.Nordic.Combined.WC.2015.03.07.Lahti.Team.Sprint.Gundersen.2x7.5.km.mkv", &["halfEpisode"]);
}

/// should not detect half episode from a day and month before the year
#[test]
fn should_not_detect_half_episode_from_a_day_and_month_before_the_year() {
    assert_missing("WWE.Friday.Night.Smackdown.16.5.2014.HDTV.x264-jkkk.mp4", &["halfEpisode"]);
}

/// should detect the oad episode labelled with a half episode instead of the half episode
#[test]
fn should_detect_the_oad_episode_labelled_with_a_half_episode_instead_of_the_half_episode() {
    assert_includes(
        "[Cat66] Shingeki no Kyojin - 3.5 OAD1 - Ilse's Notebook (BD 1920x1080 x265 FLAC).mkv",
        json!({ "title": "Shingeki no Kyojin", "episodes": [1] }),
    );
    assert_missing(
        "[Cat66] Shingeki no Kyojin - 3.5 OAD1 - Ilse's Notebook (BD 1920x1080 x265 FLAC).mkv",
        &["halfEpisode"],
    );
}
