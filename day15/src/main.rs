//! day15 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = 2000000)]
    target: i64,

    #[arg(long, default_value_t = 0)]
    boundingx: i64,

    #[arg(long, default_value_t = 4000000)]
    boundingy: i64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Ent(i64, i64);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let (mut minx, mut maxx, mut miny, mut maxy) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    let mut inp = HashMap::new();
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
        inp.insert(Ent(x, y), dist);
        inp.insert(Ent(bx, by), 0);
    }

    let width = maxx - minx + 1;
    let height = maxy - miny + 1;
    println!(
        "{minx},{miny} - {maxx},{maxy} - {width}x{height} = {}",
        width * height
    );

    // Find all covered spaces on row args.target and then subtract off any beacons/sensors on that line.
    let now = Instant::now();
    let c = cover2_row(&inp, args.target, minx, true);
    let elapsed = Instant::now().duration_since(now);
    println!("{elapsed:?} - sum - {c}");

    let now = Instant::now();
    for y in args.boundingx..=args.boundingy {
        if cover2_row(&inp, y, args.boundingx, false) == -1 {
            break;
        }
    }
    let elapsed = Instant::now().duration_since(now);

    println!("{elapsed:?}");

    Ok(())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Interval(i64, i64);

fn cover2_row(inp: &HashMap<Ent, u64>, target: i64, start_min: i64, count: bool) -> i64 {
    let mut ints = Vec::new();
    let mut beacons = 0;
    for (k, v) in inp {
        // we now skip beacons.
        if *v == 0 {
            if k.1 == target {
                beacons += 1;
            }
            continue;
        }
        let dist = k.1.abs_diff(target);
        if dist <= *v {
            let tot = ((*v * 2 + 1) - dist * 2) as i64;
            let start = k.0 - (tot / 2) as i64;
            ints.push(Interval(start, tot));
        }
    }
    ints.sort();
    let mut min = start_min;
    let mut c = 0;
    for i in ints {
        // Skip if outside our box/overlap with an existing span.
        if i.0 + i.1 < min {
            continue;
        }
        if i.0 > min {
            if count {
                c += i.1 - i.0;
            } else {
                // First gap we can just abort.
                if !inp.contains_key(&Ent(min, target)) {
                    println!("{},{target}", min);
                    println!("freq = {}", min * 4000000 + target);
                    return -1;
                }
            }
        } else {
            if count {
                c += i.1 - (min - i.0);
            }
        }

        if i.0 < min {
            min = (min - (min - i.0)) + i.1;
        } else {
            min += (i.0 - min) + (i.1 - i.0);
        }
    }
    c -= beacons;
    if count {
        return c;
    }
    return i64::MIN;
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
