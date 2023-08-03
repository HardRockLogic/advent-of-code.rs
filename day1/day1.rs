use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("elf_ccal.txt").expect("no such file");
    let buff_rd = BufReader::new(file);

    top_three(buff_rd);
}

fn top_three(handle: BufReader<File>) {
    let mut total: Vec<i32> = Vec::new();
    let mut group_summ = 0;
    //let mut max = [0, 0]; // [0] is index; [1] is summ;
    //let mut ind = 0;

    for line in handle.lines() {
        let line_str = line.unwrap();

        if line_str.is_empty() {
            total.push(group_summ);
            group_summ = 0;
        } else {
            group_summ += line_str.parse::<i32>().expect("parse error");
        }
    }

    total.sort();
    total.reverse();
    let max = total[0] + total[1] + total[2];

    println!("Top 3 is: {:?}", max);
}
