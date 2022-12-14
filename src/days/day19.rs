use std::cmp::max;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct SimulationState {
    // [ore, clay, obsidian]
    resources: [u32; 3],
    // [ore, clay, obsidian robots]
    robots: [u32; 3],
    // number of minutes that have passed
    time: u32,
    // number of geode robots
    geode_robots: u32,
    // number of geodes mined so far
    geodes_mined: u32,
}

fn part1() {
    let reader = util::get_day_reader(19);
    let mut quality_sum = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // cost is in the form [ore, clay, obsidian]
        // for each of the robot types [ore, clay, obsidian, geode]
        let robot_costs = parse_blueprint(line);
        let max_geodes_mined = mine_geodes(robot_costs, 24);
        quality_sum += (i as u32 + 1) * max_geodes_mined;
    }
    println!("Sum of quality levels: {}", quality_sum);
}

fn part2() {
    let reader = util::get_day_reader(19);
    let mut geode_prod = 1;
    for (i, line) in reader.lines().enumerate() {
        if i >= 3 {
            break;
        }
        let line = line.unwrap();
        let robot_costs = parse_blueprint(line);
        let max_geodes_mined = mine_geodes(robot_costs, 32);
        geode_prod *= max_geodes_mined;
    }
    println!("Product of max geodes mined: {}", geode_prod);
}

// Returns: maximum number of geodes that can be mined with these robot costs in 24 minutes
fn mine_geodes(costs: [[u32; 3]; 4], end_time: u32) -> u32 {
    let mut max_geodes_mined = 0;
    // figure out how many of each type of robot we should build
    // we should never build more of a type of robot if building more of it won't speed up
    // production by producing more of a limiting resource
    let mut max_robots = [0; 3];
    for i in 0..costs.len() {
        for j in 0..costs[i].len() {
            max_robots[j] = max(max_robots[j], costs[i][j]);
        }
    }
    // simulate possibilities with DFS over a sequence of robot-building decisions
    let mut stack: Vec<SimulationState> = Vec::new();
    stack.push(
        SimulationState {
            resources: [0, 0, 0],
            robots: [1, 0, 0],
            time: 0,
            geode_robots: 0,
            geodes_mined: 0,
        }
    );
    while !stack.is_empty() {
        match stack.pop().unwrap() {
            SimulationState { resources, robots, time, geode_robots, geodes_mined } => {
                // try to build each type of robot
                for i in 0..costs.len() {
                    // don't build robot if it won't produce a limiting resource
                    // no max on geode robots
                    if i < robots.len() && robots[i] == max_robots[i] {
                        continue;
                    }
                    let cost = costs[i];
                    // check we have enough resources to build the robot
                    // figure out how long we need to wait to gather resources to build the bot
                    // if we don't have enough resources
                    let mut max_wait_time = 0;
                    for j in 0..resources.len() {
                        if resources[j] < cost[j] {
                            if robots[j] > 0 {
                                // cost[j] - resources[j] is how much more of a resource we need
                                // the - 1 and + 1 makes sure we wait for the appropriate amount of time
                                // the + 1 makes sure we always wait for at least 1 minute
                                // the - 1 makes sure we don't wait longer than we need to due to integer division
                                // e.g. if we need 3 ore and we have 3 ore robots, we only wait 1 minute, not 2
                                max_wait_time = max(max_wait_time, (cost[j] - resources[j] - 1)/robots[j] + 1);
                            } else {
                                // we currently have no robots gathering this resource
                                // infeasible to build this robot by just waiting
                                max_wait_time = end_time;
                            }
                        }
                    }
                    // won't be able to build robot in time or
                    // built robot won't get a chance to mine anything
                    if time + max_wait_time + 1 >= end_time {
                        continue;
                    }
                    let mut new_resources = resources;
                    // gather resources with existing robots
                    for j in 0..resources.len() {
                        // the + 1 is because we're simulating until the end of the minute in which
                        // the chosen robot is built
                        new_resources[j] += robots[j] * (max_wait_time + 1);
                        // we also spend resources to build robot
                        new_resources[j] -= cost[j];
                    }
                    let mut new_robots = robots;
                    let mut new_geode_robots = geode_robots;
                    // robot is built
                    if i < robots.len() {
                        new_robots[i] += 1;
                    } else {
                        new_geode_robots += 1;
                    }
                    // add state to stack
                    stack.push(SimulationState {
                        resources: new_resources,
                        robots: new_robots,
                        time: time + max_wait_time + 1,
                        geode_robots: new_geode_robots,
                        geodes_mined: geodes_mined + geode_robots * (max_wait_time + 1),
                    });
                }
                // simulate how many geodes we'd mine if we kept mining with current number of
                // geode robots
                max_geodes_mined = max(max_geodes_mined, geodes_mined + geode_robots * (end_time - time));
            }
        }
    }
    return max_geodes_mined;
}

fn parse_blueprint(s: String) -> [[u32; 3]; 4] {
    let info: Vec<&str> = s.split_whitespace().collect();
    let ore_robot_cost = [info[6].parse().unwrap(), 0, 0];
    let clay_robot_cost = [info[12].parse().unwrap(), 0, 0];
    let obsidian_robot_cost = [info[18].parse().unwrap(), info[21].parse().unwrap(), 0];
    let geode_robot_cost = [info[27].parse().unwrap(), 0, info[30].parse().unwrap()];
    return
        [
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost
        ];
}
