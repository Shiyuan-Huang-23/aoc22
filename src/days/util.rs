use std::fs::File;
use std::io::{BufReader};

pub fn get_day_reader(day: i32) -> BufReader<File> {
    let file = File::open("inputs/day".to_owned() + &day.to_string() + ".txt").unwrap();
    return BufReader::new(file);
}

pub fn get_file_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    return BufReader::new(file);
}