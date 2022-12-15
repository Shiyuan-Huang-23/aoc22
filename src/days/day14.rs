use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
}

fn part1() {
    let reader = util::get_day_reader(14);
    for line in reader.lines() {
        let line = line.unwrap();
        let coords: Vec<&str> = line.split(" -> ").collect().unwrap();
    }
}
