//! day7 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use slab_tree::tree::TreeBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Debug)]
struct Ent {
    name: String,
    size: usize,
}

const TOTAL_SIZE: usize = 70_000_000;
const REQUIRED: usize = 30_000_000;

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut tree = TreeBuilder::new()
        .with_root(Ent {
            name: String::from("/"),
            size: 0,
        })
        .build();

    let root_id = tree.root_id().unwrap();
    let mut cur_id = root_id;

    for (line_num, line) in lines.iter().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();

        assert!(
            fields.len() == 2 || fields.len() == 3,
            "{} - bad line - {line}",
            line_num + 1
        );

        if *fields.first().unwrap() == "$" {
            match *fields.get(1).unwrap() {
                "cd" => {
                    assert!(fields.len() == 3, "{} - bad line - {line}", line_num + 1);
                    match *fields.get(2).unwrap() {
                        "/" => {
                            cur_id = root_id;
                        }
                        ".." => {
                            cur_id = tree.get(cur_id).unwrap().parent().unwrap().node_id();
                        }
                        _ => {
                            let node = tree.get(cur_id).unwrap();
                            let mut found = false;
                            for n in node.children() {
                                if n.data().name == fields[2] {
                                    cur_id = n.node_id();
                                    found = true;
                                    break;
                                }
                            }
                            assert!(
                                found,
                                "cd {} on line {} but not found",
                                fields[2],
                                line_num + 1
                            );
                        }
                    }
                }
                "ls" => {
                    // nothing happens here
                }
                _ => panic!(
                    "unknown field on line {} - {line} - {}",
                    line_num + 1,
                    fields[1]
                ),
            }
        } else {
            assert!(fields.len() == 2, "{} - bad line - {line}", line_num + 1);
            let mut node = tree.get_mut(cur_id).unwrap();
            let mut size = 0;
            if fields[0] != "dir" {
                size = fields[0].parse::<usize>()?;
            }
            node.append(Ent {
                name: String::from(fields[1]),
                size,
            });
        }
    }
    let mut s = String::new();
    if args.debug {
        tree.write_formatted(&mut s)?;
        println!("{s}");
    }

    let mut hm = HashMap::new();
    for node in tree.root().unwrap().traverse_post_order() {
        if let Some(p) = node.parent() {
            let mut size = node.data().size;
            // Directories have no size initially but the hash
            // map entry for it will have the size we need to insert.
            // Then we add the size to the parent node.
            if size == 0 {
                size = hm[&node.node_id()];
            }
            if args.debug {
                println!(
                    "adding {} from node {} to parent {}",
                    size,
                    node.data().name,
                    p.data().name
                );
            }
            hm.entry(p.node_id())
                .and_modify(|s| *s += size)
                .or_insert(size);
        }
    }

    let mut sum = 0;
    let unused = TOTAL_SIZE - hm[&root_id];
    let needed = REQUIRED - unused;
    let mut choices = Vec::new();
    for k in hm.keys() {
        let mut node = tree.get_mut(*k).unwrap();
        let size = hm[k];
        if size <= 100_000 {
            sum += size;
        }
        if size >= needed {
            choices.push(size);
        }
        node.data().size = size;
    }
    let mut s = String::new();
    if args.debug {
        tree.write_formatted(&mut s)?;
        println!("{s}");
    }
    println!("part1: {sum}");
    if args.debug {
        println!("unused: {unused}");
        println!("needed: {needed}");
    }
    println!("part2: {}", choices.iter().min().unwrap());
    Ok(())
}
