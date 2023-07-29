#![allow(dead_code, unused_imports)]
use std::{dbg, fs};

enum Shapes {
    Horizontal,
    Plus,
    ReversedL,
    Vertical,
    Cube,
}

fn main() {
    let line = fs::read_to_string("test.txt").unwrap();

    line.strip_suffix("\n")
        .unwrap()
        .chars()
        .for_each(|l| match l {
            '<' => println!("left jet"),
            '>' => println!("right jet"),
            _ => unreachable!(),
        })
}
