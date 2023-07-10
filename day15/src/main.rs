mod parse;
use std::dbg;

use nom::Finish;
use parse::parse_cave;

fn main() {
    let file = std::fs::read_to_string("test.txt").unwrap();

    let res = parse_cave(&file).finish().unwrap().1;

    dbg!(res);
}
