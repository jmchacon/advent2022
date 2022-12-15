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
    infinity: bool,

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
        assert!(parts.len() >= 2, "{} - bad line {line}", line_num + 1);
        let old = parts[0].split(",").collect::<Vec<_>>();
        assert!(old.len() == 2, "{} - bad line {line}", line_num + 1);
        let mut oldx = i32::from_str_radix(old[0], 10).unwrap();
        let mut oldy = i32::from_str_radix(old[1], 10).unwrap();

        for p in &parts[1..] {
            let xy = p.split(",").collect::<Vec<_>>();
            assert!(xy.len() == 2, "{} - bad line {line}", line_num + 1);
            let x = i32::from_str_radix(xy[0], 10).unwrap();
            let y = i32::from_str_radix(xy[1], 10).unwrap();
            match (x == oldx, y == oldy) {
                (true, true) | (false, false) => {
                    panic!("{} - bad line {line}", line_num + 1);
                }
                (true, false) => {
                    let (l, h);
                    if oldy < y {
                        (l, h) = (oldy, y);
                    } else {
                        (l, h) = (y, oldy);
                    }
                    for d in l..=h {
                        hm.insert(Location(x, d), Type::Rock);
                    }
                }
                (false, true) => {
                    let (l, h);
                    if oldx < x {
                        (l, h) = (oldx, x);
                    } else {
                        (l, h) = (x, oldx);
                    }
                    for d in l..=h {
                        hm.insert(Location(d, y), Type::Rock);
                    }
                }
            }
            oldx = x;
            oldy = y;
        }
    }
    let mut maxx = i32::MIN;
    let mut minx = i32::MAX;
    let mut maxy = i32::MIN;
    for l in hm.keys() {
        if l.0 > maxx {
            maxx = l.0;
        }
        if l.0 < minx {
            minx = l.0;
        }
        if l.1 > maxy {
            maxy = l.1;
        }
    }
    let bot = maxy + 2;
    println!("{minx} -> {maxx} | {maxy} + bot {bot}");
    println!("{}x{}", maxx - minx + 1, bot);
    let mut sand = 0;
    let mut state = State::Falling;
    let mut steps = 0;
    loop {
        sand += 1;
        let mut cur = Location(500, 0);
        _ = args.draw && print_board(&hm, &cur, minx, maxx, maxy);
        if hm.contains_key(&cur) {
            println!("blocked at source?");
            break;
        }

        loop {
            steps += 1;
            if !args.infinity {
                if cur.0 < minx || cur.0 > maxx || cur.1 > maxy {
                    println!("inf cur - {cur:?}");
                    state = State::Infinity;
                    break;
                }
            }
            cur = Location(cur.0, cur.1 + 1);
            // Straight down ok, continue
            if !hm.contains_key(&cur) {
                if args.infinity {
                    if cur.1 == bot {
                        cur = Location(cur.0, cur.1 - 1);
                        state = State::Stopped;
                        break;
                    }
                }
                _ = args.draw && print_board(&hm, &cur, minx, maxx, maxy);
                continue;
            }
            // Try left
            cur = Location(cur.0 - 1, cur.1);
            if !hm.contains_key(&cur) {
                _ = args.draw && print_board(&hm, &cur, minx, maxx, maxy);
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
        _ = args.draw && print_board(&hm, &cur, minx, maxx, maxy);
        hm.insert(cur, Type::Sand);
    }
    // Counted infinity particle
    sand -= 1;
    println!("state - {state}");
    println!("steps - {steps}");
    println!("sand - {sand}");
    Ok(())
}

fn print_board(
    hm: &HashMap<Location, Type>,
    cur: &Location,
    minx: i32,
    maxx: i32,
    maxy: i32,
) -> bool {
    for y in 0..=maxy {
        for x in 0..=maxx - minx {
            let mut print;
            let check = Location(x + minx, y);
            if !hm.contains_key(&check) {
                print = ".";
            } else {
                match hm.get(&check).unwrap() {
                    Type::Rock => {
                        print = "#";
                    }
                    Type::Sand => {
                        print = "o";
                    }
                }
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
