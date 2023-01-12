//! day25 advent 2022
use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use num::pow;
use std::fmt::Write;
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut nums = Vec::new();
    for line in &lines {
        nums.push(snafu(line).unwrap());
    }
    for n in &nums {
        println!("{n}");
    }
    let sum = nums.iter().sum::<usize>() as f64;
    println!("sum {sum}");

    println!("convert: {}", convert(sum));
    Ok(())
}

fn snafu(inp: &str) -> Result<usize> {
    let mut num: usize = 0;
    for (pos, i) in inp.as_bytes().iter().rev().enumerate() {
        let p = pow(5, pos);
        match i {
            b'1' => num += p,
            b'2' => num += 2 * p,
            b'=' => num -= 2 * p,
            b'-' => num -= p,
            b'0' => {}
            _ => {
                return Err(eyre!("Invalid line {inp}"));
            }
        }
    }
    Ok(num)
}

fn convert(sum: f64) -> String {
    let mut s = String::new();

    // Do an initial conversion to base 5
    // If it doesn't contain anything above a 2
    // in it this matches the snafu repr.
    let mut pow = (sum.ln() / 5.0_f64.ln()).trunc() as usize;
    let mut rem = sum as usize;
    while rem != 0 {
        let n = 5.0_f64.powi(pow as i32) as usize;
        let d = rem / n;
        //println!("n {n} d {d} rem {rem} pow {pow}");
        rem %= n;
        pow -= 1;
        write!(s, "{d}").unwrap();
    }
    for _ in 0..=pow {
        write!(s, "0").unwrap();
    }
    if let Ok(new) = snafu(&s) {
        if new == sum as usize {
            return s;
        }
    }

    // Convert to snafu by working right to left
    s.clear();
    rem = sum as usize;
    while rem != 0 {
        match rem % 5 {
            0 => write!(s, "0").unwrap(),
            1 => write!(s, "1").unwrap(),
            2 => write!(s, "2").unwrap(),
            3 => write!(s, "=").unwrap(),
            4 => write!(s, "-").unwrap(),
            _ => panic!(),
        }
        rem = (rem + 2) / 5;
    }
    s.chars().rev().collect::<String>()
}
