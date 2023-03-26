//! day14 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
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

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = false)]
    draw: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i32, i32);

#[derive(Clone, Debug, Display, PartialEq)]
enum State {
    Infinity,
    Stopped,
    Falling,
}

#[derive(Clone, Debug, Display, PartialEq)]
enum Type {
    Rock,
    Sand,
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hm = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        parse_line(&parts, &mut hm, line, line_num)?;
    }
    for (pos, infinity) in [false, true].iter().enumerate() {
        let mut hm = hm.clone();
        let mut max_x = i32::MIN;
        let mut minx = i32::MAX;
        let mut max_y = i32::MIN;
        for l in hm.keys() {
            if l.0 > max_x {
                max_x = l.0;
            }
            if l.0 < minx {
                minx = l.0;
            }
            if l.1 > max_y {
                max_y = l.1;
            }
        }
        let bot = max_y + 2;
        if args.debug {
            println!("infinity: {infinity}");
            println!("{minx} -> {max_x} | {max_y} + bot {bot}");
            println!("{}x{}", max_x - minx + 1, bot);
        }
        let mut sand = 0;
        let mut state = State::Falling;
        let mut steps = 0;
        loop {
            sand += 1;
            let mut cur = Location(500, 0);
            _ = args.debug && args.draw && print_board(&hm, &cur, minx, max_x, max_y);
            if hm.contains_key(&cur) {
                if args.debug {
                    println!("blocked at source?");
                }
                break;
            }

            loop {
                steps += 1;
                if !*infinity && (cur.0 < minx || cur.0 > max_x || cur.1 > max_y) {
                    if args.debug {
                        println!("inf cur - {cur:?}");
                    }
                    state = State::Infinity;
                    break;
                }
                cur = Location(cur.0, cur.1 + 1);
                // Straight down ok, continue
                if !hm.contains_key(&cur) {
                    if *infinity && cur.1 == bot {
                        cur = Location(cur.0, cur.1 - 1);
                        state = State::Stopped;
                        break;
                    }
                    _ = args.debug && args.draw && print_board(&hm, &cur, minx, max_x, max_y);
                    continue;
                }
                // Try left
                cur = Location(cur.0 - 1, cur.1);
                if !hm.contains_key(&cur) {
                    _ = args.debug && args.draw && print_board(&hm, &cur, minx, max_x, max_y);
                    continue;
                }
                // Try right - if not we're stuck.
                cur = Location(cur.0 + 2, cur.1);
                if hm.contains_key(&cur) {
                    cur = Location(cur.0 - 1, cur.1 - 1);
                    state = State::Stopped;
                    break;
                }
            }
            if state == State::Infinity {
                break;
            }
            _ = args.debug && args.draw && print_board(&hm, &cur, minx, max_x, max_y);
            hm.insert(cur, Type::Sand);
        }
        // Counted infinity particle
        sand -= 1;
        if args.debug {
            println!("state - {state}");
            println!("steps - {steps}");
        }
        println!("part{} - sand - {sand}", pos + 1);
    }
    Ok(())
}

fn parse_line(
    parts: &Vec<&str>,
    hm: &mut HashMap<Location, Type>,
    line: &str,
    line_num: usize,
) -> Result<()> {
    assert!(parts.len() >= 2, "{} - bad line {line}", line_num + 1);
    let old = parts[0].split(',').collect::<Vec<_>>();
    assert!(old.len() == 2, "{} - bad line {line}", line_num + 1);
    let mut old_x = old[0].parse::<i32>()?;
    let mut old_y = old[1].parse::<i32>()?;

    for p in &parts[1..] {
        let xy = p.split(',').collect::<Vec<_>>();
        assert!(xy.len() == 2, "{} - bad line {line}", line_num + 1);
        let x = xy[0].parse::<i32>()?;
        let y = xy[1].parse::<i32>()?;
        match (x == old_x, y == old_y) {
            (true, true) | (false, false) => {
                panic!("{} - bad line {line}", line_num + 1);
            }
            (true, false) => {
                let (l, h);
                if old_y < y {
                    (l, h) = (old_y, y);
                } else {
                    (l, h) = (y, old_y);
                }
                for d in l..=h {
                    hm.insert(Location(x, d), Type::Rock);
                }
            }
            (false, true) => {
                let (l, h);
                if old_x < x {
                    (l, h) = (old_x, x);
                } else {
                    (l, h) = (x, old_x);
                }
                for d in l..=h {
                    hm.insert(Location(d, y), Type::Rock);
                }
            }
        }
        old_x = x;
        old_y = y;
    }
    Ok(())
}

fn print_board(
    hm: &HashMap<Location, Type>,
    cur: &Location,
    minx: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    for y in 0..=max_y {
        for x in 0..=max_x - minx {
            let mut print;
            let check = Location(x + minx, y);
            if hm.contains_key(&check) {
                match hm.get(&check).unwrap() {
                    Type::Rock => {
                        print = "#";
                    }
                    Type::Sand => {
                        print = "o";
                    }
                }
            } else {
                print = ".";
            }

            if &check == cur {
                print = "o";
            }
            print!("{print}");
        }
        println!();
    }
    println!();
    true
}
