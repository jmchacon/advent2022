//! day6 advent 2022
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    const SIZE: usize = 14; // use 4 for part 1
    for (line_num, l) in lines.iter().enumerate() {
        let line = l.as_str().as_bytes();
        assert!(line.len() >= 4, "{} - bad line {l}", line_num + 1);
        let mut tot = SIZE - 1;
        let mut tracking = Vec::<u8>::new();
        for i in 0..=tot {
            tracking.push(line[i]);
        }
        for i in SIZE..line.len() {
            tracking.push(line[i]);
            tot += 1;
            let mut test = HashMap::new();
            for j in tot - (SIZE - 1)..=tot {
                test.entry(tracking[j]).and_modify(|v| *v += 1).or_insert(1);
            }
            let mut done = true;
            for k in test.keys() {
                if test[k] > 1 {
                    done = false;
                    break;
                }
            }
            if done {
                println!("index {}", tot + 1);
                break;
            }
        }
    }
    Ok(())
}
