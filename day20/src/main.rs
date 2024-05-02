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

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for (part, (scale, rounds)) in [(1, 1), (811_589_153, 10)].iter().enumerate() {
        let mut input = Vec::new();
        for line in &lines {
            let val = line.parse::<i64>()?;
            input.push(val * *scale);
        }
        let mut indexes = Vec::new();
        for i in 0..input.len() {
            indexes.push(i);
        }
        if args.debug {
            println!("{input:?}");
        }
        // We're modding indexes which are 0 based, not 1 so subtract 1.
        let len = std::convert::TryInto::<i64>::try_into(indexes.len()).unwrap() - 1;
        for _ in 0..*rounds {
            for (pos, i) in input.iter().enumerate() {
                let cur = indexes[pos];

                if *i == 0 {
                    continue;
                }

                let mut new = std::convert::TryInto::<i64>::try_into(cur).unwrap() + i;
                // This mess deals with going negative.
                // Move into the right range then add and mod off to get positive if needed.
                new = ((new % len) + len) % len;

                // Now back to something we can assign.
                let a: usize = new.try_into()?;
                indexes[pos] = a;

                for (i, v) in indexes.iter_mut().enumerate() {
                    // If it moved right
                    if i != pos && *v >= cur && *v <= new.try_into()? {
                        *v -= 1;
                        continue;
                    }
                    if i != pos && *v >= new.try_into()? && *v < cur {
                        *v += 1;
                    }
                }
            }
        }
        let f = make_indexes(&input, &indexes);
        if args.debug {
            println!("{f:?}");
        }
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
        if args.debug {
            println!("0 at index {zero}");
            println!("1000 is {thousand}");
            println!("2000 is {two_thousand}");
            println!("3000 is {three_thousand}");
        }
        println!(
            "part{} - {}",
            part + 1,
            thousand + two_thousand + three_thousand
        );
    }
    Ok(())
}

fn make_indexes(input: &[i64], indexes: &Vec<usize>) -> Vec<i64> {
    let mut fixed = vec![0; indexes.len()];
    for (pos, i) in indexes.iter().enumerate() {
        fixed[*i] = input[pos];
    }
    fixed
}
