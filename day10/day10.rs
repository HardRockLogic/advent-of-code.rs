use std::collections::HashMap;
use std::fs;

#[derive(Debug, Default)]
struct Signal {
    counter: i32,
    value: i32,
    map: HashMap<i32, i32>,
}

impl Signal {
    fn map_init(&mut self, vec: Vec<i32>) {
        for i in vec.iter() {
            self.map.insert(*i, 0);
        }
    }

    fn noop(&mut self) {
        self.check_signal();
        self.counter += 1;
    }

    fn addx(&mut self, val: i32) {
        self.check_signal();
        self.counter += 1;

        self.check_signal();
        self.counter += 1;
        self.value += val;
    }

    fn check_signal(&mut self) {
        if let Some(iteration) = self.map.get_mut(&self.counter) {
            *iteration = self.counter * self.value;
        }
    }

    fn total_signal_kill(self) -> i32 {
        let total: i32 = self.map.into_values().sum();

        total
    }
}

fn main() {
    let file = fs::read_to_string("day10.txt").unwrap();

    let mut signal = Signal {
        value: 1,
        counter: 1,
        ..Default::default()
    };

    let vec = vec![20, 60, 100, 140, 180, 220];
    signal.map_init(vec);

    for line in file.lines() {
        if line == "noop" {
            signal.noop();
        } else if &line[..4] == "addx" {
            let register_num = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            signal.addx(register_num);
        }
    }
    println!("Sum of signals is: {}", signal.total_signal_kill());
}
