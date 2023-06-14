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

    let mut iter = buff_rd.lines();

    loop {
        let first: HashSet<char> = match iter.next() {
            Some(Ok(line)) => line.chars().collect::<HashSet<char>>(),
            _ => break,
        };
        let sec: HashSet<char> = match iter.next() {
            Some(Ok(line)) => line.chars().collect::<HashSet<char>>(),
            _ => break,
        };
        let third: HashSet<char> = match iter.next() {
            Some(Ok(line)) => line.chars().collect::<HashSet<char>>(),
            _ => break,
        };

        for chr in first.iter() {
            if sec.contains(chr) && third.contains(chr) {
                total += char_map.get(chr).unwrap();
            }
        }
    }

    println!("Total is: {total}");
}
