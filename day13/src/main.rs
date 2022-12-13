//! day13 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str;
use strum_macros::{Display, EnumString};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,
}

#[derive(Clone, Debug, Display, EnumString)]
enum Entry {
    Val(i32),
    List(Vec<Entry>),
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(&self, &other)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Entry {}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut entries = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let b = line.as_bytes();

        // Line always starts as a list
        let mut c: usize = 1;
        entries.push(parse_list(b, &mut c));
    }
    for e in &entries {
        println!("{e:?}");
    }

    let mut entries2 = entries.clone();
    let two = Entry::List(vec![Entry::Val(2)]);
    let six = Entry::List(vec![Entry::Val(6)]);
    entries2.push(Entry::List(vec![two.clone()]));
    entries2.push(Entry::List(vec![six.clone()]));
    println!("\nentries2");
    entries2.sort();
    let (mut ind1, mut ind2) = (0 as usize, 0 as usize);
    for (i, e) in entries2.iter().enumerate() {
        if e == &two {
            ind1 = i + 1;
        }
        if e == &six {
            ind2 = i + 1;
        }
        println!("{e:?}");
    }

    let mut pos = 0;
    let mut good = Vec::new();
    loop {
        if pos >= entries.len() {
            break;
        }
        if compare(&entries[pos], &entries[pos + 1]) == Ordering::Less {
            good.push(pos / 2 + 1);
        }
        pos += 2;
    }
    println!("good - {good:?}");
    println!("sum - {}", good.iter().sum::<usize>());
    println!("2 * 6 = {}", ind1 * ind2);
    Ok(())
}

fn compare(entry1: &Entry, entry2: &Entry) -> Ordering {
    match (entry1, entry2) {
        (Entry::Val(_), Entry::Val(_)) => {
            panic!("can't get here");
        }
        (Entry::Val(_), Entry::List(_)) => {
            return compare(&Entry::List(vec![entry1.clone()]), entry2);
        }
        (Entry::List(_), Entry::Val(_)) => {
            return compare(entry1, &Entry::List(vec![entry2.clone()]));
        }
        (Entry::List(a), Entry::List(b)) => {
            let mut aa = a.iter();
            let mut bb = b.iter();

            if a.len() == 0 && b.len() == 0 {
                return Ordering::Equal;
            }
            loop {
                let nexta = aa.next();
                let nextb = bb.next();
                // If they both go empty together this is equality, not left running out first.
                if nexta.is_none() && nextb.is_none() {
                    return Ordering::Equal;
                }
                // Left side running out is fine.
                if nexta.is_none() {
                    return Ordering::Less;
                }
                // 2nd one running out is bad.
                if nextb.is_none() {
                    return Ordering::Greater;
                }
                let nexta = nexta.unwrap();
                let nextb = nextb.unwrap();
                if let (Entry::Val(compa), Entry::Val(compb)) = (nexta, nextb) {
                    if compa > compb {
                        return Ordering::Greater;
                    }
                    if compa < compb {
                        return Ordering::Less;
                    }
                } else {
                    let ret = compare(nexta, nextb);
                    if ret != Ordering::Equal {
                        return ret;
                    }
                }
            }
        }
    }
}

fn parse_list(b: &[u8], c: &mut usize) -> Entry {
    let mut entry = Vec::new();
    let mut start = None;
    loop {
        if *c >= b.len() {
            break;
        }
        match b[*c] {
            b'[' => {
                *c += 1;
                let e = parse_list(b, c);
                entry.push(e);
            }
            // At the top level comma's are just skipped.
            b',' | b']' => {
                if let Some(s) = start {
                    let sub = str::from_utf8(&b[s..*c]).unwrap();
                    let val = i32::from_str_radix(sub, 10).unwrap();
                    entry.push(Entry::Val(val));
                    start = None;
                }
                if b[*c] == b']' {
                    break;
                }
            }
            _ => {
                if start.is_none() {
                    start = Some(*c);
                }
            }
        }
        *c += 1;
    }
    Entry::List(entry)
}
