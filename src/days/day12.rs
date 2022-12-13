use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    hill_helper(false);
    hill_helper(true);
}

fn hill_helper(explore_other_starts: bool) {
    let reader = util::get_day_reader(12);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut a_pos: Vec<(usize, usize)> = Vec::new();
    // parse input
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (grid.len(), i);
                row.push('a');
            } else if c == 'E' {
                end = (grid.len(), i);
                row.push('z');
            } else {
                if c == 'a' {
                    a_pos.push((grid.len(), i));
                }
                row.push(c);
            }
        }
        grid.push(row);
    }
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    // starting positions
    let starts: Vec<(usize, usize)> =
        if explore_other_starts {
            a_pos.push(start);
            a_pos
        } else {
            vec![start]
        };
    let mut min_steps = i32::MAX;
    for s in starts {
        let mut curr_min_steps = -1;
        let directions: Vec<(i32, i32)>= vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
        queue.push_back((s, 0));
        seen.insert(s);
        while !queue.is_empty() && curr_min_steps == -1 {
            let ((x, y), steps) = queue.pop_front().unwrap();
            for i in 0..directions.len() {
                let (dx, dy) = directions[i];
                let new_x = dx + x as i32;
                let new_y = dy + y as i32;
                let is_valid = new_x >= 0 && new_x < height && new_y >= 0 && new_y < width;
                if is_valid {
                    let new_coords = (new_x as usize, new_y as usize);
                    let height_diff = grid[new_coords.0][new_coords.1] as i32 - grid[x][y] as i32;
                    if !seen.contains(&new_coords) && height_diff <= 1 {
                        if new_coords == end {
                            curr_min_steps = steps + 1;
                            min_steps = min(min_steps, curr_min_steps);
                            break;
                        }
                        queue.push_back((new_coords, steps + 1));
                        seen.insert(new_coords);
                    }
                }
            }
        }
    }
    println!("Minimum steps: {}", min_steps);
}