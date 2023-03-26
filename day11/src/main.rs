//! day11 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::{Display, EnumString};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
enum OpVal {
    Val(i128),
    Old,
}
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i128>,
    op: Operation,
    op_val: OpVal,
    test: i128,
    choice: [usize; 2],
    inspected: u32,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut monkeys = Vec::new();
    let mut it = lines.iter().enumerate();
    loop {
        let Some((line_num, line)) = it.next() else { break; };
        let parts: Vec<&str> = line.split_whitespace().collect();

        parse_monkey(&mut it, &parts, &mut monkeys, line, line_num)?;
    }
    let mut lcm = 1;
    for monkey in &monkeys {
        lcm *= monkey.test;
        if args.debug {
            println!("{monkey:?}");
        }
    }

    for (pos, (rounds, divide)) in [(20, true), (10_000, false)].iter().enumerate() {
        // Make a copy since we change it for each run below.
        let mut monkeys = monkeys.clone();
        for _ in 0..*rounds {
            for i in 0..monkeys.len() {
                let mut monkey = &mut monkeys[i];
                let mut new = Vec::new();
                for item in &monkey.items {
                    let mut worry = *item;
                    let val = match monkey.op_val {
                        OpVal::Val(v) => v,
                        OpVal::Old => worry,
                    };
                    match monkey.op {
                        Operation::Add => {
                            worry += val;
                        }
                        Operation::Multiply => {
                            worry *= val;
                        }
                    }
                    if *divide {
                        worry /= 3;
                    } else {
                        worry %= lcm;
                    }
                    let index = if worry % monkey.test == 0 {
                        monkey.choice[0]
                    } else {
                        monkey.choice[1]
                    };
                    new.push((index, worry));
                    monkey.inspected += 1;
                }
                monkey.items.clear();
                for (index, worry) in new {
                    monkeys[index].items.push(worry);
                }
            }
        }
        if args.debug {
            println!();
            for monkey in &monkeys {
                println!("{monkey:?}");
            }
        }
        let mut inspected = Vec::new();
        for monkey in &monkeys {
            inspected.push(monkey.inspected);
        }
        inspected.sort_unstable();
        inspected.reverse();
        let tot = u128::from(inspected[0]) * u128::from(inspected[1]);
        println!(
            "part{} - top 2 - {} * {} = {tot}",
            pos + 1,
            inspected[0],
            inspected[1],
        );
    }
    Ok(())
}

fn parse_monkey(
    it: &mut std::iter::Enumerate<std::slice::Iter<'_, std::string::String>>,
    parts: &Vec<&str>,
    monkeys: &mut Vec<Monkey>,
    line: &str,
    line_num: usize,
) -> Result<()> {
    if parts.is_empty() {
        return Ok(());
    }
    if parts.first().unwrap() == &"Monkey" {
        let mut monkey = Monkey {
            items: Vec::new(),
            op: Operation::Add,
            op_val: OpVal::Val(0),
            test: 0,
            choice: [0, 0],
            inspected: 0,
        };

        // Starting items: x, y
        let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() > 2, "{} - bad line {line}", line_num + 1);
        for item in parts[2..].iter() {
            let i = item.trim_matches(',').parse::<i128>()?;
            monkey.items.push(i);
        }

        // Operation: new = old + old
        let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 6, "{} - bad line {line}", line_num + 1);
        if parts[4] == "*" {
            monkey.op = Operation::Multiply;
        }
        if parts[5] == "old" {
            monkey.op_val = OpVal::Old;
        } else {
            monkey.op_val = OpVal::Val(parts[5].parse::<i128>()?);
        }

        // Test: divisible by 17
        let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 4, "{} - bad line {line}", line_num + 1);
        monkey.test = parts[3].parse::<i128>()?;

        // If true: throw to monkey 4
        let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 6, "{} - bad line {line}", line_num + 1);
        monkey.choice[0] = parts[5].parse::<usize>()?;

        // If false: throw to monkey 5
        let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 6, "{} - bad line {line}", line_num + 1);
        monkey.choice[1] = parts[5].parse::<usize>()?;

        monkeys.push(monkey);
    }
    Ok(())
}
