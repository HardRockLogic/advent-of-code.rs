mod parse;

use nom::Finish;
use std::{fs, println};

fn main() {
    let file = fs::read_to_string("test.txt").unwrap();

    let valves: Vec<_> = file
        .lines()
        .map(|line| parse::Valve::scrap_valve(line).finish().unwrap().1)
        .collect();

    for valve in &valves {
        println!("{valve:?}");
    }
}
