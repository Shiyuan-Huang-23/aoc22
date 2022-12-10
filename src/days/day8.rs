use std::cmp::max;
use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    // parse grid
    let grid: Vec<Vec<i32>> = parse_grid();

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    // trees that are visible
    let mut is_visible: Vec<Vec<bool>> = Vec::new();
    for _ in 0..num_rows {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..num_cols {
            row.push(false);
        }
        is_visible.push(row);
    }
    // trees visible from left or right
    for r in 0..num_rows {
        let left_visible = get_visible(grid[r].clone());
        let mut reversed_row = grid[r].clone();
        reversed_row.reverse();
        let right_visible = get_visible(reversed_row);
        for c in 0..num_cols {
            is_visible[r][c] = is_visible[r][c] || left_visible[c] || right_visible[num_cols - c - 1];
        }
    }
    // trees visible from top or bottom
    for c in 0..num_cols {
        let mut col: Vec<i32> = Vec::new();
        for r in 0..num_rows {
            col.push(grid[r][c]);
        }
        let top_visible = get_visible(col.clone());
        col.reverse();
        let bottom_visible = get_visible(col.clone());
        for r in 0..num_rows {
            is_visible[r][c] = is_visible[r][c] || top_visible[r] || bottom_visible[num_rows - r - 1];
        }
    }
    // count number of visible trees
    let mut total_visible = 0;
    for row in is_visible {
        for b in row {
            if b {
                total_visible += 1;
            }
        }
    }
    println!("Total trees visible: {}", total_visible);
}

// get trees visible if looking from the left
fn get_visible(heights: Vec<i32>) -> Vec<bool> {
    let mut curr_tallest = -1;
    let mut is_visible: Vec<bool> = Vec::new();
    for height in heights {
        if height > curr_tallest {
            is_visible.push(true);
            curr_tallest = height;
        } else {
            is_visible.push(false);
        }
    }
    return is_visible;
}

fn part2() {
    let grid = parse_grid();
    let mut max_scenic_score = 0;
    // ignore trees on edge
    for r in 1..(grid.len() - 1) {
        for c in 1..(grid[r].len() - 1) {
            let score = calc_scenic_score(grid.clone(), r, c);
            max_scenic_score = max(max_scenic_score, score);
        }
    }
    println!("Highest scenic score: {}", max_scenic_score);
}

fn calc_scenic_score(grid: Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
    let mut score = 1;
    let height = grid[r][c];
    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;

    let steps: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in steps {
        let mut view = 0;
        let mut curr_r: i32 = (r as i32) + dr;
        let mut curr_c: i32 = (c as i32) + dc;
        while 0 <= curr_r && curr_r < num_rows && 0 <= curr_c && curr_c < num_cols {
            view += 1;
            if grid[curr_r as usize][curr_c as usize] >= height {
                break;
            }
            curr_r += dr;
            curr_c += dc;
        }
        score *= view;
    }
    return score;
}

fn parse_grid() -> Vec<Vec<i32>> {
    let reader = util::get_day_reader(8);
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let offset = '0' as i32;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push((c as i32) - offset);
        }
        grid.push(row);
    }
    return grid;
}