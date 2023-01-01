use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
}

// cost is in the form [ore, clay, obsidian]
#[derive(Debug)]
struct RobotCosts {
    ore_robot_cost: [i32; 3],
    clay_robot_cost: [i32; 3],
    obsidian_robot_cost: [i32; 3],
    geode_robot_cost: [i32; 3],
}

fn part1() {
    let reader = util::get_day_reader(19);
    for line in reader.lines() {
        let line = line.unwrap();
        let robot_costs = parse_blueprint(line);
    }
}

fn parse_blueprint(s: String) -> RobotCosts {
    let info: Vec<&str> = s.split_whitespace().collect();
    let ore_robot_cost = [info[6].parse().unwrap(), 0, 0];
    let clay_robot_cost = [info[12].parse().unwrap(), 0, 0];
    let obsidian_robot_cost = [info[18].parse().unwrap(), info[21].parse().unwrap(), 0];
    let geode_robot_cost = [info[27].parse().unwrap(), 0, info[30].parse().unwrap()];
    return RobotCosts {
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost
    }
}
