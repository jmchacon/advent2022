//! day15 advent 2022
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

    #[arg(long, default_value_t = 2000000)]
    target: i64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Ent(i64, i64, u64);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let (mut minx, mut maxx, mut miny, mut maxy) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    let mut inp = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 10, "{} - bad line {line}", line_num + 1);
        let x = i64::from_str_radix(
            parts[2].split("=").collect::<Vec<_>>()[1].trim_end_matches(","),
            10,
        )
        .unwrap();
        let y = i64::from_str_radix(
            parts[3].split("=").collect::<Vec<_>>()[1].trim_end_matches(":"),
            10,
        )
        .unwrap();
        let bx = i64::from_str_radix(
            parts[8].split("=").collect::<Vec<_>>()[1].trim_end_matches(","),
            10,
        )
        .unwrap();
        let by = i64::from_str_radix(parts[9].split("=").collect::<Vec<_>>()[1], 10).unwrap();
        let dist = x.abs_diff(bx) + y.abs_diff(by);

        min_check(x, &mut minx, dist);
        min_check(bx, &mut minx, 0);
        max_check(x, &mut maxx, dist);
        max_check(bx, &mut maxx, 0);
        min_check(y, &mut miny, dist);
        min_check(by, &mut miny, 0);
        max_check(y, &mut maxy, dist);
        max_check(by, &mut maxy, 0);
        inp.push(Ent(x, y, dist));
        inp.push(Ent(bx, by, 0));
    }

    let width = maxx - minx + 1;
    let height = maxy - miny + 1;
    println!(
        "{minx},{miny} - {maxx},{maxy} - {width}x{height} = {}",
        width * height
    );

    // Find all covered spaces on row args.target and then subtract off any beacons/sensors on that line.
    let mut covered = vec![false; width as usize];
    for i in &inp {
        // we now skip beacons.
        if i.2 == 0 {
            continue;
        }
        let dist = i.1.abs_diff(args.target);
        if dist <= i.2 {
            let tot = ((i.2 * 2 + 1) - dist * 2) as usize;
            let start = ((i.0 - (tot / 2) as i64) - minx) as usize;
            for i in start..start + tot {
                covered[i] = true;
            }
        }
    }
    for i in &inp {
        // See if it's a beacon/sensor on this row. That can't be a beacon then.
        if i.1 == args.target {
            covered[(i.1 - minx) as usize] = false;
        }
    }
    let sum = covered
        .iter()
        .map(|v| if *v { 1 } else { 0 })
        .sum::<usize>();
    println!("sum - {sum}");
    //println!("{covered:?}");
    Ok(())
}

fn min_check(a: i64, min: &mut i64, dist: u64) {
    if a < *min {
        *min = a;
    }
    if a - (dist as i64) < *min {
        *min = a - dist as i64;
    }
}

fn max_check(a: i64, max: &mut i64, dist: u64) {
    if a > *max {
        *max = a;
    }
    if a + (dist as i64) > *max {
        *max = a + dist as i64;
    }
}
