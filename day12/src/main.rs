//! day12 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use pathfinding::prelude::astar;
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Location(usize, usize);

impl Location {
    fn distance(&self, other: &Location) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1))
            .try_into()
            .unwrap()
    }

    fn successors(&self, grid: &Vec<Vec<u8>>) -> Vec<(Self, u32)> {
        let (x, y): (i32, i32) = (self.0.try_into().unwrap(), self.1.try_into().unwrap());
        let mut v = Vec::new();
        let here = u32::from(grid[self.1][self.0]);
        for i in [(x, y + 1), (x + 1, y), (x - 1, y), (x, y - 1)] {
            if i.0 >= grid[0].len().try_into().unwrap()
                || i.1 >= grid.len().try_into().unwrap()
                || i.0 < 0
                || i.1 < 0
            {
                continue;
            }
            #[allow(clippy::cast_sign_loss)]
            let (x, y) = (i.0 as usize, i.1 as usize);
            let val = u32::from(grid[y][x]);
            // Technically this can be handled with BFS so we're just reducing to that
            // by only adding nodes we like, not all nodes.
            if val <= here + 1 {
                v.push((Self(x, y), 1));
            }
        }
        v
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut line_len = 0;
    let mut grid = Vec::new();
    let mut begin = Location(0, 0);
    let mut end = Location(0, 0);
    for (line_num, line) in lines.iter().enumerate() {
        if line_num == 0 {
            line_len = line.len();
        } else {
            assert!(line.len() == line_len, "{} - bad line {line}", line_num + 1);
        }

        let mut entry = Vec::new();
        for (pos, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'S' => {
                    begin = Location(pos, grid.len());
                    entry.push(1);
                }
                b'E' => {
                    end = Location(pos, grid.len());
                    entry.push(26);
                }
                _ => {
                    entry.push(c - b'a' + 1);
                }
            }
        }
        grid.push(entry);
    }

    // A* path from S -> E
    let res = astar(
        &begin,
        |p| p.successors(&grid),
        |p| p.distance(&end),
        |p| *p == end,
    )
    .unwrap();
    if args.debug {
        for g in &grid {
            println!("{g:?}");
        }
        println!("begin - {begin:?} end - {end:?}");
    }
    println!("part1 - {}", res.0.len() - 1);

    let mut best = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 1 {
                let res = astar(
                    &Location(x, y),
                    |p| p.successors(&grid),
                    |p| p.distance(&Location(x, y)),
                    |p| *p == end,
                );
                //println!("alt begin {x},{y} - {}", res.0.len() - 1);
                if let Some(res) = res {
                    best.push(res.0.len() - 1);
                }
            }
        }
    }
    println!("part2 - {:?}", best.iter().min().unwrap());
    Ok(())
}
