//! day16 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

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

                if total_flow > max {
                    max = total_flow;
                    best = x.clone();
                }
            }
        }
        println!("AA -> {best:?} {max}");
    }
    let cur = String::from("AA");
    let flows = nodes.keys().cloned().collect::<Vec<_>>();
    let now = Instant::now();
    let ret = find_best(&cur, &flows, &nodes, &paths, 30);
    let elapsed = Instant::now().duration_since(now);
    println!("{elapsed:?} AA -> {:?} {}", ret.1, ret.0);

    // Generate all P(flows.len(), flows.len()/2) perms and then for each one of those run again on the remainder
    // NOTE: We can't just generate them all with itertools.permutate() since many paths have early ends (see test
    // in find_best/find_best2). So those have to expand the permutations manually and then prune whole subpaths that
    // can't work. This drastically reduces the search space to something which takes 5s to run.
    let mut v = Vec::<String>::new();
    let now = Instant::now();
    let new = find_best2(
        &cur,
        &flows,
        &nodes,
        &paths,
        26,
        0,
        &mut v,
        0,
        flows.len() / 2,
    );
    let elapsed = Instant::now().duration_since(now);
    println!("{elapsed:?} score {} from {:?}", new.0, new.1);

    Ok(())
}

fn find_best(
    cur: &String,
    flows: &Vec<&String>,
    nodes: &HashMap<&String, &Location>,
    paths: &HashMap<String, Vec<&String>>,
    minutes: usize,
) -> (usize, Vec<String>) {
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
        let mut new = find_best(f, &rest, nodes, paths, minutes - steps);
        let mut p = Vec::from([f.clone()]);
        p.append(&mut new.1);
        choices.push((new_flow_rate + new.0, p));
    }
    choices
        .iter()
        .max()
        .unwrap_or(&(0, Vec::from([cur.clone()])))
        .clone()
}

fn find_best2(
    cur: &String,
    flows: &Vec<&String>,
    nodes: &HashMap<&String, &Location>,
    paths: &HashMap<String, Vec<&String>>,
    minutes: usize,
    cur_flow: usize,
    cur_path: &mut Vec<String>,
    depth: usize,
    max_depth: usize,
) -> (usize, Vec<String>) {
    let mut choices = Vec::new();
    cur_path.push(cur.clone());
    for f in flows.iter().cloned() {
        if depth < max_depth {
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
            choices.push(find_best2(
                f,
                &rest,
                nodes,
                paths,
                minutes - steps,
                cur_flow + new_flow_rate,
                cur_path,
                depth + 1,
                max_depth,
            ));
        } else {
            // Here we've completed a valid P(X,R) based on depth. Now given the rest of
            // the nodes send that off to find_best to get 2nd path. We're looking for the max
            // of path1+path2 which isn't necessarily the max of "find the largest P(X,R) and then find
            // the value of the remainder" as ordering/weight can cause the first one to be slightly
            // under the max and the 2nd path picks up the slack.
            let mut p = cur_path.clone();
            p.push(f.clone());
            // Might get here with minutes remaining since the previous one above this didn't know
            // our depth was expiring. So we need to add on the remainder from its last flow rate.
            // Thankfully we know that since the last node in cur_path is the one to lookup and add.
            let mut cur_flow = cur_flow;
            if minutes > 1 {
                cur_flow += (minutes - 1) * nodes[&cur_path[cur_path.len() - 1]].flow;
            }
            let mut new = find_best(&cur_path[0], &flows, &nodes, &paths, 26);
            p.append(&mut new.1);
            choices.push((new.0 + cur_flow, p));
            break;
        }
    }
    cur_path.pop();
    choices
        .iter()
        .max()
        .unwrap_or(&(0, Vec::from([cur.clone()])))
        .clone()
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
