use std::collections::HashSet;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(3);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut seen = HashSet::new();
    let priorities: Vec<i32> =
        lines
            .iter()
            .map(|s| {
                let mut res = 0;
                let n = s.len() / 2;
                for (i, c) in s.chars().enumerate() {
                    if i < n {
                        seen.insert(c);
                    } else if seen.contains(&c) {
                        res = get_priority(c);
                        seen.clear();
                        break;
                    }
                };
                res
            })
            .collect();
    let total_priority: i32 = priorities.iter().sum();
    println!("Total priority: {}", total_priority);
}

fn part2() {
    let reader = util::get_day_reader(3);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    // item types carried by each group
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut curr_group: Vec<HashSet<char>> = Vec::new();
    for line in lines {
        curr_group.push(HashSet::from_iter(line.chars()));
        if curr_group.len() == 3 {
            groups.push(curr_group.clone());
            curr_group.clear();
        }
    }
    let priorities: Vec<i32> =
        groups
            .iter()
            .map(|group| {
                // find item type held by all three elves
                let temp: HashSet<char> =
                    group[0]
                        .intersection(&group[1])
                        .map(|&x| x)
                        .collect();
                let intersection: HashSet<&char> = temp.intersection(&group[2]).collect();
                let c = intersection.iter().next().unwrap();
                get_priority(**c)
            })
            .collect();
    let total_priority: i32 = priorities.iter().sum();
    println!("Total priority: {}", total_priority);
}

fn get_priority(c: char) -> i32 {
    return if c.is_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    };
}