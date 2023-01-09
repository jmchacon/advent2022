//! day22 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
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

    #[arg(long, default_value_t = 50)]
    width: usize,

    #[arg(long, default_value_t = 1)]
    corners: usize,
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

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Block {
    corner: Location,
    forest: Vec<Vec<Forest>>,
    transforms: HashMap<Facing, (usize, Facing)>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let (corners, width) = match args.corners {
        1 => (
            [
                Location(50, 0),
                Location(100, 0),
                Location(50, 50),
                Location(0, 100),
                Location(50, 100),
                Location(0, 150),
            ],
            50,
        ),
        2 => (
            [
                Location(8, 0),
                Location(0, 4),
                Location(4, 4),
                Location(8, 4),
                Location(8, 8),
                Location(12, 8),
            ],
            4,
        ),
        _ => panic!(),
    };
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

    // Create blocks
    let mut blocks = Vec::new();
    let mut blocks2 = Vec::new();
    for (pos, c) in corners.iter().enumerate() {
        let (mut b, mut b2) = match (pos, args.corners) {
            (0, 1) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (1, Facing::East)),
                        (Facing::South, (2, Facing::South)),
                        (Facing::West, (1, Facing::West)),
                        (Facing::North, (4, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (1, Facing::East)),
                        (Facing::South, (2, Facing::South)),
                        (Facing::West, (3, Facing::East)),
                        (Facing::North, (5, Facing::East)),
                    ]),
                },
            ),
            (1, 1) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (0, Facing::East)),
                        (Facing::South, (1, Facing::South)),
                        (Facing::West, (0, Facing::West)),
                        (Facing::North, (1, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (4, Facing::West)),
                        (Facing::South, (2, Facing::West)),
                        (Facing::West, (0, Facing::West)),
                        (Facing::North, (5, Facing::North)),
                    ]),
                },
            ),
            (2, 1) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (2, Facing::East)),
                        (Facing::South, (4, Facing::South)),
                        (Facing::West, (2, Facing::West)),
                        (Facing::North, (0, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (1, Facing::North)),
                        (Facing::South, (4, Facing::South)),
                        (Facing::West, (3, Facing::South)),
                        (Facing::North, (0, Facing::North)),
                    ]),
                },
            ),
            (3, 1) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (4, Facing::East)),
                        (Facing::South, (5, Facing::South)),
                        (Facing::West, (4, Facing::West)),
                        (Facing::North, (5, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (4, Facing::East)),
                        (Facing::South, (5, Facing::South)),
                        (Facing::West, (0, Facing::East)),
                        (Facing::North, (2, Facing::East)),
                    ]),
                },
            ),
            (4, 1) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (3, Facing::East)),
                        (Facing::South, (0, Facing::South)),
                        (Facing::West, (3, Facing::West)),
                        (Facing::North, (2, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (1, Facing::West)),
                        (Facing::South, (5, Facing::West)),
                        (Facing::West, (3, Facing::West)),
                        (Facing::North, (2, Facing::North)),
                    ]),
                },
            ),
            (5, 1) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (5, Facing::East)),
                        (Facing::South, (3, Facing::South)),
                        (Facing::West, (5, Facing::West)),
                        (Facing::North, (3, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (4, Facing::North)),
                        (Facing::South, (1, Facing::South)),
                        (Facing::West, (0, Facing::South)),
                        (Facing::North, (3, Facing::North)),
                    ]),
                },
            ),
            (0, 2) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (0, Facing::East)),
                        (Facing::South, (3, Facing::South)),
                        (Facing::West, (0, Facing::West)),
                        (Facing::North, (4, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (5, Facing::West)),
                        (Facing::South, (3, Facing::South)),
                        (Facing::West, (2, Facing::South)),
                        (Facing::North, (1, Facing::South)),
                    ]),
                },
            ),
            (1, 2) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (2, Facing::East)),
                        (Facing::South, (1, Facing::South)),
                        (Facing::West, (3, Facing::West)),
                        (Facing::North, (1, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (2, Facing::East)),
                        (Facing::South, (4, Facing::North)),
                        (Facing::West, (5, Facing::North)),
                        (Facing::North, (0, Facing::South)),
                    ]),
                },
            ),
            (2, 2) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (3, Facing::East)),
                        (Facing::South, (2, Facing::South)),
                        (Facing::West, (1, Facing::West)),
                        (Facing::North, (2, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (3, Facing::East)),
                        (Facing::South, (4, Facing::East)),
                        (Facing::West, (1, Facing::West)),
                        (Facing::North, (0, Facing::East)),
                    ]),
                },
            ),
            (3, 2) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (1, Facing::East)),
                        (Facing::South, (4, Facing::South)),
                        (Facing::West, (2, Facing::West)),
                        (Facing::North, (0, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (5, Facing::South)),
                        (Facing::South, (4, Facing::South)),
                        (Facing::West, (2, Facing::West)),
                        (Facing::North, (0, Facing::North)),
                    ]),
                },
            ),
            (4, 2) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (5, Facing::East)),
                        (Facing::South, (0, Facing::South)),
                        (Facing::West, (5, Facing::West)),
                        (Facing::North, (3, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (5, Facing::East)),
                        (Facing::South, (1, Facing::North)),
                        (Facing::West, (2, Facing::North)),
                        (Facing::North, (3, Facing::North)),
                    ]),
                },
            ),
            (5, 2) => (
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (4, Facing::East)),
                        (Facing::South, (5, Facing::South)),
                        (Facing::West, (4, Facing::West)),
                        (Facing::North, (5, Facing::North)),
                    ]),
                },
                Block {
                    corner: c.clone(),
                    forest: Vec::new(),
                    transforms: HashMap::from([
                        (Facing::East, (0, Facing::West)),
                        (Facing::South, (1, Facing::East)),
                        (Facing::West, (4, Facing::West)),
                        (Facing::North, (3, Facing::West)),
                    ]),
                },
            ),
            _ => panic!(),
        };
        for y in 0..width {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(forest[c.1 + y][c.0 + x].clone());
            }
            b.forest.push(row.clone());
            b2.forest.push(row);
        }
        blocks.push(b);
        blocks2.push(b2);
    }

    for f in &forest {
        println!("{f:?}");
    }
    for (pos, b) in blocks.iter().enumerate() {
        println!("block {pos}");
        println!("corner: {}", b.corner);
        for f in &b.forest {
            println!("{f:?}");
        }
    }
    for (pos, b) in blocks2.iter().enumerate() {
        println!("block2 {pos}");
        println!("corner: {}", b.corner);
        for f in &b.forest {
            println!("{f:?}");
        }
    }

    println!("{path}");
    for m in &moves {
        println!("{m:?}");
    }

    let facing = Facing::East;
    let mut loc = Location(0, 0);
    for (pos, c) in forest[0].iter().enumerate() {
        if *c == Forest::Path {
            loc = Location(pos, 0);
            break;
        }
    }
    println!("starting at {loc:?} facing {facing}");
    let (loc, block_num, facing) = compute_path(&moves, &blocks);
    let b = &blocks2[block_num];
    println!("Block {block_num} at location {loc} facing {facing}");
    println!("{}", b.corner);

    let f: usize = match facing {
        Facing::North => 3,
        Facing::South => 1,
        Facing::East => 0,
        Facing::West => 2,
    };
    let part1 = 1000 * (b.corner.1 + loc.1 + 1) + 4 * (b.corner.0 + loc.0 + 1) + f;

    let (loc, block_num, facing) = compute_path(&moves, &blocks2);
    let b = &blocks2[block_num];
    println!("Block {block_num} at location {loc} facing {facing}");
    println!("{}", b.corner);
    let f: usize = match facing {
        Facing::North => 3,
        Facing::South => 1,
        Facing::East => 0,
        Facing::West => 2,
    };
    let part2 = 1000 * (b.corner.1 + loc.1 + 1) + 4 * (b.corner.0 + loc.0 + 1) + f;
    println!();
    println!("part1 {part1}");
    println!("part2 {part2}");
    Ok(())
}

