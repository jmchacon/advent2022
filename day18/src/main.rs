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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i64, i64, i64);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut squares = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split(",").collect::<Vec<_>>();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
        squares.push(Location(
            i64::from_str_radix(parts[0], 10).unwrap(),
            i64::from_str_radix(parts[1], 10).unwrap(),
            i64::from_str_radix(parts[2], 10).unwrap(),
        ))
    }
    let mut lava = HashSet::new();
    squares.iter().for_each(|v| {
        lava.insert(v.clone());
    });
    // If nothing touches we get 6 faces per square.
    let (mut maxx, mut maxy, mut maxz) = (i64::MIN, i64::MIN, i64::MIN);
    let (mut minx, mut miny, mut minz) = (i64::MAX, i64::MAX, i64::MAX);
    let mut faces = squares.len() * 6;
    for i in 0..squares.len() {
        let compare = &squares[i];
        if compare.0 > maxx {
            maxx = compare.0
        }
        if compare.1 > maxy {
            maxy = compare.1
        }
        if compare.2 > maxz {
            maxz = compare.2
        }
        if compare.0 < minx {
            minx = compare.0
        }
        if compare.1 < miny {
            miny = compare.1
        }
        if compare.2 < minz {
            minz = compare.2
        }
        for j in i + 1..squares.len() {
            faces -= touching(compare, &squares[j]);
        }
    }
    // Make a bounding box one larger to surround this.
    minx -= 1;
    miny -= 1;
    minz -= 1;
    maxx += 1;
    maxy += 1;
    maxz += 1;

    let mut choices = Vec::from([Location(minx, miny, minz)]);
    let mut water = HashSet::new();
    while choices.len() > 0 {
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
            if dir.0 < minx
                || dir.0 > maxx
                || dir.1 < miny
                || dir.1 > maxy
                || dir.2 < minz
                || dir.2 > maxz
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
    println!("min,max x {minx},{maxx}");
    println!("min,max y {miny},{maxy}");
    println!("min,max z {minz},{maxz}");
    println!("{faces}");
    println!("{faces2}");

    Ok(())
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
