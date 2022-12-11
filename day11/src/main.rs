//! day11 advent 2022
use color_eyre::eyre::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use std::io::BufRead;
use std::mem::replace;
use std::path::Path;
use std::{fs::File, num::Wrapping};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
enum OpVal {
    Val(BigUint),
    Old,
}
#[derive(Debug)]
struct Monkey {
    items: Vec<BigUint>,
    op: Operation,
    op_val: OpVal,
    test: u32,
    choice: [usize; 2],
    inspected: u32,
}

const ROUNDS: usize = 1000;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input2.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut monkeys = Vec::new();
    let mut it = lines.iter().enumerate();
    loop {
        let Some((line_num, line)) = it.next() else { break; };
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 0 {
            continue;
        }
        if parts[0] == "Monkey" {
            let mut monkey = Monkey {
                items: Vec::new(),
                op: Operation::Add,
                op_val: OpVal::Val(Zero::zero()),
                test: 0,
                choice: [0, 0],
                inspected: 0,
            };

            // Starting items: x, y
            let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts.len() > 2, "{} - bad line {line}", line_num + 1);
            for item in parts[2..].iter() {
                let i = u32::from_str_radix(item.trim_matches(','), 10).unwrap();
                monkey.items.push(BigUint::new(vec![i]));
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
                monkey.op_val = OpVal::Val(BigUint::new(vec![
                    u32::from_str_radix(parts[5], 10).unwrap()
                ]));
            }

            // Test: divisible by 17
            let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts.len() == 4, "{} - bad line {line}", line_num + 1);
            monkey.test = u32::from_str_radix(parts[3], 10).unwrap();

            // If true: throw to monkey 4
            let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts.len() == 6, "{} - bad line {line}", line_num + 1);
            monkey.choice[0] = usize::from_str_radix(parts[5], 10).unwrap();

            // If false: throw to monkey 5
            let Some((line_num, line)) = it.next() else { panic!("{} - bad line {line}", line_num+1); };
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts.len() == 6, "{} - bad line {line}", line_num + 1);
            monkey.choice[1] = usize::from_str_radix(parts[5], 10).unwrap();

            monkeys.push(monkey);
        }
    }
    for monkey in &monkeys {
        println!("{:?}", monkey);
    }

    for round in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut new = Vec::new();
            for item in &monkey.items {
                let mut worry = item.clone();
                /*let val = match monkey.op_val {
                    OpVal::Val(v) => &v,
                    OpVal::Old => &worry.clone(),
                };*/
                match monkey.op {
                    Operation::Add => {
                        if let OpVal::Val(v) = &monkey.op_val {
                            worry += v;
                        } else {
                            worry *= 2 as u8;
                        }
                    }
                    Operation::Multiply => {
                        if let OpVal::Val(v) = &monkey.op_val {
                            worry *= v;
                        } else {
                            worry = worry.pow(2);
                        }
                    }
                }
                //worry /= 3 as u8;
                let index;
                if &worry % monkey.test == Zero::zero() {
                    index = monkey.choice[0];
                } else {
                    index = monkey.choice[1];
                }
                new.push((index, worry));
                monkey.inspected += 1;
            }
            monkey.items.clear();
            drop(monkey);
            for (index, worry) in new {
                monkeys[index].items.push(worry);
            }
        }
        /*for monkey in &monkeys {
            println!("{round} - {:?}", monkey);
        }*/
        println!("{round}");
    }
    let mut inspected = Vec::new();
    for monkey in monkeys {
        inspected.push(monkey.inspected);
    }
    println!("inspected: {inspected:?}");
    inspected.sort();
    inspected.reverse();
    println!(
        "top 2 - {} * {} = {}",
        inspected[0],
        inspected[1],
        inspected[0] * inspected[1]
    );
    Ok(())
}
