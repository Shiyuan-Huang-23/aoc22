use std::cmp::max;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let max_calories =
        parse_calories()
            .iter()
            .fold(0, |acc, c| max(acc, *c));
    println!("Maximum calories: {}", max_calories);
}

fn part2() {
    let top_three_calories =
        parse_calories()
            .iter()
            .fold(vec![0, 0, 0, 0], |mut acc, c|
                {
                    if *c >= acc[1] {
                        acc[0] = *c;
                        acc.sort();
                        acc[0] = 0;
                    }
                    acc
                },
            );
    let top_calories_sum: u32 = top_three_calories.iter().sum();
    println!("Sum of top 3 calories: {}", top_calories_sum);
}

// Returns: total number of calories carried by each elf
fn parse_calories() -> Vec<u32> {
    let reader = util::get_day_reader(1);
    let snacks: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut calories: Vec<u32> = vec![0];
    let mut last_index = 0;
    for snack in snacks {
        if !snack.is_empty() {
            calories[last_index] += snack.parse::<u32>().unwrap();
        } else {
            calories.push(0);
            last_index += 1;
        }
    }
    return calories;
}