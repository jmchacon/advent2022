//! day6 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

const PART1: usize = 4;
const PART2: usize = 14;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for (line_num, l) in lines.iter().enumerate() {
        println!("part1 - {}", decode(l, PART1, line_num));
        println!("part2 - {}", decode(l, PART2, line_num));
    }
    Ok(())
}

fn decode(l: &String, size: usize, line_num: usize) -> usize {
    let line = l.as_str().as_bytes();
    assert!(line.len() >= 4, "{} - bad line {l}", line_num + 1);
    let mut tot = size - 1;
    let mut tracking = Vec::<u8>::new();
    for l in line.iter().take(tot + 1) {
        tracking.push(*l);
    }
    for l in line.iter().skip(size) {
        tracking.push(*l);
        tot += 1;
        let mut test = HashMap::new();
        for t in tracking.iter().take(tot + 1).skip(tot - (size - 1)) {
            test.entry(t).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut done = true;
        for k in test.keys() {
            if test[k] > 1 {
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }
    tot + 1
}
