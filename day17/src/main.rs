//! day17 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::cmp::Ordering;
use std::collections::HashSet;
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

    #[arg(long, default_value_t = 2022)]
    rocks: usize,
}

#[derive(Clone, Debug, Display, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
}

#[derive(Clone, Debug, Display, PartialEq)]
enum Rock {
    HLine,
    Plus,
    Corner,
    VLine,
    Square,
}

impl Rock {
    fn next(self) -> Self {
        match self {
            Rock::HLine => Rock::Plus,
            Rock::Plus => Rock::Corner,
            Rock::Corner => Rock::VLine,
            Rock::VLine => Rock::Square,
            Rock::Square => Rock::HLine,
        }
    }

    fn height(&self) -> usize {
        match self {
            Rock::HLine => 1,
            Rock::Plus => 3,
            Rock::Corner => 3,
            Rock::VLine => 4,
            Rock::Square => 2,
        }
    }

    fn covers(&self, bottom_left: Location) -> HashSet<Location> {
        let mut v = HashSet::new();
        match self {
            Rock::HLine => {
                for x in 0..4 {
                    v.insert(Location(bottom_left.0 + x, bottom_left.1));
                }
            }
            Rock::Plus => {
                for x in 0..3 {
                    v.insert(Location(bottom_left.0 + x, bottom_left.1 + 1));
                }
                v.insert(Location(bottom_left.0 + 1, bottom_left.1));
                v.insert(Location(bottom_left.0 + 1, bottom_left.1 + 2));
            }
            Rock::Corner => {
                for x in 0..3 {
                    v.insert(Location(bottom_left.0 + x, bottom_left.1));
                }
                for y in 1..3 {
                    v.insert(Location(bottom_left.0 + 2, bottom_left.1 + y));
                }
            }
            Rock::VLine => {
                for y in 0..4 {
                    v.insert(Location(bottom_left.0, bottom_left.1 + y));
                }
            }
            Rock::Square => {
                for x in 0..2 {
                    v.insert(Location(bottom_left.0 + x, bottom_left.1));
                }
                for x in 0..2 {
                    v.insert(Location(bottom_left.0 + x, bottom_left.1 + 1));
                }
            }
        }
        v
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(usize, usize);

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            match self.0.cmp(&other.0) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        } else {
            self.1.cmp(&other.1)
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut air = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        for i in line.as_bytes() {
            match i {
                b'>' => {
                    air.push(Dir::Right);
                }
                b'<' => {
                    air.push(Dir::Left);
                }
                _ => {
                    panic!("{} - bad line {line}", line_num + 1);
                }
            }
        }
    }

    let mut filled = HashSet::new();
    for x in &[0, 8] {
        for y in 0..=4 {
            filled.insert(Location(*x, y));
        }
    }
    for x in 0..=8 {
        filled.insert(Location(x, 0));
    }
    let mut highest = 0;
    let mut start_y;
    let start_x = 3;
    let mut cur = Rock::Square;
    let mut air_pos = 0;
    print_board(&filled);
    for i in 0..args.rocks {
        cur = cur.next();
        start_y = highest + 4;
        for y in start_y..start_y + cur.height() {
            filled.insert(Location(0, y));
            filled.insert(Location(8, y));
        }

        let mut covered = cur.covers(Location(start_x, start_y));
        print_board(&filled.clone().union(&covered).cloned().collect());

        //if cur == Rock::Square {
        //    break;
        //}
        loop {
            let a = &air[air_pos];
            air_pos += 1;
            if air_pos >= air.len() {
                air_pos = 0;
            }

            // We don't care for the air direction if we moved or not.
            // Just make sure covered is accurate in case we did.
            check_move(a, &mut covered, &filled);
            print_board(&filled.clone().union(&covered).cloned().collect());

            // For moving down if it can't we know to build up.
            if !check_move(&Dir::Down, &mut covered, &filled) {
                let mut max = 0;
                filled = filled.union(&covered).cloned().collect();
                for n in filled.iter().filter_map(|l| {
                    if l.0 > 0 && l.0 < 8 && l.1 != 0 {
                        Some(l)
                    } else {
                        None
                    }
                }) {
                    if n.1 > max {
                        max = n.1
                    }
                }
                highest = max;

                println!("{i} - highest now: {highest}");
                break;
            }
            print_board(&filled.clone().union(&covered).cloned().collect());
        }
    }

    println!("highest: {highest}");
    Ok(())
}

fn check_move(a: &Dir, covered: &mut HashSet<Location>, filled: &HashSet<Location>) -> bool {
    let mut to_move = true;
    for c in covered.iter() {
        let check = match a {
            Dir::Left => Location(c.0 - 1, c.1),
            Dir::Right => Location(c.0 + 1, c.1),
            Dir::Down => Location(c.0, c.1 - 1),
        };
        if filled.contains(&check) {
            to_move = false;
            break;
        }
    }
    if to_move {
        let mut new = HashSet::new();
        for c in covered.iter() {
            let check = match a {
                Dir::Left => Location(c.0 - 1, c.1),
                Dir::Right => Location(c.0 + 1, c.1),
                Dir::Down => Location(c.0, c.1 - 1),
            };
            new.insert(check);
        }
        *covered = new;
    }
    to_move
}

fn print_board(filled: &HashSet<Location>) {
    return;
    let mut points = filled.iter().collect::<Vec<_>>();
    points.sort();
    let mut cury = usize::MAX;
    let mut curx = usize::MIN;
    for p in points.iter().cloned().rev() {
        if p.1 < cury {
            println!();
            curx = 0;
            cury = p.1;
        }
        if p.0 - curx > 1 {
            for _ in curx..p.0 - 1 {
                print!(" ");
            }
        }
        print!("#");
        curx = p.0;
    }
    println!();
    println!();
}
