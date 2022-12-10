//! day10 advent 2022
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut x = 1;
    let mut cycles = 0;
    let mut sum = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        assert!(
            parts.len() == 1 || parts.len() == 2,
            "{} - bad line {line}",
            line_num + 1
        );

        let mut run: usize = 1;
        let mut add = None;
        match parts[0] {
            "noop" => {}
            "addx" => {
                run = 2;
                let val = i32::from_str_radix(parts[1], 10).unwrap();
                add = Some(val);
            }
            _ => {
                panic!("{} - base line {line}", line_num + 1);
            }
        }
        for _ in 0..run {
            cycles += 1;
            if cycles == 20 || (cycles > 20 && (cycles - 20) % 40 == 0) {
                sum += cycles * x;
                println!("{cycles} {}", cycles * x);
            }
        }
        if let Some(v) = add {
            x += v;
        }
    }
    println!("sum {sum}");
    Ok(())
}
