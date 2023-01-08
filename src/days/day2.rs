use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let scores: Vec<i32> =
        parse_guide()
            .iter()
            .map(|&(opp_shape, your_shape)| {
                let diff = your_shape - opp_shape;
                let outcome_score =
                    match diff {
                        0 => 3, // tie
                        1 | -2 => 6, // win
                        _ => 0, // loss
                    };
                let shape_score = your_shape + 1;
                outcome_score + shape_score
            })
            .collect();
    let total_score: i32 = scores.iter().sum();
    println!("Total score: {}", total_score);
}

fn part2() {
    let scores: Vec<i32> =
        parse_guide()
            .iter()
            .map(|&(opp_shape, outcome)| {
                let outcome_score = outcome * 3;
                let shape_score =
                    match outcome {
                        0 => {
                            // if you lose, you played the shape that lost to your opponent
                            if opp_shape >= 1 { opp_shape } else { 3 }
                        }
                        1 => {
                            // if you tie, you played the same shape as your opponent
                            opp_shape + 1
                        }
                        _ => {
                            // if you win, you played the shape that beat your opponent
                            if opp_shape < 2 { opp_shape + 2 } else { 1 }
                        }
                    };
                outcome_score + shape_score
            })
            .collect();
    let total_score: i32 = scores.iter().sum();
    println!("Total score: {}", total_score)
}

// Returns: a pair of offsets for each line in the strategy guide
// (0, 0) represents ('A', 'X'), (1, 1) => ('B', 'Y'), (2, 2) => ('C', 'Z')
fn parse_guide() -> Vec<(i32, i32)> {
    let reader = util::get_day_reader(2);
    let x = 'X' as i32;
    let a = 'A' as i32;
    let lines: Vec<(char, char)> =
        reader
            .lines()
            .filter(|x| !x.as_ref().unwrap().is_empty())
            .map(|x| {
                let chars: Vec<char> = x.unwrap().chars().collect();
                (chars[0], chars[2])
            })
            .collect();
    let offsets: Vec<(i32, i32)> =
        lines
            .iter()
            .map(|(c1, c2)| (*c1 as i32 - a, *c2 as i32 - x))
            .collect();
    return offsets;
}