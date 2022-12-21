//! day20 advent 2022
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

    #[arg(long, default_value_t = 1)]
    scale: i64,

    #[arg(long, default_value_t = 1)]
    rounds: usize,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut input = Vec::new();
    for line in &lines {
        //println!("{line}");
        let val = i64::from_str_radix(line, 10).unwrap();
        input.push(val * args.scale);
    }
    let mut indexes = Vec::new();
    for i in 0..input.len() {
        indexes.push(i);
    }
    println!("{input:?}");
    // We're modding indexes which are 0 based, not 1 so subtract 1.
    let len = indexes.len() as i64 - 1;
    for _ in 0..args.rounds {
        for (pos, i) in input.iter().enumerate() {
            let i = i; // % len;

            let cur = indexes[pos];

            if *i == 0 {
                continue;
            }

            let mut new = cur as i64 + i;
            // This mess deals with going negative.
            // Move into the right range then add and mod off to get positive if needed.
            new = ((new % len) + len) % len;

            //println!("Moving {i} to {new}");
            indexes[pos] = new as usize;

            for i in 0..indexes.len() {
                // If it moved right
                if i != pos && indexes[i] >= cur && indexes[i] <= new as usize {
                    /*println!(
                        "{i},{pos} Shifting {} from {} to {}",
                        input[i],
                        indexes[i],
                        indexes[i] - 1
                    );*/
                    indexes[i] -= 1;
                    continue;
                }
                if i != pos && indexes[i] >= new as usize && indexes[i] < cur {
                    /*println!(
                        "{i},{pos} Shifting {} from {} to {}",
                        input[i],
                        indexes[i],
                        indexes[i] + 1
                    );*/
                    indexes[i] += 1;
                }
            }
            //print_indexes(&input, &indexes);
        }
    }
    let f = print_indexes(&input, &indexes);
    let mut pos = 0;
    for i in &input {
        if *i == 0 {
            break;
        }
        pos += 1;
    }
    let zero = indexes[pos];
    let thousand = f[(zero + 1000) % indexes.len()];
    let two_thousand = f[(zero + 2000) % indexes.len()];
    let three_thousand = f[(zero + 3000) % indexes.len()];
    println!("0 at index {}", zero);
    println!("1000 is {}", thousand);
    println!("2000 is {}", two_thousand);
    println!("3000 is {}", three_thousand);
    println!("sum is {}", thousand + two_thousand + three_thousand);
    Ok(())
}

fn print_indexes(input: &Vec<i64>, indexes: &Vec<usize>) -> Vec<i64> {
    let mut fixed = Vec::new();
    fixed.resize(indexes.len(), 0);
    for (pos, i) in indexes.iter().enumerate() {
        fixed[*i] = input[pos];
    }
    println!("{fixed:?}");
    fixed
}
