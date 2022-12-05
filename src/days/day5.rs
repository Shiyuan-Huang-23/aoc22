use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    crates_helper(false);
}

fn part2() {
    crates_helper(true);
}

fn crates_helper(use_new_crane: bool) {
    let reader = util::get_day_reader(5);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks_parsed = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            stacks_parsed = true;
            continue;
        }
        if !stacks_parsed && stacks.is_empty() {
            let n = (line.len() + 2) / 4;
            for _ in 0..n {
                stacks.push(Vec::new());
            }
        }
        if !stacks_parsed {
            // parse stacks
            for (i, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    let index = (i + 1) / 4;
                    stacks[index].insert(0, c);
                }
            }
        } else {
            // move crates around
            let words : Vec<&str> = line.split(" ").collect();
            let num_crates : usize = words[1].parse().unwrap();
            let from : usize = words[3].parse().unwrap();
            let to : usize = words[5].parse().unwrap();
            let mut stack: Vec<char> = Vec::new();
            for _ in 0..num_crates {
                let c = stacks[from - 1].pop().unwrap();
                if use_new_crane {
                    stack.push(c);
                } else {
                    stacks[to - 1].push(c);
                }
            }
            if use_new_crane {
                while !stack.is_empty() {
                    let c = stack.pop().unwrap();
                    stacks[to - 1].push(c);
                }
            }
        }
    }
    for stack in stacks {
        if stack.is_empty() {
            print!(" ")
        } else {
            print!("{}", stack.last().unwrap());
        }
    }
    println!()
}