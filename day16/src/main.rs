#![allow(dead_code, unused_imports)]

mod parse;

use nom::Finish;
use parse::{Name, Valve};
use std::{
    collections::{HashMap, VecDeque},
    fs,
    iter::once,
};

#[derive(Debug)]
struct State {
    lable: Name,
    value: u32,
    time: u32,
    history: Vec<Name>,
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
    // struct State {
    //     lable: Name,
    //     value: u32,
    //     time: u32,
    //     history: Vec<Name>,
    // }
    fn best_option_greedy(&self, state: State) -> Vec<State> {
        let mut queue: VecDeque<(Name, u32)> = VecDeque::new();
        let mut enqueued: Vec<Name> = Vec::new();
        let mut options: Vec<State> = Vec::new();
        queue.push_back((state.lable, state.time));

        while !queue.is_empty() {
            let (name, elapsed) = queue.pop_front().unwrap();

            if elapsed <= 1 {
                break;
            }

            let node = self.data.get(&name).unwrap();

            if node.1 != 0 && !state.history.contains(&name) {
                options.push(State {
                    lable: name,
                    value: state.value + node.1 * (elapsed - 1),
                    time: elapsed - 1,
                    history: state.history.iter().copied().chain(once(name)).collect(),
                });
            }
            node.2.iter().for_each(|v| {
                if !enqueued.contains(&v) {
                    queue.push_back((*v, elapsed - 1));
                    enqueued.push(*v);
                }
            });
        }
        options
    }
}

// #[derive(Debug)]
// struct Valves {
//     map: Vec<Valve>,
// }
//
// impl Valves {
//     fn init(s: &str) -> Self {
//         Self {
//             map: s
//                 .lines()
//                 .map(|line| Valve::scrap_valve(line).finish().unwrap().1)
//                 .collect(),
//         }
//     }
// }

fn main() {
    let file = fs::read_to_string("test.txt").unwrap();

    let map = DistMap::init(&file);
    let state = State {
        lable: Name::from("EE"),
        value: 1639,
        time: 9,
        history: vec![
            Name::from("DD"),
            Name::from("BB"),
            Name::from("JJ"),
            Name::from("HH"),
            Name::from("EE"),
        ],
    };

    dbg!(map.best_option_greedy(state));

    // for valve in &map.map {
    //     println!("{valve:?}");
    // }
}
