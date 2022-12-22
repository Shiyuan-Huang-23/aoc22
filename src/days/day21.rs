use std::collections::HashMap;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    // parse  + populate maps
    let (monkeys, numbers) = parse_input();
    let res = eval("root".to_string(), monkeys, numbers, false).unwrap();
    println!("The monkey named root yells: {}", res);
}

fn part2() {
    let (monkeys, numbers) = parse_input();
    let res = solve_for_humn(monkeys, numbers);
    println!("humn should yell: {}", res);
}

// parse monkey-yelling input, returning a tuple (monkeys, numbers)
// monkeys is a map of monkeys yelling results of operations to what they yell
// numbers is a map of monkeys to the numbers they yell
fn parse_input() -> (HashMap<String, Vec<String>>, HashMap<String, i64>) {
    let reader = util::get_day_reader(21);

    let mut monkeys: HashMap<String, Vec<String>> = HashMap::new();
    let mut numbers: HashMap<String, i64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let info: Vec<&str> = line.split(":").collect();
        let monkey = info[0].to_string();
        let op: Vec<String> = info[1].split_whitespace().map(|x| x.to_string()).collect();
        if op.len() == 1 {
            let number: i64 = op[0].parse().unwrap();
            numbers.insert(monkey, number);
        } else {
            monkeys.insert(monkey, op);
        }
    }
    return (monkeys, numbers);
}

// evaluates the monkey with name `target`
// returns None when evaluating "humn" if `err_on_humn`
fn eval(target: String, monkeys: HashMap<String, Vec<String>>, numbers: HashMap<String, i64>, err_on_humn: bool) -> Option<i64> {
    // calculate number yelled by target monkey
    if numbers.contains_key(&target) {
        return
            if &target == "humn" && err_on_humn { None }
            else { Some(*numbers.get(&target).unwrap()) }
    }
    // for each monkey, look up or calculate its number, using a stack to keep track of dependencies
    let mut stack: Vec<String> = vec![target.to_string()];
    let mut numbers = numbers.clone();
    while !stack.is_empty() {
        let monkey = stack.last().unwrap();
        let info = monkeys.get(monkey).unwrap();
        // calculate dependencies if needed
        if !numbers.contains_key(&*info[0]) {
            stack.push(info[0].to_string());
            continue;
        }
        if !numbers.contains_key(&*info[2]) {
            stack.push(info[2].to_string());
            continue;
        }
        if err_on_humn && (info[0] == "humn" || info[2] == "humn") {
            return None
        }
        // dependencies calculated- compute result
        let op = info[1].as_str();
        let a = *numbers.get(&*info[0]).unwrap();
        let b = *numbers.get(&*info[2]).unwrap();
        let res =
            if op == "+" { a + b }
            else if op == "-" { a - b }
            else if op == "*" { a * b }
            else { a / b };
        numbers.insert(monkey.clone(), res);
        stack.pop();
    }
    return Some(*numbers.get(&*target.to_string()).unwrap());
}

// returns the number that needs to be yelled by humn in order for the two numbers yelled by monkey `root` to be equal
// None is returned if the solve failed
fn solve_for_humn(monkeys: HashMap<String, Vec<String>>, numbers: HashMap<String, i64>) -> i64 {
    let mut rhs = 0;
    let mut curr_monkey = "".to_string();
    // set up both sides of equation
    // rhs is the number that is fully computed
    // curr_monkey is the monkey whose output depends on the number yelled by humn
    let left_monkey = monkeys.get("root").unwrap().first().unwrap().clone();
    let right_monkey = monkeys.get("root").unwrap().last().unwrap().clone();
    let left = eval(left_monkey.clone(), monkeys.clone(), numbers.clone(), true);
    let right = eval(right_monkey.clone(), monkeys.clone(), numbers.clone(), true);
    match left {
        Some(n) => rhs = n,
        None => curr_monkey = left_monkey.clone(),
    }
    match right {
        Some(n) => rhs = n,
        None => curr_monkey = right_monkey.clone(),
    }
    // solve for the number yelled by humn
    while curr_monkey != "humn" {
        let info = monkeys.get(&curr_monkey).unwrap();
        let op = info.get(1).unwrap().clone();
        let left_monkey = info.first().unwrap().clone();
        let right_monkey = info.last().unwrap().clone();
        let left = eval(left_monkey.clone(), monkeys.clone(), numbers.clone(), true);
        let right = eval(right_monkey.clone(), monkeys.clone(), numbers.clone(), true);
        match left {
            Some(n) => {
                rhs =
                    if op == "+" { rhs - n }
                    else if op == "-" { n - rhs }
                    else if op == "*" { rhs / n }
                    else { n / rhs }
            },
            None => curr_monkey = left_monkey.clone(),
        }
        match right {
            Some(n) => {
                rhs =
                    if op == "+" { rhs - n }
                    else if op == "-" { rhs + n }
                    else if op == "*" { rhs / n }
                    else { rhs * n }
            },
            None => curr_monkey = right_monkey.clone(),
        }
    }
    return rhs;
}