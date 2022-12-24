use std::io::BufRead;
use crate::days::util;

pub fn main() {
    solve(false, 1);
    solve(true, 10);
}

fn solve(use_decryption_key: bool, num_rounds: usize) {
    let reader = util::get_day_reader(20);
    let decryption_key = 811589153;

    let file: Vec<i64> = reader.lines().map(|x| x.unwrap().parse::<i64>().unwrap()).collect();
    // remember original orders of numbers
    let mut mixed_file: Vec<(usize, i64)> = file.iter().enumerate().map(|x| (x.0, *x.1)).collect();
    if use_decryption_key {
        mixed_file = mixed_file.iter().map(|(o, n)| (*o, *n * decryption_key)).collect();
    }
    // mix the numbers
    let n = file.len() as i64;
    for _ in 0..num_rounds {
        for i in 0..file.len() {
            // find correct element to move
            let j = mixed_file.iter().position(|&(o, _)| o == i).unwrap();
            let moves = mixed_file[j].1;
            mixed_file.remove(j);
            let temp = (j as i64 + moves) % (n - 1);
            let mut new_pos: usize =
                if temp >= 0 { temp as usize } else { ((temp + n - 1) % (n - 1)) as usize };
            if new_pos == 0 && moves < 0 {
                new_pos = file.len() - 1;
            }
            mixed_file.insert(new_pos, (i, moves));
        }
    }
    // find where 0 is located
    let j = mixed_file.iter().position(|&(_, n)| n == 0).unwrap();
    let mut res = 0;
    for i in 1..4 {
        let temp = mixed_file[(j + (i * 1000)) % file.len()].1;
        res += temp;
    }
    println!("Sum of grove coordinates: {}", res);
}
