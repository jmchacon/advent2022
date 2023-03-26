//! day5 advent 2022
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

    let mut stacks = vec![
        vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
        vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
        vec!['L', 'Q', 'V'],
        vec!['N', 'B', 'S', 'W', 'R', 'Q'],
        vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
        vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
        vec!['D', 'B', 'Q', 'J'],
        vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
        vec!['B', 'N', 'H', 'M', 'S'],
    ];
    let mut stacks9001 = vec![
        vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
        vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
        vec!['L', 'Q', 'V'],
        vec!['N', 'B', 'S', 'W', 'R', 'Q'],
        vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
        vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
        vec!['D', 'B', 'Q', 'J'],
        vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
        vec!['B', 'N', 'H', 'M', 'S'],
    ];
    for (line_num, l) in lines.iter().enumerate() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.is_empty() || parts[0] != "move" {
            if args.debug {
                println!("skipping - {l}");
            }
            continue;
        }
        assert!(parts.len() == 6, "{} - bad line - {l}", line_num + 1);
        let num = parts[1].parse::<u32>()?;
        let src = parts[3].parse::<usize>()?;
        let dest = parts[5].parse::<usize>()?;

        let mut t = Vec::<char>::new();
        for i in 0..num {
            let v = stacks[src - 1].pop().unwrap();
            if args.debug {
                println!("{} - {l} - {i} {src} {dest}", line_num + 1);
            }
            stacks[dest - 1].push(v);
            let v = stacks9001[src - 1].pop().unwrap();
            t.push(v);
        }
        t.reverse();
        for v in t {
            stacks9001[dest - 1].push(v);
        }
    }
    print!("part1 - ");
    for mut v in stacks {
        print!("{}", v.pop().unwrap());
    }
    println!();
    print!("part2 - ");
    for mut v in stacks9001 {
        print!("{}", v.pop().unwrap());
    }
    println!();
    Ok(())
}
