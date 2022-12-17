//! day16 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location {
    flow: usize,
    neighbors: Vec<String>,
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
        assert!(parts.len() >= 10, "{} - bad line {line}", line_num + 1);

        let rate = usize::from_str_radix(
            parts[4]
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(";")
                .unwrap(),
            10,
        )
        .unwrap();
        let mut n = Vec::new();
        for p in &parts[9..] {
            n.push(String::from(p.trim_end_matches(",")));
        }
        hm.insert(
            String::from(parts[1]),
            Location {
                flow: rate,
                neighbors: n,
            },
        );
    }
    for (k, v) in &hm {
        println!("{k} - {v:?}");
    }
    let perms = hm
        .keys()
        .filter(|k| *k == "AA" || hm[*k].flow != 0)
        .permutations(2)
        .collect::<Vec<_>>();
    println!("perms");
    let mut paths = HashMap::new();
    for p in &perms {
        let key = p[0].clone() + "->" + p[1];
        paths.insert(key.clone(), find_path(&hm, p[0], p[1]));
        println!("{key} - {:?}", paths[&key]);
    }
    let nodes = hm
        .iter()
        .filter(|(_, v)| v.flow != 0)
        .collect::<HashMap<_, _>>();
    println!("nodes - {nodes:?}");

    let mut best = Vec::new();
    let mut max = 0;
    if nodes.len() <= 7 {
        let mut c: u64 = 0;
        for x in nodes.keys().permutations(nodes.len()) {
            c += 1;
            if c % 1000000 == 0 {
                println!("{c}");
            }

            //println!("{x:?}");
            let mut cur = String::from("AA");
            let mut minutes = 0;
            let mut flow_rate = 0;
            let mut total_flow = 0;
            for p in &x {
                let key = cur.clone() + "->" + p;
                let steps = paths[&key].len();
                let new_flow_rate = hm[paths[&key][steps - 1]].flow;
                total_flow += steps * flow_rate;
                flow_rate += new_flow_rate;

                minutes += steps;
                cur = (**p).clone();
            }
            if minutes <= 30 {
                let left = 30 - minutes;
                total_flow += left * flow_rate;
                //minutes += left;
                //println!("AA -> {x:?} {minutes} {total_flow}");
                if total_flow > max {
                    max = total_flow;
                    best = x.clone();
                }
            } else {
                //println!("AA -> {x:?} too long at {minutes} minutes")
            }
        }
        println!("AA -> {best:?} {max}");
        let cur = String::from("AA");
        let flows = nodes.keys().cloned().collect::<Vec<_>>();
        max = find_best(&cur, &flows, &nodes, &paths, 30);
    } else {
        let cur = String::from("AA");
        let flows = nodes.keys().cloned().collect::<Vec<_>>();
        max = find_best(&cur, &flows, &nodes, &paths, 30);
    }
    println!("AA -> {best:?} {max}");

    Ok(())
}

fn find_best(
    cur: &String,
    flows: &Vec<&String>,
    nodes: &HashMap<&String, &Location>,
    paths: &HashMap<String, Vec<&String>>,
    minutes: usize,
) -> usize {
    let mut choices = Vec::new();

    for f in flows.iter().cloned() {
        let key = cur.clone() + "->" + f;
        let steps = paths[&key].len();

        // Prune off branches that can't finish.
        // In the input set this turns 15! into a lot smaller space.
        if steps >= minutes {
            continue;
        }

        let new_flow_rate = nodes[f].flow * (minutes - steps);

        let rest = flows
            .iter()
            .filter(|v| **v != f)
            .cloned()
            .collect::<Vec<_>>();
        choices.push(new_flow_rate + find_best(f, &rest, nodes, paths, minutes - steps));
    }
    *choices.iter().max().unwrap_or(&0)
}

fn find_path<'a>(
    hm: &'a HashMap<String, Location>,
    start: &'a String,
    end: &'a String,
) -> Vec<&'a String> {
    let mut deq = VecDeque::new();
    deq.push_back(Vec::from([start]));
    let mut path;
    while deq.len() > 0 {
        path = deq.pop_front().unwrap();
        let check = path[path.len() - 1];
        if check == end {
            return path;
        }

        'outer: for n in &hm[check].neighbors {
            // Check for loops. TODO()- use a hash
            for check in &path {
                if **check == *n {
                    continue 'outer;
                }
            }
            let mut newpath = path.clone();
            newpath.push(n);
            deq.push_back(newpath);
        }
    }
    return Vec::new();
}
