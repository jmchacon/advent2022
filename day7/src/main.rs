//! day7 advent 2022
use color_eyre::eyre::Result;
use slab_tree::tree::TreeBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Ent {
    name: String,
    size: usize,
}

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    const TOTAL_SIZE: usize = 70000000;
    const REQUIRED: usize = 30000000;

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

        match fields[0] {
            "$" => {
                match fields[1] {
                    "cd" => {
                        assert!(fields.len() == 3, "{} - bad line - {line}", line_num + 1);
                        match fields[2] {
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
                                if !found {
                                    panic!(
                                        "cd {} on line {} but not found",
                                        fields[2],
                                        line_num + 1
                                    );
                                }
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
            }
            _ => {
                assert!(fields.len() == 2, "{} - bad line - {line}", line_num + 1);
                let mut node = tree.get_mut(cur_id).unwrap();
                let mut size = 0;
                if fields[0] != "dir" {
                    size = usize::from_str_radix(fields[0], 10).unwrap();
                }
                node.append(Ent {
                    name: String::from(fields[1]),
                    size: size,
                });
            }
        };
    }
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");

    let mut hm = HashMap::new();
    for node in tree.root().unwrap().traverse_post_order() {
        if let Some(p) = node.parent() {
            let mut size = node.data().size;
            if size == 0 {
                size = hm[&node.node_id()];
            }
            println!(
                "adding {} from node {} to parent {}",
                size,
                node.data().name,
                p.data().name
            );
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
        if size <= 100000 {
            sum += size;
        }
        if size >= needed {
            choices.push(size);
        }
        node.data().size = size;
    }
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");
    println!("sum: {sum}");
    println!("unused: {unused}");
    println!("needed: {needed}");
    println!("min: {}", choices.iter().min().unwrap());
    Ok(())
}
