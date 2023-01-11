//! day24 advent 2022
use crate::{Facing::*, Spot::*, Storm::*};
use clap::Parser;
use color_eyre::eyre::Result;
use num::integer::lcm;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = 10)]
    minutes: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Storm {
    Single(Facing),
    Multiple(Vec<Facing>),
}

impl fmt::Display for Storm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Single(facing) => write!(f, "{facing}"),
            Multiple(v) => write!(f, "{}", v.len()),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Facing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            North => write!(f, "^"),
            South => write!(f, "v"),
            East => write!(f, ">"),
            West => write!(f, "<"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Spot {
    Wall,
    Path,
    Blizzard(Storm),
    Expedition,
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wall => write!(f, "#"),
            Spot::Path => write!(f, "."),
            Blizzard(b) => write!(f, "{b}"),
            Expedition => write!(f, "E"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Location(usize, usize);

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want to sort by row, then column order.
        // If we derive we get column/row instead unless we reverse
        // the tuple.
        let o = self.1.cmp(&other.1);
        match o {
            std::cmp::Ordering::Less => o,
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            std::cmp::Ordering::Greater => o,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut grid = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let mut g = Vec::new();
        for b in line.as_bytes() {
            match b {
                b'#' => g.push(Wall),
                b'.' => g.push(Path),
                b'>' => g.push(Blizzard(Single(East))),
                b'<' => g.push(Blizzard(Single(West))),
                b'^' => g.push(Blizzard(Single(North))),
                b'v' => g.push(Blizzard(Single(South))),
                _ => panic!("{} - bad line {line}", line_num + 1),
            }
        }
        grid.push(g);
    }

    // Find the first open hole and put the expedition there.
    // Also find the end.
    let mut exp = Location(0, 0);
    for (pos, g) in grid[0].iter().enumerate() {
        if *g == Path {
            exp.0 = pos;
            break;
        }
    }

    let mut end = Location(0, grid.len() - 1);
    for (pos, g) in grid[grid.len() - 1].iter().enumerate() {
        if *g == Path {
            end.0 = pos;
            break;
        }
    }

    println!("Start at {exp}");
    println!("End at {end}");
    print_board(&grid, &exp);
    println!();

    // Since the blizzard paths are symetric they simply repeat overall at the
    // lcm(width,length) of that field.
    let lcm = lcm(grid.len() - 2, grid[0].len() - 2);
    println!("lcm {lcm}");

    // Use that lcm to generate all the boards we'd ever need for lookup
    let mut boards = Vec::from([grid.clone()]);
    for _ in 0..lcm - 1 {
        move_blizzard(&mut grid);
        boards.push(grid.clone());
    }

    let len = bfs(&boards, &exp, &end);
    println!("cost is {len}");
    Ok(())
}

fn bfs(boards: &Vec<Vec<Vec<Spot>>>, start: &Location, dest: &Location) -> usize {
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();

    // Use Reverse on the queue to turn this into a min based PQ. Otherwise
    // max based makes Dijkstra unhappy and won't terminate :)
    q.push(Reverse((0, start.clone())));

    // We will backtrack but we can't repeat the same location/time
    // or else we'll loop.
    seen.insert((0, start.clone()));

    let lcm = boards.len();
    while let Some(e) = q.pop() {
        let loc = e.0 .1;
        let cost = e.0 .0;
        if loc == *dest {
            return cost;
        }
        let new = cost + 1;
        let b = &boards[new % lcm];

        // If we're at the start we can only go down or stay.
        // Otherwise we can always tests N/S/E/W without worry as the walls
        // will be in the way and not under/overflow.
        let tests = if loc == *start {
            Vec::from([loc.clone(), Location(loc.0, loc.1 + 1)])
        } else {
            let x = loc.0;
            let y = loc.1;
            Vec::from([
                loc.clone(),
                Location(x + 1, y),
                Location(x - 1, y),
                Location(x, y + 1),
                Location(x, y - 1),
            ])
        };

        for t in &tests {
            // Only attempt places that have paths as everything else
            // is either a wall or blizzard.
            if b[t.1][t.0] == Path {
                if seen.insert((new, t.clone())) {
                    q.push(Reverse((new, t.clone())));
                }
            }
        }
    }

    usize::MAX
}

fn print_board(grid: &Vec<Vec<Spot>>, exp: &Location) {
    for (y, spots) in grid.iter().enumerate() {
        for (x, spot) in spots.iter().enumerate() {
            let spot = if Location(x, y) == *exp {
                &Expedition
            } else {
                spot
            };

            print!("{spot}");
        }
        println!();
    }
}

fn move_blizzard(grid: &mut Vec<Vec<Spot>>) {
    // We know this is an interior grid except the start and end holes
    // but there are no blizzards going N/S on the first/last row so those will
    // never get hit.
    let mut newgrid = grid.clone();
    // Create a copy we zero out and start moving into.
    // We'll clone this back into grid when we're done.
    for y in 0..grid.len() - 1 {
        for x in 0..grid[0].len() - 1 {
            if newgrid[y][x] != Wall {
                newgrid[y][x] = Path;
            }
        }
    }

    for y in 0..grid.len() - 1 {
        for x in 0..grid[0].len() - 1 {
            if let Blizzard(b) = &grid[y][x] {
                match b {
                    Single(s) => do_move(&mut newgrid, s, x, y),
                    Multiple(m) => {
                        for loc in m {
                            do_move(&mut newgrid, loc, x, y);
                        }
                    }
                }
            }
        }
    }

    *grid = newgrid.clone();
}

fn do_move(newgrid: &mut Vec<Vec<Spot>>, s: &Facing, x: usize, y: usize) {
    let new = match s {
        North => {
            let mut new = Location(x, y - 1);
            if newgrid[y - 1][x] == Wall {
                new = Location(x, newgrid.len() - 2);
            }
            new
        }
        South => {
            let mut new = Location(x, y + 1);
            if newgrid[y + 1][x] == Wall {
                new = Location(x, 1);
            }
            new
        }
        East => {
            let mut new = Location(x + 1, y);
            if newgrid[y][x + 1] == Wall {
                new = Location(1, y);
            }
            new
        }
        West => {
            let mut new = Location(x - 1, y);
            if newgrid[y][x - 1] == Wall {
                new = Location(newgrid[0].len() - 2, y);
            }
            new
        }
    };
    match &mut newgrid[new.1][new.0] {
        Wall => panic!(),
        Spot::Path => newgrid[new.1][new.0] = Blizzard(Single(s.clone())),

        Blizzard(b) => match b {
            Single(single) => {
                newgrid[new.1][new.0] = Blizzard(Multiple(Vec::from([single.clone(), s.clone()])))
            }
            Multiple(v) => v.push(s.clone()),
        },
        Expedition => panic!(),
    }
}
