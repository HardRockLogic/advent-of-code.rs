use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Default)]
struct Signal {
    counter: i32,
    value: i32,
    pixel: usize,
    current_bound: i32,
    crt_raws: [i32; 6],
    map: BTreeMap<i32, [char; 40]>,
}
impl Signal {
    fn map_init(&mut self, arr: [i32; 6]) {
        for i in arr.iter() {
            self.map.insert(*i, [' '; 40]);
        }
        self.crt_raws = arr;
    }
    fn noop(&mut self) {
        self.check_signal();
        self.counter += 1;
        self.pixel += 1;
    }
    fn addx(&mut self, val: i32) {
        self.check_signal();
        self.counter += 1;
        self.pixel += 1;

        self.check_signal();
        self.counter += 1;
        self.pixel += 1;
        self.value += val;
    }
    fn return_bound(&self) -> i32 {
        for bound in self.crt_raws.iter() {
            if bound >= &self.counter {
                return *bound;
            }
        }
        panic!("more iterations than expected");
    }
    fn check_signal(&mut self) {
        let bound = self.return_bound();

        if self.current_bound != bound {
            self.pixel = 0;
            self.current_bound = bound;
        }

        let lit: char = '@';

        if let Some(line) = self.map.get_mut(&bound) {
            if self.pixel == self.value as usize {
                line[self.pixel] = lit;
            } else if self.pixel == (self.value - 1) as usize {
                line[self.pixel] = lit;
            } else if self.pixel == (self.value + 1) as usize {
                line[self.pixel] = lit;
            } else {
                line[self.pixel] = '.'
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("day10.txt").unwrap();

    let mut signal = Signal {
        value: 1,
        counter: 1,
        ..Default::default()
    };

    let arr = [40, 80, 120, 160, 200, 240];
    signal.map_init(arr);

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
    for val in signal.map.values() {
        let output = val.iter().collect::<String>();
        println!("{output}");
    }
}
