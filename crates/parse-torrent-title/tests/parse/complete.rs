// Ported from test/complete.js of the js library.

//! Parsing complete collection

use crate::common::*;

/// should detect complete series with full seasons
#[test]
fn should_detect_complete_series_with_full_seasons() {
    assert_includes("[Furi] Avatar - The Last Airbender [720p] (Full 3 Seasons + Extr", json!({ "complete": true }));
}

/// should detect complete collection
#[test]
fn should_detect_complete_collection() {
    assert_includes("Harry.Potter.Complete.Collection.2001-2011.1080p.BluRay.DTS-ETRG", json!({ "complete": true }));
}

/// should detect complete collection with all seasons
#[test]
fn should_detect_complete_collection_with_all_seasons() {
    assert_includes("Game of Thrones All 7 Seasons 1080p ~∞~ .HakunaMaKoko", json!({ "complete": true }));
}

/// should detect complete collection with full series
#[test]
fn should_detect_complete_collection_with_full_series() {
    assert_includes("Avatar: The Last Airbender Full Series 720p", json!({ "complete": true }));
}

/// should detect complete collection with ultimate collection
#[test]
fn should_detect_complete_collection_with_ultimate_collection() {
    assert_includes("Dora the Explorer - Ultimate Collection", json!({ "complete": true }));
}

/// should detect complete collection with complete pack
#[test]
fn should_detect_complete_collection_with_complete_pack() {
    assert_includes("Mr Bean Complete Pack (Animated, Tv series, 2 Movies) DVDRIP (WA", json!({ "complete": true }));
}

/// should detect complete collection with complete set
#[test]
fn should_detect_complete_collection_with_complete_set() {
    assert_includes("American Pie - Complete set (8 movies) 720p mkv - YIFY", json!({ "complete": true }));
}

/// should detect complete collection with complete filmography
#[test]
fn should_detect_complete_collection_with_complete_filmography() {
    assert_includes("Charlie Chaplin - Complete Filmography (87 movies)", json!({ "complete": true }));
}

/// should detect complete collection with movies complete
#[test]
fn should_detect_complete_collection_with_movies_complete() {
    assert_includes("Monster High Movies Complete 2014", json!({ "complete": true }));
}

/// should detect complete collection all movies
#[test]
fn should_detect_complete_collection_all_movies() {
    assert_includes("Harry Potter All Movies Collection 2001-2011 720p Dual KartiKing", json!({ "complete": true }));
}

/// should detect complete movie collection
#[test]
fn should_detect_complete_movie_collection() {
    assert_includes("The Clint Eastwood Movie Collection", json!({ "complete": true }));
}

/// should detect complete collection movies
#[test]
fn should_detect_complete_collection_movies() {
    assert_includes("Clint Eastwood Collection - 15 HD Movies", json!({ "complete": true }));
}

/// should detect complete movies collection
#[test]
fn should_detect_complete_movies_collection() {
    assert_includes("Official  IMDb  Top  250  Movies  Collection  6/17/2011", json!({ "complete": true }));
}

/// should detect collection
#[test]
fn should_detect_collection() {
    assert_includes("The Texas Chainsaw Massacre Collection (1974-2017) BDRip 1080p", json!({ "complete": true }));
}

/// should detect duology
#[test]
fn should_detect_duology() {
    assert_includes("Snabba.Cash.I-II.Duology.2010-2012.1080p.BluRay.x264.anoXmous", json!({ "complete": true }));
}

/// should detect trilogy
#[test]
fn should_detect_trilogy() {
    assert_includes("Star Wars Original Trilogy 1977-1983 Despecialized 720p", json!({ "complete": true }));
}

/// should detect quadrology
#[test]
fn should_detect_quadrology() {
    assert_includes("The.Wong.Kar-Wai.Quadrology.1990-2004.1080p.BluRay.x264.AAC.5.1-", json!({ "complete": true }));
}

/// should detect quadrilogy
#[test]
fn should_detect_quadrilogy() {
    assert_includes("Lethal.Weapon.Quadrilogy.1987-1992.1080p.BluRay.x264.anoXmous", json!({ "complete": true }));
}

/// should detect tetralogy
#[test]
fn should_detect_tetralogy() {
    assert_includes("X-Men.Tetralogy.BRRip.XviD.AC3.RoSubbed-playXD", json!({ "complete": true }));
}

/// should detect pentalogy
#[test]
fn should_detect_pentalogy() {
    assert_includes("Mission.Impossible.Pentalogy.1996-2015.1080p.BluRay.x264.AAC.5.1", json!({ "complete": true }));
}

