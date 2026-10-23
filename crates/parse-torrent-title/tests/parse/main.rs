//! The test suite of the js library, ported one module per js test file.

// the json! macro recurses once per element of the longest expected episode list
#![recursion_limit = "512"]

mod common;

mod audio;
mod codec;
mod complete;
mod container;
mod convert;
mod customization;
mod date;
mod dubbed;
mod episode;
mod episode_code;
mod extended;
mod extras;
mod group;
mod half_episode;
mod hardcoded;
mod hdr;
mod language;
mod proper;
mod random_releases;
mod region;
mod remastered;
mod repack;
mod resolution;
mod retail;
mod season;
mod source;
mod three_d;
mod title;
mod uncut;
mod unrated;
mod volume;
mod year;
