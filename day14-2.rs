use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn from(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn get_full_range(&self, other: Self) -> std::ops::RangeInclusive<u32> {
        if self.x == other.x {
            let start = self.y.min(other.y);
            let end = self.y.max(other.y);
            return start..=end;
        } else {
            let start = self.x.min(other.x);
            let end = self.x.max(other.x);
            return start..=end;
        }
    }
}

#[derive(Debug, Default)]
struct Points {
    storage: HashSet<Coord>,
    storage_copy: HashSet<Coord>,
    inter_queue: VecDeque<Coord>,
    sand: HashSet<Coord>,
    deepest_y: u32,
    prev: Coord,
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

                builded.inter_queue.push_back(Coord::from(x, y));
            }
            builded.interpolate();
        }
        let deepest = builded.storage.iter().max_by_key(|&item| item.y).unwrap();
        let left_x = builded.storage.iter().min_by_key(|&item| item.x).unwrap();
        let right_x = builded.storage.iter().max_by_key(|&item| item.x).unwrap();

        builded.left_x = left_x.x;
        builded.right_x = right_x.x;
        builded.deepest_y = deepest.y;
        builded.storage_copy = builded.storage.clone();

        builded
    }

    fn interpolate(&mut self) {
        let mut start = self.inter_queue.pop_front().unwrap();

        for _ in 0..self.inter_queue.len() {
            let end = self.inter_queue.pop_front().unwrap();

            if start.x == end.x {
                for filament in start.get_full_range(end) {
                    self.storage.insert(Coord::from(start.x, filament));
                }
            } else if start.y == end.y {
                for filament in start.get_full_range(end) {
                    self.storage.insert(Coord::from(filament, start.y));
                }
            }
            start = end;
        }
        self.inter_queue.clear();
    }

    fn simulate(&mut self) -> u32 {
        let mut unit = Coord::from(500, 0);

        loop {
            if unit.y == self.deepest_y + 2 {
                self.in_rest += 1;
                self.storage.insert(self.prev);
                self.sand.insert(self.prev);
                unit = Coord::from(500, 1);
                self.prev = Coord::from(500, 0);
            }

            match self.storage.get(&unit) {
                Some(_) => {
                    unit.x -= 1;

                    match self.storage.get(&unit) {
                        Some(_) => {
                            unit.x += 2;

                            match self.storage.get(&unit) {
                                Some(_) => {
                                    self.in_rest += 1;
                                    self.storage.insert(self.prev);
                                    self.sand.insert(self.prev);
                                    if self.prev == Coord::from(500, 0) {
                                        return self.in_rest;
                                    }
                                    unit = Coord::from(500, 1);
                                    self.prev = Coord::from(500, 0);
                                }
                                None => {
                                    self.prev = unit;
                                    unit.y += 1;
                                }
                            }
                        }
                        None => {
                            self.prev = unit;
                            unit.y += 1;
                        }
                    }
                }
                None => {
                    self.prev = unit;
                    unit.y += 1;
                }
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("day14.txt").unwrap();

    let mut points = Points::build(&file);

    points.simulate();

    for i in 0..=173 {
        print!("{}) ", i);
        for j in 420..=580 {
            match points.storage_copy.get(&Coord::from(j, i)) {
                Some(_) => print!("\x1b[32;1m{}\x1b[0m", "#"),
                None => match points.sand.get(&Coord::from(j, i)) {
                    Some(_) => print!("o"),
                    None => print!("."),
                },
            }
        }
        println!();
    }
    dbg!(points.sand.len());
}
