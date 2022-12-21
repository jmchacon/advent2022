//! day18 advent 2022
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i64, i64, i64);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut squares = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split(",").collect::<Vec<_>>();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
        squares.push(Location(
            i64::from_str_radix(parts[0], 10).unwrap(),
            i64::from_str_radix(parts[1], 10).unwrap(),
            i64::from_str_radix(parts[2], 10).unwrap(),
        ))
    }
    // If nothing touches we get 6 faces per square.
    let mut faces = squares.len() * 6;
    for i in 0..squares.len() {
        let compare = &squares[i];
        for j in i + 1..squares.len() {
            faces -= touching(compare, &squares[j]);
        }
    }
    println!("{faces}");

    Ok(())
}

fn touching(entry: &Location, compare: &Location) -> usize {
    let mut tot = 0;
    // Check x side
    if (entry.0 - compare.0).abs() == 1 && entry.1 == compare.1 && entry.2 == compare.2 {
        tot += 2;
    }
    // Check y side
    if (entry.1 - compare.1).abs() == 1 && entry.0 == compare.0 && entry.2 == compare.2 {
        tot += 2;
    }
    // Check z side
    if (entry.2 - compare.2).abs() == 1 && entry.0 == compare.0 && entry.1 == compare.1 {
        tot += 2;
    }
    tot
}
