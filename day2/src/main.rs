use std::io::{BufRead, BufReader};
use std::fs::File;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";
const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

enum PlayMove {
    Rock,
    Paper,
    Scissors
}

enum PlayResult {
    Win,
    Lose,
    Draw
}

fn main() {
    println!("Hello, world!");
    calculate_my_score();
}

fn calculate_my_score() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let file = BufReader::new(file);

    let mut total_score = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let row: Vec<&str> = line.split(" ").collect();
        let opponent_move = parse_move(row[0]);
        let my_move = parse_move(row[1]);
        let result = play_round(&opponent_move, &my_move);
        total_score += evaluate_score(&my_move, &result);
    }
    println!("The total score is {}", total_score);
}

fn parse_move(play_move: &str) -> PlayMove {
    if play_move == OPPONENT_ROCK || play_move == MY_ROCK {
        PlayMove::Rock
    }
    else if play_move == OPPONENT_PAPER || play_move == MY_PAPER {
        PlayMove::Paper
    }
    else if play_move == OPPONENT_SCISSORS || play_move == MY_SCISSORS {
        PlayMove::Scissors
    }
    else {
        panic!("Unrecognized move {}", play_move);
    }
}

fn play_round(opponent_move: &PlayMove, my_move: &PlayMove) -> PlayResult {
    match (opponent_move, my_move) {
        (PlayMove::Rock, PlayMove::Rock) => PlayResult::Draw,
        (PlayMove::Paper, PlayMove::Paper) => PlayResult::Draw,
        (PlayMove::Scissors, PlayMove::Scissors) => PlayResult::Draw,
        (PlayMove::Rock, PlayMove::Paper) => PlayResult::Win,
        (PlayMove::Rock, PlayMove::Scissors) => PlayResult::Lose,
        (PlayMove::Paper, PlayMove::Scissors) => PlayResult::Win,
        (PlayMove::Paper, PlayMove::Rock) => PlayResult::Lose,
        (PlayMove::Scissors, PlayMove::Rock) => PlayResult::Win,
        (PlayMove::Scissors, PlayMove::Paper) => PlayResult::Lose,
    }
}

fn evaluate_score(my_move: &PlayMove, result: &PlayResult) -> u32 {
    let mut score = 0;

    if let PlayMove::Rock = my_move {
        score += 1;
    }
    else if let PlayMove::Paper = my_move {
        score += 2;
    }
    else if let PlayMove::Scissors = my_move {
        score += 3;
    }

    if let PlayResult::Draw = result {
        score += 3;
    }
    else if let PlayResult::Win = result {
        score += 6;
    }

    return score;
}