fn compute_path(moves: &Vec<Move>, blocks: &Vec<Block>) -> (Location, usize, Facing) {
    let mut block_num = 0;
    let mut b = &blocks[block_num];
    let mut loc = Location(0, 0);
    let edge = b.forest.len() - 1;
    let mut facing = Facing::East;
    for (pos, c) in b.forest[0].iter().enumerate() {
        if *c == Forest::Path {
            loc = Location(pos, 0);
            break;
        }
    }
    let loc2 = (b.corner.0 + loc.0, b.corner.1 + loc.1);
    println!("Now facing {facing} at {loc2:?} in block {}", block_num + 1);
    for m in moves {
        match m {
            Move::Steps(s) => {
                for _ in 0..*s {
                    loc = match &facing {
                        Facing::North => {
                            // Easy..we aren't going off the block so just test for a wall.
                            if loc.1 != 0 {
                                if b.forest[loc.1 - 1][loc.0] == Forest::Path {
                                    Location(loc.0, loc.1 - 1)
                                } else {
                                    loc
                                }
                            } else {
                                // Otherwise find the block and new facing we might need.
                                let next = b.transforms.get(&facing).unwrap();
                                // If we're at the same facing, easy. Just check for path and if it works
                                // move to new block and position at bottom of it (for north here).
                                if next.1 == facing {
                                    let newy = edge;
                                    if blocks[next.0].forest[newy][loc.0] == Forest::Path {
                                        b = &blocks[next.0];
                                        block_num = next.0;
                                        Location(loc.0, newy)
                                    } else {
                                        loc
                                    }
                                } else {
                                    // Complicated part. Directions shifting.
                                    match &next.1 {
                                        Facing::East => {
                                            // N -> E is x = 0, y = old x
                                            let newy = loc.0;
                                            if blocks[next.0].forest[newy][0] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(0, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::West => {
                                            // N -> W is x = right edge, y = old x
                                            let newx = edge;
                                            let newy = loc.0;
                                            if blocks[next.0].forest[newy][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::South => {
                                            // N -> S is x = edge - oldx, y = oldy
                                            let newx = edge - loc.0;
                                            if blocks[next.0].forest[loc.1][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, loc.1)
                                            } else {
                                                loc
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                            }
                        }
                        Facing::South => {
                            // Easy..we aren't going off the block so just test for a wall.
                            if loc.1 != edge {
                                if b.forest[loc.1 + 1][loc.0] == Forest::Path {
                                    Location(loc.0, loc.1 + 1)
                                } else {
                                    loc
                                }
                            } else {
                                // Otherwise find the block and new facing we might need.
                                let next = b.transforms.get(&facing).unwrap();
                                // If we're at the same facing, easy. Just check for path and if it works
                                // move to new block and position at bottom of it (for north here).
                                if next.1 == facing {
                                    let newy = 0;
                                    if blocks[next.0].forest[newy][loc.0] == Forest::Path {
                                        b = &blocks[next.0];
                                        block_num = next.0;
                                        Location(loc.0, newy)
                                    } else {
                                        loc
                                    }
                                } else {
                                    // Complicated part. Directions shifting.
                                    match &next.1 {
                                        Facing::East => {
                                            // S -> E is x = 0, y = old x
                                            let newy = loc.0;
                                            if blocks[next.0].forest[newy][0] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(0, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::West => {
                                            // S -> W is x = right edge, y = old x
                                            let newx = edge;
                                            let newy = loc.0;
                                            if blocks[next.0].forest[newy][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::North => {
                                            // S -> N is x = edge - oldx, y = oldy
                                            let newx = edge - loc.0;
                                            if blocks[next.0].forest[loc.1][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, loc.1)
                                            } else {
                                                loc
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                            }
                        }
                        Facing::East => {
                            // Easy..we aren't going off the block so just test for a wall.
                            if loc.0 != edge {
                                if b.forest[loc.1][loc.0 + 1] == Forest::Path {
                                    Location(loc.0 + 1, loc.1)
                                } else {
                                    loc
                                }
                            } else {
                                // Otherwise find the block and new facing we might need.
                                let next = b.transforms.get(&facing).unwrap();
                                // If we're at the same facing, easy. Just check for path and if it works
                                // move to new block and position at bottom of it (for north here).
                                if next.1 == facing {
                                    if blocks[next.0].forest[loc.1][0] == Forest::Path {
                                        b = &blocks[next.0];
                                        block_num = next.0;
                                        Location(0, loc.1)
                                    } else {
                                        loc
                                    }
                                } else {
                                    // Complicated part. Directions shifting.
                                    match &next.1 {
                                        Facing::North => {
                                            // E -> N is x = oldy, y = bottom
                                            let newx = loc.1;
                                            let newy = edge;
                                            if blocks[next.0].forest[newy][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::South => {
                                            // E -> S is x = edge - oldy, y = 0
                                            let newx = edge - loc.1;
                                            if blocks[next.0].forest[0][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, 0)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::West => {
                                            // E -> W is x = oldx, y = edge - oldy
                                            let newy = edge - loc.1;
                                            if blocks[next.0].forest[newy][loc.0] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(loc.0, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                            }
                        }
                        Facing::West => {
                            // Easy..we aren't going off the block so just test for a wall.
                            if loc.0 != 0 {
                                if b.forest[loc.1][loc.0 - 1] == Forest::Path {
                                    Location(loc.0 - 1, loc.1)
                                } else {
                                    loc
                                }
                            } else {
                                // Otherwise find the block and new facing we might need.
                                let next = b.transforms.get(&facing).unwrap();
                                // If we're at the same facing, easy. Just check for path and if it works
                                // move to new block and position at bottom of it (for north here).
                                if next.1 == facing {
                                    if blocks[next.0].forest[loc.1][edge] == Forest::Path {
                                        b = &blocks[next.0];
                                        block_num = next.0;
                                        Location(edge, loc.1)
                                    } else {
                                        loc
                                    }
                                } else {
                                    // Complicated part. Directions shifting.
                                    match &next.1 {
                                        Facing::North => {
                                            // W -> N is x = oldy, y = bottom
                                            let newx = loc.1;
                                            let newy = edge;
                                            if blocks[next.0].forest[newy][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::South => {
                                            // W -> S is x = oldy, y = 0;
                                            let newx = loc.1;
                                            if blocks[next.0].forest[0][newx] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(newx, 0)
                                            } else {
                                                loc
                                            }
                                        }
                                        Facing::East => {
                                            // W -> E is x = oldx, y = edge - oldy
                                            let newy = edge - loc.1;
                                            if blocks[next.0].forest[newy][loc.0] == Forest::Path {
                                                b = &blocks[next.0];
                                                block_num = next.0;
                                                facing = next.1.clone();
                                                Location(loc.0, newy)
                                            } else {
                                                loc
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
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
        let loc2 = (b.corner.0 + loc.0, b.corner.1 + loc.1);
        println!(
            "Now facing {facing} at {loc2:?} ({loc:?}) in block {}",
            block_num + 1
        );
    }
    (loc, block_num, facing)
}
