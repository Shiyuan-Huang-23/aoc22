use std::collections::{HashMap, HashSet};
use std::io::{BufRead};
use crate::days::util;

pub fn main() {
    solve();
}

fn solve() {
    let debug = false;
    let reader = util::get_day_reader(7);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut i: usize = 0;
    // present working directory
    // [] represents /, ["dir1"] represents /dir1, ["dir1","dir2"] represents /dir1/dir2
    let mut pwd: Vec<String> = Vec::new();
    // child directories of a given directory
    let mut children: HashMap<String, HashSet<String>> = HashMap::new();
    // start off with not knowing anything about children of root directory
    children.insert("/".to_string(), HashSet::new());
    // size of files contained in each directory, not counting files contained in child directories
    let mut file_size: HashMap<String, i32> = HashMap::new();
    // directories whose files have already been listed by ls
    let mut listed: HashSet<String> = HashSet::new();

    while i < lines.len() {
        let split: Vec<&str> = lines[i].split_whitespace().collect();
        if debug {
            println!("Split: {:?}", split);
        }
        // invariant: split[0] == $, since non-commands are already processed
        if split[1] == "cd" {
            if split[2] == ".." {
                pwd.pop();
            } else if split[2] == "/" {
                pwd = Vec::new();
            } else {
                let curr_dir = "/".to_owned() + &*pwd.join("/");
                // make sure child dir is added to children
                if !children.contains_key(&curr_dir) {
                    children.insert(curr_dir.clone(), HashSet::new());
                }
                children.get_mut(&curr_dir).unwrap().insert(split[2].to_string());
                // navigate to child
                pwd.push(split[2].to_string());
            }
            i += 1;
        } else if split[1] == "ls" {
            i += 1;
            let curr_dir = "/".to_owned() + &*pwd.join("/");
            // add current directory to maps if needed
            if !listed.contains(&curr_dir) {
                file_size.insert(curr_dir.clone(), 0);
            }
            if !children.contains_key(&curr_dir) {
                children.insert(curr_dir.clone(), HashSet::new());
            }
            while i < lines.len() && lines[i].chars().nth(0).unwrap() != '$' {
                // directory has not already been listed, record information
                if !listed.contains(&curr_dir) {
                    let info: Vec<&str> = lines[i].split_whitespace().collect();
                    if info[0] == "dir" {
                        children.get_mut(&curr_dir).unwrap().insert(info[1].to_string());
                    } else {
                        let curr_file_size = file_size.get(&curr_dir).unwrap();
                        let size: i32 = info[0].parse().unwrap();
                        file_size.insert(curr_dir.clone(), curr_file_size + size);
                    }
                }
                i += 1;
            }
            listed.insert(curr_dir.to_string());
        }
        if debug {
            println!("Children: {:?}", children);
            println!("File sizes: {:?}", file_size);
        }
    }
    // compute total size of each directory, and output answers
    compute_total_size(file_size.clone(), children.clone());
}

fn compute_total_size(file_size: HashMap<String, i32>, children: HashMap<String, HashSet<String>>) {
    let debug = false;
    let mut total_file_size = file_size.clone();
    let mut stack: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    stack.push("/".to_string());
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if seen.contains(&node.clone()) {
            // calculate final file size of node by adding sizes of its directories to curr size
            let mut total = 0;
            for child in children.get(&node.clone()).unwrap() {
                if node == "/" {
                    total += total_file_size.get(&(node.clone() + child)).unwrap();
                } else {
                    total += total_file_size.get(&(node.clone() + "/" + child)).unwrap();
                }
            }
            let mut curr_size = total_file_size.get(&node).unwrap();
            total_file_size.insert(node, curr_size + total);
        } else {
            stack.push(node.clone());
            // add children to be traversed
            for child in children.get(&node.clone()).unwrap() {
                if node == "/" {
                    stack.push(node.clone() + child);
                } else {
                    stack.push(node.clone() + "/" + child);
                }
            }
            seen.insert(node.to_string());
        }
    }
    if debug {
        println!("Total file size: {:?}", total_file_size);
    }
    let root_dir_size = *total_file_size.get("/").unwrap();
    let target = root_dir_size - 40000000;
    let mut dir_sum = 0;
    let mut smallest_valid_dir = root_dir_size;
    for (_, value) in &total_file_size {
        if *value <= 100000 {
            dir_sum += *value;
        }
        if *value >= target && *value <= smallest_valid_dir {
            smallest_valid_dir = *value;
        }
    }
    println!("Sum of directories: {}", dir_sum);
    println!("Smallest directory to delete: {}", smallest_valid_dir);
}