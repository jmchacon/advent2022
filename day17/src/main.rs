//! day17 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::{cmp::Ordering, time::Instant};
use strum_macros::{Display, EnumCount as EnumCountMacro};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = false)]
    print_each_step: bool,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
}

#[derive(Clone, Debug, Display, EnumCountMacro, Eq, Hash, PartialEq)]
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
            Rock::Plus | Rock::Corner => 3,
            Rock::VLine => 4,
            Rock::Square => 2,
        }
    }

    fn covers(&self, bottom_left: &Location) -> HashSet<Location> {
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

    for (pos, rocks) in [2022, 1_000_000_000_000].iter().enumerate() {
        let now = Instant::now();
        let highest = compute(*rocks, args.debug, args.print_each_step, &air);
        let elapsed = now.elapsed();
        println!("part{} - {elapsed:?} highest: {highest}", pos + 1);
    }
    Ok(())
}

fn compute(iterations: usize, debug: bool, print_each: bool, air: &Vec<Dir>) -> usize {
    let mut filled = HashSet::new();
    for x in &[0, 8] {
        for y in 0..=4 {
            filled.insert(Location(*x, y));
        }
    }
    for x in 0..=8 {
        filled.insert(Location(x, 0));
    }
    print_board(debug && print_each, &filled, &HashSet::new());

    let mut highest = 0;
    let mut start_y;
    let start_x = 3;
    let mut cur = Rock::Square;
    let mut air_pos = 0;
    let mut tracked: HashMap<(Rock, usize), (usize, usize)> = HashMap::new();
    for i in 0..iterations {
        cur = cur.next();
        start_y = highest + 4;
        for y in start_y..start_y + cur.height() {
            filled.insert(Location(0, y));
            filled.insert(Location(8, y));
        }

        let mut covered = cur.covers(&Location(start_x, start_y));
        print_board(print_each, &filled, &covered);

        loop {
            let a = &air[air_pos];
            air_pos += 1;
            if air_pos >= air.len() {
                air_pos = 0;
            }

            // We have 5 rocks and some airflow pattern that will eventually repeat a pattern.
            // Technically there are many cycles contained in various places. We want a specific one.
            // Even if the air flow vector is huge there's a limit since it only has 2 cases in it.
            // So height grows less than the number of air moves we make.
            //
            // Assume if we've dropped 1000 rocks we're past the point where the original bottom
            // (which is 100% covered) is not part of our period.
            //
            // At this point starting tracking Rock and the current position in the air vec.
            // This will eventually repeat itself. Check if this is our cycle by seeing if
            // the current iteration mod period equals total iterations mod period. If that's
            // the case we have an integer number of periods to go which is easy to compute.
            // By sliding around looking for this we eventually find the magic period which
            // means the height at period start + (remaining periods * height diff) == total.
            if i > 1000 {
                let key = (cur.clone(), air_pos);
                if let std::collections::hash_map::Entry::Vacant(e) = tracked.entry(key.clone()) {
                    e.insert((i, highest));
                } else {
                    let v = tracked[&key];
                    let period = i - v.0;
                    if i % period == iterations % period {
                        if debug {
                            println!("period {period} detected iterations {i} - {}", v.0,);
                        }
                        let h = highest - v.1;
                        let remaining = iterations - i;
                        let c = (remaining / period) + 1;
                        return v.1 + (h * c);
                    }
                }
            }
            // We don't care for the air direction if we moved or not.
            // Just make sure covered is accurate in case we did.
            check_move(a, &mut covered, &filled);
            print_board(print_each, &filled, &covered);

            // For moving down if it can't we know to build up.
            if !check_move(&Dir::Down, &mut covered, &filled) {
                let mut max = 0;
                for c in &covered {
                    filled.insert(c.clone());
                }
                for n in filled.iter().filter(|l| l.0 > 0 && l.0 < 8 && l.1 != 0) {
                    if n.1 > max {
                        max = n.1;
                    }
                }
                highest = max;
                break;
            }
            print_board(print_each, &filled, &covered);
        }
    }
    highest
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

fn print_board(print: bool, filled: &HashSet<Location>, covered: &HashSet<Location>) {
    if print {
        let mut combined = filled.clone();
        combined = combined.union(covered).cloned().collect();
        let mut points = combined.iter().collect::<Vec<_>>();
        points.sort();
        let mut cur_y = usize::MAX;
        let mut cur_x = usize::MIN;
        for p in points.iter().copied().rev() {
            if p.1 < cur_y {
                println!();
                cur_x = 0;
                cur_y = p.1;
            }
            if p.0 - cur_x > 1 {
                for _ in cur_x..p.0 - 1 {
                    print!(" ");
                }
            }
            print!("#");
            cur_x = p.0;
        }
        println!();
        println!();
    }
}
