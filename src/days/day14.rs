use std::cmp::max;
use std::collections::HashMap;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    // # represents rock, o represents sand
    let (mut cave, lowest_rock) = parse_cave();

    // simulate sand
    let mut abyss_reached = false;
    let mut num_sand_units = 0;
    while !abyss_reached {
        // release one grain of sand
        num_sand_units += 1;
        let mut sand_pos = (500, 0);
        let mut rested = false;
        // determine where the sand comes to rest
        while !rested {
            if sand_pos.1 == lowest_rock {
                abyss_reached = true;
                break;
            }
            // move either down, down to the left, or down to the right
            let down = (sand_pos.0, sand_pos.1 + 1);
            if !cave.contains_key(&down) {
                sand_pos = down;
                continue;
            }
            let down_to_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            if !cave.contains_key(&down_to_left) {
                sand_pos = down_to_left;
                continue;
            }
            let down_to_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if !cave.contains_key(&down_to_right) {
                sand_pos = down_to_right;
                continue;
            }
            // sand cannot move, and has come to rest
            rested = true;
            cave.insert(sand_pos, 'o');
        }
    }
    println!("Num units of sand before abyss: {}", num_sand_units - 1);
}

fn part2() {
    let (mut cave, lowest_rock) = parse_cave();

    // simulate sand
    let mut source_blocked = false;
    let mut num_sand_units = 0;
    while !source_blocked {
        // release one grain of sand
        num_sand_units += 1;
        let mut sand_pos = (500, 0);
        let mut rested = false;
        // determine where the sand comes to rest
        while !rested {
            // move either down, down to the left, or down to the right
            let down = (sand_pos.0, sand_pos.1 + 1);
            if !cave.contains_key(&down) && sand_pos.1 < lowest_rock + 1 {
                sand_pos = down;
                continue;
            }
            let down_to_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            if !cave.contains_key(&down_to_left) && sand_pos.1 < lowest_rock + 1 {
                sand_pos = down_to_left;
                continue;
            }
            let down_to_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if !cave.contains_key(&down_to_right) && sand_pos.1 < lowest_rock + 1 {
                sand_pos = down_to_right;
                continue;
            }
            // sand cannot move, and has come to rest
            rested = true;
            cave.insert(sand_pos, 'o');
            if sand_pos == (500, 0) {
                source_blocked = true;
                break;
            }
        }
    }
    println!("Num units of sand before source blocked: {}", num_sand_units);
}

// returns map of rock structures in cave, and lowest rock structure (rock with highest y coord)
fn parse_cave() -> (HashMap<(i32, i32), char>, i32) {
    let reader = util::get_day_reader(14);
    let mut cave: HashMap<(i32, i32), char> = HashMap::new();
    let mut lowest_rock = 0;
    // set up rock structures in cave
    for line in reader.lines() {
        let line = line.unwrap();
        let coordinates: Vec<&str> = line.split(" -> ").collect();
        let mut prev = get_coordinates(coordinates[0]);
        for i in 1..coordinates.len() {
            let curr = get_coordinates(coordinates[i]);
            // figure out what direction to move in
            let diff = curr.0 - prev.0 + curr.1 - prev.1;
            let sign = diff / diff.abs();
            let delta = if prev.0 == curr.0 { (0, sign) } else { (sign, 0) };
            // insert rocks in a line from prev rock to curr rock
            let mut pos = prev;
            while pos != curr {
                lowest_rock = max(lowest_rock, pos.1);
                cave.insert(pos, '#');
                pos = (pos.0 + delta.0, pos.1 + delta.1);
            }
            cave.insert(curr, '#');
            lowest_rock = max(lowest_rock, curr.1);
            prev = curr;
        }
    }
    return (cave, lowest_rock);
}

// converts a coordinate string "x,y" into a tuple (x,y)
fn get_coordinates(s: &str) -> (i32, i32) {
    let coordinates: Vec<&str> = s.split(",").collect();
    let x: i32 = coordinates[0].parse().unwrap();
    let y: i32 = coordinates[1].parse().unwrap();
    return (x, y);
}