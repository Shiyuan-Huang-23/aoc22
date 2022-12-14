use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(13);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut i = 0;
    let mut sum = 0;
    while i < lines.len() {
        let left = lines[i].clone();
        let right = lines[i + 1].clone();
        let result = compare(left.clone(), right.clone());
        if result < 0 {
            if i == 0 {
                sum += 1;
            } else {
                sum += (i / 3) + 1;
            }
        }
        i += 3;
    }
    println!("Sum of indices: {}", sum);
}

fn part2() {
    let reader = util::get_day_reader(13);
    let mut num_first = 0;
    let mut num_second = 0;
    let divider1 = "[[2]]".to_string();
    let divider2 = "[[6]]".to_string();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let before_divider1 = compare(line.clone(), divider1.clone()) < 0;
        if before_divider1 {
            num_first += 1;
        } else {
            let before_divider2 = compare(line.clone(), divider2.clone()) < 0;
            if before_divider2 {
                num_second += 1;
            }
        }
    }
    println!("Decoder key: {}", (num_first + 1) * (num_first + num_second + 2));
}

fn compare(left: String, right: String) -> i32 {
    let is_list_left = is_list(left.clone());
    let is_list_right = is_list(right.clone());
    let mut new_left = left.clone();
    let mut new_right = right.clone();
    if !is_list_left && !is_list_right {
        let l: i32 = left.parse().unwrap();
        let r: i32 = right.parse().unwrap();
        return l - r;
    }
    if !is_list_left {
        new_left = "[".to_owned() + &new_left + "]";
    }
    if !is_list_right {
        new_right = "[".to_owned() + &new_right + "]";
    }
    // handle case where they're both lists
    let mut left_start = 1;
    let mut right_start = 1;
    while left_start < new_left.len() - 1 && right_start < new_right.len() - 1 {
        let mut num_extra_opens = 0;
        let mut curr_end = left_start;
        let mut new_left_prime: Vec<String> = Vec::new();
        let mut new_right_prime: Vec<String> = Vec::new();
        while curr_end < new_left.len() - 1 {
            let c = new_left.chars().nth(curr_end).unwrap();
            if c == '[' {
                num_extra_opens += 1;
            } else if c == ']' {
                num_extra_opens -= 1;
            } else if c == ',' && num_extra_opens == 0 {
                break;
            }
            new_left_prime.push(c.to_string());
            curr_end += 1;
        }
        left_start = curr_end + 1;
        num_extra_opens = 0;
        curr_end = right_start;
        while curr_end < new_right.len() - 1 {
            let c = new_right.chars().nth(curr_end).unwrap();
            if c == '[' {
                num_extra_opens += 1;
            } else if c == ']' {
                num_extra_opens -= 1;
            } else if c == ',' && num_extra_opens == 0 {
                break;
            }
            new_right_prime.push(c.to_string());
            curr_end += 1;
        }
        right_start = curr_end + 1;
        let result = compare(new_left_prime.join(""), new_right_prime.join(""));
        if result != 0 {
            return result;
        }
    }
    let final_result = if left_start >= new_left.len() - 1 && right_start >= new_right.len() - 1 { 0 }
        else if left_start >= new_left.len() - 1 { -1 }
        else { 1 };
    return final_result;
}

fn is_list(s: String) -> bool {
    return s.chars().nth(0).unwrap() == '[' && s.chars().last().unwrap() == ']';
}