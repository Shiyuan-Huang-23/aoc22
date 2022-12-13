use std::io::BufRead;
use crate::days::util;

pub fn main() {
    monkey_helper(20, true);
    monkey_helper(10000, false);
}

fn monkey_helper(num_rounds: i32, reduce_worry: bool) {
    let reader = util::get_day_reader(11);
    let mut curr_monkey: usize = 0;
    let mut monkey_items: Vec<Vec<i128>> = Vec::new();
    let mut ops: Vec<Vec<String>> = Vec::new();
    let mut moduli: Vec<i128> = Vec::new();
    let mut toss_targets: Vec<(i128, i128)> = Vec::new();
    let mut num_items_inspected: Vec<i128> = Vec::new();
    // parse input
    for line in reader.lines() {
        let line = line.unwrap();
        let info: Vec<&str> = line.split_whitespace().collect();
        if info.is_empty() {
            continue;
        }
        if info[0] == "Monkey" {
            curr_monkey = info[1].chars().nth(0).unwrap() as usize - '0' as usize;
            num_items_inspected.push(0);
        } else if info[0] == "Starting" {
            let mut item = Vec::new();
            for i in 2..info.len() {
                let n: Vec<&str> = info[i].split(",").collect();
                item.push(n[0].parse().unwrap());
            }
            monkey_items.push(item);
        } else if info[0] == "Operation:" {
            ops.push(vec!(info[3].to_string(), info[4].to_string(), info[5].to_string()));
        } else if info[0] == "Test:" {
            moduli.push(info[3].parse().unwrap());
        } else if info[0] == "If" && info[1] == "true:" {
            toss_targets.push((info[5].parse().unwrap(), 0));
        } else if info[0] == "If" && info[1] == "false:" {
            let (t, _) = toss_targets[curr_monkey];
            toss_targets[curr_monkey] = (t, info[5].parse().unwrap());
        }
    }
    let mut moduli_prod = 1;
    for i in 0..moduli.len() {
        moduli_prod *= moduli[i];
    }
    // simulate rounds
    for _ in 0..num_rounds {
        for i in 0..monkey_items.len() {
            let items = monkey_items[i].clone();
            let op = ops[i].clone();
            let modulus = moduli[i];
            let (t, f) = toss_targets[i];
            for item in items {
                num_items_inspected[i] += 1;
                let operand1 = item;
                let mut operand2 = item;
                if op[2] != "old" {
                    operand2 = op[2].parse().unwrap();
                }
                let mut updated =
                    if op[1] == "+" { (operand1 + operand2) % moduli_prod }
                    else { (operand1 * operand2) % moduli_prod };
                if reduce_worry {
                    updated = updated / 3;
                }
                if updated % modulus == 0 {
                    monkey_items[t as usize].push(updated);
                } else {
                    monkey_items[f as usize].push(updated);
                }
            }
            monkey_items[i] = Vec::new();
        }
    }
    num_items_inspected.sort();
    num_items_inspected.reverse();
    println!("Monkey business: {}", num_items_inspected[0] * num_items_inspected[1]);
}