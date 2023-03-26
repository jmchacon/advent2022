//! day1 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut elves: Vec<u64> = Vec::new();
    let mut cur: usize = 0;

    for (line_num, line) in lines.flatten().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() < 2, "{}: invalid - {line}", line_num + 1);

        if fields.is_empty() {
            cur += 1;
        } else {
            let val = fields[0].parse::<u64>()?;
            if elves.len() == (cur + 1) {
                elves[cur] += val;
            } else {
                elves.push(val);
            }
        }
    }
    if args.debug {
        println!("{} elves", elves.len());
    }
    elves.sort_unstable();
    let last = elves.len() - 1;

    println!("part1 - max {}", elves[last]);
    let top3 = elves[last] + elves[last - 1] + elves[last - 2];
    println!("part2 - top3 {top3}");
    Ok(())
}
