use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let x = 'X' as i32;
    let a = 'A' as i32;
    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.split(" ").collect();
        let your_offset = vec[1].chars().nth(0).unwrap() as i32 - x;
        let opponent_offset = vec[0].chars().nth(0).unwrap() as i32 - a;
        let mut score = 0;
        let diff = your_offset - opponent_offset;
        if diff == 0 {
            score = 3;
        } else if diff == 1 || diff == -2 {
            score = 6;
        }
        score += your_offset + 1;
        total_score += score;
    }
    println!("Total score: {}", total_score);
}

fn part2() {
    let file = File::open("inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let x = 'X' as i32;
    let a = 'A' as i32;
    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.split(" ").collect();
        let your_offset = vec[1].chars().nth(0).unwrap() as i32 - x;
        let opponent_offset = vec[0].chars().nth(0).unwrap() as i32 - a;
        let mut score = your_offset * 3;
        if your_offset == 1 {
            // if you tie, you played the same shape as your opponent
            score += opponent_offset + 1;
        } else if your_offset == 0 {
            // if you lose, you played the shape that lost to your opponent
            if opponent_offset >= 1 {
                score += opponent_offset;
            } else {
                score += 3;
            }
        } else {
            // if you win, you played the shape that beat your opponent
            if opponent_offset < 2 {
                score += opponent_offset + 2;
            } else {
                score += 1;
            }
        }
        total_score += score;
    }
    println!("Total score: {}", total_score)
}