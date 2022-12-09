//! day9 advent 2022
use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
enum Moves {
    R,
    U,
    L,
    D,
}

#[derive(Debug, Display, EnumString)]
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

const SIZE: usize = 1;
const TAIL: usize = SIZE - 1;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hm = HashSet::new();
    let mut cur = Vec::new();
    let mut adj = Vec::new();
    for _ in 0..SIZE {
        adj.push(Adjacent::On);
        cur.push(Location(0, 0));
    }
    hm.insert(cur[TAIL].clone());

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
        let moves = usize::from_str_radix(parts[1], 10).unwrap();
        for _ in 0..moves {
            process(0, &mut adj, &dir, &mut cur);
            //println!("{line} - {:?} {:?}", adj, cur);
            hm.insert(cur[TAIL].clone());
        }
    }
    println!("moves: {}", hm.len());
    Ok(())
}

fn process(i: usize, adj: &mut Vec<Adjacent>, dir: &Moves, cur: &mut Vec<Location>) {
    if i > TAIL {
        return;
    }
    match (&adj[i], dir) {
        (Adjacent::On, Moves::R) => {
            adj[i] = Adjacent::W;
        }
        (Adjacent::On, Moves::L) => {
            adj[i] = Adjacent::E;
        }
        (Adjacent::On, Moves::U) => {
            adj[i] = Adjacent::S;
        }
        (Adjacent::On, Moves::D) => {
            adj[i] = Adjacent::N;
        }

        (Adjacent::W, Moves::R) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::W, Moves::L) => {
            adj[i] = Adjacent::On;
        }
        (Adjacent::W, Moves::U) => {
            adj[i] = Adjacent::SW;
        }
        (Adjacent::W, Moves::D) => {
            adj[i] = Adjacent::NW;
        }

        (Adjacent::E, Moves::R) => {
            adj[i] = Adjacent::On;
        }
        (Adjacent::E, Moves::L) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::E, Moves::U) => {
            adj[i] = Adjacent::SE;
        }
        (Adjacent::E, Moves::D) => {
            adj[i] = Adjacent::NE;
        }

        (Adjacent::S, Moves::R) => {
            adj[i] = Adjacent::SW;
        }
        (Adjacent::S, Moves::L) => {
            adj[i] = Adjacent::SE;
        }
        (Adjacent::S, Moves::U) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0, cur[i].1 + 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::S, Moves::D) => {
            adj[i] = Adjacent::On;
        }

        (Adjacent::N, Moves::R) => {
            adj[i] = Adjacent::NW;
        }
        (Adjacent::N, Moves::L) => {
            adj[i] = Adjacent::NE;
        }
        (Adjacent::N, Moves::U) => {
            adj[i] = Adjacent::On;
        }
        (Adjacent::N, Moves::D) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0, cur[i].1 - 1);
            process(i + 1, adj, dir, cur);
        }

        (Adjacent::SE, Moves::R) => {
            adj[i] = Adjacent::S;
        }
        (Adjacent::SE, Moves::L) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 + 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::SE, Moves::U) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 + 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::SE, Moves::D) => {
            adj[i] = Adjacent::E;
        }

        (Adjacent::SW, Moves::R) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 + 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::SW, Moves::L) => {
            adj[i] = Adjacent::S;
        }
        (Adjacent::SW, Moves::U) => {
            adj[i] = Adjacent::S;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 + 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::SW, Moves::D) => {
            adj[i] = Adjacent::W;
        }

        (Adjacent::NE, Moves::R) => {
            adj[i] = Adjacent::N;
        }
        (Adjacent::NE, Moves::L) => {
            adj[i] = Adjacent::E;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 - 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::NE, Moves::U) => {
            adj[i] = Adjacent::E;
        }
        (Adjacent::NE, Moves::D) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0 - 1, cur[i].1 - 1);
            process(i + 1, adj, dir, cur);
        }

        (Adjacent::NW, Moves::R) => {
            adj[i] = Adjacent::W;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 - 1);
            process(i + 1, adj, dir, cur);
        }
        (Adjacent::NW, Moves::L) => {
            adj[i] = Adjacent::N;
        }
        (Adjacent::NW, Moves::U) => {
            adj[i] = Adjacent::W;
        }
        (Adjacent::NW, Moves::D) => {
            adj[i] = Adjacent::N;
            cur[i] = Location(cur[i].0 + 1, cur[i].1 - 1);
            process(i + 1, adj, dir, cur);
        }
    }
}
