use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug, Default)]
struct Points {
    storage: HashSet<[u32; 2]>,
    inter_queue: VecDeque<[u32; 2]>,
}

impl Points {
    fn interpolate(&mut self) {
        let mut start = self.inter_queue.pop_front().unwrap();

        for _ in 0..self.inter_queue.len() {
            let end = self.inter_queue.pop_front().unwrap();

            if start[0] == end[0] {
                for filament in fill(start, end) {
                    self.storage.insert([start[0], filament]);
                }
            } else if start[1] == end[1] {
                for filament in fill(start, end) {
                    self.storage.insert([filament, start[1]]);
                }
            }
            start = end;
        }
        self.inter_queue.clear();
    }
}

fn fill(l: [u32; 2], r: [u32; 2]) -> std::ops::RangeInclusive<u32> {
    if l[0] == r[0] {
        let start = l[1].min(l[1]);
        let end = l[1].max(r[1]);
        return start..=end;
    } else {
        let start = l[0].min(r[0]);
        let end = l[0].max(r[0]);
        return start..=end;
    }
}

fn main() {
    let file = fs::read_to_string("day14.txt").unwrap();

    let mut points = Points::default();

    for line in file.lines() {
        for sub in line.split_whitespace().filter(|item| *item != "->") {
            let x = sub.split(",").nth(0).unwrap().parse::<u32>().unwrap();
            let y = sub.split(",").nth(1).unwrap().parse::<u32>().unwrap();

            points.inter_queue.push_back([x, y]);
        }
        points.interpolate();
    }

    dbg!(points.storage.len());

    // points
    //     .storage
    //     .into_iter()
    //     .for_each(|coord| println!("{:?}", coord));
}
