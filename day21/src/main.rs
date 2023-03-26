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

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Unknown,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Item {
    Value(i64),
    Operation(Operation),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Entry<'a> {
    name: &'a str,
    value: Item,
}

#[derive(Clone, Debug)]
struct Definition<'a> {
    children: Vec<&'a str>,
    op: Entry<'a>,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let hm = parse_lines(&lines)?;
    if args.debug {
        for (k, v) in &hm {
            println!("{k} -> {v:?}");
        }
    }
    let mut tree = make_tree(&hm);

    let mut s = String::new();
    if args.debug {
        tree.write_formatted(&mut s)?;
        println!("{s}");
    }

    transform_tree(&mut tree, false);

    if args.debug {
        s.clear();
        tree.write_formatted(&mut s)?;
        println!("{s}");
    }

    let Item::Value(v) = tree.root().unwrap().data().value else {
        panic!()
    };
    println!("part1 - {v}");
    if args.debug {
        println!("part1 - root is {:?}", tree.root().unwrap().data());
    }

    let mut tree = make_tree(&hm);
    transform_tree(&mut tree, true);

    if args.debug {
        s.clear();
        tree.write_formatted(&mut s)?;
        println!("{s}");
    }

    let mut cur = 0;
    let mut node = tree.root().unwrap();
    for c in tree.root().unwrap().children() {
        if let Item::Value(v) = c.data().value {
            // Normally not good but we know the input data fits
            cur = v;
            continue;
        }
        if let Item::Operation(_) = c.data().value {
            node = c;
        }
    }
    if args.debug {
        println!("{:?} = {cur}", node.data());
    }
    let mut ops = 0;
    loop {
        // Each step is
        //
        // cur = x op value
        //
        // or
        //
        // cur = value op x
        //
        // Invert that so cur becomes a new value based on inverse of op and value
        // and set child to the node which is the op.
        // Stop when we find the unknown node and report cur.
        let Item::Operation(op) = node.data().value.clone() else {
            panic!();
        };

        // This is the humn node so we're done.
        if op == Operation::Unknown {
            break;
        }
        ops += 1;
        let children = node.children().collect::<Vec<_>>();
        let v = if let Item::Value(v) = children[0].data().value {
            v
        } else if let Item::Value(v) = children[1].data().value {
            v
        } else {
            panic!();
        };
        let idx: usize = if let Item::Operation(_) = children[0].data().value {
            0
        } else {
            1
        };
        let child1_op = !matches!(children[0].data().value, Item::Operation(_));
        if args.debug {
            println!("{op} {v} - {cur}");
        }
        match op {
            Operation::Plus => {
                // order here doesn't matter.
                //
                // cur = x + value
                // x = cur - value
                cur -= v;
            }
            Operation::Minus => {
                // order here does matter.
                //
                // cur = x - value
                // x = cur + value
                //
                // cur = value - x
                // x = value - cur;
                if child1_op {
                    cur += v;
                } else {
                    cur = v - cur;
                }
            }
            Operation::Multiply => {
                // order here doesn't matter.
                //
                // cur = x * value
                // x = cur / value
                cur /= v;
            }
            Operation::Divide => {
                // order here does matter.
                //
                // cur = x / value
                // x = cur * value
                //
                // cur = value / x
                // x = value / cur
                if child1_op {
                    cur *= v;
                } else {
                    cur = v / cur;
                }
            }
            Operation::Unknown => panic!(),
        };
        if args.debug {
            println!("{cur}");
        }
        node = tree.get(children[idx].node_id()).unwrap();
    }
    println!("part2 - {cur} in {ops} ops");
    Ok(())
}

fn parse_lines(lines: &[String]) -> Result<HashMap<&str, Definition>> {
    let mut hm = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let key = parts[0].trim_end_matches(':');
        match parts.len() {
            2 => {
                let val = parts[1].parse::<i64>()?;
                hm.insert(
                    key,
                    Definition {
                        children: vec![],
                        op: Entry {
                            name: key,
                            value: Item::Value(val),
                        },
                    },
                );
            }
            4 => {
                let op = match *parts.get(2).unwrap() {
                    "+" => Operation::Plus,
                    "-" => Operation::Minus,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => panic!("{} - bad line {line}", line_num + 1),
                };
                hm.insert(
                    key,
                    Definition {
                        children: vec![parts[1], parts[3]],
                        op: Entry {
                            name: key,
                            value: Item::Operation(op),
                        },
                    },
                );
            }
            _ => panic!("{} - bad line {line}", line_num + 1),
        }
    }
    Ok(hm)
}

fn make_tree<'a>(hm: &'a HashMap<&str, Definition>) -> Tree<Entry<'a>> {
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
        let op = hm.get(e.1).unwrap();
        let mut n = tree.get_mut(e.0).unwrap();
        match &op.op.value {
            Item::Value(v) => {
                n.append(Entry {
                    name: op.op.name,
                    value: Item::Value(*v),
                });
            }
            Item::Operation(o) => {
                let id = n
                    .append(Entry {
                        name: op.op.name,
                        value: Item::Operation(o.clone()),
                    })
                    .node_id();
                // This pushes the children in reverse order. Account below when we do the op.
                work.push((id, op.children[0]));
                work.push((id, op.children[1]));
            }
        }
    }
    tree
}

fn transform_tree(tree: &mut Tree<Entry>, part2: bool) {
    for n in tree
        .root()
        .unwrap()
        .traverse_post_order()
        .map(|n| n.node_id())
        .collect::<Vec<_>>()
    {
        if part2 {
            let mut skip = false;
            for c in tree.get(n).unwrap().children() {
                if c.data().name == "humn" {
                    let humn = c.node_id();
                    let mut node = tree.get_mut(humn).unwrap();
                    node.data().value = Item::Operation(Operation::Unknown);
                    skip = true;
                    break;
                }
                if let Item::Operation(_) = c.data().value {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }
        }
        let node = tree.get(n).unwrap();
        // Values are end nodes so we don't care here.
        if let Item::Value(_) = node.data().value {
            continue;
        }
        let Item::Operation(op) = node.data().value.clone() else {
            panic!();
        };
        let vals = node
            .children()
            .map(|v| match v.data().value {
                Item::Value(v) => v,
                Item::Operation(_) => {
                    panic!()
                }
            })
            .collect::<Vec<_>>();
        let new: i64 = match op {
            Operation::Plus => vals.iter().sum(),
            Operation::Minus => vals[1] - vals[0],
            Operation::Multiply => vals[1] * vals[0],
            Operation::Divide => vals[1] / vals[0],
            Operation::Unknown => panic!(),
        };
        let mut node = tree.get_mut(n).unwrap();
        node.data().value = Item::Value(new);
    }
}
