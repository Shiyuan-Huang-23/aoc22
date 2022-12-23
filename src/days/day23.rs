use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    solve();
}

fn solve() {
    // parse input
    let reader = util::get_day_reader(23);
    let mut elf_pos: HashSet<(i32, i32)> = HashSet::new();
    for (r, line) in reader.lines().enumerate()  {
        let line = line.unwrap();
        for (c, tile) in line.chars().enumerate() {
            if tile == '#' {
                elf_pos.insert((r as i32, c as i32));
            }
        }
    }
    let adj_dirs = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];
    let prop_dirs = vec![
        vec![(-1, 0), (-1, 1), (-1, -1)],
        vec![(1, 0), (1, 1), (1, -1)],
        vec![(0, -1), (-1, -1), (1, -1)],
        vec![(0, 1), (-1, 1), (1, 1)]
    ];
    let mut start_dir = 0;
    // map of proposed destination to the elves that proposed that destination
    let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    // simulate rounds
    let mut round_number = 1;
    loop {
        // first half of round
        for (r, c) in &elf_pos {
            // figure out whether all adjacent tiles are empty
            let mut adj_tiles_empty = true;
            for (adj_dx, adj_dy) in &adj_dirs {
                if elf_pos.contains(&(*r + *adj_dx, *c + *adj_dy)) {
                    adj_tiles_empty = false;
                    break;
                }
            }
            // make proposals
            if !adj_tiles_empty {
                for i in 0..prop_dirs.len() {
                    let cur_dirs_index = (start_dir + i) % prop_dirs.len();
                    let mut is_empty = true;
                    for (prop_dx, prop_dy) in &prop_dirs[cur_dirs_index] {
                        if elf_pos.contains(&(*r + *prop_dx, *c + *prop_dy)) {
                            is_empty = false;
                            break;
                        }
                    }
                    if is_empty {
                        let (dx, dy) = prop_dirs[cur_dirs_index][0];
                        let dest = (*r + dx, *c + dy);
                        if proposals.contains_key(&dest) {
                            let curr_elves = proposals.get_mut(&dest).unwrap();
                            curr_elves.push((*r, *c));
                        } else {
                            proposals.insert(dest, vec![(*r, *c)]);
                        }
                        break;
                    }
                }
            }
        }
        // second half of round
        for (dest, elves) in &proposals {
            if elves.len() == 1 {
                elf_pos.remove(&elves[0]);
                elf_pos.insert(*dest);
            }
        }
        if round_number == 10 || (proposals.is_empty() && round_number < 10) {
            let mut min_r = i32::MAX;
            let mut max_r = i32::MIN;
            let mut min_c = i32::MAX;
            let mut max_c = i32::MIN;
            for (r, c) in &elf_pos {
                min_r = min(min_r, *r);
                max_r = max(max_r, *r);
                min_c = min(min_c, *c);
                max_c = max(max_c, *c);
            }
            let rect_area = (max_r - min_r + 1) * (max_c - min_c + 1);
            println!("Number of empty tiles after 10 rounds: {}", rect_area - (elf_pos.len() as i32));
        }
        if proposals.is_empty() {
            println!("First round where no elf moved: {}", round_number);
            break;
        }
        proposals.clear();
        start_dir = (start_dir + 1) % prop_dirs.len();
        round_number += 1;
    }
}
