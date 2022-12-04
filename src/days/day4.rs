use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(4);
    let mut res = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let sections = parse_line(line);
        let first = sections[0] <= sections[2] && sections[1] >= sections[3];
        let second = sections[2] <= sections[0] && sections[3] >= sections[1];
        if first || second {
            res += 1;
        }
    }
    println!("The result is: {}", res);
}

fn part2() {
    let reader = util::get_day_reader(4);
    let mut res = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let sections = parse_line(line);
        let first = contains(sections[0], sections[1], sections[2]) || contains(sections[0], sections[1], sections[3]);
        let second = contains(sections[2], sections[3], sections[0]) || contains(sections[2], sections[3], sections[1]);
        if first || second {
            res += 1
        }
    }
    println!("The result is: {}", res);
}

fn parse_line(line: String) -> Vec<i32> {
    let result = str::replace(&line, "-", ",");
    let sections: Vec<&str> = result.split(",").collect();
    return sections.iter().map(|x| x.parse().unwrap()).collect();
}

fn contains(start: i32, end: i32, point: i32) -> bool {
    return start <= point && point <= end;
}