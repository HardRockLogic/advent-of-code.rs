use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day4.txt").expect("no such file");
    let buf_rd = BufReader::new(file);

    let mut overlaps = 0;

    for line in buf_rd.lines() {
        let line_str = line.unwrap();

        let left = line_str
            .split(",")
            .nth(0)
            .unwrap()
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        let right = line_str
            .split(",")
            .nth(1)
            .unwrap()
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        if left[0] <= right[0] && left[1] >= right[1] {
            overlaps += 1;
        } else if right[0] <= left[0] && right[1] >= left[1] {
            overlaps += 1;
        }
    }
    println!("Total overlaps: {}", overlaps);
}
