use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
}

fn part1() {
    let reader = util::get_day_reader(25);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let nums: Vec<i64> = lines.iter().map(|x| snafu_to_decimal((*x).clone())).collect();
    let sum = nums.iter().fold(0, |acc, x| acc + *x);
    println!("SNAFU number: {}", decimal_to_snafu(sum));
}

fn snafu_to_decimal(snafu: String) -> i64 {
    let mut digits: Vec<char> = snafu.chars().collect();
    digits.reverse();
    let mut power = 0;
    let mut place = 1;
    let mut acc = 0;
    while power < digits.len() {
        let c = digits[power];
        let multiplier =
            if c.is_ascii_digit() { c as i64 - '0' as i64 }
            else if c == '-' { -1 }
            else { -2 };
        acc += multiplier * place;
        power += 1;
        place *= 5;
    }
    return acc;
}

fn decimal_to_snafu(n: i64) -> String {
    let mut digits: Vec<String> = Vec::new();
    // figure out first digit
    let mut power = 0;
    let mut place = 1;
    let mut acc = 0;
    loop {
        let temp = (place - 1) / 2;
        let (low, high) = (place - temp, 2 * place + temp);
        if low <= n && n <= high {
            let (low1, high1) = (place - temp, place + temp);
            if low1 <= n && n <= high1 {
                digits.push("1".to_string());
                acc += place;
            } else {
                digits.push("2".to_string());
                acc += 2 * place;
            }
            break;
        }
        power += 1;
        place *= 5;
    }
    power -= 1;
    place /= 5;

    // figure out rest of digits
    while acc != n {
        let temp = (place - 1) / 2;
        for multiplier in -2..3 {
            let (low, high) = (acc + (multiplier * place) - temp, acc + (multiplier * place) + temp);
            if n >= low && n <= high {
                digits.push(
                    if multiplier >= 0 { multiplier.to_string() }
                        else if multiplier == -1 { "-".to_string() }
                        else { "=".to_string() }
                );
                acc += (multiplier * place);
                break;
            }
        }
        power -= 1;
        place /= 5;
    }
    while power > -1 {
        digits.push("0".to_string());
        power -= 1;
    }
    return digits.join("");
}