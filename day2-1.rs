use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut outcomes = HashMap::new();

    outcomes.insert(("A", "X"), 3); // l -> scissors 3
    outcomes.insert(("B", "Y"), 5); // t -> paper 2
    outcomes.insert(("C", "Z"), 7); // w -> rock 1

    outcomes.insert(("A", "Y"), 4); // t -> rock 1
    outcomes.insert(("B", "Z"), 9); // w -> scissors 3
    outcomes.insert(("C", "X"), 2); // l -> paper 2

    outcomes.insert(("A", "Z"), 8); // w -> paper 2
    outcomes.insert(("B", "X"), 1); // l -> rock 1
    outcomes.insert(("C", "Y"), 6); // t -> scrissors 3

    let file = File::open("rps.txt").expect("file does not exists");
    let buf_rd = BufReader::new(file);

    let mut total = 0;

    for line in buf_rd.lines() {
        let line_str = line.unwrap();
        let tempo = line_str.split_whitespace().collect::<Vec<&str>>();

        if let Some(game) = outcomes.get(&(tempo[0], tempo[1])) {
            total += game;
        }
    }
    println!("{total}");
}
