//! day8 advent 2022
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut map = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        let chars = line.split("");
        println!("{} - line {line}", line_num + 1);
        for c in chars {
            if c == "" {
                continue;
            }
            //println!("{} - processing {c}", line_num + 1);
            let v = usize::from_str_radix(c, 10).unwrap();
            row.push(v);
        }
        map.push(row);
    }
    let maxy = map.len() - 1;
    let maxx = map[0].len() - 1;
    let mut seen = Vec::new();
    for y in 0..=maxy {
        let mut row = Vec::new();
        let mut val = false;
        if y == 0 || y == maxx {
            val = true;
        }
        for x in 0..=maxx {
            if x == 0 || x == maxx {
                row.push(true);
            } else {
                row.push(val)
            }
        }
        seen.push(row);
    }
    let mut scenic = 0;
    for y in 0..=maxy {
        // edge always visible
        let mut tallest = map[y][0];
        for x in 0..=maxx {
            if map[y][x] > tallest && x != maxx {
                if y != 0 && y != maxy && x != 0 {
                    seen[y][x] = true;
                    tallest = map[y][x];
                }
            }
        }

        if y == 0 {
            for x in 0..=maxx {
                let mut tallest = map[0][x];
                for yy in 0..=maxy {
                    if map[yy][x] > tallest && yy != maxy {
                        if yy != 0 && x != 0 && x != maxx {
                            seen[yy][x] = true;
                            tallest = map[yy][x];
                        }
                    }
                }
            }
        }
        if y == maxy {
            for x in 0..=maxx {
                let mut tallest = map[maxy][x];
                for yy in (0..=maxy).rev() {
                    if map[yy][x] > tallest && yy != 0 {
                        if yy != maxy && x != 0 && x != maxx {
                            seen[yy][x] = true;
                            tallest = map[yy][x];
                        }
                    }
                }
            }
        }

        tallest = map[y][maxx];
        for x in (0..=maxx).rev() {
            if map[y][x] > tallest && x != 0 {
                if y != 0 && y != maxy && x != maxx {
                    seen[y][x] = true;
                    tallest = map[y][x];
                }
            }
        }

        if y != 0 && y != maxy {
            for x in 1..maxx {
                let here = map[y][x];
                let mut sum = 1;
                // look up
                let mut dist = 0;
                for yy in (0..=y - 1).rev() {
                    dist += 1;
                    if map[yy][x] >= here {
                        break;
                    }
                }
                sum *= dist;

                dist = 0;
                // look down
                for yy in y + 1..=maxy {
                    dist += 1;
                    if map[yy][x] >= here {
                        break;
                    }
                }
                sum *= dist;

                dist = 0;
                // look left
                for xx in (0..=x - 1).rev() {
                    dist += 1;
                    if map[y][xx] >= here {
                        break;
                    }
                }
                sum *= dist;

                dist = 0;
                // look right
                for xx in x + 1..=maxx {
                    dist += 1;
                    if map[y][xx] >= here {
                        break;
                    }
                }
                sum *= dist;

                if sum > scenic {
                    scenic = sum;
                }
            }
        }
    }
    let mut sum = 0;
    seen.iter().for_each(|v| {
        v.iter().for_each(|v| {
            if *v {
                sum += 1;
            }
        })
    });
    for y in 0..=maxy {
        println!("{:?}", seen[y]);
    }
    println!("sum: {sum}");
    println!("scenic: {scenic}");
    Ok(())
}
