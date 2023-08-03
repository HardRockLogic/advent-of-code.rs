use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut outcomes = HashMap::new();
    // Tie
    outcomes.insert(("A", "X"), 4);
    outcomes.insert(("B", "Y"), 5);
    outcomes.insert(("C", "Z"), 6);
    // Win
    outcomes.insert(("A", "Y"), 8);
    outcomes.insert(("B", "Z"), 9);
    outcomes.insert(("C", "X"), 7);
    // loss
    outcomes.insert(("A", "Z"), 3);
    outcomes.insert(("B", "X"), 1);
    outcomes.insert(("C", "Y"), 2);

    println!("{:?}", outcomes);

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
