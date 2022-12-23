use std::cmp::max;
use std::collections::HashMap;
use std::io::BufRead;
use crate::days::util;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let (map, path) = parse_input();
    let moves: HashMap<char, (i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0)),
    ]);
    let rotations: Vec<char> = vec!['>', 'v', '<', '^'];
    // start in leftmost open tile of top row, facing right
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut dir = '>';
    let num_dirs = 4;
    while map[row as usize][col as usize] != '.' {
        col += 1;
    }
    for insn in path {
        if insn == "L" || insn == "R" {
            let index = rotations.iter().position(|&d| d == dir).unwrap();
            let new_index =
                if insn == "L" {
                    (index + 3) % num_dirs
                } else {
                    (index + 1) % num_dirs
                };
            dir = *rotations.get(new_index).unwrap();
        } else {
            let (dx, dy) = moves.get(&dir).unwrap();
            let mut num_moves: usize = insn.parse().unwrap();
            while num_moves > 0 {
                let new_row = row + *dx;
                let new_col = col + *dy;
                if is_valid_coords(new_row, new_col, map.len(), map[0].len()) && map[new_row as usize][new_col as usize] != ' ' {
                    // coordinate is valid- check if it's an open space or wall
                    let tile = map[new_row as usize][new_col as usize];
                    if tile == '.' {
                        row = new_row;
                        col = new_col;
                    } else {
                        // hit a wall
                        break;
                    }
                } else {
                    // walked off edge of map, need to wrap around
                    let index = rotations.iter().position(|&d| d == dir).unwrap();
                    // look in opposite direction
                    let look_dir = rotations[(index + 2) % num_dirs];
                    let (look_dx, look_dy) = moves.get(&look_dir).unwrap();
                    let mut temp_row = row;
                    let mut temp_col = col;
                    while is_valid_coords(temp_row + *look_dx, temp_col + *look_dy, map.len(), map[0].len())
                        && map[(temp_row + *look_dx) as usize][(temp_col + *look_dy) as usize] != ' ' {
                        temp_row += *look_dx;
                        temp_col += *look_dy;
                    }
                    if map[temp_row as usize][temp_col as usize] == '.' {
                        row = temp_row;
                        col = temp_col;
                    } else {
                        // ran into wall
                        break;
                    }
                }
                num_moves -= 1;
            }
        }
    }
    let facing = rotations.iter().position(|&d| d == dir).unwrap() as i32;
    let password = (row + 1) * 1000 + (col + 1) * 4 + facing;
    println!("Final password: {}", password);
}