/// should detect hexalogy
#[test]
fn should_detect_hexalogy() {
    assert_includes("Mission.Impossible.Hexalogy.1996-2018.SweSub.1080p.x264-Justiso", json!({ "complete": true }));
}

/// should detect hexalogy
#[test]
fn should_detect_hexalogy_2() {
    assert_includes("American.Pie.Heptalogy.SWESUB.DVDRip.XviD-BaZZe", json!({ "complete": true }));
}

/// should detect anthalogy
#[test]
fn should_detect_anthalogy() {
    assert_includes("The Exorcist 1, 2, 3, 4, 5 - Complete Horror Anthology 1973-2005", json!({ "complete": true }));
}

/// should detect saga
#[test]
fn should_detect_saga() {
    assert_includes("Harry.Potter.Complete.Saga. I - VIII .1080p.Bluray.x264.anoXmous", json!({ "complete": true }));
}

/// should detect italian complete
#[test]
fn should_detect_italian_complete() {
    assert_includes("Inganno - Miniserie (2024) [COMPLETA] SD H264 ITA AAC-UBi", json!({ "complete": true }));
}

/// should not remove collection from title
#[test]
fn should_not_remove_collection_from_title() {
    assert_includes(
        "[Erai-raws] Ninja Collection - 05 [720p][Multiple Subtitle].mkv",
        json!({ "title": "Ninja Collection", "complete": true }),
    );
}

/// should not remove kolekcja from title
#[test]
fn should_not_remove_kolekcja_from_title() {
    assert_includes(
        "Kolekcja Halloween (1978-2022) [720p] [BRRip] [XviD] [AC3-ELiTE] [Lektor PL]",
        json!({ "title": "Halloween", "complete": true }),
    );
}

/// should remove czech kolekce from title
#[test]
fn should_remove_czech_kolekce_from_title() {
    assert_includes(
        "Terminátor - Kolekce CZ/EN 2160p",
        json!({ "title": "Terminátor", "complete": true, "languages": ["english", "czech"] }),
    );
}

/// should detect a russian dilogy as complete
#[test]
fn should_detect_a_russian_dilogy_as_complete() {
    assert_includes(
        "Ночь в музее. Дилогия / Night at the Museum. Dilogy [2006 / 2009 г. фэнтези, США, комедия]",
        json!({ "complete": true }),
    );
}

/// should keep the english title of a russian trilogy
#[test]
fn should_keep_the_english_title_of_a_russian_trilogy() {
    assert_includes(
        "Ледниковый период [Трилогия] / Ice Age [Trilogy] (2002-2009) HDRip- BDRip-AVC 720p",
        json!({ "title": "Ice Age", "complete": true }),
    );
}

/// should detect a russian count of films as complete
#[test]
fn should_detect_a_russian_count_of_films_as_complete() {
    assert_includes(
        "Короткие истории о любви - 9 фильмов [Драма, SATRip] [Израиль]",
        json!({ "title": "Короткие истории о любви", "complete": true }),
    );
    assert_missing("Короткие истории о любви - 9 фильмов [Драма, SATRip] [Израиль]", &["episodes"]);
}

/// should detect a bracketed russian count of films as complete
#[test]
fn should_detect_a_bracketed_russian_count_of_films_as_complete() {
    assert_includes(
        "Мистер Питкин / Mister Pitkin [12 фильмов из 12] (1953-1966) DVDRip от MediaClub",
        json!({ "complete": true }),
    );
    assert_missing("Мистер Питкин / Mister Pitkin [12 фильмов из 12] (1953-1966) DVDRip от MediaClub", &["episodes"]);
}

/// should not detect a word film without a count as complete
#[test]
fn should_not_detect_a_word_film_without_a_count_as_complete() {
    assert_missing("Надо снимать фильмы о любви (2024) WEBRip-AVC от ExKinoRay", &["complete"]);
}

/// should remove czech kolekce filmů from title
#[test]
fn should_remove_czech_kolekce_filmu_from_title() {
    assert_includes("Harry Potter Kolekce filmů CZ", json!({ "title": "Harry Potter", "complete": true }));
}

/// should not detect the kolekcia release group as complete
#[test]
fn should_not_detect_the_kolekcia_release_group_as_complete() {
    assert_missing("Criminal 2016.480p.BluRay.x264.AC3-KOLEKCiA", &["complete"]);
}

/// should detect trailing complete and keep it out of the title
#[test]
fn should_detect_trailing_complete_and_keep_it_out_of_the_title() {
    assert_includes("Gurren Lagann Complete [BD 1080p]", json!({ "title": "Gurren Lagann", "complete": true }));
}

