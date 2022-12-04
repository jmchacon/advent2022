//! day4 advent 2022
use color_eyre::eyre::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut all_overlap = 0;
    let mut any_overlap = 0;
    for (line_num, l) in lines.iter().enumerate() {
        let p: Vec<_> = l.split(",").collect();
        assert!(p.len() == 2, "{} - bad line {l}", line_num + 1);
        let range1: Vec<_> = p[0].split("-").collect();
        assert!(range1.len() == 2, "{} - bad line {l}", line_num + 1);
        let range2: Vec<_> = p[1].split("-").collect();
        assert!(range2.len() == 2, "{} - bad line {l}", line_num + 1);
        let low1 = usize::from_str_radix(range1[0], 10)?;
        let high1 = usize::from_str_radix(range1[1], 10)?;
        let low2 = usize::from_str_radix(range2[0], 10)?;
        let high2 = usize::from_str_radix(range2[1], 10)?;
        println!("{} - {low1} - {high1} --- {low2} - {high2}", line_num + 1);

        let (mut comp_low1, mut comp_high1, mut comp_low2, mut comp_high2) =
            (low1, high1, low2, high2);
        if high1 - low1 > high2 - low2 {
            (comp_low1, comp_high1, comp_low2, comp_high2) = (low2, high2, low1, high1);
        }

        if comp_low1 >= comp_low2 && comp_high1 <= comp_high2 {
            println!("{} inside", line_num + 1);
            all_overlap += 1;
        }

        if max(low1, low2) <= min(high1, high2) {
            println!("{} any", line_num + 1);
            any_overlap += 1;
        }
    }
    println!("all_overlap - {all_overlap}");
    println!("any_overlap - {any_overlap}");
    Ok(())
}
