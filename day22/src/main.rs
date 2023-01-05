//! day22 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Forest {
    Void,
    Path,
    Wall,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Move {
    Steps(usize),
    Left,
    Right,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut forest = Vec::new();
    let mut forest_done = false;
    let mut path = "";
    let mut moves = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            forest_done = true;
            continue;
        }
        if forest_done {
            path = line;
            let mut start = 0;
            for (pos, c) in line.bytes().enumerate() {
                match c {
                    b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {}
                    b'R' | b'L' => {
                        let m = usize::from_str_radix(&line[start..pos], 10).unwrap();
                        start = pos + 1;
                        moves.push(Move::Steps(m));
                        if c == b'R' {
                            moves.push(Move::Right);
                        } else {
                            moves.push(Move::Left);
                        }
                    }
                    _ => panic!("{} - bad line {line}", line_num + 1),
                }
            }
            // Get any trailing number.
            if start != line.len() {
                let m = usize::from_str_radix(&line[start..line.len()], 10).unwrap();
                moves.push(Move::Steps(m));
            }
            break;
        }
        let mut l = vec![Forest::Void; line.len()];
        for (pos, c) in line.bytes().enumerate() {
            match c {
                b' ' => {}
                b'.' => l[pos] = Forest::Path,
                b'#' => l[pos] = Forest::Wall,
                _ => panic!("{} - bad line {line}", line_num + 1),
            }
        }
        forest.push(l);
    }
    for f in &forest {
        println!("{f:?}");
    }
    println!("{path}");
    for m in &moves {
        println!("{m:?}");
    }
    Ok(())
}
