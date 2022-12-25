use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = util::get_day_reader(18);
    let mut surface_area = 0;
    let mut lava: HashSet<(i32, i32, i32)> = HashSet::new();
    let sides: Vec<(i32, i32, i32)> = vec![(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
    for line in reader.lines() {
        let line = line.unwrap();
        let info: Vec<&str> = line.split(",").collect();
        let coord_info: Vec<i32> = info.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let (x, y, z) = (coord_info[0], coord_info[1], coord_info[2]);
        let mut added_area = 6;
        for (dx, dy, dz) in &sides {
            if lava.contains(&(x + *dx, y + *dy, z + *dz)) {
                surface_area -= 1;
                added_area -= 1;
            }
        }
        lava.insert((x, y, z));
        surface_area += added_area;
    }
    println!("Surface area of droplet: {}", surface_area);
}

fn part2() {
    let reader = util::get_day_reader(18);
    let mut lava: HashSet<(i32, i32, i32)> = HashSet::new();
    let sides: Vec<(i32, i32, i32)> = vec![(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
    // keep track of x, y, z coordinates of box containing lava droplet
    let mut extrema: Vec<i32> = vec![i32::MAX, i32::MIN, i32::MAX, i32::MIN, i32::MAX, i32::MIN];
    for line in reader.lines() {
        let line = line.unwrap();
        let info: Vec<&str> = line.split(",").collect();
        let coord_info: Vec<i32> = info.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        for i in 0..coord_info.len() {
            let coord = coord_info[i];
            extrema[2*i] = min(extrema[2*i], coord - 1);
            extrema[2*i + 1] = max(extrema[2*i + 1], coord + 1);
        }
        let (x, y, z) = (coord_info[0], coord_info[1], coord_info[2]);
        lava.insert((x, y, z));
    }
    // flood fill box around lava droplet with steam
    let mut steam: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut stack: Vec<(i32, i32, i32)> = vec![(extrema[0], extrema[2], extrema[4])];
    let mut surface_area = 0;
    while !stack.is_empty() {
        let (x, y, z) = stack.pop().unwrap();
        for (dx, dy, dz) in &sides {
            let next = (x + *dx, y + *dy, z + *dz);
            let in_range = next.0 >= extrema[0] && next.0 <= extrema[1]
                && next.1 >= extrema[2] && next.1 <= extrema[3]
                && next.2 >= extrema[4] && next.2 <= extrema[5];
            if lava.contains(&next) {
                surface_area += 1;
            } else if !steam.contains(&next) && in_range {
                stack.push(next);
                steam.insert(next);
            }
        }
    }
    println!("External surface area of droplet: {}", surface_area);
}
