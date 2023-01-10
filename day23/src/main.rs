//! day23 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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

    #[arg(long, default_value_t = 10)]
    rounds: usize,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Location(i64, i64);

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

    let rules = Vec::from([Facing::North, Facing::South, Facing::West, Facing::East]);
    let mut map = HashSet::new();
    for (line_num, line) in lines.iter().enumerate() {
        for (pos, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => {
                    map.insert(Location(
                        pos.try_into().unwrap(),
                        line_num.try_into().unwrap(),
                    ));
                }
                b'.' => {}
                _ => panic!("{} - bad line {line}", line_num + 1),
            }
        }
    }

    let orig = map.clone();
    print_board(0, &map);

    let rounds = run_rounds(args.rounds, &mut map, &rules);
    let (mut minx, mut maxx, mut miny, mut maxy) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    for m in &map {
        if m.0 < minx {
            minx = m.0;
        }
        if m.0 > maxx {
            maxx = m.0;
        }
        if m.1 < miny {
            miny = m.1;
        }
        if m.1 > maxy {
            maxy = m.1;
        }
    }
    println!("rounds = {rounds}");
    println!("{minx}-{maxx} x {miny}-{maxy}");
    let area = ((maxx - minx + 1) * (maxy - miny + 1)) - map.len() as i64;
    println!("area = {area}");

    map = orig.clone();
    let rounds = run_rounds(usize::MAX, &mut map, &rules);
    println!("No movement on round {rounds}");
    Ok(())
}

fn run_rounds(rounds: usize, map: &mut HashSet<Location>, rules: &Vec<Facing>) -> usize {
    let mut start_rule = 0;
    for r in 0..rounds {
        let mut proposed = HashMap::new();
        println!("Starting with rule {}", rules[start_rule]);
        for l in map.iter() {
            let mut has_neighbors = false;
            for tests in [
                &Location(l.0 - 1, l.1),     // W
                &Location(l.0 + 1, l.1),     // E
                &Location(l.0, l.1 - 1),     // N
                &Location(l.0, l.1 + 1),     // S
                &Location(l.0 - 1, l.1 - 1), // NW
                &Location(l.0 - 1, l.1 + 1), // SW
                &Location(l.0 + 1, l.1 - 1), // NE
                &Location(l.0 + 1, l.1 + 1), // SE
            ] {
                if map.contains(tests) {
                    has_neighbors = true;
                    break;
                }
            }
            // Don't move if you're alone.
            if !has_neighbors {
                continue;
            }
            for r in 0..rules.len() {
                let i = (start_rule + r) % rules.len();
                match rules[i] {
                    Facing::North => {
                        if test_dir(
                            &map,
                            l,
                            [
                                &Location(l.0, l.1 - 1),
                                &Location(l.0 - 1, l.1 - 1),
                                &Location(l.0 + 1, l.1 - 1),
                            ],
                            Location(l.0, l.1 - 1),
                            &mut proposed,
                        ) {
                            break;
                        }
                    }
                    Facing::South => {
                        if test_dir(
                            &map,
                            l,
                            [
                                &Location(l.0, l.1 + 1),
                                &Location(l.0 - 1, l.1 + 1),
                                &Location(l.0 + 1, l.1 + 1),
                            ],
                            Location(l.0, l.1 + 1),
                            &mut proposed,
                        ) {
                            break;
                        }
                    }
                    Facing::West => {
                        if test_dir(
                            &map,
                            l,
                            [
                                &Location(l.0 - 1, l.1),
                                &Location(l.0 - 1, l.1 - 1),
                                &Location(l.0 - 1, l.1 + 1),
                            ],
                            Location(l.0 - 1, l.1),
                            &mut proposed,
                        ) {
                            break;
                        }
                    }
                    Facing::East => {
                        if test_dir(
                            &map,
                            l,
                            [
                                &Location(l.0 + 1, l.1),
                                &Location(l.0 + 1, l.1 - 1),
                                &Location(l.0 + 1, l.1 + 1),
                            ],
                            Location(l.0 + 1, l.1),
                            &mut proposed,
                        ) {
                            break;
                        }
                    }
                }
            }
        }

        if proposed.len() == 0 {
            return r + 1;
        }
        for p in &proposed {
            if p.1.len() == 1 {
                map.remove(&p.1[0]);
                map.insert(p.0.clone());
            }
        }
        start_rule += 1;
        start_rule %= rules.len();
        print_board(r + 1, &map);
    }
    rounds
}

fn print_board(round: usize, map: &HashSet<Location>) {
    println!("round {round}");
    for y in -2..=9 {
        for x in -3..=10 {
            if map.contains(&Location(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn test_dir(
    map: &HashSet<Location>,
    l: &Location,
    tests: [&Location; 3],
    insert: Location,
    proposed: &mut HashMap<Location, Vec<Location>>,
) -> bool {
    if !map.contains(tests[0]) && !map.contains(tests[1]) && !map.contains(tests[2]) {
        proposed
            .entry(insert)
            .and_modify(|v| v.push(l.clone()))
            .or_insert(vec![l.clone()]);
        return true;
    }
    false
}
