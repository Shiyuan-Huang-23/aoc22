use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(15);

    let target_row = 2000000;
    // x positions in target_row that are covered by sensors
    let mut covered_pos: HashSet<i32> = HashSet::new();
    // x positions where a beacon can be present, since a beacon is already there
    let mut beacons: HashSet<i32> = HashSet::new();

    // parse input + figure out coverage of each sensor in the given row
    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let sensor_pos = parse_pos(words[2].to_owned() + words[3].trim_end_matches(":"));
        let beacon_pos = parse_pos(words[8].to_owned() + words[9]);
        if beacon_pos.1 == target_row {
            beacons.insert(beacon_pos.0);
        }
        let cover_dist = (sensor_pos.0 - beacon_pos.0).abs() + (sensor_pos.1 - beacon_pos.1).abs();
        let target_dist = (sensor_pos.1 - target_row).abs();
        if cover_dist >= target_dist {
            covered_pos.insert(sensor_pos.0);
            let row_coverage = cover_dist - target_dist;
            for x in 1..(row_coverage + 1) {
                covered_pos.insert(sensor_pos.0 + x);
                covered_pos.insert(sensor_pos.0 - x);
            }
        }
    }
    // certain positions can contain beacons because there's already a beacon there
    for beacon in beacons {
        if covered_pos.contains(&beacon) {
            covered_pos.remove(&beacon);
        }
    }
    println!("Number of positions that cannot contain a beacon: {}", covered_pos.len());
}

fn part2() {
    let reader = util::get_day_reader(15);
    let n = 4000000;
    // intervals in each row that are covered by sensors
    let mut intervals: Vec<Vec<(i32, i32)>> = Vec::new();
    for _ in 0..(n + 1) {
        intervals.push(Vec::new());
    }
    // figure out intervals in each row covered by each sensor
    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let sensor_pos = parse_pos(words[2].to_owned() + words[3].trim_end_matches(":"));
        let beacon_pos = parse_pos(words[8].to_owned() + words[9]);
        for row in 0..(n + 1) {
            let cover_dist = (sensor_pos.0 - beacon_pos.0).abs() + (sensor_pos.1 - beacon_pos.1).abs();
            let row_dist = (sensor_pos.1 - row).abs();
            if cover_dist >= row_dist {
                let row_coverage: i32 = cover_dist - row_dist;
                let coverage = (max(0, sensor_pos.0 - row_coverage), min(n,sensor_pos.0 + row_coverage));
                intervals[row as usize].push(coverage);
            }
        }
    }
    // merge intervals in each row to figure out location of beacon
    let mut merged_intervals: Vec<Vec<(i32, i32)>> = Vec::new();
    for i in 0..intervals.len() {
        merged_intervals.push(Vec::new());
        let mut row = intervals[i].clone();
        row.sort();
        let mut merged_interval = row[0];
        for j in 1..row.len() {
            let (x1, y1) = row[j];
            if x1 <= merged_interval.1 + 1 {
                merged_interval = (merged_interval.0, max(merged_interval.1, y1));
            } else {
                let x: i128 = (merged_interval.1 + 1) as i128;
                let y = i as i128;
                println!("Beacon at (x={}, y={}) with tuning frequency: {}", x, y, x * 4000000 + y);
                merged_intervals[i].push(merged_interval);
                merged_interval = (x1, y1);
            }
        }
        if merged_intervals[i].is_empty() || *merged_intervals[i].last().unwrap() != merged_interval {
            merged_intervals[i].push(merged_interval);
        }
    }
}

// parses a string in the form x=coord1,y=coord2 to (coord1, coord2)
fn parse_pos(s: String) -> (i32, i32) {
    let info: Vec<&str> = s.split(",").collect();
    let x_info: Vec<&str> = info[0].split("=").collect();
    let y_info: Vec<&str> = info[1].split("=").collect();
    let x: i32 = x_info[1].parse().unwrap();
    let y: i32 = y_info[1].parse().unwrap();
    return (x, y);
}
