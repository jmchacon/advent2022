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
    let mut entries = HashSet::new();
    squares.iter().for_each(|v| {
        entries.insert(v);
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
    let mut faces2 = faces;
    for x in minx..=maxx {
        for y in miny..=maxy {
            for z in minz..=maxz {
                let t = Location(x, y, z);
                if !entries.contains(&t) {
                    let mut sum = 0;
                    for compare in &squares {
                        sum += touching(&t, compare);
                    }
                    if sum == 12 {
                        faces2 -= 6;
                        println!("Trying {t:?} = {}", sum);
                    }
                }
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
