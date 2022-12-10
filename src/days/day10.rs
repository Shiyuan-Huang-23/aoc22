use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(10);
    let mut x = 1;
    let mut cycle_num = 1;
    let mut signal_sum = 0;
    for line in reader.lines() {
        let op = line.unwrap();
        let offset = if op == "noop" { 1 } else { 2 };
        for _ in 0..offset {
            if (cycle_num - 20) % 40 == 0 {
                signal_sum += cycle_num * x;
            }
            cycle_num += 1;
        }
        if op != "noop" {
            let info: Vec<&str> = op.split_whitespace().collect();
            let n: i32 = info[1].parse().unwrap();
            x += n;
        }
    }
    println!("Signal strength: {}", signal_sum);
}

fn part2() {
    let reader = util::get_day_reader(10);
    let width = 40;
    let height = 6;
    let mut grid: Vec<Vec<&str>> = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(".");
        }
        grid.push(row);
    }

    let mut x: i32 = 1;
    let mut cycle_num = 1;
    for line in reader.lines() {
        let op = line.unwrap();
        let offset = if op == "noop" { 1 } else { 2 };
        for _ in 0..offset {
            let r = (cycle_num - 1) / width;
            let c = (cycle_num - 1) % width;
            if c >= x - 1 && c <= x + 1 {
                grid[r as usize][c as usize] = "#";
            }
            cycle_num += 1;
        }
        if op != "noop" {
            let info: Vec<&str> = op.split_whitespace().collect();
            let n: i32 = info[1].parse().unwrap();
            x += n;
        }
    }
    println!("Pixels:");
    for r in 0..height {
        println!("{}", grid[r].clone().join(""));
    }
}