fn part2() {
    let (map, path) = parse_input();
    let num_rows = map.len();
    let num_cols = map[0].len();
    // calculate size of each cube face
    let mut num_tiles = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] != ' ' {
                num_tiles += 1;
            }
        }
    }
    let side_len = ((num_tiles / 6) as f64).sqrt() as i32;
    let moves: HashMap<char, (i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0)),
    ]);
    let rotations: Vec<char> = vec!['>', 'v', '<', '^'];
    let num_dirs = 4;
    // where and in what direction you end up if you walk off the edge in a certain direction
    let mut edge_map: HashMap<(char, (i32, i32)), (char, (i32, i32))> = HashMap::new();

    // figure out how the cube is folded- determine which sides are next to each other
    // overview of algorithm
    // identify "corners", or empty spaces that are adjacent to two non-empty spaces
    // zip together those edges, marching away from the corner, until both hit a corner of the map at the same time
    // that is, one of the marches can reach the edge of the map and continue following the edge of the cube,
    // but we stop if both hit corners at the same time
    // iterate until all possible zips have been made
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let r_int = r as i32;
            let c_int = c as i32;
            // find corners
            if map[r][c] == ' ' {
                let mut neighbors: Vec<(char, (i32, i32))> = Vec::new();
                for i in 0..num_dirs {
                    let temp_dir = rotations[i];
                    let (dx, dy) = *moves.get(&temp_dir).unwrap();
                    if is_valid_coords(r_int + dx, c_int + dy, num_rows, num_cols)
                        && map[(r_int + dx) as usize][(c_int + dy) as usize] != ' ' {
                        neighbors.push((temp_dir, (r_int + dx, c_int + dy)));
                    }
                }
                // start zipping
                if neighbors.len() == 2 {
                    let mut fst_march_pos = neighbors[0].1;
                    let mut snd_march_pos = neighbors[1].1;
                    let mut index = rotations.iter().position(|&d| d == neighbors[1].0).unwrap();
                    let mut fst_march_gravity = neighbors[0].0;
                    let mut fst_march_anti_gravity =
                        rotations[(rotations.iter().position(|&d| d == fst_march_gravity).unwrap() + 2) % num_dirs];
                    let mut fst_march_dir = rotations[(index + 2) % num_dirs];
                    let mut fst_march_move = *moves.get(&fst_march_dir).unwrap();

                    index = rotations.iter().position(|&d| d == neighbors[0].0).unwrap();
                    let mut snd_march_gravity = neighbors[1].0;
                    let mut snd_march_anti_gravity =
                        rotations[(rotations.iter().position(|&d| d == snd_march_gravity).unwrap() + 2) % num_dirs];

                    let mut snd_march_dir = rotations[(index + 2) % num_dirs];
                    let mut snd_march_move = *moves.get(&snd_march_dir).unwrap();
                    let mut stop = false;
                    while !stop {
                        for i in 0..side_len {
                            edge_map.insert((fst_march_anti_gravity, fst_march_pos), (snd_march_gravity, snd_march_pos));
                            edge_map.insert((snd_march_anti_gravity, snd_march_pos), (fst_march_gravity, fst_march_pos));
                            let new_fst_march_pos = (fst_march_pos.0 + fst_march_move.0, fst_march_pos.1 + fst_march_move.1);
                            let new_snd_march_pos = (snd_march_pos.0 + snd_march_move.0, snd_march_pos.1 + snd_march_move.1);
                            if i + 1 != side_len {
                                // march forward
                                fst_march_pos = new_fst_march_pos;
                                snd_march_pos = new_snd_march_pos;
                            } else {
                                // figure out if both marches have reached a corner
                                stop = true;
                                if is_valid_coords(new_fst_march_pos.0, new_fst_march_pos.1, num_rows, num_cols)
                                    && map[new_fst_march_pos.0 as usize][new_fst_march_pos.1 as usize] != ' ' {
                                    stop = false;
                                    fst_march_pos = new_fst_march_pos;
                                } else {
                                    // reached a corner- turn as needed
                                    index = rotations.iter().position(|&d| d == fst_march_dir).unwrap();
                                    for i in vec![1, 3] {
                                        let temp_dir = rotations[(index + i) % num_dirs];
                                        let temp_move = *moves.get(&temp_dir).unwrap();
                                        let temp_fst_pos = (fst_march_pos.0 + temp_move.0, fst_march_pos.1 + temp_move.1);
                                        if is_valid_coords(temp_fst_pos.0, temp_fst_pos.1, num_rows, num_cols)
                                            && map[temp_fst_pos.0 as usize][temp_fst_pos.1 as usize] != ' ' {
                                            fst_march_dir = temp_dir;
                                            fst_march_move = *moves.get(&fst_march_dir).unwrap();
                                            fst_march_gravity =
                                                rotations[(rotations.iter().position(|&d| d == fst_march_gravity).unwrap() + i) % num_dirs];
                                            fst_march_anti_gravity =
                                                rotations[(rotations.iter().position(|&d| d == fst_march_gravity).unwrap() + 2) % num_dirs];
                                        }
                                    }
                                }
                                if is_valid_coords(new_snd_march_pos.0, new_snd_march_pos.1, num_rows, num_cols)
                                    && map[new_snd_march_pos.0 as usize][new_snd_march_pos.1 as usize] != ' ' {
                                    stop = false;
                                    snd_march_pos = new_snd_march_pos;
                                } else {
                                    // reached a corner- turn as needed
                                    index = rotations.iter().position(|&d| d == snd_march_dir).unwrap();
                                    for i in vec![1, 3] {
                                        let temp_dir = rotations[(index + i) % num_dirs];
                                        let temp_move = *moves.get(&temp_dir).unwrap();
                                        let temp_snd_pos = (snd_march_pos.0 + temp_move.0, snd_march_pos.1 + temp_move.1);
                                        if is_valid_coords(temp_snd_pos.0, temp_snd_pos.1, num_rows, num_cols)
                                            && map[temp_snd_pos.0 as usize][temp_snd_pos.1 as usize] != ' ' {
                                            snd_march_dir = temp_dir;
                                            snd_march_move = *moves.get(&snd_march_dir).unwrap();
                                            snd_march_gravity =
                                                rotations[(rotations.iter().position(|&d| d == snd_march_gravity).unwrap() + i) % num_dirs];
                                            snd_march_anti_gravity =
                                                rotations[(rotations.iter().position(|&d| d == snd_march_gravity).unwrap() + 2) % num_dirs];
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // simulate walking
    // start in leftmost open tile of top row, facing right
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut dir = '>';
    while map[row as usize][col as usize] != '.' {
        col += 1;
    }
    for insn in path {
        if insn == "L" || insn == "R" {
            let index = rotations.iter().position(|&d| d == dir).unwrap();
            let new_index =
                if insn == "L" {
                    (index + 3) % num_dirs
                } else {
                    (index + 1) % num_dirs
                };
            dir = *rotations.get(new_index).unwrap();
        } else {
            let mut num_moves: usize = insn.parse().unwrap();
            while num_moves > 0 {
                let (dx, dy) = moves.get(&dir).unwrap();
                let new_row = row + *dx;
                let new_col = col + *dy;
                if is_valid_coords(new_row, new_col, map.len(), map[0].len()) && map[new_row as usize][new_col as usize] != ' ' {
                    // coordinate is valid- check if it's an open space or wall
                    let tile = map[new_row as usize][new_col as usize];
                    if tile == '.' {
                        row = new_row;
                        col = new_col;
                    } else {
                        // hit a wall
                        break;
                    }
                } else {
                    // walked off edge of map, need to wrap around
                    let (wrap_dir, (wrap_row, wrap_col)) = *edge_map.get(&(dir, (row, col))).unwrap();
                    if map[wrap_row as usize][wrap_col as usize] == '.' {
                        row = wrap_row;
                        col = wrap_col;
                        dir = wrap_dir;
                    } else {
                        // ran into wall
                        break;
                    }
                }
                num_moves -= 1;
            }
        }
    }
    let facing = rotations.iter().position(|&d| d == dir).unwrap() as i32;
    let password = (row + 1) * 1000 + (col + 1) * 4 + facing;
    println!("Final password: {}", password);
}

// parses input, returning the map and path
// the map is padded to a grid
fn parse_input() -> (Vec<Vec<char>>, Vec<String>) {
    let reader = util::get_day_reader(22);
    // ' ' is not a tile; '.' is an open tile; '#' is a wall
    let mut map: Vec<Vec<char>> = Vec::new();
    // each element in path is either a number, "L", or "R"
    let mut path: Vec<String> = Vec::new();
    let mut max_row_len = 0;

    // whether to read map or path
    let mut read_map = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            read_map = false;
            continue;
        }
        if read_map {
            map.push(line.chars().collect());
            max_row_len = max(max_row_len, line.len());
        } else {
            let mut acc = "".to_string();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    acc = acc + &c.to_string();
                } else {
                    if !acc.is_empty() {
                        path.push(acc.to_string());
                        acc = "".to_string();
                    }
                    path.push(c.to_string());
                }
            }
            if !acc.is_empty() {
                path.push(acc);
            }
        }
    }
    // pad map to a grid
    for i in 0..map.len() {
        while map[i].len() < max_row_len {
            map[i].push(' ');
        }
    }
    return (map, path);
}

fn is_valid_coords(r: i32, c: i32, num_rows: usize, num_cols: usize) -> bool {
    return r >= 0 && (r as usize) < num_rows
        && c >= 0 && (c as usize) < num_cols;
}