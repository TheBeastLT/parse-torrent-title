//! Measures parsing speed over the titles of stdin: `cargo run --release --example bench < titles.txt`

use std::hint::black_box;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
    let titles = io::stdin().lock().lines().collect::<io::Result<Vec<_>>>()?;

    let start = Instant::now();
    black_box(parse_torrent_title::parse(&titles[0]));
    println!("first parse (compiles the handlers): {:?}", start.elapsed());
    let start = Instant::now();
    for title in titles.iter().skip(1).take(200) {
        black_box(parse_torrent_title::parse(title));
    }
    println!("next 200 titles (compiling handlers on first use): {:?}", start.elapsed());

    for title in &titles {
        black_box(parse_torrent_title::parse(title));
    }
    let mut runs: Vec<f64> = (0..5)
        .map(|_| {
            let start = Instant::now();
            for title in &titles {
                black_box(parse_torrent_title::parse(title));
            }
            start.elapsed().as_secs_f64() * 1e6 / titles.len() as f64
        })
        .collect();
    runs.sort_by(f64::total_cmp);
    println!("{} titles: median {:.1}us/title, best {:.1}us/title", titles.len(), runs[2], runs[0]);
    Ok(())
}
