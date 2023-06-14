//lazy way of puzzle solutions without recreateing actual tree

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::PathBuf;

fn main() {
    let mut paths_sizes = HashMap::new();
    let mut path_state = Vec::new();

    let file = File::open("day7.txt").expect("no such file");
    let bur_rd = BufReader::new(file);

    for line in bur_rd.lines() {
        let term_out = line.unwrap();

        if term_out.starts_with("$ ls") || term_out.starts_with("dir") {
            continue;
        }

        let components: Vec<_> = term_out.split_whitespace().collect();

        match components[..] {
            ["$", "cd", ".."] => {
                path_state.pop();
            }
            ["$", "cd", dir] => {
                path_state.push(dir.to_string());
            }
            [file_size, _] => {
                let size: u32 = file_size.parse().unwrap();
                for i in 0..path_state.len() {
                    let path = PathBuf::from_iter(&path_state[0..=i]);
                    *paths_sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    //println!("{:?}", paths_sizes);
    let output: u32 = paths_sizes.into_values().filter(|x| *x <= 100_000).sum();

    println!("{output}");
}
