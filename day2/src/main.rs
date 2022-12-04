//! day2 advent 2022
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

enum RPS {
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
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;
    let mut score2 = 0;
    for (line_num, line) in lines.flatten().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() == 2, "{}: invalid - {line}", line_num + 1);

        let play1 = match fields[0] {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("invalid {} - {line}", line_num + 1),
        };
        let (play2, exp) = match fields[1] {
            "X" => (RPS::Rock, Res::Lose),
            "Y" => (RPS::Paper, Res::Draw),
            "Z" => (RPS::Scissors, Res::Win),
            _ => panic!("invalid {} - {line}", line_num + 1),
        };

        score += match (&play1, play2) {
            (RPS::Rock, RPS::Rock) => 1 + 3,
            (RPS::Rock, RPS::Paper) => 2 + 6,
            (RPS::Rock, RPS::Scissors) => 3 + 0,
            (RPS::Paper, RPS::Rock) => 1 + 0,
            (RPS::Paper, RPS::Paper) => 2 + 3,
            (RPS::Paper, RPS::Scissors) => 3 + 6,
            (RPS::Scissors, RPS::Rock) => 1 + 6,
            (RPS::Scissors, RPS::Paper) => 2 + 0,
            (RPS::Scissors, RPS::Scissors) => 3 + 3,
        };
        score2 += match (&play1, exp) {
            (RPS::Rock, Res::Lose) => 3 + 0,
            (RPS::Rock, Res::Draw) => 1 + 3,
            (RPS::Rock, Res::Win) => 2 + 6,
            (RPS::Paper, Res::Lose) => 1 + 0,
            (RPS::Paper, Res::Draw) => 2 + 3,
            (RPS::Paper, Res::Win) => 3 + 6,
            (RPS::Scissors, Res::Lose) => 2 + 0,
            (RPS::Scissors, Res::Draw) => 3 + 3,
            (RPS::Scissors, Res::Win) => 1 + 6,
        }
    }
    println!("Score: {score}");
    println!("Score2: {score2}");
    Ok(())
}
