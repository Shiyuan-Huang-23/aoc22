use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
}

fn part1() {
    let reader = util::get_day_reader(24);
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    let mut blizzards: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let directions: HashMap<char, (i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0)),
    ]);

    // parse input
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    for (r, line) in lines.iter().enumerate() {
        for (c, tile) in line.chars().enumerate() {
            let coords = (r as i32, c as i32);
            if tile == '#' {
                rocks.insert(coords);
            } else if tile != '.' {
                blizzards.insert(coords, vec![tile]);
            } else if tile == '.' && r == 0 {
                start = coords;
            } else if tile == '.' && r == lines.len() - 1 {
                end = coords;
            }
        }
    }

    // explore all possible valid positions that can be occupied at each time step
    let num_rows = lines.len() as i32;
    let num_cols = lines[0].len() as i32;
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert(start);
    let moves: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];
    let mut num_minutes = 0;
    let mut end_reached = false;
    while !end_reached {
        // update blizzard positions
        let mut temp_blizzards = HashMap::new();
        for ((r, c), v) in &blizzards {
            for dir in v {
                let (dr, dc) = *directions.get(dir).unwrap();
                let mut cand = (*r + dr, *c + dc);
                // blizzard wraps around if necessary
                if cand.0 == 0 {
                    cand = (num_rows - 2, cand.1);
                } else if cand.0 == num_rows - 1 {
                    cand = (1, cand.1);
                } else if cand.1 == 0 {
                    cand = (cand.0, num_cols - 2);
                } else if cand.1 == num_cols - 1 {
                    cand = (cand.0, 1);
                }
                if !temp_blizzards.contains_key(&cand) {
                    temp_blizzards.insert(cand, vec![*dir]);
                } else {
                    let mut temp_v = temp_blizzards.get_mut(&cand).unwrap();
                    temp_v.push(*dir);
                }
            }
        }
        blizzards = temp_blizzards;

        num_minutes += 1;
        let mut temp_positions = HashSet::new();
        // explore valid positions based on currently-occupied positions
        for (r, c) in &positions {
            for (dr, dc) in &moves {
                let cand = (*r + *dr, *c + *dc);
                if !rocks.contains(&cand) && !blizzards.contains_key(&cand) && cand.0 >= 0 {
                    temp_positions.insert(cand);
                }
                if cand == end {
                    end_reached = true;
                    break;
                }
            }
            if end_reached {
                break;
            }
        }
        positions = temp_positions;
    }
    println!("Number of minutes needed to reach goal: {}", num_minutes);
}