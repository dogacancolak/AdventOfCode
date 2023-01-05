use std::io::{BufRead, BufReader};
use std::fs::File;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";
const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

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
    // Part 1
    calculate_my_score();

    // Part 2
    calculate_my_score_different_rules();
}


// Part 1 main function
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
        (PlayMove::Rock, PlayMove::Paper) => PlayResult::Win,
        (PlayMove::Rock, PlayMove::Scissors) => PlayResult::Lose,
        (PlayMove::Paper, PlayMove::Scissors) => PlayResult::Win,
        (PlayMove::Paper, PlayMove::Rock) => PlayResult::Lose,
        (PlayMove::Scissors, PlayMove::Rock) => PlayResult::Win,
        (PlayMove::Scissors, PlayMove::Paper) => PlayResult::Lose,
        _ => PlayResult::Draw,
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

// Part 2 main
fn calculate_my_score_different_rules() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let file = BufReader::new(file);

    let mut total_score = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let row: Vec<&str> = line.split(" ").collect();
        let opponent_move = parse_move(row[0]);
        let result = parse_result(row[1]);
        let my_move = get_move(&opponent_move, &result);
        total_score += evaluate_score(&my_move, &result);
    }
    println!("The total score with different rules is {}", total_score);
}

fn parse_result(play_move: &str) -> PlayResult {
    match play_move {
        LOSE => PlayResult::Lose,
        DRAW => PlayResult::Draw,
        WIN => PlayResult::Win,
        _ => panic!("Unrecognized move {}", play_move)
    }
}

fn get_move(opponent_move: &PlayMove, result: &PlayResult) -> PlayMove {
    match result {
        PlayResult::Lose => get_move_to_lose(&opponent_move),
        PlayResult::Draw => get_move_to_draw(&opponent_move),
        PlayResult::Win => get_move_to_win(&opponent_move),
    }
}

fn get_move_to_lose(opponent_move: &PlayMove) -> PlayMove {
    match opponent_move {
        PlayMove::Rock => PlayMove::Scissors,
        PlayMove::Paper => PlayMove::Rock,
        PlayMove::Scissors => PlayMove::Paper,
    }
}

fn get_move_to_draw(opponent_move: &PlayMove) -> PlayMove {
    match opponent_move {
        PlayMove::Rock => PlayMove::Rock,
        PlayMove::Paper => PlayMove::Paper,
        PlayMove::Scissors => PlayMove::Scissors,
    }
}

fn get_move_to_win(opponent_move: &PlayMove) -> PlayMove {
    match opponent_move {
        PlayMove::Rock => PlayMove::Paper,
        PlayMove::Paper => PlayMove::Scissors,
        PlayMove::Scissors => PlayMove::Rock,
    }
}
