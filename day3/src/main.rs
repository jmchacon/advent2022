//! day3 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::{HashMap, HashSet};
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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut rucks = HashMap::<char, HashSet<usize>>::new();
    let mut sum: u32 = 0;
    let mut badges: u32 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() == 1, "{}: invalid - {line}", line_num + 1);

        assert!(
            fields[0].len() % 2 == 0,
            "{}: invalid num chars - {line}",
            line_num + 1
        );
        if line_num % 3 == 0 {
            if line_num != 0 {
                for k in rucks.keys() {
                    if rucks[k].len() == 3 {
                        if args.debug {
                            println!("{} - common key - {k}", line_num + 1);
                        }
                        badges += compute(line_num, *k);
                    }
                }
            }
            // Reset every 3 lines
            rucks = HashMap::<char, HashSet<usize>>::new();
        }

        let mut comp1 = HashMap::new();
        for c in fields[0][0..fields[0].len() / 2].chars() {
            rucks
                .entry(c)
                .or_insert(HashSet::new())
                .insert(line_num % 3);
            comp1.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut comp2 = HashMap::new();
        for c in fields[0][fields[0].len() / 2..].chars() {
            rucks
                .entry(c)
                .or_insert(HashSet::new())
                .insert(line_num % 3);
            comp2.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        for k in comp1.keys() {
            if comp2.contains_key(k) {
                sum += compute(line_num, *k);
                break;
            }
        }
    }
    for k in rucks.keys() {
        if rucks[k].len() == 3 {
            if args.debug {
                println!("{} - common key - {k}", lines.len());
            }
            badges += compute(0, *k);
        }
    }

    println!("part1: {sum}");
    println!("part2: {badges}");
    Ok(())
}

fn compute(_line_num: usize, c: char) -> u32 {
    let mut top = 'A';
    let mut add: u32 = 27;
    if c >= 'a' {
        top = 'a';
        add = 1;
    }
    c as u32 - top as u32 + add
}
