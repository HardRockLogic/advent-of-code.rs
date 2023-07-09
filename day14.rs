use std::{
    collections::{HashSet, VecDeque},
    fs, print, println, todo,
};

#[derive(Debug, Default)]
struct Points {
    storage: HashSet<[u32; 2]>,
    storage_copy: HashSet<[u32; 2]>,
    inter_queue: VecDeque<[u32; 2]>,
    sand: HashSet<[u32; 2]>,
    deepest_y: u32,
    prev: [u32; 2],
    in_rest: u32,
    left_x: u32,
    right_x: u32,
}

impl Points {
    fn build(source: &str) -> Self {
        let mut builded = Points::default();
        for line in source.lines() {
            for sub in line.split_whitespace().filter(|item| *item != "->") {
                let x = sub.split(",").nth(0).unwrap().parse::<u32>().unwrap();
                let y = sub.split(",").nth(1).unwrap().parse::<u32>().unwrap();

                builded.inter_queue.push_back([x, y]);
            }
            builded.interpolate();
        }
        let deepest = builded.storage.iter().max_by_key(|&item| item[1]).unwrap();
        let left_x = builded.storage.iter().min_by_key(|&item| item[0]).unwrap();
        let right_x = builded.storage.iter().max_by_key(|&item| item[0]).unwrap();

        builded.left_x = left_x[0];
        builded.right_x = right_x[0];
        builded.deepest_y = deepest[1];
        builded.storage_copy = builded.storage.clone();

        builded
    }

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

    fn simulate(&mut self) -> u32 {
        let mut unit = [500, 0];
        self.prev = unit;
        unit[1] += 1;

        loop {
            if unit[1] > self.deepest_y {
                println!("occurs {:?}", unit);
                return self.in_rest;
            }

            match self.storage.get(&unit) {
                Some(_) => {
                    unit[0] -= 1;

                    match self.storage.get(&unit) {
                        Some(_) => {
                            unit[0] += 2;

                            match self.storage.get(&unit) {
                                Some(_) => {
                                    self.in_rest += 1;
                                    self.storage.insert(self.prev);
                                    self.sand.insert(self.prev);
                                    unit = [500, 1];
                                    self.prev = [500, 0];
                                }
                                None => {
                                    self.prev = unit;
                                    unit[1] += 1;
                                }
                            }
                        }
                        None => {
                            self.prev = unit;
                            unit[1] += 1;
                        }
                    }
                }
                None => {
                    self.prev = unit;
                    unit[1] += 1;
                }
            }
        }
    }
}

fn fill(l: [u32; 2], r: [u32; 2]) -> std::ops::RangeInclusive<u32> {
    if l[0] == r[0] {
        let start = l[1].min(r[1]);
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

    let mut points = Points::build(&file);
    points.simulate();
    for i in 0..=173 {
        print!("{}) ", i);
        for j in 455..=526 {
            match points.storage_copy.get(&[j, i]) {
                Some(_) => print!("#"),
                None => match points.sand.get(&[j, i]) {
                    Some(_) => print!("o"),
                    None => print!("."),
                },
            }
        }
        println!();
    }
    dbg!(points.sand.len());
}
