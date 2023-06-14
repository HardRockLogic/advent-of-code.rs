use std::collections::HashSet;
use std::fs;

const ARR_SIZE: usize = 14;

fn main() {
    let input = fs::read_to_string("day6.txt").unwrap();
    let mut frame: [char; ARR_SIZE] = [' '; ARR_SIZE];
    let mut count = 0;

    for chr in input.chars() {
        count += 1;

        push_inline(&mut frame, &chr);

        if unique(&frame) {
            break;
        }
    }
    frame.reverse();
    println!("{:?}", frame);
    println!("unique at: {count}");
}

fn unique<T: Eq + std::hash::Hash>(seq: &[T]) -> bool {
    let set: HashSet<_> = seq.iter().collect();

    set.len() == seq.len()
}

fn push_inline(arr: &mut [char], target: &char) {
    for i in (1..ARR_SIZE).rev() {
        arr[i] = arr[i - 1];
    }
    arr[0] = *target;
}
