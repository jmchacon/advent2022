//! day24 advent 2022
use crate::{
    Facing::{East, North, South, West},
    Spot::{Blizzard, Expedition, Path, Wall},
    Storm::{Multiple, Single},
};
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use num::integer::lcm;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path as StdPath;
use std::{fmt, iter};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
enum Spot {
    Wall,
    #[default]
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = StdPath::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut grid = Grid::<Spot>::new(lines[0].len(), lines.len());

    for (line_num, line) in lines.iter().enumerate() {
        for (pos, b) in line.as_bytes().iter().enumerate() {
            let l = Location(pos.try_into()?, line_num.try_into()?);
            match b {
                b'#' => grid.add(&l, Wall),
                b'.' => grid.add(&l, Path),
                b'>' => grid.add(&l, Blizzard(Single(East))),
                b'<' => grid.add(&l, Blizzard(Single(West))),
                b'^' => grid.add(&l, Blizzard(Single(North))),
                b'v' => grid.add(&l, Blizzard(Single(South))),
                _ => panic!("{} - bad line {line}", line_num + 1),
            };
        }
    }

    // Find the first open hole and put the expedition there.
    // Also find the end.
    let mut exp = Location(0, 0);
    for x in 0..grid.width() {
        if *grid.get(&Location(x.try_into()?, 0)) == Path {
            exp.0 = x.try_into()?;
            break;
        }
    }

    let last_row: isize = (grid.height() - 1).try_into()?;
    let mut end = Location(0, last_row);
    for x in 0..grid.width() {
        if *grid.get(&Location(x.try_into()?, last_row)) == Path {
            end.0 = x.try_into()?;
            break;
        }
    }

    if args.debug {
        println!("Start at {exp}");
        println!("End at {end}");
        print_board(&grid, &exp)?;
        println!();
    }

    // Since the blizzard paths are symetric they simply repeat overall at the
    // lcm(width,length) of that field.
    let lcm = lcm(grid.height() - 2, grid.width() - 2);
    if args.debug {
        println!("lcm {lcm}");
    }

    // Use that lcm to generate all the boards we'd ever need for lookup
    let mut boards = Vec::from([grid.clone()]);
    for _ in 0..lcm - 1 {
        move_blizzard(&mut grid)?;
        boards.push(grid.clone());
    }

    let len = bfs(&boards, 0, &exp, &end);
    println!("part1 - cost is {len}");
    let len = bfs(&boards, len, &end, &exp);
    if args.debug {
        println!("cost2 is {len}");
    }
    let len = bfs(&boards, len, &exp, &end);
    println!("part2 - cost3 is {len}");

    Ok(())
}

fn bfs(boards: &Vec<Grid<Spot>>, len: usize, start: &Location, dest: &Location) -> usize {
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();

    // Use Reverse on the queue to turn this into a min based PQ. Otherwise
    // max based makes Dijkstra sad :)
    q.push(Reverse((len, start.distance(dest), start.clone())));

    // We will backtrack but we can't repeat the same location+path length
    // or else we'll loop.
    seen.insert((len, start.clone()));

    let lcm = boards.len();
    while let Some(e) = q.pop() {
        let path_len = e.0 .0;
        // Distance is e.1 but we don't need it to compute anything.
        // It's in there so sorting for the queue uses it (a star).
        let loc = e.0 .2;
        if loc == *dest {
            return path_len;
        }
        let new = path_len + 1;
        let b = &boards[new % lcm];

        // If we're at the edge then neighbors() will filter for us.
        // Then all that remains is removing walls.
        let neighbors = b
            .neighbors(&loc)
            .iter()
            .chain(iter::once(&(loc.clone(), &Expedition))) // Add the current location in also as we might just wait
            .filter(|x| *x.1 != Wall)
            .cloned()
            .collect::<Vec<(Location, &Spot)>>();

        for t in &neighbors {
            // Only attempt places that have paths as everything else
            // is either a wall or blizzard.
            if *b.get(&t.0) == Path && seen.insert((new, t.0.clone())) {
                q.push(Reverse((new, t.0.distance(dest), t.0.clone())));
            }
        }
    }

    // Somehow didn't find the dest...
    usize::MAX
}

fn print_board(grid: &Grid<Spot>, exp: &Location) -> Result<()> {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let spot = if Location(x.try_into()?, y.try_into()?) == *exp {
                &Expedition
            } else {
                grid.get(&Location(x.try_into()?, y.try_into()?))
            };

            print!("{spot}");
        }
        println!();
    }
    Ok(())
}

fn move_blizzard(grid: &mut Grid<Spot>) -> Result<()> {
    // We know this is an interior grid except the start and end holes
    // but there are no blizzards going N/S on the first/last row so those will
    // never get hit.
    let mut newgrid = grid.clone();

    // Create a copy we zero out and start moving into.
    // We'll clone this back into grid when we're done.
    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() - 1 {
            let (x, y) = (x.try_into()?, y.try_into()?);
            if *newgrid.get(&Location(x, y)) != Wall {
                newgrid.add(&Location(x, y), Path);
            }
        }
    }

    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() - 1 {
            let (x, y) = (x.try_into()?, y.try_into()?);
            if let Blizzard(b) = grid.get(&Location(x, y)) {
                match b {
                    Single(s) => do_move(&mut newgrid, s, x, y)?,
                    Multiple(m) => {
                        for loc in m {
                            do_move(&mut newgrid, loc, x, y)?;
                        }
                    }
                }
            }
        }
    }

    *grid = newgrid.clone();
    Ok(())
}

fn do_move(newgrid: &mut Grid<Spot>, s: &Facing, x: isize, y: isize) -> Result<()> {
    let new = match s {
        North => {
            let mut new = Location(x, y - 1);
            if *newgrid.get(&new) == Wall {
                new = Location(x, (newgrid.height() - 2).try_into()?);
            }
            new
        }
        South => {
            let mut new = Location(x, y + 1);
            if *newgrid.get(&new) == Wall {
                new = Location(x, 1);
            }
            new
        }
        East => {
            let mut new = Location(x + 1, y);
            if *newgrid.get(&new) == Wall {
                new = Location(1, y);
            }
            new
        }
        West => {
            let mut new = Location(x - 1, y);
            if *newgrid.get(&new) == Wall {
                new = Location((newgrid.width() - 2).try_into()?, y);
            }
            new
        }
    };
    match newgrid.get_mut(&new) {
        Wall | Expedition => panic!(),
        Spot::Path => newgrid.add(&new, Blizzard(Single(s.clone()))),

        Blizzard(blz) => match blz {
            Single(single) => {
                // Have to move it here as inside of Vec::from it claims we're borrowing
                // twice.
                let new_facing = single.clone();
                newgrid.add(&new, Blizzard(Multiple(Vec::from([new_facing, s.clone()]))));
            }
            Multiple(val) => val.push(s.clone()),
        },
    }
    Ok(())
}
