use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    part1();
    part2();
}

fn part1() {
    // set up input
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    // initialize variables
    let mut max_calories = 0;
    let mut curr_calories = 0;
    let mut max_index = 0;
    let mut curr_index = 0;

    // process input
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            let calories: i32 = line.parse().unwrap();
            curr_calories += calories;
        } else {
            if curr_calories > max_calories {
                max_calories = curr_calories;
                max_index = curr_index;
            }
            curr_calories = 0;
            curr_index += 1;
        }
    }
    // handle last elf
    if curr_calories > max_calories {
        max_calories = curr_calories;
        max_index = curr_index;
    }
    println!("Elf {} had {} calories.", max_index + 1, max_calories);
}

fn part2() {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut top_calories : Vec<i32> = vec![0, 0, 0, 0];
    let mut curr_calories = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            let calories: i32 = line.parse().unwrap();
            curr_calories += calories;
        } else {
            top_calories[0] = curr_calories;
            top_calories.sort();
            curr_calories = 0;
        }
    }
    // handle last elf
    top_calories[0] = curr_calories;
    top_calories.sort();

    println!("The 3 elves are carrying {} calories.", top_calories[1..].iter().sum::<i32>());
}