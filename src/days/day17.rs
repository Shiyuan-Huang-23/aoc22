use std::cmp::{max};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    solve(2022);
    solve(1000000000000);
}

fn solve(num_rocks: i64) {
    let reader = util::get_day_reader(17);
    let input: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let jets: Vec<char> = input[0].chars().collect();

    // variables for simulating rocks
    // offsets to represent rock pieces
    let pieces: Vec<Vec<(i64, i32)>> = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let piece_starts: Vec<Vec<(i64, i32)>> = pieces.iter().map(
        |piece| piece.iter().map(|(r, c)| (*r + 4, *c + 2)).collect()
    ).collect();
    let chamber_width = 7;
    // position of stopped rocks
    let mut rock_pos: HashSet<(i64, i32)> = HashSet::new();
    // height of highest rock, or the floor
    let mut max_height = 0;
    let mut jet_index = 0;
    let mut piece_index = 0;
    let mut heights: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0];

    // variables for finding cycles
    let mut cycle_found = false;
    let mut layer_height = i64::MIN;
    // rock formations maps rock_num -> (jet_index, piece_index, layer_height, rocks left over after all rocks below layer_height have been cleared)
    let mut rock_formations: HashMap<i64, (usize, usize, i64, Vec<(i64, i32)>)> = HashMap::new();

    // simulate rocks
    let mut rock_num = 0;
    while rock_num < num_rocks {
        let mut piece_pos: Vec<(i64, i32)> =
            piece_starts[piece_index].iter().map(|(r, c)| (*r + max_height, *c)).collect();
        loop {
            // rock gets pushed by a jet
            let jet = jets[jet_index];
            let dc = if jet == '<' { -1 } else { 1 };
            let mut temp_piece_pos = Vec::new();
            for i in 0..piece_pos.len() {
                let (r, c) = piece_pos[i];
                let new_c = c + dc;
                if new_c < 0 || new_c >= chamber_width || rock_pos.contains(&(r, new_c)) {
                    break;
                }
                temp_piece_pos.push((r, new_c));
            }
            if temp_piece_pos.len() == piece_pos.len() {
                // all future positions of the rock are valid, so piece is successfully pushed by jet
                piece_pos = temp_piece_pos.clone();
            }
            jet_index = (jet_index + 1) % jets.len();
            // rock falls downward
            temp_piece_pos.clear();
            for i in 0..piece_pos.len() {
                let (r, c) = piece_pos[i];
                let new_r = r - 1;
                if new_r <= 0 || rock_pos.contains(&(new_r, c)) {
                    break;
                }
                temp_piece_pos.push((new_r, c));
            }
            if temp_piece_pos.len() == piece_pos.len() {
                // all future positions of the rock are valid, so piece successfully falls down
                piece_pos = temp_piece_pos;
            } else {
                // piece has come to rest
                break;
            }
        }
        for i in 0..piece_pos.len() {
            rock_pos.insert(piece_pos[i]);
            max_height = max(max_height, piece_pos[i].0);
            heights[piece_pos[i].1 as usize] = piece_pos[i].0;
            // find two layers of stopped rocks that future rocks cannot fall below
            // layer patterns like ___---- or -_-__-- are allowed
            let mut full_layer = true;
            for tc in 0..chamber_width {
                if !rock_pos.contains(&(piece_pos[i].0, tc)) && !rock_pos.contains(&(piece_pos[i].0 + 1, tc)) {
                    full_layer = false;
                }
            }
            if full_layer {
                layer_height = max(layer_height, piece_pos[i].0);
            }
        }
        piece_index = (piece_index + 1) % pieces.len();
        rock_num += 1;
        // try to find a cycle if it's too slow to simulate all rocks
        if !cycle_found && layer_height > 0 {
            // add only rocks above layer_height back to rock_pos
            let mut temp_rock_pos: Vec::<(i64, i32)> =
                rock_pos.iter()
                    .filter(|(r, _)| *r >= layer_height)
                    .map(|x| *x)
                    .collect();
            temp_rock_pos =
                temp_rock_pos.iter().map(|(r, c)| (*r - layer_height, *c)).collect();
            temp_rock_pos.sort();
            for (k, (ji, pi, h, v)) in &rock_formations {
                if *ji == jet_index && *pi == piece_index && *v == temp_rock_pos {
                    // cycle found- advance simulation
                    let cycle_len = rock_num - *k;
                    let height_diff = layer_height - *h;
                    let num_cycles = (num_rocks - rock_num - 1) / cycle_len;
                    max_height += num_cycles * height_diff;
                    rock_num += num_cycles * cycle_len;
                    for i in 0..temp_rock_pos.len() {
                        let (tr, tc) = temp_rock_pos[i];
                        rock_pos.insert((tr + layer_height + num_cycles * height_diff, tc));
                    }
                    cycle_found = true;
                    break;
                }
            }
            rock_formations.insert(rock_num, (jet_index, piece_index, layer_height, temp_rock_pos));
        }
    }
    println!("Rock tower height: {}", max_height);
}
