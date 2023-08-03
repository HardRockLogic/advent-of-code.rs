use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day8.txt").expect("no such file");

    let buf_rd = BufReader::new(file);

    let mut arr: [[u8; 99]; 99] = [[0; 99]; 99];

    for (raw, line) in buf_rd.lines().enumerate() {
        let line_str = line.unwrap();

        for (cell, chr) in line_str.chars().enumerate() {
            let digit: u8 = chr.to_digit(10).unwrap().try_into().unwrap();

            arr[raw][cell] = digit;
        }
    }
    let mut total = 392; //99 * 4 - 4(corner elements)
    let mut top_scenic = 0;

    for i in 1..arr.len() - 1 {
        for j in 1..arr.len() - 1 {
            if check_neighbours(&arr, i, j) {
                total += 1;
            }

            let scenic = count_neighbours(&arr, i, j);

            if scenic > top_scenic {
                top_scenic = scenic;
            }
        }
    }
    println!("total visible: {}", total);
    println!("top scenic count: {}", top_scenic);
}

fn check_neighbours(arr: &[[u8; 99]; 99], i: usize, j: usize) -> bool {
    let to_check = arr[i][j];
    let mut occured = 0;

    // Check horizontally
    for left_raw in 0..j {
        if arr[i][left_raw] >= to_check {
            occured += 1;
            break;
        }
    }
    for right_raw in j + 1..99 {
        if arr[i][right_raw] >= to_check {
            occured += 1;
            break;
        }
    }
    // Check vertically
    for up_cell in 0..i {
        if arr[up_cell][j] >= to_check {
            occured += 1;
            break;
        }
    }
    for down_cell in i + 1..99 {
        if arr[down_cell][j] >= to_check {
            occured += 1;
            break;
        }
    }
    if occured < 4 {
        true
    } else {
        false
    }
}

fn count_neighbours(arr: &[[u8; 99]; 99], i: usize, j: usize) -> u32 {
    let to_check = arr[i][j];
    let mut left: u32 = 0;
    let mut right: u32 = 0;
    let mut up: u32 = 0;
    let mut down: u32 = 0;

    // Check horizontally
    for left_raw in (0..j).rev() {
        if arr[i][left_raw] >= to_check {
            left += 1;
            break;
        } else {
            left += 1;
        }
    }
    for right_raw in j + 1..99 {
        if arr[i][right_raw] >= to_check {
            right += 1;
            break;
        } else {
            right += 1;
        }
    }
    // Check vertically
    for up_cell in (0..i).rev() {
        if arr[up_cell][j] >= to_check {
            up += 1;
            break;
        } else {
            up += 1;
        }
    }
    for down_cell in i + 1..99 {
        if arr[down_cell][j] >= to_check {
            down += 1;
            break;
        } else {
            down += 1;
        }
    }
    left * right * up * down
}
