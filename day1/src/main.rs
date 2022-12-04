//! day1 advent 2022
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut elves: Vec<u64> = Vec::new();
    let mut cur: usize = 0;

    for (line_num, line) in lines.flatten().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() < 2, "{}: invalid - {line}", line_num + 1);

        if fields.len() == 0 {
            cur += 1;
        } else {
            let val = u64::from_str_radix(fields[0], 10)?;
            if elves.len() != (cur + 1) {
                elves.push(val);
            } else {
                elves[cur] += val;
            }
        }
    }
    println!("{} elves", elves.len());

    elves.sort();
    let last = elves.len() - 1;

    println!("max {}", elves[last]);
    let top3 = elves[last] + elves[last - 1] + elves[last - 2];
    println!("top3 {top3}");
    Ok(())
}
