//! day3 advent 2022
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut sum: u32 = 0;
    for (line_num, line) in lines.flatten().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() == 1, "{}: invalid - {line}", line_num + 1);

        assert!(
            fields[0].len() % 2 == 0,
            "{}: invalid num chars - {line}",
            line_num + 1
        );
        let mut comp1 = HashMap::new();
        for c in fields[0][0..fields[0].len() / 2].chars() {
            if comp1.contains_key(&c) {
                let v = comp1.get_mut(&c).unwrap();
                *v += 1;
            } else {
                comp1.insert(c, 1);
            }
        }
        let mut comp2 = HashMap::new();
        for c in fields[0][fields[0].len() / 2..].chars() {
            if comp2.contains_key(&c) {
                let v = comp2.get_mut(&c).unwrap();
                *v += 1;
            } else {
                comp2.insert(c, 1);
            }
        }

        for k in comp1.keys() {
            if comp2.contains_key(k) {
                let mut top = 'A';
                let mut add: u32 = 27;
                if *k >= 'a' {
                    top = 'a';
                    add = 1;
                }
                let new = *k as u32 - top as u32 + add;
                sum += new;
                println!("{} {top} - common {k} {new}", line_num + 1);
                break;
            }
        }
    }
    println!("sum: {sum}");
    Ok(())
}
