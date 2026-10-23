//! Parses each line of stdin and prints it as a json line: `echo "Movie.2010.1080p" | cargo run --example parse`

use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let mut out = io::BufWriter::new(io::stdout().lock());
    for line in io::stdin().lock().lines() {
        let parsed = parse_torrent_title::parse(&line?);
        serde_json::to_writer(&mut out, &parsed)?;
        writeln!(out)?;
    }
    Ok(())
}
