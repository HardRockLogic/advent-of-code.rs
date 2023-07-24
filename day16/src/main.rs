#![allow(dead_code, unused_imports)]

mod parse;

use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};

use nom::Finish;
use parse::{Name, Valve};
use std::{
    collections::{HashMap, HashSet, VecDeque},
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
// impl PartialEq for State {
//     fn eq(&self, other: &Self) -> bool {
//         self.opened == other.opened
//     }
// }

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

    fn brute_search(&self) -> (Vec<(f32, f32)>, Vec<(f32, f32)>) {
        let mut queue = VecDeque::with_capacity(195_000);
        let mut highest: u32 = 0;
        queue.push_back(State {
            lable: Name::from("AA"),
            value: 0,
            time: 26,
            opened: vec![],
        });

        // Variables for statistics
        let mut points: Vec<(f32, f32)> = Vec::new();
        let mut empty_returns: Vec<(f32, f32)> = Vec::new();
        let mut count_x = 0.0;
        let mut count_y = 0.0;
        #[allow(unused_assignments)]
        let mut count = 0;
        let mut empty = 0.;
        let mut switch = true;

        while !queue.is_empty() {
            count = 0;
            count_x += 1.0;
            let state = queue.pop_front().unwrap();
            self.best_option_greedy(state)
                .into_iter()
                .for_each(|sub_state| {
                    count_y += 1.0;
                    if sub_state.value > highest {
                        highest = sub_state.value;
                    }
                    count += 1;

                    queue.push_back(sub_state);
                });
            points.push((count_x, count_y));
            if count == 0 {
                empty += 1.;
                switch = true;
            } else if switch {
                empty_returns.push((count_x, empty));
                empty = 0.;
                switch = false
            }
        }
        dbg!(highest);
        (points, empty_returns)
    }
}

fn main() {
    let file = fs::read_to_string("day16.txt").unwrap();

    let map = DistMap::init(&file);

    let (points, empties) = map.brute_search();
    // chart-width, chart-height, dataset-start, dataset-end
    // 280, 90, 0.0, 190_000.0
    Chart::new(280, 80, 0.0, 50_000.0)
        .linecolorplot(
            &Shape::Lines(&points),
            RGB8 {
                r: 0,
                g: 0,
                b: 255_u8,
            },
        )
        .display();
    Chart::new(280, 80, 0.0, 40_000.0)
        .linecolorplot(
            &Shape::Lines(&empties),
            RGB8 {
                r: 255_u8,
                g: 0,
                b: 0,
            },
        )
        .display();
}
