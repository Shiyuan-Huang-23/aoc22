use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    marker_helper(4);
}

fn part2() {
    marker_helper(14);
}

fn marker_helper(marker_size: usize) {
    let reader = util::get_day_reader(6);
    for line in reader.lines() {
        let line : Vec<char> = line.unwrap().chars().collect();
        let mut count: [i32; 26] = [0; 26];
        let mut unique_chars = 0;
        for i in 0..marker_size {
            let index = get_index(line[i]);
            if count[index] == 0 {
                unique_chars += 1;
            }
            count[index] += 1;
        }
        if unique_chars == marker_size {
            display_message(marker_size);
            continue;
        }
        for i in marker_size..line.len() {
            let old_index = get_index(line[i - marker_size]);
            count[old_index] -= 1;
            if count[old_index] == 0 {
                unique_chars -= 1;
            }
            let index = get_index(line[i]);
            count[index] += 1;
            if count[index] == 1 {
                unique_chars += 1;
            }
            if unique_chars == marker_size {
                display_message(i + 1);
                break;
            }
        }
    }
}

fn display_message(n: usize) {
    println!("Characters to be processed before start-of-packet marker: {}", n);
}

fn get_index(c: char) -> usize {
    return (c as usize) - 97;
}