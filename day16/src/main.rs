#![allow(dead_code, unused_imports)]

mod parse;

use nom::Finish;
use parse::{Name, Valve};
use std::{
    collections::{HashMap, VecDeque},
    dbg, fs,
    iter::once,
};

#[derive(Debug)]
struct State {
    lable: Name,
    value: u32,
    time: u32,
    opened: Vec<Name>,
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

            if node.1 != 0 && !state.opened.contains(&name) {
                options.push(State {
                    lable: name,
                    value: state.value + node.1 * (elapsed - 1),
                    time: elapsed - 1,
                    opened: state.opened.iter().copied().chain(once(name)).collect(),
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

    fn brute_search(&self) {
        let mut queue = VecDeque::new();
        let mut highest: u32 = 0;
        queue.push_back(State {
            lable: Name::from("AA"),
            value: 0,
            time: 30,
            opened: vec![],
        });

        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            self.best_option_greedy(state)
                .into_iter()
                .for_each(|sub_state| {
                    if sub_state.value > highest {
                        highest = sub_state.value;
                    }
                    queue.push_back(sub_state)
                });
        }
        dbg!(highest);
    }
}

fn main() {
    let file = fs::read_to_string("day16.txt").unwrap();

    let map = DistMap::init(&file);
    // let state = State {
    //     lable: Name::from("EE"),
    //     value: 1639,
    //     time: 9,
    //     opened: vec![
    //         Name::from("DD"),
    //         Name::from("BB"),
    //         Name::from("JJ"),
    //         Name::from("HH"),
    //         Name::from("EE"),
    //     ],
    // };
    map.brute_search();

    // dbg!(map.best_option_greedy(state));
}
