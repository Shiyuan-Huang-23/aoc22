use std::collections::HashSet;
use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(3);
    let mut first = HashSet::new();
    let mut second = HashSet::new();
    let mut total_priority = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.split("").collect();
        let n = vec.len() / 2;
        for (i, s) in vec.iter().enumerate() {
            if i < n {
                first.insert(s.to_string());
            } else {
                second.insert(s.to_string());
            }
        }
        let intersection: HashSet<_> = first.intersection(&second).collect();
        total_priority += get_priority(intersection);
        first.clear();
        second.clear();
    }
    println!("Total priority: {}", total_priority);
}

// 2694 is too high
fn part2() {
    let reader = util::get_day_reader(3);
    let mut set: HashSet<String> = HashSet::new();
    let mut total_priority = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.split("").collect();
        let mut curr_set: HashSet<String> = HashSet::new();
        for s in vec.iter() {
            curr_set.insert(s.to_string());
        }
        if i % 3 == 0 {
            set = curr_set;
        } else {
            let intersection: HashSet<&String> = set.intersection(&curr_set).collect();
            if i % 3 == 2 {
                total_priority += get_priority(intersection);
                set.clear();
            } else {
                set = intersection.iter().map(|x| x.to_string()).collect()
            }
        }
    }
    println!("Total priority: {}", total_priority);
}

fn get_priority(set: HashSet<&String>) -> i32 {
    for s in set.iter() {
        if !s.is_empty() {
            let c = s.chars().nth(0).unwrap();
            return if c.is_lowercase() {
                c as i32 - 96
            } else {
                // -38 = - 64 + 26
                c as i32 - 38
            }
        }
    }
    return 0;
}