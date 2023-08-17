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

pub struct RpcMatch {
    our_move: ValidHand,
    their_move: ValidHand,
}

pub trait CompareMoves {
    fn rpc_compare(&self) -> i32;
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

fn play_match(our_move: ValidHand, thier_move: ValidHand) -> MatchResult {
    let our_move_beats = our_move.beats();

    match (our_move_beats, our_move) {
        _ if our_move_beats == thier_move => MatchResult::Win(6),
        _ if our_move == thier_move => MatchResult::Draw(3),
        _ => MatchResult::Loss(0),
    }
}

impl CompareMoves for RpcMatch {
    fn rpc_compare(&self) -> i32 {
        let move_score = match self.our_move {
            ValidHand::Rock => 1,
            ValidHand::Paper => 2,
            ValidHand::Scissors => 3,
        };
        let result = play_match(self.our_move, self.their_move);
        match result {
            MatchResult::Win(i) => i + move_score,
            MatchResult::Draw(i) => i + move_score,
            MatchResult::Loss(i) => i + move_score,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Split the input and collect
    let v: Vec<&str> = buffer.trim().split('\n').collect();
    let v2: Vec<String> = v.iter()
        .map(|s| s.replace(" ", ""))
        .collect();
    
    for n in v2 {
        let iter: Vec<_> = n.chars().collect();
        let mut new_iter = iter.chunks_exact(2);
        while let Some(slice) = new_iter.next() {
            if let [first, second] = slice {
                println!("{} / {}", first, second);
            }
        }
    }


    // for pair in v2 {
    //     let mut iter = pair.chunks_exact(2);
    // }

    // let mut match_collection: Vec<RpcMatch> = Vec::new();
    // while let Some(chunk) = iter.next() {
    //     if let [first, second] = chunk {
    //         println!("Result: {} / {}", first, second);
    //     }
    // }
        
    Ok(())
}
