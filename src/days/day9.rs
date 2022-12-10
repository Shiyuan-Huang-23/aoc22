use std::collections::{HashMap, HashSet};
use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    helper(2);
}

fn part2() {
    helper(10);
}

fn helper(num_knots: usize) {
    let reader = util::get_day_reader(9);
    let moves: HashMap<&str, (i32, i32)> =
        HashMap::from([("L", (-1, 0)), ("R", (1, 0)), ("U", (0, 1)), ("D", (0, -1))]);
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut pos: Vec<Vec<i32>> = Vec::new();
    for _ in 0..num_knots {
        pos.push(vec![0, 0]);
    }
    for line in reader.lines() {
        let line = line.unwrap();
        let info: Vec<&str> = line.split_whitespace().collect();
        let (dx, dy) = *moves.get(info[0]).unwrap();
        let num_steps: i32 = info[1].parse().unwrap();
        for _ in 0..num_steps {
            // move the head
            pos[0][0] += dx;
            pos[0][1] += dy;
            // each knot follows the previous knot
            for i in 1..num_knots {
                let diff_x= pos[i - 1][0] - pos[i][0];
                let diff_y= pos[i - 1][1] - pos[i][1];
                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    pos[i][0] += diff_x.signum();
                    pos[i][1] += diff_y.signum();
                }
                // tail may have visited a new position
                if i + 1 == num_knots {
                    visited.insert((pos[i][0], pos[i][1]));
                }
            }
        }
    }
    println!("Positions visited: {}", visited.len());
}