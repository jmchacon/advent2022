//! day9 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::{Display, EnumString};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Debug, Display, EnumString)]
enum Moves {
    R,
    U,
    L,
    D,
    NE,
    SE,
    NW,
    SW,
}

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
enum Adjacent {
    On,
    W,
    E,
    S,
    N,
    SE,
    SW,
    NE,
    NW,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i32, i32);

const PART1: usize = 1;
const PART2: usize = 9;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for (pos, (size, tail)) in [(PART1, PART1 - 1), (PART2, PART2 - 1)].iter().enumerate() {
        let mut hm = HashSet::new();
        let mut cur = Vec::new();
        let mut adj = Vec::new();
        // Add an extra so we can overflow setup in recursion later w/o panic.
        // slight memory usage.
        for _ in 0..=*size {
            adj.push(Adjacent::On);
            cur.push(Location(0, 0));
        }
        hm.insert(cur[*tail].clone());

        for (line_num, line) in lines.iter().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts.len() == 2, "{} - bad line {line}", line_num + 1);
            let dir = match parts[0].as_bytes() {
                b"R" => Moves::R,
                b"U" => Moves::U,
                b"L" => Moves::L,
                b"D" => Moves::D,
                _ => {
                    panic!("{} - bad line {line}", line_num + 1);
                }
            };
            let moves = parts[1].parse::<usize>()?;
            for _ in 0..moves {
                process(0, &mut adj, &dir, &mut cur, *tail);
                if args.debug {
                    println!("{line} - {adj:?} {cur:?}");
                }
                hm.insert(cur[*tail].clone());
            }
        }
        println!("part{}: {}", pos + 1, hm.len());
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn process(i: usize, adj: &mut Vec<Adjacent>, dir: &Moves, cur: &mut Vec<Location>, tail: usize) {
    if i > tail {
        return;
    }
    match (&adj[i], dir) {
        (Adjacent::On, Moves::L)
        | (Adjacent::S, Moves::SW)
        | (Adjacent::N, Moves::NW)
        | (Adjacent::SE, Moves::D)
        | (Adjacent::NE, Moves::U) => {
            adj[i] = Adjacent::E;
        }
        (Adjacent::On, Moves::U)
        | (Adjacent::SW, Moves::L)
        | (Adjacent::SE, Moves::R)
        | (Adjacent::E, Moves::NE)
        | (Adjacent::W, Moves::NW) => {
            adj[i] = Adjacent::S;
        }
        (Adjacent::On, Moves::D)
        | (Adjacent::NW, Moves::L)
        | (Adjacent::NE, Moves::R)
        | (Adjacent::E, Moves::SE)
        | (Adjacent::W, Moves::SW) => {
            adj[i] = Adjacent::N;
        }
        (Adjacent::On, Moves::NE) | (Adjacent::S, Moves::R) | (Adjacent::W, Moves::U) => {
            adj[i] = Adjacent::SW;
        }
        (Adjacent::On, Moves::SE) | (Adjacent::N, Moves::R) | (Adjacent::W, Moves::D) => {
            adj[i] = Adjacent::NW;
        }
        (Adjacent::On, Moves::NW) | (Adjacent::S, Moves::L) | (Adjacent::E, Moves::U) => {
            adj[i] = Adjacent::SE;
        }
        (Adjacent::On, Moves::SW) | (Adjacent::N, Moves::L) | (Adjacent::E, Moves::D) => {
            adj[i] = Adjacent::NE;
        }

        (Adjacent::W, Moves::R) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1);
            process(i + 1, adj, dir, cur, tail);
        }
        (Adjacent::W, Moves::L)
        | (Adjacent::NW, Moves::NW)
        | (Adjacent::NE, Moves::NE)
        | (Adjacent::SW, Moves::SW)
        | (Adjacent::SE, Moves::SE)
        | (Adjacent::N, Moves::U)
        | (Adjacent::S, Moves::D)
        | (Adjacent::E, Moves::R) => {
            adj[i] = Adjacent::On;
        }
        (Adjacent::W, Moves::NE) | (Adjacent::SW, Moves::R) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 + 1);
            process(i + 1, adj, &Moves::NE, cur, tail);
        }
        (Adjacent::W, Moves::SE) | (Adjacent::NW, Moves::R) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 - 1);
            process(i + 1, adj, &Moves::SE, cur, tail);
        }

        (Adjacent::E, Moves::L) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1);
            process(i + 1, adj, dir, cur, tail);
        }
        (Adjacent::E, Moves::NW) | (Adjacent::SE, Moves::L) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 + 1);
            process(i + 1, adj, &Moves::NW, cur, tail);
        }
        (Adjacent::E, Moves::SW) | (Adjacent::NE, Moves::L) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 - 1);
            process(i + 1, adj, &Moves::SW, cur, tail);
        }

        (Adjacent::S, Moves::U) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0, cur[i].1 + 1);
            process(i + 1, adj, dir, cur, tail);
        }
        (Adjacent::S, Moves::NE) | (Adjacent::SW, Moves::U) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 + 1);
            process(i + 1, adj, &Moves::NE, cur, tail);
        }
        (Adjacent::S, Moves::SE)
        | (Adjacent::NW, Moves::U)
        | (Adjacent::SW, Moves::D)
        | (Adjacent::N, Moves::NE)
        | (Adjacent::On, Moves::R) => {
            adj[i] = Adjacent::W;
        }
        (Adjacent::S, Moves::NW) | (Adjacent::SE, Moves::U) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 + 1);
            process(i + 1, adj, &Moves::NW, cur, tail);
        }

        (Adjacent::N, Moves::D) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0, cur[i].1 - 1);
            process(i + 1, adj, dir, cur, tail);
        }
        (Adjacent::N, Moves::SE) | (Adjacent::NW, Moves::D) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 - 1);
            process(i + 1, adj, &Moves::SE, cur, tail);
        }
        (Adjacent::N, Moves::SW) | (Adjacent::NE, Moves::D) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 - 1);
            process(i + 1, adj, &Moves::SW, cur, tail);
        }

        (Adjacent::SE, Moves::NE) | (Adjacent::SW, Moves::NW) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0, cur[i].1 + 1);
            process(i + 1, adj, &Moves::U, cur, tail);
        }
        (Adjacent::SE, Moves::NW) => {
            adj[i] = Adjacent::SE;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 + 1);
            process(i + 1, adj, &Moves::NW, cur, tail);
        }
        (Adjacent::SE, Moves::SW) | (Adjacent::NE, Moves::NW) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1);
            process(i + 1, adj, &Moves::L, cur, tail);
        }

        (Adjacent::SW, Moves::NE) => {
            adj[i] = Adjacent::SW;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 + 1);
            process(i + 1, adj, &Moves::NE, cur, tail);
        }
        (Adjacent::SW, Moves::SE) | (Adjacent::NW, Moves::NE) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1);
            process(i + 1, adj, &Moves::R, cur, tail);
        }

        (Adjacent::NE, Moves::SE) | (Adjacent::NW, Moves::SW) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0, cur[i].1 - 1);
            process(i + 1, adj, &Moves::D, cur, tail);
        }
        (Adjacent::NE, Moves::SW) => {
            adj[i] = Adjacent::NE;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 - 1);
            process(i + 1, adj, &Moves::SW, cur, tail);
        }

        (Adjacent::NW, Moves::SE) => {
            adj[i] = Adjacent::NW;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 - 1);
            process(i + 1, adj, &Moves::SE, cur, tail);
        }
    }
}
