use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();

    map.insert(1, vec!['F', 'C', 'J', 'P', 'H', 'T', 'W']);
    map.insert(2, vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H']);
    map.insert(3, vec!['H', 'P', 'T', 'R']);
    map.insert(4, vec!['Z', 'S', 'N', 'P', 'H', 'T']);
    map.insert(5, vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D']);
    map.insert(6, vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z']);
    map.insert(7, vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P']);
    map.insert(8, vec!['N', 'D', 'S']);
    map.insert(9, vec!['D', 'Z', 'S', 'F', 'M']);

    let file = File::open("day5.txt").expect("no such file");
    let buf_rd = BufReader::new(file);

    for line in buf_rd.lines() {
        let line_str = line.unwrap();

        let amount: i32 = line_str.split_whitespace().nth(1).unwrap().parse().unwrap();
        let from: i32 = line_str.split_whitespace().nth(3).unwrap().parse().unwrap();
        let to: i32 = line_str.split_whitespace().nth(5).unwrap().parse().unwrap();

        let stack_from: &mut Vec<char> = map.get_mut(&from).unwrap();
        let i: usize = stack_from.len() - amount as usize;

        // part 2 of the puzzle requires just removing .rev() method
        let mut tempo_vec: Vec<_> = stack_from.drain(i..).rev().collect();

        let stack_to: &mut Vec<char> = map.get_mut(&to).unwrap();

        stack_to.append(&mut tempo_vec);
    }

    for i in 1..=9 {
        let vex = map.get(&i).unwrap().last().unwrap();

        println!("{:?}", vex);
    }
}
