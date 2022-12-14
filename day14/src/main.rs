//! day14 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
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

    #[arg(long, default_value_t = false)]
    infinity: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i32, i32);

#[derive(Clone, Debug, Display, PartialEq)]
enum State {
    Infinity,
    Stopped,
    Falling,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hs = HashSet::new();
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
                    if oldy < y {
                        for d in oldy..=y {
                            hs.insert(Location(x, d));
                        }
                    } else {
                        for d in (y..=oldy).rev() {
                            hs.insert(Location(x, d));
                        }
                    }
                }
                (false, true) => {
                    if oldx < x {
                        for d in oldx..=x {
                            hs.insert(Location(d, y));
                        }
                    } else {
                        for d in (x..=oldx).rev() {
                            hs.insert(Location(d, y));
                        }
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
    for l in &hs {
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
    println!("{minx} -> {maxx} | {maxy} + {bot}");

    let mut sand = 0;
    let mut state = State::Falling;
    loop {
        sand += 1;
        let mut cur = Location(500, 0);
        if hs.contains(&cur) {
            println!("blocked at source?");
            break;
        }

        loop {
            if !args.infinity {
                if cur.0 < minx || cur.0 > maxx || cur.1 > maxy {
                    println!("inf cur - {cur:?}");
                    state = State::Infinity;
                    break;
                }
            }
            cur = Location(cur.0, cur.1 + 1);
            //println!("trying {cur:?}");
            // Straight down ok, continue
            if !hs.contains(&cur) {
                if args.infinity {
                    if cur.1 == bot {
                        cur = Location(cur.0, cur.1 - 1);
                        state = State::Stopped;
                        break;
                    }
                }
                //                println!("down cur - {cur:?}");
                continue;
            }
            // Try left
            cur = Location(cur.0 - 1, cur.1);
            if !hs.contains(&cur) {
                //              println!("down left cur - {cur:?}");
                continue;
            }
            // Try right - if not we're stuck.
            cur = Location(cur.0 + 2, cur.1);
            if hs.contains(&cur) {
                //            println!("down right cur - {cur:?}");
                cur = Location(cur.0 - 1, cur.1 - 1);
                state = State::Stopped;
                break;
            }
        }
        if state == State::Infinity {
            break;
        }
        hs.insert(cur);
    }
    // Counted infinity particle
    sand -= 1;
    println!("state - {state}");
    println!("sand - {sand}");
    Ok(())
}
