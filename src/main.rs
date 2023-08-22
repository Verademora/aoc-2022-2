use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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
    fn is_beat_by(&self) -> Self;
}

impl Beats for ValidHand {
    // Returns the ValidHand the provided hand beats
    fn beats(&self) -> Self {
        match *self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    // Returns the ValidHand the provided hand is beat by
    fn is_beat_by(&self) -> Self {
        match *self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

// This function takes two ValidHands, plays out a match
// and returns a score
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

// This function takes the split characters from the rounds
// and translates them into ValidHand types
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

fn decode_part2(opponent_move: &str, match_result: &str) -> (ValidHand, ValidHand) {
    let opponents_hand = match opponent_move {
        "A" => ValidHand::Rock,
        "B" => ValidHand::Paper,
        "C" => ValidHand::Scissors,
        _ => panic!("Invalid"),
    };
    let result = match match_result {
        "X" => opponents_hand.beats(),
        "Y" => opponents_hand,
        "Z" => opponents_hand.is_beat_by(),
        _ => panic!("Invalid"),
    };
    (opponents_hand, result)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Split the input and collect
    let rounds: Vec<&str> = buffer.trim().split('\n').collect();

    let mut final_score: i32 = 0;
    let mut final_score_2: i32 = 0;

    for round in rounds {
        // Split each round into move pairs
        let move_pair: Vec<&str> = round.split(' ').collect();
        if let [first, second] = move_pair[..] {
            // Translate the pair into ValidHands and play a match
            let (their_move, our_move) = decode(first, second);
            let (their_move_2, our_move_2) = decode_part2(first, second);
            let match_result: i32 = play_match(our_move, their_move);
            let match_result_2: i32 = play_match(our_move_2, their_move_2);
            // Sum all the points as we go
            final_score += match_result;
            final_score_2 += match_result_2;
        }
    }

    println!("Final score part 1: {}", final_score);
    println!("Final score part 2: {}", final_score_2);

    Ok(())
}
