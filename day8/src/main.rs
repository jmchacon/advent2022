//! day8 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
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

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut map = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        let chars = line.split("");
        if args.debug {
            println!("{} - line {line}", line_num + 1);
        }
        for c in chars {
            if c.is_empty() {
                continue;
            }
            let v = c.parse::<usize>()?;
            row.push(v);
        }
        map.push(row);
    }
    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;
    let mut seen = Vec::new();
    for y in 0..=max_y {
        let mut row = Vec::new();
        let mut val = false;
        if y == 0 || y == max_x {
            val = true;
        }
        for x in 0..=max_x {
            if x == 0 || x == max_x {
                row.push(true);
            } else {
                row.push(val);
            }
        }
        seen.push(row);
    }
    let mut scenic = 0;
    for y in 0..=max_y {
        // edge always visible
        let mut tallest = map[y][0];
        for x in 0..=max_x {
            if map[y][x] > tallest && x != max_x && y != 0 && y != max_y && x != 0 {
                seen[y][x] = true;
                tallest = map[y][x];
            }
        }

        if y == 0 {
            for x in 0..=max_x {
                let mut tallest = map[0][x];
                for yy in 0..=max_y {
                    if map[yy][x] > tallest && yy != max_y && yy != 0 && x != 0 && x != max_x {
                        seen[yy][x] = true;
                        tallest = map[yy][x];
                    }
                }
            }
        }
        if y == max_y {
            for x in 0..=max_x {
                let mut tallest = map[max_y][x];
                for yy in (0..=max_y).rev() {
                    if map[yy][x] > tallest && yy != 0 && yy != max_y && x != 0 && x != max_x {
                        seen[yy][x] = true;
                        tallest = map[yy][x];
                    }
                }
            }
        }

        tallest = map[y][max_x];
        for x in (0..=max_x).rev() {
            if map[y][x] > tallest && x != 0 && y != 0 && y != max_y && x != max_x {
                seen[y][x] = true;
                tallest = map[y][x];
            }
        }

        if y != 0 && y != max_y {
            for x in 1..max_x {
                let here = map[y][x];
                let mut sum = 1;
                // look up
                let mut dist = 0;
                for yy in (0..y).rev() {
                    dist += 1;
                    if map[yy][x] >= here {
                        break;
                    }
                }
                sum *= dist;

                dist = 0;
                // look down
                for m in map.iter().take(max_y + 1).skip(y + 1) {
                    dist += 1;
                    if m[x] >= here {
                        break;
                    }
                }
                sum *= dist;

                dist = 0;
                // look left
                for xx in (0..x).rev() {
                    dist += 1;
                    if map[y][xx] >= here {
                        break;
                    }
                }
                sum *= dist;

                dist = 0;
                // look right
                for xx in x + 1..=max_x {
                    dist += 1;
                    if map[y][xx] >= here {
                        break;
                    }
                }
                sum *= dist;

                if sum > scenic {
                    scenic = sum;
                }
            }
        }
    }
    let mut sum = 0;
    for s in &seen {
        for v in s {
            if *v {
                sum += 1;
            }
        }
    }
    if args.debug {
        for s in seen.iter().take(max_y + 1) {
            println!("{s:?}");
        }
    }
    println!("part1: {sum}");
    println!("part2: {scenic}");
    Ok(())
}
