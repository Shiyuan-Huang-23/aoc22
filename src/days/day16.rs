use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct ValveInfo {
    // adjacency list representation of valves graph
    graph: HashMap<String, Vec<String>>,
    // valve -> flow rate, where flow rate > 0
    flow_rate: HashMap<String, u32>,
    // valves with non-zero flow
    valves: Vec<String>,
}

#[derive(Debug)]
struct SimulationState {
    // valves that have already been opened
    opened: Vec<String>,
    // current valve
    curr: String,
    // valves that have not been opened
    unopened: Vec<String>,
    // amount of pressure being released per minute
    flow: u32,
    // pressure that has already been released before reaching current valve
    pressure: u32,
    // number of minutes that have passed
    time: u32,
}

fn part1() {
    let max_pressure_released = solve(parse_input(), 30);
    println!("Maximum pressure released: {}", max_pressure_released);
}

fn part2() {
    let mut max_pressure_released: u32 = 0;
    let end_time = 26;
    match parse_input() {
        ValveInfo { graph, flow_rate, valves } => {
            // explore every possible distinct assignment of valves to human and elephant
            // don't duplicate assignments that are equivalent by symmetry
            let mut assignment_bits = 1;
            let stop: i32 = (2 as i32).pow((valves.len() - 1) as u32);
            let width = valves.len();
            while assignment_bits < stop {
                let assignment = format!("{:0width$b}", assignment_bits);
                let mut human_valves: Vec<String> = Vec::new();
                let mut elephant_valves: Vec<String> = Vec::new();
                for (i, c) in assignment.chars().enumerate() {
                    let v = valves[i].clone();
                    if c == '0' {
                        human_valves.push(v);
                    } else {
                        elephant_valves.push(v);
                    }
                }
                let human_valve_info = ValveInfo { graph: graph.clone(), flow_rate: flow_rate.clone(), valves: human_valves };
                let human_pressure_released = solve(human_valve_info, end_time);
                let elephant_valve_info = ValveInfo { graph: graph.clone(), flow_rate: flow_rate.clone(), valves: elephant_valves };
                let elephant_pressure_released = solve(elephant_valve_info, end_time);
                max_pressure_released = max(max_pressure_released, human_pressure_released + elephant_pressure_released);
                assignment_bits += 1;
            }
        }
    }
    println!("Maximum pressure released: {}", max_pressure_released);
}

fn parse_input() -> ValveInfo {
    let reader = util::get_day_reader(16);
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_rate: HashMap<String, u32> = HashMap::new();
    let mut valves: Vec<String> = Vec::new();
    // parse input
    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let valve = words[1].to_string();
        let flow_info: Vec<&str> = words[4].trim_end_matches(";").split("=").collect();
        let flow: u32 = flow_info[1].parse().unwrap();
        let mut adjacent: Vec<String> = Vec::new();
        for i in 9..words.len() {
            adjacent.push(words[i].trim_end_matches(",").to_string());
        }
        adjacent.sort();
        graph.insert(valve.clone(), adjacent);
        if flow > 0 {
            valves.push(valve.clone());
            flow_rate.insert(valve, flow);
        }
    }
    return ValveInfo {
        graph,
        flow_rate,
        valves,
    };
}

fn solve(valve_info: ValveInfo, end_time: u32) -> u32 {
    let mut max_pressure_released: u32 = 0;
    match valve_info {
        ValveInfo { graph, flow_rate, valves } => {
            // valves, along with the start valve
            let mut valves_with_start = valves.clone();
            valves_with_start.insert(0, "AA".to_string());
            // shortest distance between each pair of non-zero valves, or "AA"
            let mut distances: HashMap<(String, String), u32> = HashMap::new();
            // build map of distances between valves
            for i in 0..valves_with_start.len() {
                let start = valves_with_start[i].clone();
                let mut frontier: HashSet<String> = HashSet::new();
                frontier.insert(start.clone());
                let mut visited: HashSet<String> = HashSet::new();
                let mut dist = 0;
                while !frontier.is_empty() {
                    let mut new_frontier: HashSet<String> = HashSet::new();
                    for v in &frontier {
                        for adj in graph.get(v).unwrap() {
                            if !visited.contains(adj) {
                                new_frontier.insert(adj.clone());
                            }
                        }
                        if valves_with_start.contains(v) {
                            distances.insert((start.clone(), v.clone()), dist);
                        }
                        visited.insert(v.clone());
                    }
                    dist += 1;
                    frontier = new_frontier;
                }
            }

            // explore all possible orders in which non-zero valves can be opened
            let mut stack: Vec<SimulationState> = Vec::new();
            // add all possible starting valves to stack
            let mut unopened = valves.clone();
            for i in 0..valves.len() {
                let v = valves[i].clone();
                unopened.remove(i);
                stack.push(
                    SimulationState {
                        opened: Vec::new(),
                        curr: v.clone(),
                        unopened: unopened.clone(),
                        flow: 0,
                        pressure: 0,
                        time: *distances.get(&("AA".to_string(), v.clone())).unwrap(),
                    }
                );
                unopened.insert(i, v);
            }
            // DFS to explore all possible orderings
            while !stack.is_empty() {
                match stack.pop().unwrap() {
                    SimulationState { mut opened, curr, mut unopened, mut flow, mut pressure, mut time } => {
                        // ran out of time, don't continue considering this ordering
                        if time >= end_time {
                            // no time to open valve, record pressure released
                            if time == end_time {
                                max_pressure_released = max(max_pressure_released, pressure);
                            }
                            continue;
                        }
                        // open current valve
                        opened.push(curr.clone());
                        for i in 0..unopened.len() {
                            if unopened[i] == curr {
                                unopened.remove(i);
                                break;
                            }
                        }
                        pressure += flow;
                        flow += flow_rate.get(&curr).unwrap();
                        time += 1;
                        // simulate pressure being released until time is up
                        // handles cases when all valves are opened, or when there's not enough time to reach the next valve
                        max_pressure_released = max(max_pressure_released, pressure + (end_time - time) * flow);

                        // consider next valve to open
                        let mut temp_unopened = unopened.clone();
                        for i in 0..unopened.len() {
                            let v = unopened[i].clone();
                            temp_unopened.remove(i);
                            let d = *distances.get(&(curr.clone(), v.clone())).unwrap();
                            stack.push(
                                SimulationState {
                                    opened: opened.clone(),
                                    curr: v.clone(),
                                    unopened: temp_unopened.clone(),
                                    flow,
                                    pressure: pressure + d * flow,
                                    time: time + d,
                                }
                            );
                            temp_unopened.insert(i, v);
                        }
                    }
                }
            }
        }
    }
    return max_pressure_released;
}
