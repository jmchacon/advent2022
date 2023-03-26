//! day2 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
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

enum Rps {
    Rock,
    Paper,
    Scissors,
}

enum Res {
    Lose,
    Draw,
    Win,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;
    let mut score2 = 0;
    for (line_num, line) in lines.flatten().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() == 2, "{}: invalid - {line}", line_num + 1);

        let play1 = match *fields.first().unwrap() {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("invalid {} - {line}", line_num + 1),
        };
        let (play2, exp) = match *fields.get(1).unwrap() {
            "X" => (Rps::Rock, Res::Lose),
            "Y" => (Rps::Paper, Res::Draw),
            "Z" => (Rps::Scissors, Res::Win),
            _ => panic!("invalid {} - {line}", line_num + 1),
        };

        score += match (&play1, play2) {
            (Rps::Rock, Rps::Rock) => 1 + 3,
            (Rps::Rock, Rps::Paper) => 2 + 6,
            (Rps::Rock, Rps::Scissors) => 3, //  + 0,
            (Rps::Paper, Rps::Rock) => 1,    //  + 0,
            (Rps::Paper, Rps::Paper) => 2 + 3,
            (Rps::Paper, Rps::Scissors) => 3 + 6,
            (Rps::Scissors, Rps::Rock) => 1 + 6,
            (Rps::Scissors, Rps::Paper) => 2, //  + 0,
            (Rps::Scissors, Rps::Scissors) => 3 + 3,
        };
        score2 += match (&play1, exp) {
            (Rps::Rock, Res::Lose) => 3, // + 0,
            (Rps::Rock, Res::Draw) => 1 + 3,
            (Rps::Rock, Res::Win) => 2 + 6,
            (Rps::Paper, Res::Lose) => 1, //  + 0,
            (Rps::Paper, Res::Draw) => 2 + 3,
            (Rps::Paper, Res::Win) => 3 + 6,
            (Rps::Scissors, Res::Lose) => 2, //  + 0,
            (Rps::Scissors, Res::Draw) => 3 + 3,
            (Rps::Scissors, Res::Win) => 1 + 6,
        }
    }
    println!("part1: {score}");
    println!("part2: {score2}");
    Ok(())
}
