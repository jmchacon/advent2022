//! day22 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
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
enum Forest {
    Void,
    Path,
    Wall,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Move {
    Steps(usize),
    Left,
    Right,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(usize, usize);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut forest = Vec::new();
    let mut forest_done = false;
    let mut path = "";
    let mut moves = Vec::new();
    let mut max = 0;
    for (line_num, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            forest_done = true;
            continue;
        }
        if forest_done {
            path = line;
            let mut start = 0;
            for (pos, c) in line.bytes().enumerate() {
                match c {
                    b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {}
                    b'R' | b'L' => {
                        let m = usize::from_str_radix(&line[start..pos], 10).unwrap();
                        start = pos + 1;
                        moves.push(Move::Steps(m));
                        if c == b'R' {
                            moves.push(Move::Right);
                        } else {
                            moves.push(Move::Left);
                        }
                    }
                    _ => panic!("{} - bad line {line}", line_num + 1),
                }
            }
            // Get any trailing number.
            if start != line.len() {
                let m = usize::from_str_radix(&line[start..line.len()], 10).unwrap();
                moves.push(Move::Steps(m));
            }
            break;
        }
        let mut l = vec![Forest::Void; line.len()];
        if line.len() > max {
            max = line.len();
        }
        for (pos, c) in line.bytes().enumerate() {
            match c {
                b' ' => {}
                b'.' => l[pos] = Forest::Path,
                b'#' => l[pos] = Forest::Wall,
                _ => panic!("{} - bad line {line}", line_num + 1),
            }
        }
        forest.push(l);
    }

    // Fixup the grid. It may be asymetric now but we want each row to be the same length
    // (otherwise the logic below can be out of bounds). So for each row fill in anything
    // missing with a void.
    for f in &mut forest {
        for _ in f.len()..max {
            f.push(Forest::Void);
        }
    }
    for f in &forest {
        println!("{f:?}");
    }
    println!("{path}");
    for m in &moves {
        println!("{m:?}");
    }

    let mut facing = Facing::East;
    let mut loc = Location(0, 0);
    for (pos, c) in forest[0].iter().enumerate() {
        if *c == Forest::Path {
            loc = Location(pos, 0);
            break;
        }
    }
    println!("starting at {loc:?} facing {facing}");
    for m in &moves {
        println!("Moving {m:?}");
        match m {
            Move::Steps(s) => {
                for _ in 0..*s {
                    loc = match facing {
                        Facing::North => {
                            let mut new = if loc.1 == 0 {
                                Location(loc.0, forest.len() - 1)
                            } else {
                                Location(loc.0, loc.1 - 1)
                            };
                            // If the one above is blank, back up one
                            // and look south for path. The last one is our
                            // new spot.
                            println!("new {new:?}");
                            if forest[new.1][new.0] == Forest::Void {
                                for p in loc.1..forest.len() {
                                    if forest[p][new.0] != Forest::Void {
                                        new = Location(new.0, p);
                                    }
                                }
                            }
                            // If we can walk we update. Otherwise we stay put.
                            if forest[new.1][new.0] == Forest::Path {
                                new
                            } else {
                                loc
                            }
                        }
                        Facing::South => {
                            let mut new = if loc.1 == forest.len() - 1 {
                                Location(loc.0, 0)
                            } else {
                                Location(loc.0, loc.1 + 1)
                            };
                            // If the one below is blank, back up one
                            // and look north for path. The last one is our
                            // new spot.
                            if forest[new.1][new.0] == Forest::Void {
                                for p in (0..=loc.1).rev() {
                                    if forest[p][new.0] != Forest::Void {
                                        new = Location(new.0, p);
                                    }
                                }
                            }
                            // If we can walk we update. Otherwise we stay put.
                            if forest[new.1][new.0] == Forest::Path {
                                new
                            } else {
                                loc
                            }
                        }
                        Facing::East => {
                            let mut new = if loc.0 == forest[loc.1].len() - 1 {
                                Location(0, loc.1)
                            } else {
                                Location(loc.0 + 1, loc.1)
                            };
                            // If the one east is blank, back up one
                            // and look east for path. The last one is our
                            // new spot.
                            if forest[new.1][new.0] == Forest::Void {
                                for p in (0..=loc.0).rev() {
                                    if forest[new.1][p] != Forest::Void {
                                        new = Location(p, new.1);
                                    }
                                }
                            }
                            // If we can walk we update. Otherwise we stay put.
                            if forest[new.1][new.0] == Forest::Path {
                                new
                            } else {
                                loc
                            }
                        }
                        Facing::West => {
                            let mut new = if loc.0 == 0 {
                                Location(forest[loc.1].len() - 1, loc.1)
                            } else {
                                Location(loc.0 - 1, loc.1)
                            };
                            // If the one west is blank, back up one
                            // and look west for path. The last one is our
                            // new spot.
                            if forest[new.1][new.0] == Forest::Void {
                                for p in loc.0..forest[loc.1].len() {
                                    if forest[new.1][p] != Forest::Void {
                                        new = Location(p, new.1);
                                    }
                                }
                            }
                            // If we can walk we update. Otherwise we stay put.
                            if forest[new.1][new.0] == Forest::Path {
                                new
                            } else {
                                loc
                            }
                        }
                    }
                }
            }
            Move::Left => {
                facing = match facing {
                    Facing::North => Facing::West,
                    Facing::South => Facing::East,
                    Facing::East => Facing::North,
                    Facing::West => Facing::South,
                };
            }
            Move::Right => {
                facing = match facing {
                    Facing::North => Facing::East,
                    Facing::South => Facing::West,
                    Facing::East => Facing::South,
                    Facing::West => Facing::North,
                };
            }
        }
        println!("Now facing {facing} at {loc:?}");
    }
    let f: usize = match facing {
        Facing::North => 3,
        Facing::South => 1,
        Facing::East => 0,
        Facing::West => 2,
    };
    println!("final is {}", 1000 * (loc.1 + 1) + 4 * (loc.0 + 1) + f);
    Ok(())
}
