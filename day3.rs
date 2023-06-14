use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut char_map: HashMap<char, u32> = HashMap::new();
    let mut value = 1;

    for ch in 'a'..='z' {
        char_map.insert(ch, value);
        value += 1;
    }
    for ch in 'A'..='Z' {
        char_map.insert(ch, value);
        value += 1;
    }

    let file = File::open("day3.txt").expect("no such file");
    let buff_rd = BufReader::new(file);

    let mut total: u32 = 0;

    for line in buff_rd.lines() {
        let line_str = line.unwrap();

        let ln = line_str.len() / 2;
        let left = &line_str[..ln];
        let right = &line_str[ln..];

        let lft_vec = left.chars().collect::<HashSet<char>>();
        let rght_vec = right.chars().collect::<HashSet<char>>();

        for chr in lft_vec.iter() {
            if rght_vec.contains(chr) {
                total += char_map.get(chr).unwrap();
            }
        }
    }
    println!("Total is: {total}");
}
