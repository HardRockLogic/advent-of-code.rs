#![allow(dead_code, unused_imports)]

mod parse;

use nom::Finish;
use parse::{Name, Valve};
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
struct ProtoState {
    lable: Name,
    value: u32,
    time: u32,
}

#[derive(Debug, Default)]
struct DistMap {
    first: Name,
    data: HashMap<Name, (usize, u32, Vec<Name>)>,
    dist: Vec<Vec<u32>>,
}

impl DistMap {
    fn init(s: &str) -> Self {
        let mut dist = DistMap::default();

        for (i, line) in s.lines().enumerate() {
            let valve = Valve::scrap_valve(line).finish().unwrap().1;
            if i == 0 {
                dist.first = valve.name;
            }

            dist.data
                .insert(valve.name, (i, valve.flow, valve.adjecents));
        }

        dist
    }

    fn best_option_greedy(&self, valve: Name, time: u32) -> Vec<ProtoState> {
        let mut queue: VecDeque<(Name, u32)> = VecDeque::new();
        let mut enqueued: Vec<Name> = Vec::new();
        let mut options: Vec<ProtoState> = Vec::new();
        queue.push_back((valve, time));

        while !queue.is_empty() {
            let (name, elapsed) = queue.pop_front().unwrap();
            if elapsed <= 1 {
                break;
            }
            let node = self.data.get(&name).unwrap();
            if node.1 != 0 {
                options.push(ProtoState {
                    lable: name,
                    value: node.1 * (elapsed - 1),
                    time: elapsed - 1,
                });
            }
            node.2.iter().copied().for_each(|v| {
                if !enqueued.contains(&v) {
                    queue.push_back((v, elapsed - 1));
                    enqueued.push(v);
                }
            });
        }
        options
    }
}

#[derive(Debug)]
struct Valves {
    map: Vec<Valve>,
}

impl Valves {
    fn init(s: &str) -> Self {
        Self {
            map: s
                .lines()
                .map(|line| Valve::scrap_valve(line).finish().unwrap().1)
                .collect(),
        }
    }
}

fn main() {
    let file = fs::read_to_string("test.txt").unwrap();

    let map = DistMap::init(&file);
    dbg!(map.best_option_greedy(Name::from("AA"), 30));

    // for valve in &map.map {
    //     println!("{valve:?}");
    // }
}
