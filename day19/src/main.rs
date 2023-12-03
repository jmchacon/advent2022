//! day19 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::{Display, EnumCount as EnumCountMacro};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = 24)]
    turns: usize,

    #[arg(long, default_value_t = 32)]
    turns2: usize,
}

#[derive(Clone, Debug, Display, EnumCountMacro, Eq, Hash, PartialEq)]
enum Robot {
    Ore(usize),
    Clay(usize),
    Obsidion(usize, usize),
    Geode(usize, usize),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut blueprints = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 32, "{} - bad line {line}", line_num + 1);

        let mut entry = Vec::new();

        let ore = Robot::Ore(parts[6].parse::<usize>()?);
        entry.push(ore);
        let clay = Robot::Clay(parts[12].parse::<usize>()?);
        entry.push(clay);
        let obsidion = Robot::Obsidion(parts[18].parse::<usize>()?, parts[21].parse::<usize>()?);
        entry.push(obsidion);
        let geode = Robot::Geode(parts[27].parse::<usize>()?, parts[30].parse::<usize>()?);
        entry.push(geode);
        blueprints.push(entry);
    }

    let mut quality = Vec::new();
    let mut max_ores = Vec::new();

    for b in &blueprints {
        let mut max_ore = 0;
        for r in b {
            match r {
                Robot::Ore(o) | Robot::Clay(o) | Robot::Obsidion(o, _) | Robot::Geode(o, _) => {
                    max_ore = max_ore.max(*o);
                }
            }
        }
        if args.debug {
            println!("{b:?} - {max_ore}");
        }
        max_ores.push(max_ore);
    }
    for i in 0..blueprints.len() {
        let q = build(
            args.turns,
            max_ores[i],
            &blueprints[i],
            [0, 0, 0, 0],
            [1, 0, 0, 0],
        );
        if args.debug {
            println!("{:?} quality - {q}", blueprints[i]);
        }
        quality.push(q);
    }
    let mut sum = 0;
    for (i, v) in quality.iter().enumerate() {
        sum += (i + 1) * v;
    }
    println!("part1 - {sum}");

    quality.clear();
    for i in 0..3.min(blueprints.len()) {
        let q = build(
            args.turns2,
            max_ores[i],
            &blueprints[i],
            [0, 0, 0, 0],
            [1, 0, 0, 0],
        );
        if args.debug {
            println!("{:?} quality - {q}", blueprints[i]);
        }
        quality.push(q);
    }
    sum = 1;
    for i in quality {
        sum *= i;
    }
    println!("part2 - {sum}");
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn build(
    turns: usize,
    max_ore: usize,
    blueprint: &Vec<Robot>,
    rocks: [usize; 4],
    robots: [usize; 4],
) -> usize {
    // No turns left return the geodes we've mined.
    if turns == 0 {
        return rocks[3];
    }
    let mut choices = Vec::new();
    let Robot::Ore(ore_ore) = blueprint[0] else {
        panic!()
    };
    let Robot::Clay(clay_ore) = blueprint[1] else {
        panic!()
    };
    let Robot::Obsidion(obs_ore, obs_clay) = blueprint[2] else {
        panic!()
    };
    let Robot::Geode(geode_ore, geode_obs) = blueprint[3] else {
        panic!()
    };
    let (new_ore, new_clay, new_obsidion, new_geode) = (robots[0], robots[1], robots[2], robots[3]);
    if turns > 1 && rocks[0] >= geode_ore && rocks[2] >= geode_obs {
        // Check geode robot first.
        choices.push(build(
            turns - 1,
            max_ore,
            blueprint,
            [
                rocks[0] - geode_ore + new_ore,
                rocks[1] + new_clay,
                rocks[2] - geode_obs + new_obsidion,
                rocks[3] + new_geode,
            ],
            [robots[0], robots[1], robots[2], robots[3] + 1],
        ));
    } else {
        if turns > 2
            && rocks[0] >= obs_ore
            && rocks[1] >= obs_clay
            && robots[2] < geode_obs
            && (robots[2] * turns + rocks[2] < turns * geode_obs)
        {
            // Check obsidion robot.
            choices.push(build(
                turns - 1,
                max_ore,
                blueprint,
                [
                    rocks[0] - obs_ore + new_ore,
                    rocks[1] - obs_clay + new_clay,
                    rocks[2] + new_obsidion,
                    rocks[3] + new_geode,
                ],
                [robots[0], robots[1], robots[2] + 1, robots[3]],
            ));
        }
        if turns > 6
            && rocks[0] >= clay_ore
            && robots[1] < obs_clay
            && (robots[1] * turns + rocks[1] < turns * obs_clay)
        {
            // Check clay robot.
            choices.push(build(
                turns - 1,
                max_ore,
                blueprint,
                [
                    rocks[0] - clay_ore + new_ore,
                    rocks[1] + new_clay,
                    rocks[2] + new_obsidion,
                    rocks[3] + new_geode,
                ],
                [robots[0], robots[1] + 1, robots[2], robots[3]],
            ));
        }
        if turns > 2
            && rocks[0] >= ore_ore
            && robots[0] < max_ore
            && (robots[0] * turns + rocks[0] < turns * max_ore)
        {
            // Finally check ore robot.
            choices.push(build(
                turns - 1,
                max_ore,
                blueprint,
                [
                    rocks[0] - ore_ore + new_ore,
                    rocks[1] + new_clay,
                    rocks[2] + new_obsidion,
                    rocks[3] + new_geode,
                ],
                [robots[0] + 1, robots[1], robots[2], robots[3]],
            ));
        } else {
            // Always do a run where just wait
            choices.push(build(
                turns - 1,
                max_ore,
                blueprint,
                [
                    rocks[0] + new_ore,
                    rocks[1] + new_clay,
                    rocks[2] + new_obsidion,
                    rocks[3] + new_geode,
                ],
                [robots[0], robots[1], robots[2], robots[3]],
            ));
        }
    }
    *choices.iter().max().unwrap()
}
