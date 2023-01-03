use std::cmp::max;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
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
    let reader = util::get_file_reader("short19.txt");
    let mut quality_sum = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // cost is in the form [ore, clay, obsidian]
        // for each of the robot types [ore, clay, obsidian, geode]
        let robot_costs = parse_blueprint(line);
        let max_geodes_mined = mine_geodes(robot_costs);
        println!("Max geodes mined using blueprint {}: {}", i + 1, max_geodes_mined);
        quality_sum += (i as u32 + 1) * max_geodes_mined;
    }
    println!("Sum of quality levels: {}", quality_sum);
}

// Returns: maximum number of geodes that can be mined with these robot costs in 24 minutes
fn mine_geodes(costs: [[u32; 3]; 4]) -> u32 {
    let end_time = 24;
    let mut max_geodes_mined = 0;
    // simulate robot-building decisions
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
                    let cost = costs[i];
                    // check we have enough resources to build the robot
                    // figure out how long we need to wait to gather resources to build the bot
                    // if we don't have enough resources
                    let mut max_wait_time = 0;
                    for j in 0..resources.len() {
                        if resources[j] < cost[j] {
                            if robots[j] > 0 {
                                // cost[j] - resources[j] is how much more of a resource we need
                                // + 1 accounts for the fact that we'll need to wait to gather resources during this minute as well
                                max_wait_time = max(max_wait_time, (cost[j] - resources[j])/robots[j] + 1);
                            } else {
                                // we currently have no robots gathering this resource
                                // infeasible to build this robot
                                max_wait_time = end_time;
                            }
                        }
                    }
                    // won't be able to build robot in time
                    if time + max_wait_time >= end_time {
                        continue;
                    }
                    let mut new_resources = resources;
                    // gather resources with existing robots
                    for j in 0..resources.len() {
                        // the +1 is because we're simulating until the end of the minute in which
                        // the chosen robot is built
                        new_resources[j] += robots[j] * (max_wait_time + 1);
                    }
                    let mut new_robots = robots;
                    let mut new_geode_robots = geode_robots;
                    // spend resources to build robot
                    for j in 0..resources.len() {
                        new_resources[j] -= cost[j];
                    }
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
