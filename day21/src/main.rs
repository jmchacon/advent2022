//! day21 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use slab_tree::tree::Tree;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Entry {
    Value(i64),
    Operation(Operation),
}

#[derive(Clone, Debug)]
struct Definition<'a> {
    children: Vec<&'a str>,
    op: Entry,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hm = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let key = parts[0].trim_end_matches(":");
        match parts.len() {
            2 => {
                let val = i64::from_str_radix(parts[1], 10).unwrap();
                hm.insert(
                    key,
                    Definition {
                        children: vec![],
                        op: Entry::Value(val),
                    },
                );
            }
            4 => {
                let op = match parts[2] {
                    "+" => Operation::Plus,
                    "-" => Operation::Minus,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => {
                        panic!("{} - bad line {line}", line_num + 1);
                    }
                };
                hm.insert(
                    key,
                    Definition {
                        children: vec![parts[1], parts[3]],
                        op: Entry::Operation(op),
                    },
                );
            }
            _ => {
                panic!("{} - bad line {line}", line_num + 1);
            }
        }
    }

    for (k, v) in &hm {
        println!("{k} -> {v:?}");
    }

    let mut tree: Tree<Entry> = Tree::new();
    let mut work = Vec::new();
    tree.set_root(hm.get(&"root").unwrap().op.clone());
    for c in &hm.get(&"root").unwrap().children {
        work.push((tree.root_id().unwrap(), *c));
    }
    loop {
        let Some(e) = work.pop() else {
            break
        };
        //println!("processing {e:?}");
        let op = hm.get(e.1).unwrap();
        let mut n = tree.get_mut(e.0).unwrap();
        match &op.op {
            Entry::Value(v) => {
                n.append(Entry::Value(*v));
            }
            Entry::Operation(o) => {
                let id = n.append(Entry::Operation(o.clone())).node_id();
                // This pushes the children in reverse order. Account below when we do the op.
                work.push((id, op.children[0]));
                work.push((id, op.children[1]));
            }
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");

    for n in tree
        .root()
        .unwrap()
        .traverse_post_order()
        .map(|n| n.node_id())
        .collect::<Vec<_>>()
    {
        let node = tree.get(n).unwrap();
        // Values are end nodes so we don't care here.
        if let Entry::Value(_) = node.data() {
            continue;
        }
        let Entry::Operation(op) = node.data().clone() else {
            panic!();
        };
        let vals = node
            .children()
            .map(|v| match v.data() {
                Entry::Value(v) => *v,
                Entry::Operation(_) => {
                    panic!()
                }
            })
            .collect::<Vec<_>>();
        let new: i64 = match op {
            Operation::Plus => vals.iter().sum(),
            Operation::Minus => vals[1] - vals[0],
            Operation::Multiply => vals[1] * vals[0],
            Operation::Divide => vals[1] / vals[0],
        };
        let mut node = tree.get_mut(n).unwrap();
        *node.data() = Entry::Value(new);
    }

    s.clear();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");

    println!("root is {:?}", tree.root().unwrap().data());
    Ok(())
}
