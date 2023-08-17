use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValidHand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MatchResult {
    Win(i32),
    Loss(i32),
    Draw(i32),
}

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for ValidHand {
    fn beats(&self) -> Self {
        match *self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        } 
    }
}

fn play_match(our_move: ValidHand, thier_move: ValidHand) -> i32 {
    let our_move_beats = our_move.beats();
    let move_score = match our_move {
        ValidHand::Rock => 1,
        ValidHand::Paper => 2,
        ValidHand::Scissors => 3,
    };

    let result = match (our_move_beats, our_move) {
        _ if our_move_beats == thier_move => MatchResult::Win(6),
        _ if our_move == thier_move => MatchResult::Draw(3),
        _ => MatchResult::Loss(0),
    };

    match result {
            MatchResult::Win(i) => i + move_score,
            MatchResult::Draw(i) => i + move_score,
            MatchResult::Loss(i) => i + move_score,
    }
}

fn decode(opponent: &str, player: &str) -> (ValidHand, ValidHand) {
    let opponents_move = match opponent {
        "A" => ValidHand::Rock,
        "B" => ValidHand::Paper,
        "C" => ValidHand::Scissors,
        _ => panic!("Invalid"),
    };
    let player_move = match player {
        "X" => ValidHand::Rock,
        "Y" => ValidHand::Paper,
        "Z" => ValidHand::Scissors,
        _ => panic!("Invalid"),
    };
    (opponents_move, player_move)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Split the input and collect
    let v: Vec<&str> = buffer.trim().split('\n').collect();
    for set in v {
        let move_pair: Vec<&str> = set.split(' ').collect();
        if let [first, second] = move_pair[..] {
            let (their_move, our_move) = decode(first, second);
            let match_result: i32 = play_match(our_move, their_move);
            println!("Result: {}", match_result);
        }
    }

    Ok(())
}
