//! day5 advent 2022
use color_eyre::eyre::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for (line_num, l) in lines.iter().enumerate() {
    }
    Ok(())
}
