//! day18 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashSet;
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i64, i64, i64);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let squares = parse_lines(&lines)?;
    let mut lava = HashSet::new();
    for v in &squares {
        lava.insert(v.clone());
    }
    // If nothing touches we get 6 faces per square.
    let (mut max_x, mut max_y, mut max_z) = (i64::MIN, i64::MIN, i64::MIN);
    let (mut min_x, mut min_y, mut min_z) = (i64::MAX, i64::MAX, i64::MAX);
    let mut faces = squares.len() * 6;
    for i in 0..squares.len() {
        let compare = &squares[i];
        if compare.0 > max_x {
            max_x = compare.0;
        }
        if compare.1 > max_y {
            max_y = compare.1;
        }
        if compare.2 > max_z {
            max_z = compare.2;
        }
        if compare.0 < min_x {
            min_x = compare.0;
        }
        if compare.1 < min_y {
            min_y = compare.1;
        }
        if compare.2 < min_z {
            min_z = compare.2;
        }
        for s in squares.iter().skip(i + 1) {
            faces -= touching(compare, s);
        }
    }
    // Make a bounding box one larger to surround this.
    min_x -= 1;
    min_y -= 1;
    min_z -= 1;
    max_x += 1;
    max_y += 1;
    max_z += 1;

    let mut choices = Vec::from([Location(min_x, min_y, min_z)]);
    let mut water = HashSet::new();
    while !choices.is_empty() {
        let cur = choices.pop().unwrap();
        for dir in &[
            Location(cur.0 + 1, cur.1, cur.2),
            Location(cur.0 - 1, cur.1, cur.2),
            Location(cur.0, cur.1 + 1, cur.2),
            Location(cur.0, cur.1 - 1, cur.2),
            Location(cur.0, cur.1, cur.2 + 1),
            Location(cur.0, cur.1, cur.2 - 1),
        ] {
            // If this has lava we don't fill through it.
            if lava.contains(dir) {
                //println!("skipping {cur:?} because {dir:?} in lava");
                continue;
            }
            // Outside box. Just skip.
            if dir.0 < min_x
                || dir.0 > max_x
                || dir.1 < min_y
                || dir.1 > max_y
                || dir.2 < min_z
                || dir.2 > max_z
            {
                //println!("skipping {cur:?} because {dir:?} outside");
                continue;
            }

            if !water.contains(dir) {
                water.insert(dir.clone());
                choices.push(dir.clone());
            }
        }
    }

    //println!("water: {water:?}");
    let mut faces2 = 0;
    for cur in &lava {
        for dir in &[
            Location(cur.0 + 1, cur.1, cur.2),
            Location(cur.0 - 1, cur.1, cur.2),
            Location(cur.0, cur.1 + 1, cur.2),
            Location(cur.0, cur.1 - 1, cur.2),
            Location(cur.0, cur.1, cur.2 + 1),
            Location(cur.0, cur.1, cur.2 - 1),
        ] {
            if water.contains(dir) {
                faces2 += 1;
            }
        }
    }
    if args.debug {
        println!("min,max x {min_x},{max_x}");
        println!("min,max y {min_y},{max_y}");
        println!("min,max z {min_z},{max_z}");
    }
    println!("part1 - {faces}");
    println!("part2 - {faces2}");

    Ok(())
}

fn parse_lines(lines: &[String]) -> Result<Vec<Location>> {
    let mut squares = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split(',').collect::<Vec<_>>();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
        let x = parts[0].parse::<i64>()?;
        let y = parts[1].parse::<i64>()?;
        let z = parts[2].parse::<i64>()?;
        squares.push(Location(x, y, z));
    }
    Ok(squares)
}

fn touching(entry: &Location, compare: &Location) -> usize {
    let mut tot = 0;
    // Check x side
    if (entry.0 - compare.0).abs() == 1 && entry.1 == compare.1 && entry.2 == compare.2 {
        tot += 2;
    }
    // Check y side
    if (entry.1 - compare.1).abs() == 1 && entry.0 == compare.0 && entry.2 == compare.2 {
        tot += 2;
    }
    // Check z side
    if (entry.2 - compare.2).abs() == 1 && entry.0 == compare.0 && entry.1 == compare.1 {
        tot += 2;
    }
    tot
}