/// should detect spanish complete series with an episode count
#[test]
fn should_detect_spanish_complete_series_with_an_episode_count() {
    assert_includes(
        "Mazinger Z La Serie Completa 92 Capitulos [DVDRip]",
        json!({ "title": "Mazinger Z", "complete": true }),
    );
}

/// should detect portuguese completo
#[test]
fn should_detect_portuguese_completo() {
    assert_includes(
        "Dragon Ball Kai Completo (2011) [Dual Audio]",
        json!({ "title": "Dragon Ball Kai", "complete": true }),
    );
}

/// should detect french integrale in parentheses
#[test]
fn should_detect_french_integrale_in_parentheses() {
    assert_includes("Death Note (Integrale) [VOSTFR 1080p]", json!({ "title": "Death Note", "complete": true }));
}

/// should detect spanish complete season
#[test]
fn should_detect_spanish_complete_season() {
    assert_includes(
        "Ghost in the Shell -Primera Temporada Completa",
        json!({ "title": "Ghost in the Shell", "complete": true }),
    );
}

/// should keep complete in a complete edition title
#[test]
fn should_keep_complete_in_a_complete_edition_title() {
    assert_field_contains(
        "World Masterpiece Theater Complete Edition - Anne of Green Gables [DVDRip]",
        "title",
        "Complete Edition",
    );
}

/// should strip trailing full batch
#[test]
fn should_strip_trailing_full_batch() {
    assert_includes("Onigashima Paced - Full Batch", json!({ "title": "Onigashima Paced", "complete": true }));
}

/// should strip bracketed batch
#[test]
fn should_strip_bracketed_batch() {
    assert_includes(
        "[Golumpa] 18if (Batch) [English Dub] [FuniDub 1080p x264 AAC] [MKV]",
        json!({ "title": "18if", "complete": true }),
    );
}

/// should strip leading unofficial batch
#[test]
fn should_strip_leading_unofficial_batch() {
    assert_includes(
        "(Unofficial Batch) Cross Ange (BD)(Hi10)(720)(AAC)",
        json!({ "title": "Cross Ange", "complete": true }),
    );
}

/// should strip complete batch
#[test]
fn should_strip_complete_batch() {
    assert_includes(
        "Shinryaku! Ika Musume (Complete Batch) [10bit BD 720p]",
        json!({ "title": "Shinryaku! Ika Musume", "complete": true }),
    );
}

/// should not strip batch inside a longer bracket
#[test]
fn should_not_strip_batch_inside_a_longer_bracket() {
    assert_includes(
        "[blad761] Princess Principal. [Batch 01 - 12] 720p [MKV Hi444PP AAC]",
        json!({ "title": "Princess Principal." }),
    );
}

/// should not strip bad batch title
#[test]
fn should_not_strip_bad_batch_title() {
    assert_includes("Star.Wars.The.Bad.Batch.S02E03.1080p", json!({ "title": "Star Wars The Bad Batch" }));
    assert_missing("Star.Wars.The.Bad.Batch.S02E03.1080p", &["complete"]);
}

/// should detect a leading spanish duologia as complete
#[test]
fn should_detect_a_leading_spanish_duologia_as_complete() {
    assert_includes(
        "Duologia Guardianes de la Galaxia [BluRay 1080p HEVC X265][10Bits][AC3 5.1 Castellano AC3 5.1-Ingles+Subs][ES-EN]",
        json!({ "title": "Duologia Guardianes de la Galaxia", "complete": true }),
    );
}

/// should detect dualogy as complete
#[test]
fn should_detect_dualogy_as_complete() {
    assert_includes(
        "The Witch Dualogy Part 1 & 2 The Subversion & The Other One Multiple Audios DDP5 1 1080p",
        json!({ "complete": true }),
    );
}

/// should detect an accented spanish trilogia as complete
#[test]
fn should_detect_an_accented_spanish_trilogia_as_complete() {
    assert_includes("El Padrino Trilogía 1080p BluRay", json!({ "title": "El Padrino Trilogía", "complete": true }));
}

/// should detect bracketed seasons without a number as complete
#[test]
fn should_detect_bracketed_seasons_without_a_number_as_complete() {
    assert_includes("The.Lowdown.(Seasons).LostFilm", json!({ "title": "The Lowdown", "complete": true }));
}

/// should detect bracketed seasons as complete for a single word title
#[test]
fn should_detect_bracketed_seasons_as_complete_for_a_single_word_title() {
    assert_includes("Time.(Seasons).LostFilm", json!({ "title": "Time", "complete": true }));
}
