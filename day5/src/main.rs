//! day5 advent 2022
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

    let mut stacks = vec![
        vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
        vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
        vec!['L', 'Q', 'V'],
        vec!['N', 'B', 'S', 'W', 'R', 'Q'],
        vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
        vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
        vec!['D', 'B', 'Q', 'J'],
        vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
        vec!['B', 'N', 'H', 'M', 'S'],
    ];
    let mut stacks9001 = vec![
        vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
        vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
        vec!['L', 'Q', 'V'],
        vec!['N', 'B', 'S', 'W', 'R', 'Q'],
        vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
        vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
        vec!['D', 'B', 'Q', 'J'],
        vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
        vec!['B', 'N', 'H', 'M', 'S'],
    ];
    for (line_num, l) in lines.iter().enumerate() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.len() == 0 || parts[0] != "move" {
            println!("skipping - {l}");
            continue;
        }
        assert!(parts.len() == 6, "{} - bad line - {l}", line_num + 1);
        let num = u32::from_str_radix(parts[1], 10)?;
        let src = usize::from_str_radix(parts[3], 10)?;
        let dest = usize::from_str_radix(parts[5], 10)?;

        let mut t = Vec::<char>::new();
        for _ in 0..num {
            let v = stacks[src - 1].pop().unwrap();
            //println!("{} - {l} - {i} {src} {dest}", line_num + 1);
            stacks[dest - 1].push(v);
            let v = stacks9001[src - 1].pop().unwrap();
            t.push(v);
        }
        t.reverse();
        for v in t {
            stacks9001[dest - 1].push(v);
        }
    }
    for mut v in stacks {
        println!("top - {}", v.pop().unwrap());
    }
    for mut v in stacks9001 {
        println!("top9001 - {}", v.pop().unwrap());
    }
    Ok(())
}
